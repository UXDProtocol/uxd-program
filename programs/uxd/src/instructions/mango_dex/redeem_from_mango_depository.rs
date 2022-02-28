use crate::error::UxdError;
use crate::MANGO_PERP_MAX_FILL_EVENTS;
// use crate::events::RedeemFromMangoDepositoryEvent;
use crate::mango_utils::derive_order_delta;
use crate::mango_utils::limit_price;
use crate::mango_utils::price_to_lot_price;
use crate::mango_utils::PerpInfo;
use crate::Controller;
use crate::MangoDepository;
use crate::COLLATERAL_PASSTHROUGH_NAMESPACE;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use crate::SLIPPAGE_BASIS;
use anchor_comp::mango_markets_v3;
use anchor_comp::mango_markets_v3::MangoMarketV3;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Burn;
use anchor_spl::token::CloseAccount;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;
use mango::matching::OrderType;
use mango::matching::Side;
use mango::state::MangoAccount;
use mango::state::PerpAccount;

/// Takes 25 accounts - 10 used locally - 10 for MangoMarkets CPI - 4 Programs - 1 Sysvar
#[derive(Accounts)]
pub struct RedeemFromMangoDepository<'info> {
    /// #1 Public call accessible to any user
    /// Note - Mut required for WSOL unwrapping
    #[account(mut)]
    pub user: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump,
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #4 UXDProgram on chain account bound to a Controller instance.
    /// The `MangoDepository` manages a MangoAccount for a single Collateral.
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.bump,
        has_one = controller @UxdError::InvalidController,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub depository: Box<Account<'info, MangoDepository>>,

    /// #5 The collateral mint used by the `depository` instance
    /// Required to create the user_collateral ATA if needed
    #[account(
        constraint = collateral_mint.key() == depository.collateral_mint @UxdError::InvalidCollateralMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #6 The redeemable mint managed by the `controller` instance
    /// Tokens will be burnt during this instruction
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.redeemable_mint_bump,
        constraint = redeemable_mint.key() == controller.redeemable_mint @UxdError::InvalidRedeemableMint
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// #7 The `user`'s ATA for the `depository`'s `collateral_mint`
    /// Will be credited during this instruction
    #[account(
        init_if_needed,
        associated_token::mint = collateral_mint,
        associated_token::authority = user,
        payer = payer,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #8 The `user`'s ATA for the `controller`'s `redeemable_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        seeds = [user.key.as_ref(), token_program.key.as_ref(), controller.redeemable_mint.as_ref()],
        bump,
        seeds::program = AssociatedToken::id(),
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #9 The `depository`'s TA for its `collateral_mint`
    /// MangoAccounts can only transact with the TAs owned by their authority
    /// and this only serves as a passthrough
    #[account(
        mut,
        seeds = [COLLATERAL_PASSTHROUGH_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.collateral_passthrough_bump,
        constraint = depository.collateral_passthrough == depository_collateral_passthrough_account.key() @UxdError::InvalidCollateralPassthroughAccount,
    )]
    pub depository_collateral_passthrough_account: Box<Account<'info, TokenAccount>>,

    /// #10 The MangoMarkets Account (MangoAccount) managed by the `depository`
    /// CHECK : Seeds checked. Depository registered
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.mango_account_bump,
        constraint = depository.mango_account == depository_mango_account.key() @UxdError::InvalidMangoAccount,
    )]
    pub depository_mango_account: AccountInfo<'info>,

    /// #11 [MangoMarkets CPI] Index grouping perp and spot markets
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_group: UncheckedAccount<'info>,

    /// #12 [MangoMarkets CPI] Cache
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_cache: UncheckedAccount<'info>,

    /// #13 [MangoMarkets CPI] Signer PDA
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_signer: UncheckedAccount<'info>,

    /// #14 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_root_bank: UncheckedAccount<'info>,

    /// #15 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_node_bank: UncheckedAccount<'info>,

    /// #16 [MangoMarkets CPI] Vault for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_vault: UncheckedAccount<'info>,

    /// #17 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_perp_market: UncheckedAccount<'info>,

    /// #18 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook bids
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_bids: UncheckedAccount<'info>,

    /// #19 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook asks
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_asks: UncheckedAccount<'info>,

    /// #20 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market event queue
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_event_queue: UncheckedAccount<'info>,

    /// #21 System Program
    pub system_program: Program<'info, System>,

    /// #22 Token Program
    pub token_program: Program<'info, Token>,

    /// #23 Associated Token Program
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// #24 MangoMarketv3 Program
    pub mango_program: Program<'info, MangoMarketV3>,

    /// #25 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<RedeemFromMangoDepository>,
    redeemable_amount: u64,
    slippage: u16,
) -> Result<()> {
    let depository = &ctx.accounts.depository;
    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        depository.collateral_mint.as_ref(),
        &[depository.bump],
    ]];

    // - 1 [CLOSE THE EQUIVALENT PERP SHORT ON MANGO] -------------------------

    // - [Get perp information]
    let perp_info = ctx.accounts.perpetual_info()?;

    // - [Calculates the quantity of short to close]
    let quote_exposure_delta = I80F48::from_num(redeemable_amount);

    // - [Find the max taker fees mango will take on the perp order and remove it from the exposure delta to be sure the amount order + fees don't overflow the redeemed amount]
    let max_fee_amount = quote_exposure_delta
        .checked_mul(perp_info.effective_fee)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_ceil()
        .ok_or_else(|| error!(UxdError::MathError))?;
    let quote_exposure_delta_minus_fees = quote_exposure_delta
        .checked_sub(max_fee_amount)
        .ok_or_else(|| error!(UxdError::MathError))?;

    // - [Perp account state PRE perp order]
    let pre_pa = ctx.accounts.perp_account(&perp_info)?;

    // - [Find out how the best price and quantity for our order]
    let max_quote_quantity = quote_exposure_delta_minus_fees
        .checked_div(perp_info.quote_lot_size)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_to_num()
        .ok_or_else(|| error!(UxdError::MathError))?;
    // Note : Reduce the delta neutral position, increasing long exposure, by buying perp.
    //        [BID: taker (us, the caller) | ASK: maker]
    let taker_side = Side::Bid;
    let limit_price = limit_price(perp_info.price, slippage, taker_side)?;
    let limit_price_lot = price_to_lot_price(limit_price, &perp_info)?;

    // - [CPI MangoMarkets - Place perp order]
    mango_markets_v3::place_perp_order2(
        ctx.accounts
            .into_close_mango_short_perp_context()
            .with_signer(depository_signer_seed),
        taker_side,
        limit_price_lot.to_num(),
        i64::MAX,
        max_quote_quantity,
        0,
        OrderType::ImmediateOrCancel,
        true,
        None,
        MANGO_PERP_MAX_FILL_EVENTS,
    )?;

    // - [Perp account state POST perp order]
    let post_pa = ctx.accounts.perp_account(&perp_info)?;

    // - 2 [BURN REDEEMABLES] -------------------------------------------------
    if pre_pa.taker_quote < post_pa.taker_quote {
        return Err(error!(UxdError::InvalidOrderDirection));
    }
    let order_delta = derive_order_delta(&pre_pa, &post_pa, &perp_info)?;
    msg!("order_delta {:?}", order_delta);

    // The resulting UXD amount is equal to the quote delta plus the fees.
    // By burning the amount plus the fees, the user is paying them. (and we made sure
    // in the beginning to downsize the redeemable amount to keep enough space for these)
    let redeemable_delta = order_delta
        .quote
        .checked_add(order_delta.fee)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_abs()
        .ok_or_else(|| error!(UxdError::MathError))?;
    let redeemable_burn_amount = redeemable_delta
        .checked_to_num()
        .ok_or_else(|| error!(UxdError::MathError))?;

    token::burn(
        ctx.accounts.into_burn_redeemable_context(),
        redeemable_burn_amount,
    )?;

    // - 3 [WITHDRAW COLLATERAL FROM MANGO THEN RETURN TO USER] ---------------
    let collateral_withdraw_amount = order_delta
        .base
        .checked_abs()
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_to_num()
        .ok_or_else(|| error!(UxdError::MathError))?;

    // - [CPI MangoMarkets - Withdraw]
    mango_markets_v3::withdraw(
        ctx.accounts
            .into_withdraw_collateral_from_mango_context()
            .with_signer(depository_signer_seed),
        collateral_withdraw_amount,
        false,
    )?;

    // - [Return collateral back to user ATA]
    token::transfer(
        ctx.accounts
            .into_transfer_collateral_to_user_context()
            .with_signer(depository_signer_seed),
        collateral_withdraw_amount,
    )?;

    // - [If ATA mint is WSOL, unwrap]
    if depository.collateral_mint == spl_token::native_mint::id() {
        token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;
    }

    // - 4 [UPDATE ACCOUNTING] ------------------------------------------------

    ctx.accounts.update_onchain_accounting(
        collateral_withdraw_amount.into(),
        redeemable_burn_amount.into(),
        order_delta.fee.abs().to_num(),
    )?;

    // emit!(RedeemFromMangoDepositoryEvent {
    //     version: controller.version,
    //     controller: controller.key(),
    //     depository: depository.key(),
    //     user: ctx.accounts.user.key(),
    //     redeemable_amount,
    //     slippage,
    //     base_delta: order_delta.base.to_num(),
    //     quote_delta: order_delta.quote.to_num(),
    //     fee_delta: order_delta.fee.to_num(),
    // });

    Ok(())
}

// MARK: - Contexts -----

impl<'info> RedeemFromMangoDepository<'info> {
    pub fn into_burn_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.redeemable_mint.to_account_info(),
            to: self.user_redeemable.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_close_mango_short_perp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::PlacePerpOrder2<'info>> {
        let cpi_accounts = mango_markets_v3::PlacePerpOrder2 {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            perp_market: self.mango_perp_market.to_account_info(),
            bids: self.mango_bids.to_account_info(),
            asks: self.mango_asks.to_account_info(),
            event_queue: self.mango_event_queue.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_withdraw_collateral_from_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Withdraw<'info>> {
        let cpi_accounts = mango_markets_v3::Withdraw {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_root_bank.to_account_info(),
            node_bank: self.mango_node_bank.to_account_info(),
            vault: self.mango_vault.to_account_info(),
            token_account: self
                .depository_collateral_passthrough_account
                .to_account_info(),
            signer: self.mango_signer.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_collateral_to_user_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::Transfer {
            from: self
                .depository_collateral_passthrough_account
                .to_account_info(),
            to: self.user_collateral.to_account_info(),
            authority: self.depository.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_unwrap_wsol_by_closing_ata_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount {
            account: self.user_collateral.to_account_info(),
            destination: self.user.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Additional convenience methods related to the inputted accounts
impl<'info> RedeemFromMangoDepository<'info> {
    // Return general information about the perpetual related to the collateral in use
    fn perpetual_info(&self) -> Result<PerpInfo> {
        let perp_info = PerpInfo::new(
            &self.mango_group,
            &self.mango_cache,
            &self.depository_mango_account,
            self.mango_perp_market.key,
            self.mango_group.key,
            self.mango_program.key,
        )?;
        // msg!("perp_info {:?}", perp_info);
        Ok(perp_info)
    }

    // Return the uncommitted PerpAccount that represent the account balances
    fn perp_account(&self, perp_info: &PerpInfo) -> Result<PerpAccount> {
        // - loads Mango's accounts
        let mango_account = MangoAccount::load_checked(
            &self.depository_mango_account,
            self.mango_program.key,
            self.mango_group.key,
        )
        .map_err(|me| ProgramError::from(me))?;
        Ok(mango_account.perp_accounts[perp_info.market_index])
    }

    // Update the accounting in the Depository and Controller Accounts to reflect changes
    fn update_onchain_accounting(
        &mut self,
        collateral_withdrawn_amount: u128,
        redeemable_burnt_amount: u128,
        fee_amount: u128,
    ) -> Result<()> {
        let depository = &mut self.depository;
        let controller = &mut self.controller;
        // Mango Depository
        depository.collateral_amount_deposited = depository
            .collateral_amount_deposited
            .checked_sub(collateral_withdrawn_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.redeemable_amount_under_management = depository
            .redeemable_amount_under_management
            .checked_add(redeemable_burnt_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.total_amount_paid_taker_fee = depository
            .total_amount_paid_taker_fee
            .wrapping_add(fee_amount);
        // Controller
        controller.redeemable_circulating_supply = controller
            .redeemable_circulating_supply
            .checked_add(redeemable_burnt_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        Ok(())
    }
}

// Validate input arguments
impl<'info> RedeemFromMangoDepository<'info> {
    pub fn validate(&self, redeemable_amount: u64, slippage: u16) -> Result<()> {
        if slippage > SLIPPAGE_BASIS {
            return Err(error!(UxdError::InvalidSlippage));
        }
        if redeemable_amount == 0 {
            return Err(error!(UxdError::InvalidRedeemableAmount));
        }
        if self.user_redeemable.amount < redeemable_amount {
            return Err(error!(UxdError::InsufficientRedeemableAmount));
        }
        Ok(())
    }
}
