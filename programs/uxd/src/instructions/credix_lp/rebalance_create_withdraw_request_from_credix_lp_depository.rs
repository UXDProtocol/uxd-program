use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::TokenAccount;

use crate::error::UxdError;
use crate::events::RebalanceCreateWithdrawRequestFromCredixLpDepositoryEvent;
use crate::state::controller::Controller;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::state::identity_depository::IdentityDepository;
use crate::state::mercurial_vault_depository::MercurialVaultDepository;
use crate::state::AlloyxVaultDepository;
use crate::utils::calculate_credix_lp_depository_target_amount;
use crate::utils::checked_add;
use crate::utils::checked_as_u64;
use crate::utils::checked_sub;
use crate::utils::compute_value_for_shares_amount_floor;
use crate::validate_is_program_frozen;
use crate::CONTROLLER_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_NAMESPACE;
use crate::CREDIX_LP_EXTERNAL_PASS_NAMESPACE;
use crate::CREDIX_LP_EXTERNAL_WITHDRAW_EPOCH_NAMESPACE;

#[derive(Accounts)]
pub struct RebalanceCreateWithdrawRequestFromCredixLpDepository<'info> {
    /// #1
    /// Permissionless IX that can be called by anyone at any time
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #2
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.identity_depository == identity_depository.key() @UxdError::InvalidDepository,
        constraint = controller.load()?.mercurial_vault_depository == mercurial_vault_depository.key() @UxdError::InvalidDepository,
        constraint = controller.load()?.credix_lp_depository == credix_lp_depository.key() @UxdError::InvalidDepository,
        constraint = controller.load()?.alloyx_vault_depository == alloyx_vault_depository.key() @UxdError::InvalidDepository,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #6
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #3
    #[account(
        mut,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
    )]
    pub identity_depository: AccountLoader<'info, IdentityDepository>,

    /// #4
    #[account(
        mut,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
    )]
    pub mercurial_vault_depository: AccountLoader<'info, MercurialVaultDepository>,

    /// #5
    #[account(
        mut,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        constraint = credix_lp_depository.load()?.depository_shares == credix_lp_depository_shares.key() @UxdError::InvalidDepositoryShares,
        has_one = credix_global_market_state @UxdError::InvalidCredixGlobalMarketState,
        has_one = credix_signing_authority @UxdError::InvalidCredixSigningAuthority,
        has_one = credix_liquidity_collateral @UxdError::InvalidCredixLiquidityCollateral,
        has_one = credix_shares_mint @UxdError::InvalidCredixSharesMint,
    )]
    pub credix_lp_depository: AccountLoader<'info, CredixLpDepository>,

    /// #7
    #[account(mut)]
    pub credix_lp_depository_shares: Box<Account<'info, TokenAccount>>,

    /// #8
    pub credix_global_market_state: Box<Account<'info, credix_client::GlobalMarketState>>,

    /// #9
    /// CHECK: unused by us, checked by credix
    pub credix_signing_authority: AccountInfo<'info>,

    /// #10
    #[account(mut)]
    pub credix_liquidity_collateral: Box<Account<'info, TokenAccount>>,

    /// #11
    #[account(mut)]
    pub credix_shares_mint: Box<Account<'info, Mint>>,

    /// #12
    #[account(
        owner = credix_client::ID,
        seeds = [
            credix_global_market_state.key().as_ref(),
            credix_lp_depository.key().as_ref(),
            CREDIX_LP_EXTERNAL_PASS_NAMESPACE
        ],
        bump,
        seeds::program = credix_client::ID,
        constraint = credix_pass.user == credix_lp_depository.key() @UxdError::InvalidCredixPass,
        constraint = credix_pass.disable_withdrawal_fee() @UxdError::InvalidCredixPassNoFees,
    )]
    pub credix_pass: Account<'info, credix_client::CredixPass>,

    /// #13
    #[account(
        mut,
        owner = credix_client::ID,
        seeds = [
            credix_global_market_state.key().as_ref(),
            &credix_global_market_state.latest_withdraw_epoch_idx.to_le_bytes(),
            CREDIX_LP_EXTERNAL_WITHDRAW_EPOCH_NAMESPACE
        ],
        bump,
        seeds::program = credix_client::ID,
    )]
    pub credix_withdraw_epoch: Account<'info, credix_client::WithdrawEpoch>,

    /// #6
    #[account(
        mut,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
    )]
    pub alloyx_vault_depository: AccountLoader<'info, AlloyxVaultDepository>,

    /// #14
    pub system_program: Program<'info, System>,
    /// #15
    pub credix_program: Program<'info, credix_client::program::Credix>,
}

pub(crate) fn handler(
    ctx: Context<RebalanceCreateWithdrawRequestFromCredixLpDepository>,
) -> Result<()> {
    let credix_global_market_state = ctx
        .accounts
        .credix_lp_depository
        .load()?
        .credix_global_market_state;
    let collateral_mint = ctx.accounts.credix_lp_depository.load()?.collateral_mint;
    let depository_pda_signer: &[&[&[u8]]] = &[&[
        CREDIX_LP_DEPOSITORY_NAMESPACE,
        credix_global_market_state.as_ref(),
        collateral_mint.as_ref(),
        &[ctx.accounts.credix_lp_depository.load()?.bump],
    ]];

    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Check if the withdraw epoch request's phase is active at the moment
    // ---------------------------------------------------------------------

    require!(
        ctx.accounts.credix_withdraw_epoch.status()?
            == credix_client::WithdrawEpochStatus::RequestPhase,
        UxdError::InvalidCredixWithdrawEpochRequestPhase,
    );

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Compute the profits and overflow amounts of the credix_lp_depository
    // ---------------------------------------------------------------------

    let redeemable_amount_under_management = checked_as_u64(
        ctx.accounts
            .credix_lp_depository
            .load()?
            .redeemable_amount_under_management,
    )?;

    let profits_collateral_amount = {
        let liquidity_collateral_amount: u64 = ctx.accounts.credix_liquidity_collateral.amount;
        let outstanding_collateral_amount: u64 = ctx
            .accounts
            .credix_global_market_state
            .pool_outstanding_credit;
        let total_shares_supply: u64 = ctx.accounts.credix_shares_mint.supply;
        let total_shares_value: u64 =
            checked_add(liquidity_collateral_amount, outstanding_collateral_amount)?;
        let owned_shares_amount: u64 = ctx.accounts.credix_lp_depository_shares.amount;
        let owned_shares_value: u64 = compute_value_for_shares_amount_floor(
            owned_shares_amount,
            total_shares_supply,
            total_shares_value,
        )?;
        checked_sub(owned_shares_value, redeemable_amount_under_management)?
    };

    let overflow_value = {
        let redeemable_amount_under_management_target_amount =
            calculate_credix_lp_depository_target_amount(
                &ctx.accounts.controller,
                &ctx.accounts.identity_depository,
                &ctx.accounts.mercurial_vault_depository,
                &ctx.accounts.credix_lp_depository,
                &ctx.accounts.alloyx_vault_depository,
            )?;
        if redeemable_amount_under_management < redeemable_amount_under_management_target_amount {
            0
        } else {
            checked_sub(
                redeemable_amount_under_management,
                redeemable_amount_under_management_target_amount,
            )?
        }
    };

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- We want to withdraw the sum of the profits and overflow at the same time
    // -- We just create the credix withdraw request with the computed withdrawal amount
    // ---------------------------------------------------------------------

    let requested_collateral_amount = checked_add(profits_collateral_amount, overflow_value)?;
    msg!(
        "[rebalance_create_withdraw_request_from_credix_lp_depository:requested_collateral_amount:{}]",
        requested_collateral_amount
    );
    credix_client::cpi::create_withdraw_request(
        ctx.accounts
            .into_create_withdraw_request_from_credix_lp_context()
            .with_signer(depository_pda_signer),
        requested_collateral_amount,
    )?;

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- Emit resulting event
    // ---------------------------------------------------------------------

    emit!(RebalanceCreateWithdrawRequestFromCredixLpDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: ctx.accounts.credix_lp_depository.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.credix_lp_depository.key(),
        overflow_value,
        profits_collateral_amount,
        requested_collateral_amount,
    });

    // Done
    Ok(())
}

// Into functions
impl<'info> RebalanceCreateWithdrawRequestFromCredixLpDepository<'info> {
    pub fn into_create_withdraw_request_from_credix_lp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, credix_client::cpi::accounts::CreateWithdrawRequest<'info>>
    {
        let cpi_accounts = credix_client::cpi::accounts::CreateWithdrawRequest {
            payer: self.payer.to_account_info(),
            investor: self.credix_lp_depository.to_account_info(),
            investor_lp_token_account: self.credix_lp_depository_shares.to_account_info(),
            global_market_state: self.credix_global_market_state.to_account_info(),
            signing_authority: self.credix_signing_authority.to_account_info(),
            liquidity_pool_token_account: self.credix_liquidity_collateral.to_account_info(),
            lp_token_mint: self.credix_shares_mint.to_account_info(),
            credix_pass: self.credix_pass.to_account_info(),
            withdraw_epoch: self.credix_withdraw_epoch.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        let cpi_program = self.credix_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Validate
impl<'info> RebalanceCreateWithdrawRequestFromCredixLpDepository<'info> {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        Ok(())
    }
}
