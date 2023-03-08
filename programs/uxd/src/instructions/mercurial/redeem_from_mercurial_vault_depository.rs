use crate::error::UxdError;
use crate::mercurial_utils;
use crate::mercurial_utils::check_collateral_value_changed_to_match_target;
use crate::utils;
use crate::validate_is_program_frozen;
use crate::Controller;
use crate::MercurialVaultDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Burn;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct RedeemFromMercurialVaultDepository<'info> {
    /// #1
    pub user: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.registered_mercurial_vault_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
        has_one = redeemable_mint @UxdError::InvalidRedeemableMint
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4
    #[account(
        mut,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_NAMESPACE, depository.load()?.mercurial_vault.key().as_ref(), depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = mercurial_vault @UxdError::InvalidMercurialVault,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        has_one = mercurial_vault_lp_mint @UxdError::InvalidMercurialVaultLpMint,
        constraint = depository.load()?.lp_token_vault == depository_lp_token_vault.key() @UxdError::InvalidDepositoryLpTokenVault,
    )]
    pub depository: AccountLoader<'info, MercurialVaultDepository>,

    /// #5
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.load()?.redeemable_mint_bump,
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// #6
    #[account(
        mut,
        constraint = user_redeemable.mint == controller.load()?.redeemable_mint @UxdError::InvalidRedeemableMint,
        constraint = &user_redeemable.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #7
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #8
    #[account(
        mut,
        constraint = user_collateral.mint == depository.load()?.collateral_mint @UxdError::InvalidCollateralMint,
        constraint = &user_collateral.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #9
    /// Token account holding the LP tokens minted by depositing collateral on mercurial vault
    #[account(
        mut,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE, mercurial_vault.key().as_ref(), collateral_mint.key().as_ref()],
        token::authority = depository,
        token::mint = mercurial_vault_lp_mint,
        bump = depository.load()?.lp_token_vault_bump,
    )]
    pub depository_lp_token_vault: Box<Account<'info, TokenAccount>>,

    /// #10
    #[account(
        mut,
        constraint = mercurial_vault.token_vault == mercurial_vault_collateral_token_safe.key() @UxdError::InvalidMercurialVaultCollateralTokenSafe,
    )]
    pub mercurial_vault: Box<Account<'info, mercurial_vault::state::Vault>>,

    /// #11
    #[account(mut)]
    pub mercurial_vault_lp_mint: Box<Account<'info, Mint>>,

    /// #12
    /// Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault.
    #[account(mut)]
    pub mercurial_vault_collateral_token_safe: Box<Account<'info, TokenAccount>>,

    /// #13
    pub mercurial_vault_program: Program<'info, mercurial_vault::program::Vault>,

    /// #14
    pub system_program: Program<'info, System>,

    /// #15
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<RedeemFromMercurialVaultDepository>,
    redeemable_amount: u64,
) -> Result<()> {
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

    let before_lp_token_vault_balance = ctx.accounts.depository_lp_token_vault.amount;
    let before_collateral_balance = ctx.accounts.user_collateral.amount;

    // 1 - Calculate the collateral amount to redeem. Redeem 1:1 less redeeming fees.
    let base_collateral_amount = redeemable_amount;

    let collateral_amount_less_fees = utils::calculate_amount_less_fees(
        base_collateral_amount,
        ctx.accounts.depository.load()?.redeeming_fee_in_bps,
    )?;

    let total_paid_fees = base_collateral_amount
        .checked_sub(collateral_amount_less_fees)
        .ok_or_else(|| error!(UxdError::MathError))?;

    // 2 - Calculate the right amount of lp token to withdraw to match collateral_amount_less_fees
    // This LP amount is subjected to precision loss (we handle this precision loss later)
    let lp_token_amount_to_match_collateral_amount_less_fees = ctx
        .accounts
        .calculate_lp_amount_to_withdraw_to_match_collateral(collateral_amount_less_fees)?;

    let possible_lp_token_precision_loss_collateral_value =
        mercurial_utils::calculate_possible_lp_token_precision_loss_collateral_value(
            &ctx.accounts.mercurial_vault,
            ctx.accounts.mercurial_vault_lp_mint.supply,
        )?;

    // 3 - Redeem collateral from mercurial vault for lp tokens
    mercurial_vault::cpi::withdraw(
        ctx.accounts
            .into_withdraw_collateral_from_mercurial_vault_context()
            .with_signer(depository_signer_seed),
        lp_token_amount_to_match_collateral_amount_less_fees,
        // Do not check slippage here
        0,
    )?;

    // 4 - Reload accounts impacted by the withdraw (We need updated numbers for further calculation)
    ctx.accounts.depository_lp_token_vault.reload()?;
    ctx.accounts.user_collateral.reload()?;

    // 5 - Check that a positive amount of collateral have been redeemed
    let after_collateral_balance = ctx.accounts.user_collateral.amount;

    let collateral_balance_change = after_collateral_balance
        .checked_sub(before_collateral_balance)
        .ok_or_else(|| error!(UxdError::MathError))?;

    require!(
        collateral_balance_change > 0,
        UxdError::MinimumRedeemedCollateralAmountError
    );

    // 6 - Check the amount of paid LP Token when calling mercurial_vault::cpi::withdraw
    let after_lp_token_vault_balance = ctx.accounts.depository_lp_token_vault.amount;

    let lp_token_change = before_lp_token_vault_balance
        .checked_sub(after_lp_token_vault_balance)
        .ok_or_else(|| error!(UxdError::MathError))?;

    require!(
        lp_token_change == lp_token_amount_to_match_collateral_amount_less_fees,
        UxdError::SlippageReached,
    );

    // 7 - Check that the amount of received collateral from mercurial_vault::cpi::withdraw

    // There can be precision loss when calculating how many LP token to withdraw and also when withdrawing the collateral
    // We accept theses losses as they are paid by the user
    //     redeemable amount - fees - lp token calculation precision loss - withdraw precision loss
    check_collateral_value_changed_to_match_target(
        collateral_balance_change,
        collateral_amount_less_fees,
        possible_lp_token_precision_loss_collateral_value,
    )?;

    // 8 - Burn redeemable
    token::burn(
        ctx.accounts.into_burn_redeemable_context(),
        redeemable_amount,
    )?;

    // 9 - Update Onchain accounting to reflect the changes
    let redeemable_amount_change = i128::from(redeemable_amount)
        .checked_mul(-1)
        .ok_or_else(|| error!(UxdError::MathError))?;

    let collateral_amount_deposited_change = i128::from(collateral_balance_change)
        .checked_mul(-1)
        .ok_or_else(|| error!(UxdError::MathError))?;

    ctx.accounts
        .controller
        .load_mut()?
        .update_onchain_accounting_following_mint_or_redeem(redeemable_amount_change)?;

    ctx.accounts
        .depository
        .load_mut()?
        .update_onchain_accounting_following_mint_or_redeem(
            collateral_amount_deposited_change,
            redeemable_amount_change,
            0,
            total_paid_fees.into(),
        )?;

    Ok(())
}

// Into functions
impl<'info> RedeemFromMercurialVaultDepository<'info> {
    pub fn into_withdraw_collateral_from_mercurial_vault_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mercurial_vault::cpi::accounts::Withdraw<'info>> {
        let cpi_accounts = mercurial_vault::cpi::accounts::Withdraw {
            vault: self.mercurial_vault.to_account_info(),
            token_vault: self.mercurial_vault_collateral_token_safe.to_account_info(),
            lp_mint: self.mercurial_vault_lp_mint.to_account_info(),
            user_token: self.user_collateral.to_account_info(),
            user_lp: self.depository_lp_token_vault.to_account_info(),
            user: self.depository.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.mercurial_vault_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_burn_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.redeemable_mint.to_account_info(),
            from: self.user_redeemable.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Additional convenience methods related to the inputted accounts
impl<'info> RedeemFromMercurialVaultDepository<'info> {
    fn calculate_lp_amount_to_withdraw_to_match_collateral(
        &self,
        target_collateral_value: u64,
    ) -> Result<u64> {
        let current_time = u64::try_from(Clock::get()?.unix_timestamp)
            .ok()
            .ok_or_else(|| error!(UxdError::MathError))?;

        // Because it's u64 type, we will never withdraw too much due to precision loss, but withdraw less.
        // The user pays for precision loss by getting less collateral.
        self.mercurial_vault
            .get_unmint_amount(
                current_time,
                target_collateral_value,
                self.mercurial_vault_lp_mint.supply,
            )
            .ok_or_else(|| error!(UxdError::MathError))
    }
}

// Validate
impl<'info> RedeemFromMercurialVaultDepository<'info> {
    pub fn validate(&self, redeemable_amount: u64) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;

        require!(redeemable_amount != 0, UxdError::InvalidRedeemableAmount);

        Ok(())
    }
}
