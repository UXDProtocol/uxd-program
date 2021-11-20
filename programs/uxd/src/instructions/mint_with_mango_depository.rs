use anchor_lang::prelude::*;
use anchor_lang::Discriminator;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use fixed::types::I80F48;
use mango::error::MangoResult;
use mango::state::MangoAccount;
use mango::state::MangoCache;
use mango::state::MangoGroup;
use mango::state::PerpAccount;

use crate::mango_program;
use crate::utils::perp_base_position;
use crate::utils::PerpInfo;
use crate::Controller;
use crate::MangoDepository;
use crate::UXDError;
use crate::COLLATERAL_PASSTHROUGH_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use crate::SLIPPAGE_BASIS;

// First iteration
// XXX oki this shit is complicated lets see what all is here...
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

// Second iteration
// Take Collateral from the user
// Deposit collateral on Mango (long half)
// Place immediate perp order on mango using our deposited collateral for borrowing (short half)
//   if it does not fill withing slippage, we abort
// Mint equivalent amount of UXD as the position is covering for

#[derive(Accounts)]
pub struct MintWithMangoDepository<'info> {
    // XXX again we should use approvals so user doesnt need to sign
    pub user: Signer<'info>,
    #[account(
        seeds = [
            &Controller::discriminator()[..]
        ],
         bump = controller.bump
    )]
    pub controller: Box<Account<'info, Controller>>,
    #[account(
        seeds = [
            &MangoDepository::discriminator()[..],
            collateral_mint.key().as_ref()
        ],
        bump = depository.bump
    )]
    pub depository: Box<Account<'info, MangoDepository>>,
    #[account(
        constraint = collateral_mint.key() == depository.collateral_mint @UXDError::MintMismatchCollateral
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        constraint = user_collateral.mint == depository.collateral_mint @UXDError::InvalidCollateralMint
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_redeemable.mint == controller.redeemable_mint @UXDError::InvalidUserRedeemableATAMint
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,
    #[account(
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.redeemable_mint_bump,
        constraint = redeemable_mint.key() == controller.redeemable_mint @UXDError::InvalidRedeemableMint
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [
            COLLATERAL_PASSTHROUGH_NAMESPACE,
            collateral_mint.key().as_ref()
        ],
        bump = depository.collateral_passthrough_bump,
    )]
    pub depository_collateral_passthrough_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut
        // TODO ADD SOME KIND OF CHECKS HERE I ASSUME
    )]
    pub depository_mango_account: AccountInfo<'info>,
    // Mango related accounts -------------------------------------------------
    // XXX All these account should be properly constrained
    pub mango_group: AccountInfo<'info>,
    pub mango_cache: AccountInfo<'info>,
    pub mango_root_bank: AccountInfo<'info>,
    #[account(mut)]
    pub mango_node_bank: AccountInfo<'info>,
    #[account(mut)]
    pub mango_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mango_perp_market: AccountInfo<'info>,
    #[account(mut)]
    pub mango_bids: AccountInfo<'info>,
    #[account(mut)]
    pub mango_asks: AccountInfo<'info>,
    #[account(mut)]
    pub mango_event_queue: AccountInfo<'info>,
    // ------------------------------------------------------------------------
    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub mango_program: Program<'info, mango_program::Mango>,
    // sysvar
    pub rent: Sysvar<'info, Rent>,
}

// HANDLER
pub fn handler(
    ctx: Context<MintWithMangoDepository>,
    collateral_amount: u64,
    slippage: u32,
) -> ProgramResult {
    let collateral_mint = ctx.accounts.collateral_mint.key();

    let depository_signer_seeds: &[&[&[u8]]] = &[&[
        &MangoDepository::discriminator()[..],
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];

    // - 1 [TRANSFER COLLATERAL TO MANGO (LONG)] ------------------------------

    // msg!("Transfering user collateral to the passthrough account");
    token::transfer(
        ctx.accounts
            .into_transfer_user_collateral_to_passthrough_context(),
        collateral_amount,
    )?;

    // msg!("uxd: mint uxd [Deposit Mango CPI]");
    mango_program::deposit(
        ctx.accounts
            .into_deposit_to_mango_context()
            .with_signer(depository_signer_seeds),
        collateral_amount,
    )?;

    // - 2 [OPEN SAME SIZE SHORT POSITION ] -----------------------------------

    // - [Get perp informations]
    let perp_info = ctx.accounts.perpetual_info();
    msg!("Perpetual informations: {:?}", perp_info);

    // - [Slippage calculation]
    // This is the price of one base lot in quote lot units : `perp_info.base_lot_price_in_quote_lot_unit()`
    let base_lot_price_in_quote_lot_unit =
        slippage_deduction(perp_info.base_lot_price_in_quote_lot_unit(), slippage);
    // msg!("base_lot_price_in_quote_lot_unit (after slippage deduction): {}", base_lot_price_in_quote_lot_unit);

    // - [Calculates the quantity of base lot to open short]
    // XXX assuming USDC and Redeemable (UXD) have same decimals, need to fix
    let collateral_amount_native_unit = I80F48::from_num(collateral_amount);
    let quantity_base_lot = collateral_amount_native_unit
        .checked_div(perp_info.base_lot_size)
        .unwrap();
    // msg!("quantity_base_lot: {}", quantity_base_lot);

    // - [Position PRE perp opening to calculate the % filled later on]
    let perp_account = ctx.accounts.perp_account(&perp_info)?;
    let pre_position = perp_base_position(&perp_account);

    // - [Call mango CPI to open the perp short position]
    let order_price = base_lot_price_in_quote_lot_unit.to_num::<i64>();
    let order_quantity = quantity_base_lot.to_num::<i64>();
    // msg!("order_price {} - order_quantity {}", order_price, order_quantity);
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

    // - [Position POST perp opening to calculate the % filled later on]
    let perp_account = ctx.accounts.perp_account(&perp_info)?;
    let post_position = perp_base_position(&perp_account);

    // - [Verify that the order has been 100% filled]
    check_short_perp_open_order_fully_filled(order_quantity, pre_position, post_position)?;

    // - 3 [MINTS THE HEDGED AMOUNT OF REDEEMABLE] ----------------------------
    let redeemable_amount = derive_redeemable_amount(&perp_info, &perp_account);
    msg!("redeemable_amount minted {}", redeemable_amount);

    let controller_signer_seed: &[&[&[u8]]] = &[&[
        &Controller::discriminator()[..],
        &[ctx.accounts.controller.bump],
    ]];
    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_signer_seed),
        redeemable_amount.to_num(), 
    )?;

    Ok(())
}

impl<'info> MintWithMangoDepository<'info> {
    pub fn into_transfer_user_collateral_to_passthrough_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_collateral.to_account_info(),
            to: self
                .depository_collateral_passthrough_account
                .to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_deposit_to_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::Deposit<'info>> {
        let cpi_accounts = mango_program::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_root_bank: self.mango_root_bank.to_account_info(),
            mango_node_bank: self.mango_node_bank.to_account_info(),
            mango_vault: self.mango_vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
            owner_token_account: self
                .depository_collateral_passthrough_account
                .to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_open_mango_short_perp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::PlacePerpOrder<'info>> {
        let cpi_accounts = mango_program::PlacePerpOrder {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
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

    pub fn into_mint_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.redeemable_mint.to_account_info(),
            to: self.user_redeemable.to_account_info(),
            authority: self.controller.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Additional convenience methods related to the inputed accounts
impl<'info> MintWithMangoDepository<'info> {
    // Return general information about the perpetual related to the collateral in use
    fn perpetual_info(&self) -> PerpInfo {
        let mango_group =
            MangoGroup::load_checked(&self.mango_group, self.mango_program.key).unwrap();
        let mango_cache =
            MangoCache::load_checked(&self.mango_cache, self.mango_program.key, &mango_group)
                .unwrap();
        let perp_market_index = mango_group
            .find_perp_market_index(self.mango_perp_market.key)
            .unwrap();
        PerpInfo::init(&mango_group, &mango_cache, perp_market_index)
    }

    // Return the uncommited PerpAccount that represent the account balances
    fn perp_account(&self, perp_info: &PerpInfo) -> MangoResult<PerpAccount> {
        // - loads Mango's accounts
        let mango_account = MangoAccount::load_checked(
            &self.depository_mango_account,
            self.mango_program.key,
            self.mango_group.key,
        )?;
        Ok(mango_account.perp_accounts[perp_info.market_index])
    }
}

// Returns price after slippage deduction
fn slippage_deduction(price: I80F48, slippage: u32) -> I80F48 {
    let slippage = I80F48::from_num(slippage);
    let slippage_basis = I80F48::from_num(SLIPPAGE_BASIS);
    let slippage_ratio = slippage.checked_div(slippage_basis).unwrap();
    let slippage_amount = price.checked_mul(slippage_ratio).unwrap();
    price.checked_sub(slippage_amount).unwrap()
}

// Verify that the order quantity matches the base position delta
fn check_short_perp_open_order_fully_filled(
    order_quantity: i64,
    pre_position: i64,
    post_position: i64,
) -> ProgramResult {
    let filled_amount = (post_position.checked_sub(pre_position).unwrap()).abs();
    if !(order_quantity == filled_amount) {
        return Err(UXDError::PerpOrderPartiallyFilled.into());
    }
    Ok(())
}

// Find out the amount of redeemable the program mints for the user, derived from the outcome of the perp short opening
fn derive_redeemable_amount(perp_info: &PerpInfo, perp_account: &PerpAccount) -> I80F48 {
    // Need to add a check to make sure we don't mint more UXD than collateral value `collateral_amount_native_unit` (stupid?)
    // -
    // What is the valuation of the collateral? When we enter the instruction, do we value it from Base/ Perp price?
    // -
    // We Open a short position that tries to match that deposited collateral, but it might be smaller due to slippage.
    // We then mint on the value on this short position (To make sure everything that's minted is hedged)

    // - [Calculate the actual execution price (minus the mango fees)]
    let order_price_native_unit = I80F48::from_num(perp_account.taker_quote)
        .checked_mul(perp_info.quote_lot_size)
        .unwrap();
    msg!(
        "  derive_redeemable_amount() - order_price_native_unit {}",
        order_price_native_unit
    );

    let fees = order_price_native_unit.abs() * perp_info.taker_fee;
    msg!("  derive_redeemable_amount() - fees {}", fees);

    // XXX here it's considering UXD and USDC have same decimals -- FIX LATER
    // THIS SHOULD BE THE SPOT MARKET VALUE MINTED AND NOT THE PERP VALUE CAUSE ELSE IT'S TOO MUCH
    order_price_native_unit.checked_sub(fees).unwrap()
}

#[cfg(test)]
struct Test {}
