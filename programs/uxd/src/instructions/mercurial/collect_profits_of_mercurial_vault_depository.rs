use crate::error::UxdError;
use crate::events::CollectProfitsOfMercurialVaultDepositoryEvent;
use crate::mercurial_utils;
use crate::mercurial_utils::check_collateral_value_changed_to_match_target;
use crate::utils::compute_decrease;
use crate::utils::compute_increase;
use crate::validate_is_program_frozen;
use crate::Controller;
use crate::MercurialVaultDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct CollectProfitsOfMercurialVaultDepository<'info> {
    /// #1
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #2
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.registered_mercurial_vault_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #3
    #[account(
        mut,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_NAMESPACE, depository.load()?.mercurial_vault.key().as_ref(), depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = mercurial_vault @UxdError::InvalidMercurialVault,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        has_one = mercurial_vault_lp_mint @UxdError::InvalidMercurialVaultLpMint,
        has_one = profits_beneficiary_collateral @UxdError::InvalidProfitsBeneficiaryCollateral,
        constraint = depository.load()?.lp_token_vault == depository_lp_token_vault.key() @UxdError::InvalidDepositoryLpTokenVault,
    )]
    pub depository: AccountLoader<'info, MercurialVaultDepository>,

    /// #4
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #5
    #[account(
        mut,
        constraint = profits_beneficiary_collateral.mint == depository.load()?.collateral_mint @UxdError::InvalidCollateralMint,
    )]
    pub profits_beneficiary_collateral: Box<Account<'info, TokenAccount>>,

    /// #6
    /// Token account holding the LP tokens minted by depositing collateral on mercurial vault
    #[account(mut)]
    pub depository_lp_token_vault: Box<Account<'info, TokenAccount>>,

    /// #7
    #[account(mut)]
    pub mercurial_vault: Box<Account<'info, mercurial_vault::state::Vault>>,

    /// #8
    #[account(mut)]
    pub mercurial_vault_lp_mint: Box<Account<'info, Mint>>,

    /// #9
    /// Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault.
    #[account(
        mut,
        constraint = mercurial_vault.token_vault == mercurial_vault_collateral_token_safe.key() @UxdError::InvalidMercurialVaultCollateralTokenSafe,
    )]
    pub mercurial_vault_collateral_token_safe: Box<Account<'info, TokenAccount>>,

    /// #10
    pub mercurial_vault_program: Program<'info, mercurial_vault::program::Vault>,

    /// #11
    pub system_program: Program<'info, System>,

    /// #12
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<CollectProfitsOfMercurialVaultDepository>) -> Result<()> {
    // 1 - Read all states before collect
    let lp_token_vault_amount_before = ctx.accounts.depository_lp_token_vault.amount;
    let profits_beneficiary_collateral_amount_before =
        ctx.accounts.profits_beneficiary_collateral.amount;

    // 2 - calculate the value of collectable interests and fees (in USDC unit)
    let collectable_profits_value = ctx.accounts.calculate_collectable_profits_value()?;

    // 3 - calculate the amount of LP token to withdraw to collect all interests and fees
    // This LP amount is subjected to precision loss (we handle this precision loss later)
    let lp_token_amount_to_match_collectable_profits_value = ctx
        .accounts
        .calculate_lp_token_amount_to_match_collectable_profits_value(collectable_profits_value)?;

    let possible_lp_token_precision_loss_collateral_value =
        mercurial_utils::calculate_possible_lp_token_precision_loss_collateral_value(
            &ctx.accounts.mercurial_vault,
            ctx.accounts.mercurial_vault_lp_mint.supply,
        )?;

    // 4 - depository signer
    let depository = ctx.accounts.depository.load()?;
    let mercurial_vault: Pubkey = depository.mercurial_vault;
    let collateral_mint = depository.collateral_mint;
    let depository_bump = depository.bump;
    drop(depository);

    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MERCURIAL_VAULT_DEPOSITORY_NAMESPACE,
        mercurial_vault.as_ref(),
        collateral_mint.as_ref(),
        &[depository_bump],
    ]];

    // 5 - withdraw collateral from mercurial vault for LP tokens
    mercurial_vault::cpi::withdraw(
        ctx.accounts
            .into_withdraw_collateral_from_mercurial_vault_context()
            .with_signer(depository_signer_seed),
        lp_token_amount_to_match_collectable_profits_value,
        // Do not check slippage here
        0,
    )?;

    // 6 - Reload accounts impacted by the withdraw (We need updated numbers for further calculation)
    ctx.accounts.depository_lp_token_vault.reload()?;
    ctx.accounts.profits_beneficiary_collateral.reload()?;

    // 7 - Check that a positive amount of collateral have been redeemed
    let profits_beneficiary_collateral_amount_after =
        ctx.accounts.profits_beneficiary_collateral.amount;
    let profits_beneficiary_collateral_amount_increase = compute_increase(
        profits_beneficiary_collateral_amount_before,
        profits_beneficiary_collateral_amount_after,
    )?;

    // 8 - Check the amount of paid LP Token when calling mercurial_vault::cpi::withdraw
    let lp_token_vault_amount_after = ctx.accounts.depository_lp_token_vault.amount;
    let lp_token_vault_amount_before_decrease =
        compute_decrease(lp_token_vault_amount_before, lp_token_vault_amount_after)?;

    require!(
        profits_beneficiary_collateral_amount_increase > 0,
        UxdError::MinimumRedeemedCollateralAmountError
    );

    require!(
        lp_token_vault_amount_before_decrease == lp_token_amount_to_match_collectable_profits_value,
        UxdError::SlippageReached,
    );

    // 9 - There can be precision loss when calculating how many LP token to withdraw and also when withdrawing the collateral
    // We accept theses losses as the money is still in the vault. We collect a bit less profit.
    check_collateral_value_changed_to_match_target(
        profits_beneficiary_collateral_amount_increase,
        collectable_profits_value,
        possible_lp_token_precision_loss_collateral_value,
    )?;

    // 10 - Emit event
    emit!(CollectProfitsOfMercurialVaultDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: ctx.accounts.depository.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_amount: profits_beneficiary_collateral_amount_increase,
    });

    // 11 - update accounting
    let current_time_as_unix_timestamp = u64::try_from(Clock::get()?.unix_timestamp)
        .ok()
        .ok_or(UxdError::MathOverflow)?;

    ctx.accounts
        .depository
        .load_mut()?
        .update_onchain_accounting_following_profits_collection(
            profits_beneficiary_collateral_amount_increase,
            current_time_as_unix_timestamp,
        )?;

    ctx.accounts
        .controller
        .load_mut()?
        .update_onchain_accounting_following_profits_collection(
            profits_beneficiary_collateral_amount_increase,
        )?;

    Ok(())
}

// Into functions
impl<'info> CollectProfitsOfMercurialVaultDepository<'info> {
    pub fn into_withdraw_collateral_from_mercurial_vault_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mercurial_vault::cpi::accounts::Withdraw<'info>> {
        let cpi_accounts = mercurial_vault::cpi::accounts::Withdraw {
            vault: self.mercurial_vault.to_account_info(),
            token_vault: self.mercurial_vault_collateral_token_safe.to_account_info(),
            lp_mint: self.mercurial_vault_lp_mint.to_account_info(),
            user_token: self.profits_beneficiary_collateral.to_account_info(),
            user_lp: self.depository_lp_token_vault.to_account_info(),
            user: self.depository.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.mercurial_vault_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Calculation/Check functions
impl<'info> CollectProfitsOfMercurialVaultDepository<'info> {
    fn calculate_lp_token_amount_to_match_collectable_profits_value(
        &self,
        target_value: u64,
    ) -> Result<u64> {
        let current_time = u64::try_from(Clock::get()?.unix_timestamp)
            .ok()
            .ok_or(UxdError::MathOverflow)?;

        // Because it's u64 type, we will never withdraw too much due to precision loss, but withdraw less.
        // We withdraw less interests and fee due to precision loss and that's ok
        Ok(self
            .mercurial_vault
            .get_unmint_amount(
                current_time,
                target_value,
                self.mercurial_vault_lp_mint.supply,
            )
            .ok_or(UxdError::MathOverflow)?)
    }

    pub fn calculate_collectable_profits_value(&self) -> Result<u64> {
        let owned_lp_tokens_value =
            mercurial_utils::calculate_lp_tokens_value::calculate_lp_tokens_value(
                &self.mercurial_vault,
                self.mercurial_vault_lp_mint.supply,
                self.depository_lp_token_vault.amount,
            )?;

        // Mint max supply is cap at u64, so redeemable_amount_under_management <= u64::MAX, always safe to convert
        let redeemable_amount_under_management: u64 =
            u64::try_from(self.depository.load()?.redeemable_amount_under_management)
                .ok()
                .ok_or(UxdError::MathOverflow)?;

        Ok(owned_lp_tokens_value
            .checked_sub(redeemable_amount_under_management)
            .ok_or(UxdError::MathOverflow)?)
    }
}

// Validate
impl<'info> CollectProfitsOfMercurialVaultDepository<'info> {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        Ok(())
    }
}
