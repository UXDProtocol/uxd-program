use crate::error::UxdError;
use crate::mercurial_utils;
use crate::Controller;
use crate::MercurialVaultDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;

#[derive(Accounts)]
pub struct CollectProfitOfMercurialVaultDepository<'info> {
    /// #1
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
        constraint = controller.load()?.registered_mercurial_vault_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
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
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #6
    #[account(
        mut,
        constraint = authority_collateral.mint == depository.load()?.collateral_mint @UxdError::InvalidCollateralMint,
        constraint = &authority_collateral.owner == authority.key @UxdError::InvalidOwner,
    )]
    pub authority_collateral: Box<Account<'info, TokenAccount>>,

    /// #7
    /// Token account holding the LP tokens minted by depositing collateral on mercurial vault
    #[account(
        mut,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE, mercurial_vault.key().as_ref(), collateral_mint.key().as_ref()],
        token::authority = depository,
        token::mint = mercurial_vault_lp_mint,
        bump = depository.load()?.lp_token_vault_bump,
    )]
    pub depository_lp_token_vault: Box<Account<'info, TokenAccount>>,

    /// #8
    #[account(
        mut,
        constraint = mercurial_vault.token_vault == mercurial_vault_collateral_token_safe.key() @UxdError::InvalidMercurialVaultCollateralTokenSafe,
    )]
    pub mercurial_vault: Box<Account<'info, mercurial_vault::state::Vault>>,

    /// #9
    #[account(mut)]
    pub mercurial_vault_lp_mint: Box<Account<'info, Mint>>,

    /// #10
    /// Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault.
    #[account(mut)]
    pub mercurial_vault_collateral_token_safe: Box<Account<'info, TokenAccount>>,

    /// #11
    pub mercurial_vault_program: Program<'info, mercurial_vault::program::Vault>,

    /// #12
    pub system_program: Program<'info, System>,

    /// #13
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<CollectProfitOfMercurialVaultDepository>) -> Result<()> {
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
    let before_collateral_balance = ctx.accounts.authority_collateral.amount;

    // 1 - calculate the value of collectable interests and fees (in USDC unit)
    let collectable_profits_value = ctx.accounts.calculate_collectable_profits_value()?;

    // 2 - calculate the amount of LP token to withdraw to collect all interests and fees
    // This LP amount is subjected to precision loss (we handle this precision loss later)
    let lp_token_amount_to_match_collectable_profits_value = ctx
        .accounts
        .calculate_lp_token_amount_to_match_collectable_profits_value(collectable_profits_value)?;

    let possible_lp_token_precision_loss_collateral_value =
        mercurial_utils::calculate_possible_lp_token_precision_loss_collateral_value(
            &ctx.accounts.mercurial_vault,
            ctx.accounts.mercurial_vault_lp_mint.supply,
        )?;

    // 3 - withdraw collateral from mercurial vault for LP tokens
    mercurial_vault::cpi::withdraw(
        ctx.accounts
            .into_withdraw_collateral_from_mercurial_vault_context()
            .with_signer(depository_signer_seed),
        lp_token_amount_to_match_collectable_profits_value,
        // Do not check slippage here
        0,
    )?;

    // 4 - Reload accounts impacted by the withdraw (We need updated numbers for further calculation)
    ctx.accounts.depository_lp_token_vault.reload()?;
    ctx.accounts.authority_collateral.reload()?;

    // 5 - Check that a positive amount of collateral have been redeemed
    let after_collateral_balance = ctx.accounts.authority_collateral.amount;

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
        lp_token_change == lp_token_amount_to_match_collectable_profits_value,
        UxdError::SlippageReached,
    );

    // 7 - Check that the amount of received collateral from mercurial_vault::cpi::withdraw

    // There can be precision loss when calculating how many LP token to withdraw and also when withdrawing the collateral
    // We accept theses losses as they are paid by the user
    CollectProfitOfMercurialVaultDepository::check_redeemed_collateral_amount_to_match_target(
        collateral_balance_change,
        collectable_profits_value,
        possible_lp_token_precision_loss_collateral_value,
    )?;

    // 8 - update accounting
    let current_time_as_unix_timestamp = u64::try_from(Clock::get()?.unix_timestamp)
        .ok()
        .ok_or_else(|| error!(UxdError::MathError))?;

    ctx.accounts
        .depository
        .load_mut()?
        .update_onchain_accounting_following_profits_collection(
            collateral_balance_change,
            current_time_as_unix_timestamp,
        )?;

    ctx.accounts
        .controller
        .load_mut()?
        .update_onchain_accounting_following_profits_collection(collateral_balance_change)?;

    Ok(())
}

// Into functions
impl<'info> CollectProfitOfMercurialVaultDepository<'info> {
    pub fn into_withdraw_collateral_from_mercurial_vault_context(
        &self,
    ) -> CpiContext<
        '_,
        '_,
        '_,
        'info,
        mercurial_vault::cpi::accounts::DepositWithdrawLiquidity<'info>,
    > {
        let cpi_accounts = mercurial_vault::cpi::accounts::DepositWithdrawLiquidity {
            vault: self.mercurial_vault.to_account_info(),
            token_vault: self.mercurial_vault_collateral_token_safe.to_account_info(),
            lp_mint: self.mercurial_vault_lp_mint.to_account_info(),
            user_token: self.authority_collateral.to_account_info(),
            user_lp: self.depository_lp_token_vault.to_account_info(),
            user: self.depository.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.mercurial_vault_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Calculation/Check functions
impl<'info> CollectProfitOfMercurialVaultDepository<'info> {
    fn calculate_lp_token_amount_to_match_collectable_profits_value(
        &self,
        target_value: u64,
    ) -> Result<u64> {
        let current_time = u64::try_from(Clock::get()?.unix_timestamp)
            .ok()
            .ok_or_else(|| error!(UxdError::MathError))?;

        // Because it's u64 type, we will never withdraw too much due to precision loss, but withdraw less.
        // We withdraw less interests and fee due to precision loss and that's ok
        self.mercurial_vault
            .get_unmint_amount(
                current_time,
                target_value,
                self.mercurial_vault_lp_mint.supply,
            )
            .ok_or_else(|| error!(UxdError::MathError))
    }

    pub fn calculate_collectable_profits_value(&self) -> Result<u64> {
        let owned_lp_tokens_value = I80F48::checked_from_num(
            mercurial_utils::calculate_lp_tokens_value::calculate_lp_tokens_value(
                &self.mercurial_vault,
                self.mercurial_vault_lp_mint.supply,
                self.depository_lp_token_vault.amount,
            )?,
        )
        .ok_or_else(|| error!(UxdError::MathError))?;

        let collectable_profits_amount = owned_lp_tokens_value
            .checked_sub(
                I80F48::checked_from_num(
                    self.depository.load()?.redeemable_amount_under_management,
                )
                .ok_or_else(|| error!(UxdError::MathError))?,
            )
            .ok_or_else(|| error!(UxdError::MathError))?;

        collectable_profits_amount
            .checked_to_num()
            .ok_or_else(|| error!(UxdError::MathError))
    }

    // Check that the collateral amount received by the user matches the collateral amount we wanted the user to receive:
    //     redeemable amount - fees - lp token calculation precision loss - withdraw precision loss
    fn check_redeemed_collateral_amount_to_match_target(
        redeemed_collateral_amount: u64,
        target: u64,
        possible_lp_token_precision_loss_collateral_value: u64,
    ) -> Result<()> {
        // Lp token precision loss + withdraw collateral precision loss
        let maximum_allowed_precision_loss = possible_lp_token_precision_loss_collateral_value
            .checked_add(1)
            .ok_or_else(|| error!(UxdError::MathError))?;

        let target_minimal_allowed_value = target
            .checked_sub(maximum_allowed_precision_loss)
            .ok_or_else(|| error!(UxdError::MathError))?;

        require!(
            (target_minimal_allowed_value..(target + 1)).contains(&redeemed_collateral_amount),
            UxdError::SlippageReached,
        );

        Ok(())
    }
}
