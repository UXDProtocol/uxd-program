use crate::error::UxdError;
use crate::events::RedeemFromMangoDepositoryEvent;
use crate::mango_utils::derive_order_delta;
use crate::mango_utils::price_to_lot_price;
use crate::mango_utils::PerpInfo;
use crate::validate_mango_group;
use crate::validate_perp_market_mints_matches_depository_mints;
use crate::Controller;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::MANGO_PERP_MAX_FILL_EVENTS;
use crate::REDEEMABLE_MINT_NAMESPACE;
use anchor_comp::mango_markets_v3;
use anchor_comp::mango_markets_v3::MangoMarketV3;
use anchor_lang::prelude::*;
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
use num_traits::identities::Zero;

/// Takes 22 accounts
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
        bump = controller.load()?.bump,
        constraint = controller.load()?.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
        has_one = redeemable_mint @UxdError::InvalidRedeemableMint
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4 UXDProgram on chain account bound to a Controller instance.
    /// The `MangoDepository` manages a MangoAccount for a single Collateral.
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = mango_account @UxdError::InvalidMangoAccount,
        has_one = collateral_mint @UxdError::InvalidCollateralMint
    )]
    pub depository: AccountLoader<'info, MangoDepository>,

    /// #5 The collateral mint used by the `depository` instance
    /// Required to create the user_collateral ATA if needed
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #6 The redeemable mint managed by the `controller` instance
    /// Tokens will be burnt during this instruction
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.load()?.redeemable_mint_bump,
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// #7 The `user`'s ATA for the `depository`'s `collateral_mint`
    /// Will be credited during this instruction
    #[account(
        mut,
        constraint = user_collateral.mint == depository.load()?.collateral_mint @UxdError::InvalidCollateralMint,
        constraint = &user_collateral.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #8 The `user`'s ATA for the `controller`'s `redeemable_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        constraint = user_redeemable.mint == controller.load()?.redeemable_mint @UxdError::InvalidRedeemableMint,
        constraint = &user_redeemable.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #9 The MangoMarkets Account (MangoAccount) managed by the `depository`
    /// CHECK : Seeds checked. Depository registered
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.mango_account_bump,
    )]
    pub mango_account: AccountInfo<'info>,

    /// #10 [MangoMarkets CPI] Index grouping perp and spot markets
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_group: UncheckedAccount<'info>,

    /// #11 [MangoMarkets CPI] Cache
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_cache: UncheckedAccount<'info>,

    /// #12 [MangoMarkets CPI] Signer PDA
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_signer: UncheckedAccount<'info>,

    /// #13 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_root_bank: UncheckedAccount<'info>,

    /// #14 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_node_bank: UncheckedAccount<'info>,

    /// #15 [MangoMarkets CPI] Vault for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_vault: UncheckedAccount<'info>,

    /// #16 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_perp_market: UncheckedAccount<'info>,

    /// #17 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook bids
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_bids: UncheckedAccount<'info>,

    /// #18 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook asks
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_asks: UncheckedAccount<'info>,

    /// #19 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market event queue
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_event_queue: UncheckedAccount<'info>,

    /// #20 System Program
    pub system_program: Program<'info, System>,

    /// #21 Token Program
    pub token_program: Program<'info, Token>,

    /// #22 MangoMarketv3 Program
    pub mango_program: Program<'info, MangoMarketV3>,
}

pub(crate) fn handler(
    ctx: Context<RedeemFromMangoDepository>,
    redeemable_amount: u64,
    limit_price: f32,
) -> Result<()> {
    let depository = ctx.accounts.depository.load()?;
    let collateral_mint = depository.collateral_mint;
    let depository_bump = depository.bump;
    drop(depository);

    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[depository_bump],
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

    let max_quote_quantity: i64 = quote_exposure_delta_minus_fees
        .checked_div(perp_info.quote_lot_size)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_to_num()
        .ok_or_else(|| error!(UxdError::MathError))?;

    require!(
        !max_quote_quantity.is_zero(),
        UxdError::QuantityBelowContractSize
    );

    // Note : Reduce the delta neutral position, increasing long exposure, by buying perp.
    //        [BID: taker (us, the caller) | ASK: maker]
    let taker_side = Side::Bid;
    let limit_price_fixed =
        I80F48::checked_from_num(limit_price).ok_or_else(|| error!(UxdError::MathError))?;
    let limit_price_lot = price_to_lot_price(limit_price_fixed, &perp_info)?;

    // - [CPI MangoMarkets - Place perp order]
    mango_markets_v3::place_perp_order2(
        ctx.accounts
            .to_close_mango_short_perp_context()
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
    require!(
        pre_pa.taker_quote >= post_pa.taker_quote,
        UxdError::InvalidOrderDirection
    );
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
        ctx.accounts.to_burn_redeemable_context(),
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
            .to_withdraw_collateral_from_mango_context()
            .with_signer(depository_signer_seed),
        collateral_withdraw_amount,
        false,
    )?;

    // - [If ATA mint is WSOL, unwrap]
    if collateral_mint == spl_token::native_mint::id() {
        token::close_account(ctx.accounts.to_unwrap_wsol_by_closing_ata_context())?;
    }

    // - 4 [UPDATE ACCOUNTING] ------------------------------------------------

    ctx.accounts.update_onchain_accounting(
        collateral_withdraw_amount.into(),
        redeemable_burn_amount.into(),
        order_delta.fee.abs().to_num(),
    )?;

    emit!(RedeemFromMangoDepositoryEvent {
        version: ctx.accounts.controller.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        user: ctx.accounts.user.key(),
        redeemable_amount,
        limit_price,
        base_delta: order_delta.base.to_num(),
        quote_delta: order_delta.quote.to_num(),
        fee_delta: order_delta.fee.to_num(),
    });

    Ok(())
}

// MARK: - Contexts -----

impl<'info> RedeemFromMangoDepository<'info> {
    fn to_burn_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.redeemable_mint.to_account_info(),
            from: self.user_redeemable.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn to_close_mango_short_perp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::PlacePerpOrder2<'info>> {
        let cpi_accounts = mango_markets_v3::PlacePerpOrder2 {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
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

    fn to_withdraw_collateral_from_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Withdraw<'info>> {
        let cpi_accounts = mango_markets_v3::Withdraw {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_root_bank.to_account_info(),
            node_bank: self.mango_node_bank.to_account_info(),
            vault: self.mango_vault.to_account_info(),
            token_account: self.user_collateral.to_account_info(),
            signer: self.mango_signer.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn to_unwrap_wsol_by_closing_ata_context(
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
            &self.mango_account,
            self.mango_perp_market.key,
            self.mango_group.key,
            self.mango_program.key,
        )?;
        msg!("perp_info {:?}", perp_info);
        Ok(perp_info)
    }

    // Return the uncommitted PerpAccount that represent the account balances
    fn perp_account(&self, perp_info: &PerpInfo) -> Result<PerpAccount> {
        // - loads Mango's accounts
        let mango_account = MangoAccount::load_checked(
            &self.mango_account,
            self.mango_program.key,
            self.mango_group.key,
        )
        .map_err(ProgramError::from)?;
        Ok(mango_account.perp_accounts[perp_info.market_index])
    }

    // Update the accounting in the Depository and Controller Accounts to reflect changes
    fn update_onchain_accounting(
        &mut self,
        collateral_withdrawn_amount: u128,
        redeemable_burnt_amount: u128,
        fee_amount: u128,
    ) -> Result<()> {
        let depository = &mut self.depository.load_mut()?;
        let controller = &mut self.controller.load_mut()?;
        // Mango Depository
        depository.collateral_amount_deposited = depository
            .collateral_amount_deposited
            .checked_sub(collateral_withdrawn_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.redeemable_amount_under_management = depository
            .redeemable_amount_under_management
            .checked_sub(redeemable_burnt_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.total_amount_paid_taker_fee = depository
            .total_amount_paid_taker_fee
            .checked_add(fee_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        // Controller
        controller.redeemable_circulating_supply = controller
            .redeemable_circulating_supply
            .checked_sub(redeemable_burnt_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        Ok(())
    }
}

// Validate input arguments
impl<'info> RedeemFromMangoDepository<'info> {
    pub(crate) fn validate(&self, redeemable_amount: u64, limit_price: f32) -> Result<()> {
        require!(limit_price > 0f32, UxdError::InvalidLimitPrice);
        require!(redeemable_amount != 0, UxdError::InvalidRedeemableAmount);
        require!(
            self.user_redeemable.amount >= redeemable_amount,
            UxdError::InsufficientRedeemableAmount
        );

        validate_mango_group(self.mango_group.key())?;

        validate_perp_market_mints_matches_depository_mints(
            &self.mango_group,
            self.mango_program.key,
            self.mango_perp_market.key,
            &self.depository.load()?.collateral_mint,
            &self.depository.load()?.quote_mint,
        )?;
        Ok(())
    }
}
