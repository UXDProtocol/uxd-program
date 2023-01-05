use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

use crate::error::UxdError;
use crate::events::InitializeIdentityDepositoryEvent;
use crate::state::identity_depository::IdentityDepository;
use crate::state::identity_depository::IDENTITY_DEPOSITORY_SPACE;
use crate::validate_is_program_frozen;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::DEFAULT_REDEEMABLE_UNDER_MANAGEMENT_CAP;
use crate::IDENTITY_DEPOSITORY_ACCOUNT_VERSION;
use crate::IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_NAMESPACE;

#[derive(Accounts)]
pub struct InitializeIdentityDepository<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4 UXDProgram on chain account bound to a Controller instance
    #[account(
        init,
        seeds = [IDENTITY_DEPOSITORY_NAMESPACE], // Only a single instance per controller instance
        bump,
        payer = payer,
        space = IDENTITY_DEPOSITORY_SPACE,
    )]
    pub depository: AccountLoader<'info, IdentityDepository>,

    /// #5
    /// Token account holding the collateral from minting
    #[account(
        init,
        seeds = [IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE],
        token::authority = depository,
        token::mint = collateral_mint,
        bump,
        payer = payer,
    )]
    pub collateral_vault: Account<'info, TokenAccount>,

    /// #6 The collateral mint used by the `depository` instance
    pub collateral_mint: Account<'info, Mint>,

    /// #7 System Program
    pub system_program: Program<'info, System>,

    /// #8 Token Program
    pub token_program: Program<'info, Token>,

    /// #9 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub(crate) fn handler(ctx: Context<InitializeIdentityDepository>) -> Result<()> {
    let depository_bump = *ctx
        .bumps
        .get("depository")
        .ok_or_else(|| error!(UxdError::BumpError))?;
    let collateral_vault_bump = *ctx
        .bumps
        .get("collateral_vault")
        .ok_or_else(|| error!(UxdError::BumpError))?;

    let redeemable_mint_unit = 10_u128
        .checked_pow(
            ctx.accounts
                .controller
                .load()?
                .redeemable_mint_decimals
                .into(),
        )
        .ok_or_else(|| error!(UxdError::MathError))?;

    // - Initialize Depository state
    let depository = &mut ctx.accounts.depository.load_init()?;
    depository.bump = depository_bump;
    depository.version = IDENTITY_DEPOSITORY_ACCOUNT_VERSION;
    depository.collateral_mint = ctx.accounts.collateral_mint.key();
    depository.collateral_mint_decimals = ctx.accounts.collateral_mint.decimals;
    depository.collateral_amount_deposited = u128::MIN;
    depository.collateral_vault = ctx.accounts.collateral_vault.key();
    depository.collateral_vault_bump = collateral_vault_bump;
    depository.redeemable_amount_under_management = u128::MIN;
    depository.redeemable_amount_under_management_cap = DEFAULT_REDEEMABLE_UNDER_MANAGEMENT_CAP
        .checked_mul(redeemable_mint_unit)
        .ok_or_else(|| error!(UxdError::MathError))?;
    depository.minting_disabled = true;

    depository.mango_collateral_reinjected_wsol = false;
    depository.mango_collateral_reinjected_eth = false;
    depository.mango_collateral_reinjected_btc = false;

    emit!(InitializeIdentityDepositoryEvent {
        version: ctx.accounts.controller.load()?.version,
        depository_version: depository.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
    });

    Ok(())
}

// Validate input arguments
impl<'info> InitializeIdentityDepository<'info> {
    // Only usdc should be allowed as the collateral mint of this depository
    pub fn validate_collateral_mint(&self) -> Result<()> {
        let usdc_mint: Pubkey =
            Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();

        require!(
            self.collateral_mint.key().eq(&usdc_mint),
            UxdError::CollateralMintNotAllowed,
        );

        Ok(())
    }

    pub(crate) fn validate(&self) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;

        // Collateral mint and redeemable mint should share the same decimals to justify their 1:1 swapping
        require!(
            self.collateral_mint
                .decimals
                .eq(&self.controller.load()?.redeemable_mint_decimals),
            UxdError::CollateralMintNotAllowed,
        );

        #[cfg(feature = "production")]
        self.validate_collateral_mint()?;

        Ok(())
    }
}
