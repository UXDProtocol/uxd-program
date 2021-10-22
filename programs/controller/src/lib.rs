use anchor_lang::prelude::*;
use anchor_spl::token::InitializeAccount;
use anchor_spl::token::InitializeMint;
use anchor_spl::token::Token;
use anchor_spl::token::{self, Burn, Mint, MintTo, TokenAccount};
use depository::Depository;
use fixed::types::I80F48;
use mango::state::{MangoAccount, MangoCache, MangoGroup};
use pyth_client::Price;
use std::convert::TryFrom;
use std::mem::size_of;

mod mango_program;
const MINT_SPAN: usize = 82;
const ACCOUNT_SPAN: usize = 165;
const MANGO_ACCOUNT_SPAN: usize = size_of::<MangoAccount>();
const UXD_DECIMAL: u8 = 6;

const STATE_SEED: &[u8] = b"STATE";
const UXD_SEED: &[u8] = b"STABLECOIN";
const RECORD_SEED: &[u8] = b"RECORD";
const MANGO_SEED: &[u8] = b"MANGO";
const PASSTHROUGH_SEED: &[u8] = b"PASSTHROUGH";

const SLIPPAGE_BASIS: u32 = 1000;

solana_program::declare_id!("137dXnDWhuEqfSbJGrWSbaKxcDE3sk8A8V8ze5LTb9TX");

#[program]
#[deny(unused_must_use)]
pub mod controller {

    use mango::state::PerpAccount;

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
        ctx.accounts.state.authority_key = *ctx.accounts.authority.key;
        ctx.accounts.state.uxd_mint_key = *ctx.accounts.uxd_mint.key;

        // - Initialize UXD Mint
        let uxd_mint_nonce = Pubkey::find_program_address(&[UXD_SEED], ctx.program_id).1;
        let uxd_mint_signer_seed: &[&[&[u8]]] = &[&[UXD_SEED, &[uxd_mint_nonce]]];
        token::initialize_mint(
            ctx.accounts
                .into_initialize_uxd_mint_context()
                .with_signer(uxd_mint_signer_seed),
            UXD_DECIMAL,
            &ctx.accounts.state.key(),
            None,
        )?;

        Ok(())
    }

    // REGISTER DEPOSITORY
    // authority must sign and match authority in our initial state
    // create a mango account for the coin, create an entry indicating we created and trust this depository
    // create a passthrough account for whatever coin corresponds to the depository
    // we need this because the owner of the mango account and the token account must be the same
    // and we cant make the depository own the mango account because we need to sign for these accounts
    // it seems prudent for every depository to have its own mango account
    pub fn register_depository(
        ctx: Context<RegisterDepository>,
        oracle_key: Pubkey,
    ) -> ProgramResult {
        msg!("controller: register depository");

        let coin_mint_key = ctx.accounts.coin_mint.key();

        msg!("controller: register depository [initialize Passthrough]");
        let passthrough_bump_seed = Pubkey::find_program_address(
            &[PASSTHROUGH_SEED, coin_mint_key.as_ref()],
            ctx.program_id,
        )
        .1;
        let passthrough_pda_signer_seeds: &[&[&[u8]]] = &[&[
            PASSTHROUGH_SEED,
            coin_mint_key.as_ref(),
            &[passthrough_bump_seed],
        ]];
        // init the passthrough account we use to move funds between depository and mango
        // making our depo record rather than the contr state the owner for pleasing namespacing reasons
        token::initialize_account(
            ctx.accounts
                .into_initialize_passthrough_account_context()
                .with_signer(passthrough_pda_signer_seeds),
        )?;

        msg!("controller: register depository [initialize Mango Account]");
        let depository_record_bump_seed =
            Pubkey::find_program_address(&[RECORD_SEED, coin_mint_key.as_ref()], ctx.program_id).1;
        let depository_record_pda_signer_seeds: &[&[&[u8]]] = &[&[
            RECORD_SEED,
            coin_mint_key.as_ref(),
            &[depository_record_bump_seed],
        ]];
        let instruction = solana_program::instruction::Instruction {
            program_id: ctx.accounts.mango_program.key(),
            data: mango::instruction::MangoInstruction::InitMangoAccount.pack(),
            accounts: vec![
                AccountMeta::new_readonly(ctx.accounts.mango_group.key(), false),
                AccountMeta::new(ctx.accounts.mango_account.key(), false),
                AccountMeta::new_readonly(ctx.accounts.depository_record.key(), true),
                AccountMeta::new_readonly(ctx.accounts.rent.key(), false),
            ],
        };
        let account_infos = [
            ctx.accounts.mango_program.to_account_info(),
            ctx.accounts.mango_group.to_account_info(),
            ctx.accounts.mango_account.to_account_info(),
            ctx.accounts.depository_record.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        solana_program::program::invoke_signed(
            &instruction,
            &account_infos,
            depository_record_pda_signer_seeds,
        )?;

        // - Set our depo record up
        // this later acts as proof we trust a given depository
        // we also use this to derive the depository state key, from which we get mint and account keys
        // creating a hierarchy of trust rooted at the authority key that instantiated the controller
        ctx.accounts.depository_record.bump =
            Pubkey::find_program_address(&[RECORD_SEED, coin_mint_key.as_ref()], ctx.program_id).1;
        ctx.accounts.depository_record.oracle_key = oracle_key;

        Ok(())
    }

    // MINT UXD
    // swap user redeemable for coin which we take
    // open a mango position with that using the slippage
    // then mint uxd in the amount of the mango position to the user    #[access_control(
    #[access_control(valid_slippage(slippage))]
    pub fn mint_uxd(ctx: Context<MintUxd>, coin_amount: u64, slippage: u32) -> ProgramResult {
        msg!("controller: mint uxd");

        msg!("controller: mint uxd [Burn user redeemables and withdraw the coin to our passthrough account]");
        // let depo_state: ProgramAccount<depository::State> = ctx.accounts.depository_state.from();
        depository::cpi::withdraw(
            ctx.accounts
                .into_withdraw_from_depsitory_to_passthrough_context(),
            Some(coin_amount),
        )?;

        // XXX No need for mango accounts check as they are checked extensively by their instructions?
        // TBD

        msg!("controller: mint uxd [Deposit Mango CPI]");
        let coin_mint_key = ctx.accounts.coin_mint.key();
        let depository_record_bump =
            Pubkey::find_program_address(&[RECORD_SEED, coin_mint_key.as_ref()], ctx.program_id).1;
        let depository_record_signer_seeds: &[&[&[u8]]] = &[&[
            RECORD_SEED,
            coin_mint_key.as_ref(),
            &[depository_record_bump],
        ]];
        mango_program::deposit(
            ctx.accounts
                .into_deposit_to_mango_context()
                .with_signer(depository_record_signer_seeds),
            coin_amount,
        )?;

        msg!("controller: mint uxd [calculation for perp position opening]");
        let mango_account = MangoAccount::load_checked(
            &ctx.accounts.mango_account,
            ctx.accounts.mango_program.key,
            &ctx.accounts.mango_group.key,
        )?;
        let mango_group =
            MangoGroup::load_checked(&ctx.accounts.mango_group, ctx.accounts.mango_program.key)?;
        let mango_cache = MangoCache::load_checked(
            &ctx.accounts.mango_cache,
            ctx.accounts.mango_program.key,
            &mango_group,
        )?;
        // PERP
        let perp_market_index = mango_group
            .find_perp_market_index(ctx.accounts.mango_perp_market.key)
            .unwrap();
        let coin_perp_value = mango_cache.price_cache[perp_market_index].price;
        // base and quote details
        let base_decimals = mango_group.tokens[perp_market_index].decimals;
        let base_unit = 10u64.pow(base_decimals.into());
        let base_lot_size =
            I80F48::from_num(mango_group.perp_markets[perp_market_index].base_lot_size);
        let quote_decimals = mango_group.tokens[mango::state::QUOTE_INDEX].decimals;
        let quote_unit = 10u64.pow(quote_decimals.into());
        let quote_lot_size =
            I80F48::from_num(mango_group.perp_markets[perp_market_index].quote_lot_size);
        // Slippage calulation
        let slippage = I80F48::from_num(slippage);
        let slippage_basis = I80F48::from_num(SLIPPAGE_BASIS);
        let slippage_ratio = slippage.checked_div(slippage_basis).unwrap();
        // price in quote lot unit
        let mut price = coin_perp_value
            .checked_mul(I80F48::from_num(quote_unit))
            .unwrap()
            .checked_mul(base_lot_size)
            .unwrap()
            .checked_div(quote_lot_size)
            .unwrap()
            .checked_div(I80F48::from_num(base_unit))
            .unwrap();
        // msg!("price in quote lot unit: {}", price);
        price -= price.checked_mul(slippage_ratio).unwrap();
        msg!("price in quote lot unit (w/ sleepage): {}", price);
        let coin_amount = I80F48::from_num(coin_amount);
        // HANA EXPERT o/
        // Not sure should I use the price calculated above here instead of the coin perp
        let exposure_delta = coin_perp_value
            .checked_mul(coin_amount)
            .unwrap()
            .checked_div(quote_lot_size)
            .unwrap();
        // msg!("exposure delta in quote lot unit: {}", exposure_delta);
        let execution_quantity = exposure_delta.checked_div(price).unwrap().abs();
        // msg!(
        //     "perp execution_quantity in base lot unit: {}",
        //     execution_quantity
        // );
        // We now calculate the amount pre perp opening, in order to define after if it got 100% filled or not
        let pre_position = {
            let perp_account: &PerpAccount = &mango_account.perp_accounts[perp_market_index];
            perp_account.base_position + perp_account.taker_base
        };

        msg!("controller: mint uxd [Open perp position Mango CPI]");
        // Drop ref cause they are also used in the Mango CPI destination
        drop(mango_group);
        drop(mango_cache);
        drop(mango_account);

        let depository_record_bump =
            Pubkey::find_program_address(&[RECORD_SEED, coin_mint_key.as_ref()], ctx.program_id).1;
        let depository_record_signer_seeds: &[&[&[u8]]] = &[&[
            RECORD_SEED,
            coin_mint_key.as_ref(),
            &[depository_record_bump],
        ]];
        // Call Mango CPI
        mango_program::place_perp_order(
            ctx.accounts
                .into_open_mango_short_perp_context()
                .with_signer(depository_record_signer_seeds),
            price.to_num::<i64>(),
            execution_quantity.to_num::<i64>(),
            0,
            mango::matching::Side::Ask,
            mango::matching::OrderType::ImmediateOrCancel,
            false,
        )?;

        msg!("verify that the order got 100% filled");
        let mango_account = MangoAccount::load_checked(
            &ctx.accounts.mango_account,
            ctx.accounts.mango_program.key,
            ctx.accounts.mango_group.key,
        )?;
        let perp_account: &PerpAccount = &mango_account.perp_accounts[perp_market_index];
        let post_position = perp_account.base_position + perp_account.taker_base;
        let real_execution_quantity = post_position - pre_position;
        if !(execution_quantity.to_num::<i64>() == real_execution_quantity.abs()) {
            return Err(ControllerError::PerpPartialFill.into());
        }
        // TODO NEED TO DETERMINE THE REAL EXECUTION PRICE, else we might hoard the difference on each trades
        //   This value should then be the minted value
        //
        // Determine the filled price to define how much UXD we need to mint
        // let real_execution_price = mango_group.perp_markets[perp_market_index].perp_market
        let real_execution_price = price; //

        msg!("controller: mint uxd [Mint UXD for redeemables]");
        let coin_decimals = ctx.accounts.coin_mint.decimals as u32;
        let uxd_decimals = ctx.accounts.uxd_mint.decimals as u32;
        let coin_exp = I80F48::from_num(10u64.pow(coin_decimals));
        let uxd_exp = I80F48::from_num(10u64.pow(uxd_decimals));
        // USD value of delta position
        let position_usd_value = execution_quantity * real_execution_price;
        // Converted to UXD amount (we multiply by uxd decimals and divide by coin decimals and get a uxd amount)
        let position_uxd_amount = (position_usd_value * uxd_exp) / coin_exp;

        msg!("minting {} UXD for redeemables", position_usd_value,);
        let state_signer_seed: &[&[&[u8]]] = &[&[STATE_SEED, &[ctx.accounts.state.bump]]];
        token::mint_to(
            ctx.accounts
                .into_mint_uxd_context()
                .with_signer(state_signer_seed),
            position_uxd_amount.to_num(),
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

// MARK: - Helpers  -----------------------------------------------------------

// Keep here in case it can serve, but we always FillOrKill kind of. So should never have anything pending.
// Retrieve the pubkeys for open orders of that account
// pub fn get_open_orders_in_basket(
//     mango_account: &mango::state::MangoAccount,
// ) -> [Pubkey; MAX_PAIRS] {
//     let mut pks_in_basket = [Pubkey::default(); MAX_PAIRS];
//     for i in 0..MAX_PAIRS {
//         if mango_account.in_margin_basket[i] {
//             msg!("Found an open order");
//             pks_in_basket[i] = mango_account.spot_open_orders[i];
//         }
//     }
//     pks_in_basket
// }

// MARK: - Contextes  ---------------------------------------------------------

impl<'info> MintUxd<'info> {
    fn into_deposit_to_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::Deposit<'info>> {
        let cpi_accounts = mango_program::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository_record.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_root_bank: self.mango_root_bank.to_account_info(),
            mango_node_bank: self.mango_node_bank.to_account_info(),
            mango_vault: self.mango_vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
            owner_token_account: self.coin_passthrough.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn into_open_mango_short_perp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::PlacePerpOrder<'info>> {
        let cpi_accounts = mango_program::PlacePerpOrder {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository_record.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_perp_market: self.mango_perp_market.to_account_info(),
            mango_bids: self.mango_bids.to_account_info(),
            mango_asks: self.mango_asks.to_account_info(),
            mango_event_queue: self.mango_event_queue.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
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
    // Todo can use associated_token here
    #[account(
        init,
        seeds = [UXD_SEED],
        bump,
        payer = authority,
        owner = spl_token::ID,
        space = MINT_SPAN,
    )]
    pub uxd_mint: AccountInfo<'info>,
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
        seeds = [RECORD_SEED, coin_mint.key().as_ref()],
        bump,
        payer = authority,
    )]
    pub depository_record: Box<Account<'info, DepositoryRecord>>,
    #[account(
        constraint = depository_state.key() == Pubkey::find_program_address(&[depository::STATE_SEED, depository_state.coin_mint_key.as_ref()], &Depository::id()).0,
    )]
    pub depository_state: Box<Account<'info, depository::State>>,
    #[account(constraint = coin_mint.key() == depository_state.coin_mint_key)]
    pub coin_mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        seeds = [PASSTHROUGH_SEED, coin_mint.key().as_ref()],
        bump,
        payer = authority,
        owner = spl_token::ID,
        space = ACCOUNT_SPAN,
    )]
    pub coin_passthrough: AccountInfo<'info>,
    // The mango group for the mango_account
    pub mango_group: AccountInfo<'info>,
    // The mango PDA
    #[account(
        init,
        seeds = [MANGO_SEED, coin_mint.key().as_ref()],
        bump,
        owner = mango_program::Mango::id(),
        payer = authority,
        space = MANGO_ACCOUNT_SPAN,
    )]
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
    #[account(seeds = [RECORD_SEED, coin_mint.key().as_ref()], bump)]
    pub depository_record: Box<Account<'info, DepositoryRecord>>,
    #[account(
        constraint = depository_state.key() == Pubkey::find_program_address(&[depository::STATE_SEED, depository_state.coin_mint_key.as_ref()], &Depository::id()).0,
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
        constraint = coin_amount > 0,
        constraint = user_redeemable.amount >= coin_amount,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,
    // XXX this account should be created by a client instruction
    #[account(mut, constraint = user_uxd.mint == uxd_mint.key())]
    pub user_uxd: Box<Account<'info, TokenAccount>>,
    #[account(mut, seeds = [UXD_SEED], bump)]
    pub uxd_mint: Box<Account<'info, Mint>>,
    // XXX start mango --------------------------------------------------------
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
    // The perp market for `coin_mint` on mango, and the associated required accounts
    #[account(mut)]
    pub mango_perp_market: AccountInfo<'info>,
    #[account(mut)]
    pub mango_bids: AccountInfo<'info>,
    #[account(mut)]
    pub mango_asks: AccountInfo<'info>,
    #[account(mut)]
    pub mango_event_queue: AccountInfo<'info>,
    // XXX end mango ----------------------------------------------------------
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub depository_program: Program<'info, Depository>,
    pub mango_program: Program<'info, mango_program::Mango>,
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
        constraint = depository_state.key() == Pubkey::find_program_address(&[depository::STATE_SEED, depository_state.coin_mint_key.as_ref()], &Depository::id()).0,
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
        constraint = user_uxd.amount >= uxd_amount, // THESE SHOULD USE the custom error to avoid ` custom program error: 0x8f ` -- OR the access_control
    )]
    pub user_uxd: Box<Account<'info, TokenAccount>>,
    #[account(mut, seeds = [UXD_SEED], bump)]
    pub uxd_mint: Box<Account<'info, Mint>>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub depository_program: Program<'info, Depository>,
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
pub struct DepositoryRecord {
    bump: u8,
    // XXX temp for devnet
    oracle_key: Pubkey,
    mango_account: Pubkey,
}

// MARK: - CONTEXTS  ----------------------------------------------------------

impl<'info> New<'info> {
    fn into_initialize_uxd_mint_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, InitializeMint<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = InitializeMint {
            mint: self.uxd_mint.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> RegisterDepository<'info> {
    fn into_initialize_passthrough_account_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, InitializeAccount<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = InitializeAccount {
            account: self.coin_passthrough.to_account_info(),
            mint: self.coin_mint.to_account_info(),
            authority: self.depository_record.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> MintUxd<'info> {
    fn into_withdraw_from_depsitory_to_passthrough_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, depository::cpi::accounts::Withdraw<'info>> {
        let cpi_program = self.depository_program.to_account_info();
        let cpi_accounts = depository::cpi::accounts::Withdraw {
            user: self.user.to_account_info(),
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

// MARK: - ERRORS  ------------------------------------------------------------

#[error]
pub enum ControllerError {
    #[msg("Error while getting the UXD value of the deposited coin amount.")]
    PositionAmountCalculation,
    #[msg("The associated mango root bank index cannot be found for the deposited coin..")]
    RootBankIndexNotFound,
    #[msg("The slippage value is invalid. Must be in the [0...1000] range points.")]
    InvalidSlippage,
    #[msg("The perp position could not be fully opened with the provided slippage.")]
    PerpPartialFill,
}

// MARK: - ACCESS CONTROL  ----------------------------------------------------

// Asserts that the amount of usdc for the operation is above 0.
// Asserts that the amount of usdc is available in the user account.
fn valid_slippage<'info>(slippage: u32) -> ProgramResult {
    if !(slippage <= SLIPPAGE_BASIS) {
        return Err(ControllerError::InvalidSlippage.into());
    }
    Ok(())
}
