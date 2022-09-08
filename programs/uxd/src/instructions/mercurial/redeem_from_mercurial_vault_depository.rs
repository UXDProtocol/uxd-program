use crate::error::UxdError;
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

    // 1 - Calculate the right amount of lp token to withdraw to match redeemable 1:1
    let current_time = u64::try_from(Clock::get()?.unix_timestamp)
        .ok()
        .ok_or_else(|| error!(UxdError::MathError))?;

    // Because it's u64 type, we will never withdraw too much due to precision loss, but withdraw less.
    // The user pays for precision loss by getting less collateral.
    let lp_token_amount_to_match_redeemable_amount = ctx
        .accounts
        .mercurial_vault
        .get_unmint_amount(
            current_time,
            redeemable_amount,
            ctx.accounts.mercurial_vault_lp_mint.supply,
        )
        .ok_or_else(|| error!(UxdError::MathError))?;

    // 2 - Redeem collateral from mercurial vault for lp tokens
    mercurial_vault::cpi::withdraw(
        ctx.accounts
            .into_withdraw_collateral_from_mercurial_vault_context()
            .with_signer(depository_signer_seed),
        lp_token_amount_to_match_redeemable_amount,
        // Do not check slippage here
        0,
    )?;

    // 3 - Reload accounts impacted by the deposit (We need updated numbers for further calculation)
    ctx.accounts.depository_lp_token_vault.reload()?;
    ctx.accounts.user_collateral.reload()?;

    // 4 - Check the amount of paid LP Token and check the amount of received collateral
    // Should never fail except if mercurial program do not do what it's supposed to do
    let after_lp_token_vault_balance = ctx.accounts.depository_lp_token_vault.amount;
    let after_collateral_balance = ctx.accounts.user_collateral.amount;

    let vault_lp_token_change = before_lp_token_vault_balance
        .checked_sub(after_lp_token_vault_balance)
        .ok_or_else(|| error!(UxdError::MathError))?;

    let collateral_balance_change = after_collateral_balance
        .checked_sub(before_collateral_balance)
        .ok_or_else(|| error!(UxdError::MathError))?;

    require!(
        vault_lp_token_change == lp_token_amount_to_match_redeemable_amount,
        UxdError::SlippageReached,
    );

    let redeemable_amount_minus_precision_loss = redeemable_amount
        .checked_sub(1)
        .ok_or_else(|| error!(UxdError::MathError))?;

    require!(
        collateral_balance_change == redeemable_amount
            || collateral_balance_change == redeemable_amount_minus_precision_loss,
        UxdError::SlippageReached,
    );

    // 5 - Burn redeemable
    token::burn(
        ctx.accounts.into_burn_redeemable_context(),
        redeemable_amount,
    )?;

    // 6 - Update Onchain accounting to reflect the changes
    ctx.accounts
        .update_onchain_accounting(collateral_balance_change.into(), redeemable_amount.into())?;

    Ok(())
}

// Into functions
impl<'info> RedeemFromMercurialVaultDepository<'info> {
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
    // Update the accounting in the Depository and Controller Accounts to reflect changes
    fn update_onchain_accounting(
        &mut self,
        collateral_withdrawn_amount: u128,
        redeemable_burnt_amount: u128,
    ) -> Result<()> {
        let mut depository = self.depository.load_mut()?;
        let mut controller = self.controller.load_mut()?;

        // Depository
        depository.collateral_amount_deposited = depository
            .collateral_amount_deposited
            .checked_sub(collateral_withdrawn_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;

        depository.minted_redeemable_amount = depository
            .minted_redeemable_amount
            .checked_sub(redeemable_burnt_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;

        // Controller
        controller.redeemable_circulating_supply = controller
            .redeemable_circulating_supply
            .checked_sub(redeemable_burnt_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;

        drop(depository);
        drop(controller);

        Ok(())
    }
}

// Validate
impl<'info> RedeemFromMercurialVaultDepository<'info> {
    pub fn validate(&self, redeemable_amount: u64) -> Result<()> {
        require!(redeemable_amount != 0, UxdError::InvalidRedeemableAmount);

        Ok(())
    }
}
