use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};
use solana_program::{ system_instruction::create_account, program::invoke_signed };
use spl_token::instruction::{ initialize_account, initialize_mint };
// placeholder for figuring out best way
use mango_tester::{MangoTester, InitMangoAccount};
// use mango_tester::{InitMangoAccount, Deposit};


const MINT_SPAN: u64 = 82;
const ACCOUNT_SPAN: u64 = 165;
const MINT_DECIMAL: u8 = 9;
const UXDSEEDWORD: &[u8] = b"STABLECOIN";
const PROXYSEEDWORD: &[u8] = b"PROXY";

#[program]
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

    /////// Instruction functions ///////

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let accounts = ctx.accounts.to_account_infos();

        let (dummy_addr, dummy_ctr) = Pubkey::find_program_address(&[], ctx.program_id);

        // create uxd mint
        let (uxd_addr, uxd_ctr) = Pubkey::find_program_address(&[UXDSEEDWORD], ctx.program_id);
        let uxd_seed: &[&[&[u8]]] = &[&[UXDSEEDWORD, &[uxd_ctr]]];
        let uxd_rent = ctx.accounts.rent.minimum_balance(MINT_SPAN as usize);
        let uxd_i1 = create_account(ctx.accounts.payer.key, &uxd_addr, uxd_rent, MINT_SPAN, ctx.accounts.token_program.key);
        invoke_signed(&uxd_i1, &accounts, uxd_seed)?;

        let uxd_i2 = initialize_mint(
            &spl_token::ID,
            &uxd_addr,
            &dummy_addr,
            Some(&dummy_addr),
            MINT_DECIMAL,
        )?;
        invoke_signed(&uxd_i2, &accounts, uxd_seed)?;

        // create proxy account
        let (proxy_addr, proxy_ctr) = Pubkey::find_program_address(&[PROXYSEEDWORD], ctx.program_id);
        let proxy_seed: &[&[&[u8]]] = &[&[PROXYSEEDWORD, &[proxy_ctr]]];
        let proxy_rent = ctx.accounts.rent.minimum_balance(ACCOUNT_SPAN as usize);
        let proxy_i1 = create_account(ctx.accounts.payer.key, &proxy_addr, proxy_rent, ACCOUNT_SPAN, ctx.accounts.token_program.key);
        invoke_signed(&proxy_i1, &accounts, proxy_seed)?;

        //initialize proxy account
        let proxy_i2 = initialize_account(
            &spl_token::ID,
            &proxy_addr,
            ctx.accounts.proxy_mint.key,
            &dummy_addr,
        )?;

        // Don't use state because deprecated

        // initialize mango or equivalent user account
        // using mango for now as a built in but later make different providers as separate internal functions
        // called based on a config

        /// Accounts expected by this instruction (4):
        ///
        /// 0. `[]` mango_group_ai - MangoGroup that this mango account is for
        /// 1. `[writable]` mango_account_ai - the mango account data
        /// 2. `[signer]` owner_ai - Solana account of owner of the mango account
        /// 3. `[]` rent_ai - Rent sysvar account
        let mango_cpi_program = ctx.accounts.mango_program.clone();
        let mango_cpi_accts = InitMangoAccount {
            mango_group: ctx.accounts.mango_group.to_account_info(),
            mango_account: ctx.accounts.mango_account.clone().into(),
            owner_account: ctx.accounts.proxy_account.clone().into(),
            rent: ctx.accounts.rent.clone(),
        };
        let mango_cpi_ctx = CpiContext::new(mango_cpi_program, mango_cpi_accts);
        //placeholder line
        //mango::cpi::init_mango_account(mango_cpi_ctx, data)


        // set up rebalance signers in account data
        Ok(())

    }

    // pub fn mint(ctx: Context<Mint>) -> ProgramResult {
    //     // accept depository redeemable token
    //     // validate user input
    //     // call depository proxytransfer or equivalent
    //     // with reciever being controller proxy address (or controller mango account)
    //     // transfer to controller's Mango user account
    //     // call calc_swap_position
    //     // CPI call into mango to sell swaps according to ^
    //     // mint tokens from uxd_mint to user account
    // }
    //
    // pub fn redeem(ctx: Context<Redeem>) -> ProgramResult {
    //     // validate user input
    //     // burn user uxd
    //     // exchange depository redeemable token
    // }
    //
    //
    // fn proxy_transfer(ctx: Context<Proxy_transfer>) -> ProgramResult {
    //     // called by mint or rebalance
    //     // takes arguments for depository, amount, and target (if multiple targets)
    //     // depository has a fixed and short list of acceptable proxy transfer targets
    //     // CPI call into the depository contract
    //     // validate funds receipt and depository redeemable burn (on depository side)
    // }
    //
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
    // /////// internal functions ///////
    //
    // fn calc_swap_position(collateral: Pubkey, amount: u64) -> ProgramResult {
    //     // used by mint and reblance to handle the calculation of swap positions
    //     // get collateral  oracle price from pyth (fn sig does nto reflect currently)
    //     // get swap pricing from mango api
    //     // price swaps/spot and then apply the requested amount
    // }

}

    #[derive(Accounts)]
    pub struct Initialize<'info> {
        #[account(signer, mut)]
        pub payer: AccountInfo<'info>,
        pub dummy: AccountInfo<'info>,
        pub proxy_account: AccountInfo<'info>,
        #[account(mut)]
        pub uxd_mint: AccountInfo<'info>,
        pub proxy_mint: AccountInfo<'info>,
        pub rent: Sysvar<'info, Rent>,
        pub sys: AccountInfo<'info>,
        // single depository version
        #[account(mut)]
        pub depository: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
        pub mango_program: AccountInfo<'info>,
        pub mango_group: CpiAccount<'info, MangoTester>,
        pub mango_account: AccountInfo<'info>,

        pub prog: AccountInfo<'info>,
    }
