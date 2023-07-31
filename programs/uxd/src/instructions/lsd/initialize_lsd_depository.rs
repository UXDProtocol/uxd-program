use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

use crate::error::UxdError;
use crate::events::InitializeLsdDepositoryEvent;
use crate::state::LsdDepository;
use crate::state::LSD_DEPOSITORY_SPACE;
use crate::validate_is_program_frozen;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::DEFAULT_LSD_LIQUIDATION_FEE_BPS;
use crate::DEFAULT_LSD_LIQUIDATION_LTV_THRESHOLD_BPS;
use crate::DEFAULT_LSD_MAX_LTV_BPS;
use crate::DEFAULT_REDEEMABLE_UNDER_MANAGEMENT_CAP;
use crate::LSD_DEPOSITORY_ACCOUNT_VERSION;
use crate::LSD_DEPOSITORY_NAMESPACE;
use crate::LSD_PROFITS_TOKEN_ACCOUNT_NAMESPACE;

#[derive(Accounts)]
pub struct InitializeLsdDepository<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4
    #[account(
        init,
        seeds = [LSD_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump,
        payer = payer,
        space = LSD_DEPOSITORY_SPACE,
    )]
    pub depository: AccountLoader<'info, LsdDepository>,

    /// #5
    pub collateral_mint: Account<'info, Mint>,

    /// #6
    pub liquidation_mint: Account<'info, Mint>,

    /// #7
    #[account(
        init,
        seeds = [LSD_PROFITS_TOKEN_ACCOUNT_NAMESPACE, depository.key().as_ref()],
        token::authority = depository,
        token::mint = liquidation_mint,
        bump,
        payer = payer,
    )]
    pub profits_token_account: Account<'info, TokenAccount>,

    /// #6
    pub system_program: Program<'info, System>,

    /// #7
    pub token_program: Program<'info, Token>,

    /// #8
    pub rent: Sysvar<'info, Rent>,
}

pub(crate) fn handler(ctx: Context<InitializeLsdDepository>) -> Result<()> {
    let depository_bump = *ctx
        .bumps
        .get("depository")
        .ok_or_else(|| error!(UxdError::BumpError))?;
    let profits_token_account_bump = *ctx
        .bumps
        .get("profits_token_account")
        .ok_or_else(|| error!(UxdError::BumpError))?;

    let redeemable_mint_unit = 10_u64
        .checked_pow(
            ctx.accounts
                .controller
                .load()?
                .redeemable_mint_decimals
                .into(),
        )
        .ok_or_else(|| error!(UxdError::MathOverflow))?;

    let depository = &mut ctx.accounts.depository.load_init()?;
    depository.bump = depository_bump;
    depository.version = LSD_DEPOSITORY_ACCOUNT_VERSION;
    depository.collateral_mint = ctx.accounts.collateral_mint.key();
    depository.collateral_mint_decimals = ctx.accounts.collateral_mint.decimals;
    depository.liquidation_mint = ctx.accounts.liquidation_mint.key();
    depository.liquidation_mint_decimals = ctx.accounts.liquidation_mint.decimals;
    depository.profits_token_account = ctx.accounts.profits_token_account.key();
    depository.profits_token_account_bump = profits_token_account_bump;

    // Configuration
    depository.borrowing_disabled = true;
    depository.redeemable_amount_under_management_cap = DEFAULT_REDEEMABLE_UNDER_MANAGEMENT_CAP
        .checked_mul(redeemable_mint_unit)
        .ok_or_else(|| error!(UxdError::MathOverflow))?;
    depository.borrowing_fee_bps = u8::MIN;
    depository.repay_fee_bps = u8::MIN;
    depository.max_loan_to_value_bps = DEFAULT_LSD_MAX_LTV_BPS;
    depository.liquidation_loan_to_value_threshold_bps = DEFAULT_LSD_LIQUIDATION_LTV_THRESHOLD_BPS;
    depository.liquidation_fee_bps = DEFAULT_LSD_LIQUIDATION_FEE_BPS;
    depository.profits_beneficiary = Pubkey::default();
    // TODO: initialize `collateral_oracle_params`

    // Accounting
    depository.collateral_amount_deposits = u64::MIN;
    depository.redeemable_amount_under_management = u64::MIN;

    // Stats
    depository.collateral_amount_liquidated = u128::MIN;
    depository.borrow_fee_accrued = u128::MIN;
    depository.repay_fee_accrued = u128::MIN;
    depository.liquidation_fee_accrued = u128::MIN;
    depository.profits_collected = u128::MIN;

    emit!(InitializeLsdDepositoryEvent {
        version: ctx.accounts.controller.load()?.version,
        depository_version: depository.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
        liquidation_mint: ctx.accounts.liquidation_mint.key(),
    });

    Ok(())
}

// Validate input arguments
impl<'info> InitializeLsdDepository<'info> {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;

        Ok(())
    }
}
