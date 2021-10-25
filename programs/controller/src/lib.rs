use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Burn;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use fixed::types::I80F48;
use mango::state::MangoAccount;
use mango::state::MangoCache;
use mango::state::MangoGroup;
use mango::state::PerpAccount;
use std::mem::size_of;

mod mango_program;

const MANGO_ACCOUNT_SPAN: usize = size_of::<MangoAccount>();

const UXD_DECIMAL: u8 = 6;

const STATE_SEED: &[u8] = b"STATE";
const UXD_SEED: &[u8] = b"STABLECOIN";
const DEPOSITORY_SEED: &[u8] = b"DEPOSITORY";
const PASSTHROUGH_SEED: &[u8] = b"PASSTHROUGH";
const MANGO_SEED: &[u8] = b"MANGO";

const SLIPPAGE_BASIS: u32 = 1000;

solana_program::declare_id!("5BkgzsnpEzcbftbtQZ86zb3qi4S9ZfcYhpwuWKTp9nHB");

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
    pub fn register_depository(ctx: Context<RegisterDepository>) -> ProgramResult {
        let coin_mint_key = ctx.accounts.coin_mint.key();

        // - Initialize Mango Account

        let depository_bump = Pubkey::find_program_address(
            &[DEPOSITORY_SEED, coin_mint_key.as_ref()],
            ctx.program_id,
        )
        .1;
        let depository_signer_seed: &[&[&[u8]]] =
            &[&[DEPOSITORY_SEED, coin_mint_key.as_ref(), &[depository_bump]]];
        mango_program::initialize_mango_account(
            ctx.accounts
                .into_mango_account_initialization_context()
                .with_signer(depository_signer_seed),
        )?;

        // - Set our depo record up
        // this later acts as proof we trust a given depository
        // we also use this to derive the depository state key, from which we get mint and account keys
        // creating a hierarchy of trust rooted at the authority key that instantiated the controller
        ctx.accounts.depository.bump = depository_bump;
        ctx.accounts.depository.coin_mint_key = coin_mint_key;
        ctx.accounts.depository.coin_passthrough_key = ctx.accounts.coin_passthrough.key();
        ctx.accounts.depository.mango_account_key = ctx.accounts.mango_account.key();

        Ok(())
    }

    // MINT UXD
    // transfer user coin to our passthrough. open a mango position with that
    // then mint uxd in the amount of the mango position to the user
    #[access_control(valid_slippage(slippage))]
    pub fn mint_uxd(ctx: Context<MintUxd>, coin_amount: u64, slippage: u32) -> ProgramResult {
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

        msg!("controller: mint uxd [Deposit Mango CPI]");
        let coin_mint_key = ctx.accounts.coin_mint.key();
        let depository_bump = Pubkey::find_program_address(
            &[DEPOSITORY_SEED, coin_mint_key.as_ref()],
            ctx.program_id,
        )
        .1;
        let depository_signer_seeds: &[&[&[u8]]] =
            &[&[DEPOSITORY_SEED, coin_mint_key.as_ref(), &[depository_bump]]];
        mango_program::deposit(
            ctx.accounts
                .into_deposit_to_mango_context()
                .with_signer(depository_signer_seeds),
            coin_amount,
        )?;

        // msg!("controller: mint uxd [calculation for perp position opening]");
        let collateral_amount = I80F48::from_num(coin_amount);

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
        // base and quote details
        let base_decimals = mango_group.tokens[perp_market_index].decimals;
        let base_unit = I80F48::from_num(10u64.pow(base_decimals.into()));
        let base_lot_size =
            I80F48::from_num(mango_group.perp_markets[perp_market_index].base_lot_size);
        let quote_decimals = mango_group.tokens[mango::state::QUOTE_INDEX].decimals;
        let quote_unit = I80F48::from_num(10u64.pow(quote_decimals.into()));
        let quote_lot_size =
            I80F48::from_num(mango_group.perp_markets[perp_market_index].quote_lot_size);
        msg!(
            "base decimals: {} - base unit: {} - base lot size: {}",
            base_decimals,
            base_unit,
            base_lot_size
        );
        msg!(
            "quote decimals: {} - quote unit: {} - quote lot size: {}",
            quote_decimals,
            quote_unit,
            quote_lot_size
        );

        // Slippage calulation
        let perp_value = mango_cache.price_cache[perp_market_index].price;
        let slippage = I80F48::from_num(slippage);
        let slippage_basis = I80F48::from_num(SLIPPAGE_BASIS);
        let slippage_ratio = slippage.checked_div(slippage_basis).unwrap();
        let slippage_amount = perp_value.checked_mul(slippage_ratio).unwrap();
        let price = perp_value.checked_sub(slippage_amount).unwrap();
        msg!("collateral_perp_value: {}", perp_value);
        msg!("price (after slippage calculation): {}", price);

        // Exposure delta calculation
        let deposited_value = collateral_amount
            .checked_div(base_unit)
            .unwrap()
            .checked_mul(perp_value)
            .unwrap();
        // Not sure about this one, might need to be mul by perp? or spot..
        let exposure_delta = collateral_amount.checked_mul(perp_value).unwrap();
        msg!("collateral_deposited_value: {}", deposited_value); // Is this valus good with decimals? To check
        msg!("exposure_delta: {}", exposure_delta);

        let exposure_delta_qlu = exposure_delta.checked_div(quote_lot_size).unwrap();
        msg!(
            "exposure_delta_qlu (in quote lot unit): {}",
            exposure_delta_qlu
        );

        // price in quote lot unit
        let order_price_qlu = price
            .checked_mul(quote_unit)
            .unwrap()
            .checked_mul(base_lot_size)
            .unwrap()
            .checked_div(quote_lot_size)
            .unwrap()
            .checked_div(base_unit)
            .unwrap();
        msg!("price_qlu (in quote lot unit): {}", order_price_qlu);

        // Execution quantity
        let order_quantity_blu = exposure_delta_qlu
            .checked_div(order_price_qlu)
            .unwrap()
            .abs();
        msg!("exec_qty_blu (base lot unit): {}", order_quantity_blu);

        // We now calculate the amount pre perp opening, in order to define after if it got 100% filled or not
        let pre_position = {
            let perp_account: &PerpAccount = &mango_account.perp_accounts[perp_market_index];
            perp_account.base_position + perp_account.taker_base
        };

        // Drop ref cause they are also used in the Mango CPI destination
        drop(mango_group);
        drop(mango_cache);
        drop(mango_account);

        let depository_record_bump = Pubkey::find_program_address(
            &[DEPOSITORY_SEED, coin_mint_key.as_ref()],
            ctx.program_id,
        )
        .1;
        let depository_signer_seeds: &[&[&[u8]]] = &[&[
            DEPOSITORY_SEED,
            coin_mint_key.as_ref(),
            &[depository_record_bump],
        ]];
        // Call Mango CPI
        let order_price = order_price_qlu.to_num::<i64>();
        let order_quantity = order_quantity_blu.to_num::<i64>();
        mango_program::place_perp_order(
            ctx.accounts
                .into_open_mango_short_perp_context()
                .with_signer(depository_signer_seeds),
            order_price,
            order_quantity,
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
        let execution_quantity = (post_position - pre_position).abs();
        if !(order_quantity == execution_quantity) {
            return Err(ControllerError::PerpPartiallyFilled.into());
        }
        // // XXX Here we taking the worse price, but it might have found a better exec price, and we should get that else
        // // the diff will fill the insurance fund each time, and people will always pay max slippage.
        // //
        // // Determine the filled price to define how much UXD we need to mint
        // let execution_price_qlu = order_price_qlu; //

        // // msg!("controller: mint uxd [Mint UXD for redeemables]");
        // let uxd_decimals = ctx.accounts.uxd_mint.decimals as u32;
        // let uxd_unit = I80F48::from_num(10u64.pow(uxd_decimals));
        // // USD value of delta position
        // let perp_order_uxd_value = order_quantity_blu.checked_mul(execution_price_qlu).unwrap();
        // msg!("perp_order_uxd_value : {}", perp_order_uxd_value);
        // Converted to UXD amount (we multiply by uxd decimals and divide by coin decimals and get a uxd amount)
        // let position_uxd_amount = perp_order_uxd_value
        //     .checked_mul(uxd_unit)
        //     .unwrap()
        //     .checked_div(base_unit)
        //     .unwrap();
        // msg!("position_uxd_amount : {}", position_uxd_amount);

        // For now give him for the worth slippage, see above
        let uxd_amount = collateral_amount.checked_mul(price).unwrap();
        let state_signer_seed: &[&[&[u8]]] = &[&[STATE_SEED, &[ctx.accounts.state.bump]]];
        token::mint_to(
            ctx.accounts
                .into_mint_uxd_context()
                .with_signer(state_signer_seed),
            uxd_amount.to_num(), // deposited_value best vs uxd_amount worse
        )?;

        Ok(())
    }

    // REDEEM UXD
    // burn uxd that is being redeemed. then close out mango position and return coins to user
    #[access_control(valid_slippage(slippage))]
    pub fn redeem_uxd(ctx: Context<RedeemUxd>, uxd_amount: u64, slippage: u32) -> ProgramResult {
        msg!("controller: redeem uxd");

        // - First burn the uxd they'r giving up
        token::burn(ctx.accounts.into_burn_uxd_context(), uxd_amount)?;

        // - Mango close positon and withdraw coin TODO
        // get current passthrough balance before withdrawing from mango
        // in theory this should always be zero but better safe
        let initial_passthrough_balance = I80F48::from_num(ctx.accounts.coin_passthrough.amount);

        ///////////////

        // msg!("controller: redeem uxd [calculation for perp position closing]");
        let coin_mint_key = ctx.accounts.coin_mint.key();
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
        // base and quote details
        let base_decimals = mango_group.tokens[perp_market_index].decimals;
        let base_unit = I80F48::from_num(10u64.pow(base_decimals.into()));
        let base_lot_size =
            I80F48::from_num(mango_group.perp_markets[perp_market_index].base_lot_size);
        let quote_decimals = mango_group.tokens[mango::state::QUOTE_INDEX].decimals;
        let quote_unit = I80F48::from_num(10u64.pow(quote_decimals.into()));
        let quote_lot_size =
            I80F48::from_num(mango_group.perp_markets[perp_market_index].quote_lot_size);

        // Slippage calulation
        let perp_value = mango_cache.price_cache[perp_market_index].price;
        let slippage = I80F48::from_num(slippage);
        let slippage_basis = I80F48::from_num(SLIPPAGE_BASIS);
        let slippage_ratio = slippage.checked_div(slippage_basis).unwrap();
        let slippage_amount = perp_value.checked_mul(slippage_ratio).unwrap();
        let price = perp_value.checked_add(slippage_amount).unwrap();
        // msg!("perp_value: {}", perp_value);
        // msg!("price (after slippage calculation): {}", price);

        // Exposure delta calculation
        let exposure_delta = I80F48::from_num(uxd_amount);
        // msg!("exposure_delta: {} (redeem value)", exposure_delta);

        let exposure_delta_qlu = exposure_delta.checked_div(quote_lot_size).unwrap();
        // msg!(
        //     "exposure_delta_qlu (in quote lot unit): {}",
        //     exposure_delta_qlu
        // );

        // price in quote lot unit
        let order_price_qlu = price
            .checked_mul(quote_unit)
            .unwrap()
            .checked_mul(base_lot_size)
            .unwrap()
            .checked_div(quote_lot_size)
            .unwrap()
            .checked_div(base_unit)
            .unwrap();
        // msg!("price_qlu (in quote lot unit): {}", order_price_qlu);

        // Execution quantity
        let order_quantity_blu = exposure_delta_qlu
            .checked_div(order_price_qlu)
            .unwrap()
            .abs();
        // msg!("exec_qty_blu (base lot unit): {}", order_quantity_blu);
        let execution_quantity = exposure_delta.checked_div(price).unwrap().abs();
        // msg!(
        //     "perp execution_quantity in base lot unit: {}",
        //     execution_quantity
        // );

        // We now calculate the amount pre perp closing, in order to define after if it got 100% filled or not
        let pre_position = {
            let perp_account: &PerpAccount = &mango_account.perp_accounts[perp_market_index];
            perp_account.base_position + perp_account.taker_base
        };
        // Drop ref cause they are also used in the Mango CPI destination
        drop(mango_group);
        drop(mango_cache);
        drop(mango_account);

        let depository_record_bump = Pubkey::find_program_address(
            &[DEPOSITORY_SEED, coin_mint_key.as_ref()],
            ctx.program_id,
        )
        .1;
        let depository_signer_seeds: &[&[&[u8]]] = &[&[
            DEPOSITORY_SEED,
            coin_mint_key.as_ref(),
            &[depository_record_bump],
        ]];

        // Call Mango CPI to place the order that closes short position
        let order_price = order_price_qlu.to_num::<i64>();
        let order_quantity = order_quantity_blu.to_num::<i64>();
        mango_program::place_perp_order(
            ctx.accounts
                .into_close_mango_short_perp_context()
                .with_signer(depository_signer_seeds),
            order_price,
            order_quantity,
            0,
            mango::matching::Side::Bid,
            mango::matching::OrderType::ImmediateOrCancel,
            true,
        )?;

        // msg!("verify that the order got 100% filled");
        let mango_account = MangoAccount::load_checked(
            &ctx.accounts.mango_account,
            ctx.accounts.mango_program.key,
            ctx.accounts.mango_group.key,
        )?;
        let perp_account: &PerpAccount = &mango_account.perp_accounts[perp_market_index];
        let post_position = perp_account.base_position + perp_account.taker_base;
        let execution_quantity = (post_position - pre_position).abs();
        if !(order_quantity == execution_quantity) {
            return Err(ControllerError::PerpPartiallyFilled.into());
        }

        let collateral_amount = exposure_delta_qlu.checked_div(order_price_qlu).unwrap();
        // msg!(
        //     "withdraw {} collateral_amount from mango back to passthrough account",
        //     collateral_amount
        // );
        let depository_record_bump = Pubkey::find_program_address(
            &[DEPOSITORY_SEED, coin_mint_key.as_ref()],
            ctx.program_id,
        )
        .1;
        let depository_signer_seeds: &[&[&[u8]]] = &[&[
            DEPOSITORY_SEED,
            coin_mint_key.as_ref(),
            &[depository_record_bump],
        ]];
        // Call mango CPI to withdraw collateral
        // Drop ref cause they are also used in the Mango CPI destination
        drop(mango_account);
        mango_program::withdraw(
            ctx.accounts
                .into_withdraw_from_mango_context()
                .with_signer(depository_signer_seeds),
            collateral_amount.to_num(),
            false,
        )?;

        // diff of the passthrough balance and return it
        let current_passthrough_balance = I80F48::from_num(ctx.accounts.coin_passthrough.amount);
        let collateral_amount_to_redeem = current_passthrough_balance
            .checked_sub(initial_passthrough_balance)
            .unwrap();

        // - Return collateral back to user
        let coin_mint_key = ctx.accounts.coin_mint.key();
        let depository_signer_seed: &[&[&[u8]]] = &[&[
            DEPOSITORY_SEED,
            coin_mint_key.as_ref(),
            &[ctx.accounts.depository.bump],
        ]];

        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.coin_passthrough.to_account_info(),
                to: ctx.accounts.user_coin.to_account_info(),
                authority: ctx.accounts.depository.to_account_info(),
            },
            depository_signer_seed,
        );
        token::transfer(transfer_ctx, collateral_amount_to_redeem.to_num())?;
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

// MARK: - CONTEXTS  ----------------------------------------------------------

impl<'info> RegisterDepository<'info> {
    fn into_mango_account_initialization_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::InitMangoAccount<'info>> {
        let cpi_accounts = mango_program::InitMangoAccount {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> MintUxd<'info> {
    fn into_deposit_to_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::Deposit<'info>> {
        let cpi_accounts = mango_program::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
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
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_perp_market: self.mango_perp_market.to_account_info(),
            mango_bids: self.mango_bids.to_account_info(),
            mango_asks: self.mango_asks.to_account_info(),
            mango_event_queue: self.mango_event_queue.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
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

    fn into_close_mango_short_perp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::PlacePerpOrder<'info>> {
        let cpi_accounts = mango_program::PlacePerpOrder {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_perp_market: self.mango_perp_market.to_account_info(),
            mango_bids: self.mango_bids.to_account_info(),
            mango_asks: self.mango_asks.to_account_info(),
            mango_event_queue: self.mango_event_queue.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn into_withdraw_from_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::Withdraw<'info>> {
        let cpi_accounts = mango_program::Withdraw {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_root_bank: self.mango_root_bank.to_account_info(),
            mango_node_bank: self.mango_node_bank.to_account_info(),
            mango_vault: self.mango_vault.to_account_info(),
            token_account: self.coin_passthrough.to_account_info(),
            mango_signer: self.mango_signer.to_account_info(),
            token_program: self.token_program.to_account_info(),
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
    pub depository: Box<Account<'info, Depository>>,
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
    #[account(seeds = [DEPOSITORY_SEED, coin_mint.key().as_ref()], bump)]
    pub depository: Box<Account<'info, Depository>>,
    #[account(constraint = coin_mint.key() == depository.coin_mint_key)]
    pub coin_mint: Box<Account<'info, Mint>>,
    #[account(mut, seeds = [PASSTHROUGH_SEED, coin_mint.key().as_ref()], bump)]
    pub coin_passthrough: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        // TODO - Move these to custom constraint (see new PR on anchor) - or access_control
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
    // The spot/perp market for `coin_mint` on mango, and the associated required accounts
    #[account(mut)]
    pub mango_perp_market: AccountInfo<'info>,
    #[account(mut)]
    pub mango_bids: AccountInfo<'info>,
    #[account(mut)]
    pub mango_asks: AccountInfo<'info>,
    #[account(mut)]
    pub mango_event_queue: AccountInfo<'info>,
    // XXX end mango ----------------------------------------------------------
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub mango_program: Program<'info, mango_program::Mango>,
    //
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(uxd_amount: u64)]
pub struct RedeemUxd<'info> {
    // XXX again we should use approvals so user doesnt need to sign
    pub user: Signer<'info>,
    #[account(seeds = [STATE_SEED], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(seeds = [DEPOSITORY_SEED, coin_mint.key().as_ref()], bump)]
    pub depository: Box<Account<'info, Depository>>,
    #[account(constraint = coin_mint.key() == depository.coin_mint_key)]
    pub coin_mint: Box<Account<'info, Mint>>,
    #[account(mut, seeds = [PASSTHROUGH_SEED, coin_mint.key().as_ref()], bump)]
    pub coin_passthrough: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_coin.mint == depository.coin_mint_key,
    )]
    pub user_coin: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_uxd.mint == uxd_mint.key(),
        constraint = uxd_amount > 0,
        constraint = user_uxd.amount >= uxd_amount, // THESE SHOULD USE the custom error to avoid ` custom program error: 0x8f ` -- OR the access_control
    )]
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
    // This is some mango internal stuff - name is misleading
    pub mango_signer: AccountInfo<'info>,
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
    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub mango_program: Program<'info, mango_program::Mango>,
    // sysvars
    pub rent: Sysvar<'info, Rent>,
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
pub struct Depository {
    bump: u8,
    coin_mint_key: Pubkey,
    coin_passthrough_key: Pubkey,
    mango_account_key: Pubkey,
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
    #[msg("The perp position could not be fully filled with the provided slippage.")]
    PerpPartiallyFilled,
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
