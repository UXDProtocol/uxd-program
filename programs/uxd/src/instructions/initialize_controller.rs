use crate::error::UxdError;
use crate::events::InitializeControllerEvent;
use crate::Controller;
use crate::CONTROLLER_ACCOUNT_VERSION;
use crate::CONTROLLER_NAMESPACE;
use crate::CONTROLLER_SPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use crate::SOLANA_MAX_MINT_DECIMALS;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;

/// Takes 7 accounts - 4 used locally - 0 for CPI - 2 Programs - 1 Sysvar
#[derive(Accounts)]
#[instruction(
    redeemable_mint_decimals: u8,
)]
pub struct InitializeController<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        init,
        seeds = [CONTROLLER_NAMESPACE],
        bump,
        payer = payer,
        space = CONTROLLER_SPACE
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4 The redeemable mint managed by the `controller` instance
    #[account(
        init,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump,
        mint::authority = controller,
        mint::decimals = redeemable_mint_decimals,
        payer = payer,
        constraint = redeemable_mint_decimals <= SOLANA_MAX_MINT_DECIMALS
    )]
    pub redeemable_mint: Account<'info, Mint>,

    /// #5 System Program
    pub system_program: Program<'info, System>,

    /// #6 Token Program
    pub token_program: Program<'info, Token>,

    /// #7 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub(crate) fn handler(
    ctx: Context<InitializeController>,
    redeemable_mint_decimals: u8,
) -> Result<()> {
    let controller = &mut ctx.accounts.controller.load_init()?;

    controller.bump = *ctx
        .bumps
        .get("controller")
        .ok_or_else(|| error!(UxdError::BumpError))?;
    controller.redeemable_mint_bump = *ctx
        .bumps
        .get("redeemable_mint")
        .ok_or_else(|| error!(UxdError::BumpError))?;
    controller.version = CONTROLLER_ACCOUNT_VERSION;
    controller.authority = ctx.accounts.authority.key();
    controller.redeemable_mint = ctx.accounts.redeemable_mint.key();
    controller.redeemable_mint_decimals = redeemable_mint_decimals;
    controller.redeemable_global_supply_cap = 0;
    controller.redeemable_circulating_supply = u128::MIN;
    controller.is_frozen = false;
    controller.profits_total_collected = u128::MIN;

    // Routing/balancing fields must be set by edit_controller for the routing to start working
    // Those default values will make any router-based mint/redeem impossible, on purpose.
    controller.identity_depository_weight_bps = 0;
    controller.mercurial_vault_depository_weight_bps = 0;
    controller.credix_lp_depository_weight_bps = 0;
    controller.identity_depository = Pubkey::default();
    controller.mercurial_vault_depository = Pubkey::default();
    controller.credix_lp_depository = Pubkey::default();

    // Routing outflow limitation flags
    controller.outflow_limit_per_epoch_amount = 0;
    controller.outflow_limit_per_epoch_bps = 0;
    controller.slots_per_epoch = 0;
    controller.epoch_outflow_amount = 0;
    controller.last_outflow_slot = 0;

    emit!(InitializeControllerEvent {
        version: controller.version,
        controller: ctx.accounts.controller.key(),
        authority: ctx.accounts.authority.key(),
    });
    Ok(())
}

// Validate input arguments
impl<'info> InitializeController<'info> {
    // Asserts that the redeemable mint decimals is between 0 and 9.
    pub(crate) fn validate(&self, decimals: u8) -> Result<()> {
        require!(
            decimals <= SOLANA_MAX_MINT_DECIMALS,
            UxdError::InvalidRedeemableMintDecimals
        );
        Ok(())
    }
}
