use std::ops::Div;

use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Burn;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;
use mango::state::MangoAccount;
use mango::state::MangoCache;
use mango::state::MangoGroup;
use mango::state::PerpAccount;

use crate::mango_program;
use crate::ControllerError;
use crate::Depository;
use crate::State;
use crate::DEPOSITORY_SEED;
use crate::PASSTHROUGH_SEED;
use crate::SLIPPAGE_BASIS;
use crate::STATE_SEED;
use crate::UXD_SEED;

#[derive(Accounts)]
#[instruction(uxd_amount: u64)]
pub struct RedeemUxd<'info> {
    // XXX again we should use approvals so user doesnt need to sign
    pub user: Signer<'info>,
    #[account(seeds = [STATE_SEED], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(seeds = [DEPOSITORY_SEED, collateral_mint.key().as_ref()], bump)]
    pub depository: Box<Account<'info, Depository>>,
    #[account(constraint = collateral_mint.key() == depository.collateral_mint_key)]
    pub collateral_mint: Box<Account<'info, Mint>>,
    #[account(mut, seeds = [PASSTHROUGH_SEED, collateral_mint.key().as_ref()], bump)]
    pub collateral_passthrough: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_collateral.mint == depository.collateral_mint_key,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,
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
    // The perp market for `collateral_mint` on mango, and the associated required accounts
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

pub fn handler(ctx: Context<RedeemUxd>, uxd_amount: u64, slippage: u32) -> ProgramResult {
    // - First burn the uxd they'r giving up
    token::burn(ctx.accounts.into_burn_uxd_context(), uxd_amount)?;

    // get current passthrough balance before withdrawing from mango
    // in theory this should always be zero but better safe
    // XXX cannot be updated and read through this program, only if we would be doing ledger operations.
    // let _initial_passthrough_balance = I80F48::from_num(ctx.accounts.collateral_passthrough.amount);

    // msg!("controller: redeem uxd [calculation for perp position closing]");
    let collateral_mint_key = ctx.accounts.collateral_mint.key();
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
    let taker_fee = mango_group.perp_markets[perp_market_index].taker_fee;
    // base and quote details
    let base_decimals = mango_group.tokens[perp_market_index].decimals;
    let base_unit = I80F48::from_num(10u64.pow(base_decimals.into()));
    let base_lot_size = I80F48::from_num(mango_group.perp_markets[perp_market_index].base_lot_size);
    let quote_decimals = mango_group.tokens[mango::state::QUOTE_INDEX].decimals;
    let quote_unit = I80F48::from_num(10u64.pow(quote_decimals.into()));
    let quote_lot_size =
        I80F48::from_num(mango_group.perp_markets[perp_market_index].quote_lot_size);
    // msg!("-----");
    // msg!("base_unit {}", base_unit);
    // msg!("base_lot_size {}", base_lot_size);
    // msg!("quote_unit {}", quote_unit);
    // msg!("quote_lot_size {}", quote_lot_size);
    // msg!("-----");

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
    let uxd_amount = I80F48::from_num(uxd_amount);
    let exposure_delta = uxd_amount;
    // msg!("exposure_delta (in native quote unit): {}", exposure_delta);

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
    // msg!("order_price_qlu (in quote lot unit): {}", order_price_qlu);

    // Execution quantity
    let order_quantity_blu = exposure_delta_qlu.checked_div(order_price_qlu).unwrap();
    // msg!(
    //     "order_quantity_blu (short perp quantity to close, in base lot unit): {}",
    //     order_quantity_blu
    // );

    // We now calculate the amount pre perp closing, in order to define after if it got 100% filled or not
    let pre_position = {
        let perp_account: &PerpAccount = &mango_account.perp_accounts[perp_market_index];
        // msg!("-----");
        // msg!("base_position {}", perp_account.base_position);
        // msg!("quote_position {}", perp_account.quote_position);
        // msg!("taker_base {}", perp_account.taker_base);
        // msg!("taker_quote {}", perp_account.taker_quote);
        // msg!("-----");
        perp_account.base_position + perp_account.taker_base
    };
    // msg!("pre_position {}", pre_position);
    // Drop ref cause they are also used in the Mango CPI destination
    drop(mango_group);
    drop(mango_cache);
    drop(mango_account);

    let depository_signer_seed: &[&[&[u8]]] = &[&[
        DEPOSITORY_SEED,
        collateral_mint_key.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];
    // Call Mango CPI to place the order that closes short position
    let order_price = order_price_qlu.to_num::<i64>();
    let order_quantity = order_quantity_blu.to_num::<i64>();
    mango_program::place_perp_order(
        ctx.accounts
            .into_close_mango_short_perp_context()
            .with_signer(depository_signer_seed),
        order_price,
        order_quantity,
        0,
        mango::matching::Side::Bid,
        mango::matching::OrderType::ImmediateOrCancel,
        true,
    )?;

    // Seems we need to settle that order in order to know the real filled quantity? :/
    // -> https://github.com/UXDProtocol/solana-usds/issues/33

    // msg!("verify that the order got 100% filled");
    let mango_account = MangoAccount::load_checked(
        &ctx.accounts.mango_account,
        ctx.accounts.mango_program.key,
        ctx.accounts.mango_group.key,
    )?;
    let perp_account: &PerpAccount = &mango_account.perp_accounts[perp_market_index];

    // msg!("-----");
    let post_position = perp_account.base_position + perp_account.taker_base;
    // msg!("base_position {}", perp_account.base_position);
    // msg!("quote_position {}", perp_account.quote_position);
    // msg!("taker_base {}", perp_account.taker_base);
    // msg!("taker_quote {}", perp_account.taker_quote);
    // msg!("-----");
    // msg!("pre_position {}", pre_position);
    // msg!("post_position {}", post_position);
    // msg!("-----");
    let filled = (post_position - pre_position).abs();
    // msg!("filled {}", filled);
    // msg!("order quantity {} =?= filled {}", order_quantity, filled);
    // msg!("-----");
    if !(order_quantity == filled) {
        return Err(ControllerError::PerpPartiallyFilled.into());
    }

    // - Call mango CPI to withdraw collateral
    // msg!("-----");
    let quote = I80F48::from_num(perp_account.taker_quote)
        .checked_mul(quote_lot_size)
        .unwrap()
        .abs();
    let fees = quote.checked_mul(taker_fee).unwrap();
    // msg!("quote {}", quote);
    // msg!("fees {}", fees);

    // In USDC
    let quote_withdraw_amount = quote - fees;
    let collateral_withdraw_amount = quote_withdraw_amount
        .checked_div(perp_value)
        .unwrap()
        .to_num();
    // msg!("quote_withdraw_amount {}", quote_withdraw_amount);
    // msg!("collateral_withdraw_amount {}", collateral_withdraw_amount);
    // msg!("-----");

    // Drop ref cause they are also used in the Mango CPI destination
    drop(mango_account);
    mango_program::withdraw(
        ctx.accounts
            .into_withdraw_collateral_from_mango_context()
            .with_signer(depository_signer_seed),
        collateral_withdraw_amount,
        false,
    )?;

    // - Return collateral back to user
    // diff of the passthrough balance and return it
    // XXX Doing it this way is not updated yet, cannot - this would work if we were doing the ledger change manually
    // let current_passthrough_balance = I80F48::from_num(ctx.accounts.collateral_passthrough.amount);

    token::transfer(
        ctx.accounts
            .into_transfer_collateral_to_user_context()
            .with_signer(depository_signer_seed),
        collateral_withdraw_amount,
    )?;

    Ok(())
}

// MARK: - Contexts -----

impl<'info> RedeemUxd<'info> {
    pub fn into_burn_uxd_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.uxd_mint.to_account_info(),
            to: self.user_uxd.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_close_mango_short_perp_context(
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

    pub fn into_withdraw_collateral_from_mango_context(
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
            token_account: self.collateral_passthrough.to_account_info(),
            mango_signer: self.mango_signer.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_collateral_to_user_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::Transfer {
            from: self.collateral_passthrough.to_account_info(),
            to: self.user_collateral.to_account_info(),
            authority: self.depository.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
