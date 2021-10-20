use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use anchor_spl::token::{self, Transfer, Burn, Mint, MintTo, TokenAccount};
use depository::Depository as DepositoryOld;
use pyth_client::Price;
use std::convert::TryFrom;

mod mango_program;

const UXD_DECIMAL: u8 = 6;

const STATE_SEED: &[u8] = b"STATE";
const UXD_SEED: &[u8] = b"STABLECOIN";
const DEPOSITORY_SEED: &[u8] = b"DEPOSITORY";
const RECORD_SEED: &[u8] = b"RECORD";
const PASSTHROUGH_SEED: &[u8] = b"PASSTHROUGH";

solana_program::declare_id!("2PCPrsHdeZq6CsHyqnu3NVMcWtJGjZE8mWKpF6ipTDT4");

#[program]
#[deny(unused_must_use)]
pub mod controller {
    use super::*;

    // MANGO API IN BRIEF
    // shit we care about:
    // * init account (good for a whole group aka a set of markets that can be xmargined)
    // * deposit (coin into account)
    // * withdraw (coin from account)
    // * place perp order (self explanatory. order type comes from serum i think)
    // * cancel perp order (their id and our id versions exist)
    // * settle pnl (takes two accounts and trues up)
    // settle is necessary but kinda weird in that like, you need to find a loser to match your winner
    //
    // shit we might:
    // * add to basket ("add a spot market to account basket" never made clear wtf this is)
    // * borrow (unclear if we need to borrow to short? prolly not...)
    // * place spot order (this is just a serum passthrough)
    // * cancel spot order (as above)
    // * settle funds ("settle funds from serum dex open orders" maybe just serum passthrough?)
    // * settle borrow (only if we use borrow
    // the point of serum calls is they can use the money in mango accounts
    // but... i dont think we need to mess w spot
    //
    // flow... user deposits btc, we send to mango
    // open a equiv sized short position sans whatever amount for liquidation protection
    // once the position is open it theoretically has a fix dollar value
    // (sans execution risk, sans funding, sans liquidation buffer)
    // this is the amount of uxd we mint and return to the user
    // then redemption of uxd for the underlying means we... burn uxd
    // close out an equivalent amount of position in the coin they want
    // settle pnl, withdraw coin, deliver to depository, give user redeemables
    // important that all trasaction costs and price differences *must* be passed onto the user
    // otherwise we open ourselves up to all kind of insane arbitrage attacks
    // since uxd *must* be fungible we cannot maintain accounts for individuals
    //
    // oook so... mint has to go like. for a particular depository...
    // we accept redeemable, proxy transfer coin to us, move coin onto mango (deposit)
    // create an opposite position on mango (place perp order). and then give uxd to user
    // for now we take fro granted that all deposited coins have a corresponding perp
    // if we want to take more esoteric forms of capital we may need to swap on serum
    //
    // im not sure controller should create uxd... idk what if we redeploy to a new address?
    // we should have liek... a function new, to set up the controller with state and owner
    // and a function register depository to whitelist a depository address
    // and create the mango account and such

    /////// Instruction functions ///////

    // NEW
    // create controller state, create uxd (this could happen elsewhere later)
    // the key we pass in as authority *must* be retained/protected to add depositories
    pub fn new(ctx: Context<New>) -> ProgramResult {
        msg!("controller: new");

        // - Update state
        let state_nonce = Pubkey::find_program_address(&[STATE_SEED], ctx.program_id).1;
        ctx.accounts.state.bump = state_nonce;
        ctx.accounts.state.authority_key = ctx.accounts.authority.key();
        ctx.accounts.state.uxd_mint_key = ctx.accounts.uxd_mint.key();

        Ok(())
    }

    // REGISTER DEPOSITORY
    // authority must sign and match authority in our initial state
    // create a mango account for the coin, create an entry indicating we created and trust this depository
    // create a passthrough account for whatever coin corresponds to this depository
    // we need this because the owner of the mango account and the token account must be the same
    // so we cant move funds directly from the user to mango
    pub fn register_depository(
        ctx: Context<RegisterDepository>,
        oracle_key: Pubkey,
    ) -> ProgramResult {
        msg!("controller: register depository");

        let coin_mint_key = ctx.accounts.coin_mint.key();

        // - Initialize Mango Account

        let depository_bump = Pubkey::find_program_address(&[DEPOSITORY_SEED, coin_mint_key.as_ref()], ctx.program_id).1;
        let depository_signer_seed: &[&[&[u8]]] = &[&[
            DEPOSITORY_SEED,
            coin_mint_key.as_ref(),
            &[depository_bump],
        ]];

        let instruction = solana_program::instruction::Instruction {
            program_id: ctx.accounts.mango_program.key(),
            data: mango::instruction::MangoInstruction::InitMangoAccount.pack(),
            accounts: vec![
                AccountMeta::new_readonly(ctx.accounts.mango_group.key(), false),
                AccountMeta::new(ctx.accounts.mango_account.key(), false),
                AccountMeta::new_readonly(ctx.accounts.depository.key(), true),
            ],
        };

        let account_infos = [
            ctx.accounts.mango_program.to_account_info(),
            ctx.accounts.mango_group.to_account_info(),
            ctx.accounts.mango_account.to_account_info(),
            ctx.accounts.depository.to_account_info(),
        ];

        solana_program::program::invoke_signed(
            &instruction,
            &account_infos,
            depository_signer_seed,
        )?;

        // - Set our depo record up
        // this later acts as proof we trust a given depository
        // we also use this to derive the depository state key, from which we get mint and account keys
        // creating a hierarchy of trust rooted at the authority key that instantiated the controller
        ctx.accounts.depository.bump = depository_bump;
        ctx.accounts.depository.coin_mint_key = coin_mint_key;
        ctx.accounts.depository.coin_passthrough_key = ctx.accounts.coin_passthrough.key();
        ctx.accounts.depository.mango_account_key = ctx.accounts.mango_account.key();
        ctx.accounts.depository.oracle_key = oracle_key;

        Ok(())
    }

    // MINT UXD
    // transfer user coin to our passthrough. open a mango position with that
    // then mint uxd in the amount of the mango position to the user
    pub fn mint_uxd(ctx: Context<MintUxd>, coin_amount: u64) -> ProgramResult {
        msg!("controller: mint uxd");

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_coin.to_account_info(),
                to: ctx.accounts.coin_passthrough.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, coin_amount)?;

        // - Deposit to mango and open position
        let coin_mint_key = ctx.accounts.coin_mint.key();
        let depository_bump =
            Pubkey::find_program_address(&[DEPOSITORY_SEED, coin_mint_key.as_ref()], ctx.program_id).1;
        let depository_signer_seed: &[&[&[u8]]] = &[&[
            DEPOSITORY_SEED,
            coin_mint_key.as_ref(),
            &[depository_bump],
        ]];
        let instruction = solana_program::instruction::Instruction {
            program_id: ctx.accounts.mango_program.key(),
            data: mango::instruction::MangoInstruction::Deposit {
                quantity: coin_amount,
            }
            .pack(),
            accounts: vec![
                AccountMeta::new_readonly(ctx.accounts.mango_group.key(), false),
                AccountMeta::new(ctx.accounts.mango_account.key(), false),
                AccountMeta::new_readonly(ctx.accounts.depository.key(), true),
                AccountMeta::new_readonly(ctx.accounts.mango_cache.key(), false),
                AccountMeta::new_readonly(ctx.accounts.mango_root_bank.key(), false),
                AccountMeta::new(ctx.accounts.mango_node_bank.key(), false),
                AccountMeta::new(ctx.accounts.mango_vault.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                AccountMeta::new(ctx.accounts.coin_passthrough.key(), false),
            ],
        };

        let account_infos = [
            ctx.accounts.mango_program.to_account_info(),
            ctx.accounts.mango_group.to_account_info(),
            ctx.accounts.mango_account.to_account_info(),
            ctx.accounts.depository.to_account_info(),
            ctx.accounts.mango_cache.to_account_info(),
            ctx.accounts.mango_root_bank.to_account_info(),
            ctx.accounts.mango_node_bank.to_account_info(),
            ctx.accounts.mango_vault.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.coin_passthrough.to_account_info(),
        ];

        solana_program::program::invoke_signed(
            &instruction,
            &account_infos,
            depository_signer_seed,
        )?;
        // /////////

        // XXX temporary hack, we use the registered oracle to get a coin price
        let oracle_data = ctx.accounts.oracle.try_borrow_data()?;
        let oracle = pyth_client::cast::<Price>(&oracle_data);

        if oracle.agg.price < 0 {
            panic!("ugh return an error here or check this in constraints");
        }

        // so we take the amount of coin, multiply by price
        // then divide out the price decimals. we are now in coin decimals
        // so we multiply by uxd decimals and divide by coin decimals and get a uxd amount
        // XXX replace unwrap with error when we have custom errors
        let position_uxd_value = (coin_amount as u128)
            .checked_mul(oracle.agg.price.abs() as u128)
            .and_then(|n| n.checked_div(u128::pow(10, oracle.expo.abs() as u32)))
            .and_then(|n| n.checked_mul(u128::pow(10, ctx.accounts.uxd_mint.decimals as u32)))
            .and_then(|n| n.checked_div(u128::pow(10, ctx.accounts.coin_mint.decimals as u32)))
            .and_then(|n| u64::try_from(n).ok())
            .unwrap();

        // - Mint UXD for redeemables
        let state_signer_seed: &[&[&[u8]]] = &[&[STATE_SEED, &[ctx.accounts.state.bump]]];
        token::mint_to(
            ctx.accounts
                .into_mint_uxd_context()
                .with_signer(state_signer_seed),
            position_uxd_value,
        )?;

        Ok(())
    }

    // REDEEM UXD
    // burn uxd that is being redeemed. then close out mango position and return coins to depository
    // minting redeemables for the user in the process
    pub fn redeem_uxd(ctx: Context<RedeemUxd>, uxd_amount: u64) -> ProgramResult {
        msg!("controller: redeem uxd");

        // - First burn the uxd theyre giving up
        token::burn(ctx.accounts.into_burn_uxd_context(), uxd_amount)?;

        // - Mango close positon and withdraw coin TODO
        // get current passthrough balance before withdrawing from mango
        // in theory this should always be zero but better safe
        let _passthrough_balance = ctx.accounts.coin_passthrough.amount;

        // XXX TODO FIXME in theory we get a uxd amount, close out that much position, and withdraw whatever collateral results
        // and then return to the user whatever the passthrough difference is (altho it should normally be 0 balance)
        //let collateral_size = ctx.accounts.coin_passthrough.amount - passthrough_balance;

        // XXX but we are dumb and not integrated iwth mango yet so
        let oracle_data = ctx.accounts.oracle.try_borrow_data()?;
        let oracle = pyth_client::cast::<Price>(&oracle_data);

        if oracle.agg.price < 0 {
            panic!("ugh return an error here or check this in constraints");
        }

        // here we take the amount of uxd, multiply by price decimal
        // then divide by price, multiply by coin decimal, divide by uxd decimal to get a coin amount
        // XXX replace unwrap with error when we have custom errors
        let collateral_amount = (uxd_amount as u128)
            .checked_mul(u128::pow(10, oracle.expo.abs() as u32))
            .and_then(|n| n.checked_div(oracle.agg.price.abs() as u128))
            .and_then(|n| n.checked_mul(u128::pow(10, ctx.accounts.coin_mint.decimals as u32)))
            .and_then(|n| n.checked_div(u128::pow(10, ctx.accounts.uxd_mint.decimals as u32)))
            .and_then(|n| u64::try_from(n).ok())
            .unwrap();

        // - Return mango money back to depository
        let coin_mint_key = ctx.accounts.coin_mint.key();
        let record_signer_seed: &[&[&[u8]]] = &[&[
            RECORD_SEED,
            coin_mint_key.as_ref(),
            &[ctx.accounts.depository_record.bump],
        ]];
        depository::cpi::deposit(
            ctx.accounts
                .into_return_collateral_context()
                .with_signer(record_signer_seed),
            collateral_amount,
        )?;

        Ok(())
    }

    // pub fn rebalance(ctx: Context<Rebalance>) -> ProgramResult {
    //     // validate caller is in rebalance signer(s)
    //     // WARNING DIFFICULT LOGIC
    //     // rebalance needs borrow/lending rate, outstanding pnl balance in an array across collateral types
    //     // probably better for it to just call by depository/collateral type for now,
    //     // since we're going for the single collateral version first
    //     // estimates rebalance cost eg transaction fees
    //     // uses some settable estimation constant (e?) for what the timescale to consider
    //     // if borrow * e * net pnl > est rebalance cost then rebal should go ahead
    //     // rebal for single collateral just amounts to settling some or all of the pnl and rehedging
    //     // for multi collateral there are two versions,
    //     // 1. that single collat balances in parallel for n depositories
    //         // could be a public function
    //     // 2. that optimizes for market rates across range of collateral types
    //         // will change portfolio balances in order to get the highest return on the basis trade
    //         // weighted array of parameters like liquidity, mkt cap, stability
    //         // Not a priority
    //
    // }
    //
}

// MARK: - Accounts Inputs  ---------------------------------------------------
#[derive(Accounts)]
pub struct New<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        seeds = [STATE_SEED],
        bump,
        payer = authority,
    )]
    pub state: Box<Account<'info, State>>,
    #[account(
        init,
        seeds = [UXD_SEED],
        bump,
        mint::authority = state,
        mint::decimals = UXD_DECIMAL,
        payer = authority,
    )]
    pub uxd_mint: Account<'info, Mint>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RegisterDepository<'info> {
    #[account(mut, constraint = authority.key() == state.authority_key)]
    pub authority: Signer<'info>,
    #[account(seeds = [STATE_SEED], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(
        init,
        seeds = [DEPOSITORY_SEED, coin_mint.key().as_ref()],
        bump,
        payer = authority,
    )]
    pub depository: Box<Account<'info, DepositoryNew>>,
    pub coin_mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        seeds = [PASSTHROUGH_SEED, coin_mint.key().as_ref()],
        bump,
        token::mint = coin_mint,
        token::authority = depository,
        payer = authority,
    )]
    pub coin_passthrough: Account<'info, TokenAccount>,
    // The mango group for the mango_account
    pub mango_group: AccountInfo<'info>,
    // The mango account, uninitialized
    #[account(mut, signer)]
    pub mango_account: AccountInfo<'info>,
    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub mango_program: Program<'info, mango_program::Mango>,
    // sysvar
    pub rent: Sysvar<'info, Rent>,
}

// XXX oki this shit is complicated lets see what all is here...
// basically what we do is take redeemables from the user, take coin from depository
// send coin to mango, open position, mint uxd to user
// XXX gahh this means we need our own redeemable account too...
// this is troublesome... hmm we could theoretically uhh...
// * user gives mint 1 btc-redeemable
// * we call proxy transfer which *burns* the redeemable and sends *us* 1 btc
// * we deposit that 1 btc into the mago account and create a position
// * we mint the amount of uxd that corresponds to the position size
// and then in reverse is like
// * burn the amount of uxd
// * close out a corresponding position size and redeem for coin
// * proxy transfer coin to depository which *mints* redeemable to us
// * transfer redeemable to user
// and in fact we may very well just mint directly to user
#[derive(Accounts)]
#[instruction(coin_amount: u64)]
pub struct MintUxd<'info> {
    // XXX again we should use approvals so user doesnt need to sign
    pub user: Signer<'info>,
    #[account(seeds = [STATE_SEED], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(seeds = [DEPOSITORY_SEED, coin_mint.key().as_ref()], bump)]
    pub depository: Box<Account<'info, DepositoryNew>>,
    #[account(constraint = coin_mint.key() == depository.coin_mint_key)]
    pub coin_mint: Box<Account<'info, Mint>>,
    #[account(mut, seeds = [PASSTHROUGH_SEED, coin_mint.key().as_ref()], bump)]
    pub coin_passthrough: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_coin.mint == depository.coin_mint_key,
        constraint = coin_amount > 0,
        constraint = user_coin.amount >= coin_amount,
    )]
    pub user_coin: Box<Account<'info, TokenAccount>>,
    // XXX this account should be created by a client instruction
    #[account(mut, constraint = user_uxd.mint == uxd_mint.key())]
    pub user_uxd: Box<Account<'info, TokenAccount>>,
    #[account(mut, seeds = [UXD_SEED], bump)]
    pub uxd_mint: Box<Account<'info, Mint>>,
    // XXX start mango ------------------
    // MangoGroup that this mango account is for
    pub mango_group: AccountInfo<'info>,
    // Mango Account of the Depository Record
    #[account(mut)]
    pub mango_account: AccountInfo<'info>,
    pub mango_cache: AccountInfo<'info>,
    pub mango_root_bank: AccountInfo<'info>,
    #[account(mut)]
    pub mango_node_bank: AccountInfo<'info>,
    #[account(mut)]
    pub mango_vault: Account<'info, TokenAccount>,
    // XXX end mango --------------------
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub mango_program: Program<'info, mango_program::Mango>,
    #[account(constraint = oracle.key() == depository.oracle_key)]
    pub oracle: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(uxd_amount: u64)]
pub struct RedeemUxd<'info> {
    // XXX again we should use approvals so user doesnt need to sign
    pub user: Signer<'info>,
    #[account(seeds = [STATE_SEED], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(seeds = [RECORD_SEED, coin_mint.key().as_ref()], bump)]
    pub depository_record: Box<Account<'info, DepositoryRecord>>,
    #[account(
        constraint = depository_state.key() == Pubkey::find_program_address(&[depository::STATE_SEED, depository_state.coin_mint_key.as_ref()], &DepositoryOld::id()).0,
    )]
    pub depository_state: Box<Account<'info, depository::State>>,
    #[account(mut, constraint = depository_coin.key() == depository_state.program_coin_key)]
    pub depository_coin: Box<Account<'info, TokenAccount>>,
    #[account(constraint = coin_mint.key() == depository_state.coin_mint_key)]
    pub coin_mint: Box<Account<'info, Mint>>,
    #[account(mut, seeds = [PASSTHROUGH_SEED, coin_mint.key().as_ref()], bump)]
    pub coin_passthrough: Box<Account<'info, TokenAccount>>,
    #[account(mut, constraint = redeemable_mint.key() == depository_state.redeemable_mint_key)]
    pub redeemable_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        constraint = user_redeemable.mint == depository_state.redeemable_mint_key,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_uxd.mint == uxd_mint.key(),
        constraint = uxd_amount > 0,
        constraint = user_uxd.amount >= uxd_amount,
    )]
    pub user_uxd: Box<Account<'info, TokenAccount>>,
    #[account(mut, seeds = [UXD_SEED], bump)]
    pub uxd_mint: Box<Account<'info, Mint>>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub depository_program: Program<'info, DepositoryOld>,
    // XXX FIXME below here is temporary
    // oracle: dumb hack for devnet, pending mango integration
    #[account(constraint = oracle.key() == depository_record.oracle_key)]
    pub oracle: AccountInfo<'info>,
}

// MARK: - Accounts  ----------------------------------------------------------

#[account]
#[derive(Default)]
pub struct State {
    bump: u8,
    authority_key: Pubkey,
    uxd_mint_key: Pubkey,
}

#[account]
#[derive(Default)]
pub struct DepositoryNew {
    bump: u8,
    coin_mint_key: Pubkey,
    coin_passthrough_key: Pubkey,
    mango_account_key: Pubkey,
    // XXX temp for devnet
    oracle_key: Pubkey,
}

#[account]
#[derive(Default)]
pub struct DepositoryRecord {
    bump: u8,
    oracle_key: Pubkey,
}

// MARK: - CONTEXTS  ----------------------------------------------------------

impl<'info> MintUxd<'info> {
    fn into_mint_uxd_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.uxd_mint.to_account_info(),
            to: self.user_uxd.to_account_info(),
            authority: self.state.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> RedeemUxd<'info> {
    fn into_burn_uxd_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.uxd_mint.to_account_info(),
            to: self.user_uxd.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn into_return_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, depository::cpi::accounts::Deposit<'info>> {
        let cpi_program = self.depository_program.to_account_info();
        let cpi_accounts = depository::cpi::accounts::Deposit {
            user: self.depository_record.to_account_info(),
            state: self.depository_state.to_account_info(),
            program_coin: self.depository_coin.to_account_info(),
            redeemable_mint: self.redeemable_mint.to_account_info(),
            user_coin: self.coin_passthrough.to_account_info(),
            user_redeemable: self.user_redeemable.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
