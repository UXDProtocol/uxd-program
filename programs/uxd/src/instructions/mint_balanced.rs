use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

use crate::error::UxdError;
use crate::state::controller::Controller;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::state::mercurial_vault_depository::MercurialVaultDepository;
use crate::utils::validate_collateral_amount;
use crate::validate_is_program_frozen;
use crate::CONTROLLER_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_NAMESPACE;
use crate::CREDIX_LP_EXTERNAL_PASS_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_NAMESPACE;

#[derive(Accounts)]
#[instruction(collateral_amount: u64)]
pub struct MintBalanced<'info> {
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
        constraint = controller.load()?.registered_mercurial_vault_depositories[0] == mercurial_vault_depository_0.key() @UxdError::InvalidDepository,
        constraint = controller.load()?.registered_credix_lp_depositories[0] == credix_lp_depository_0.key() @UxdError::InvalidDepository,
        has_one = redeemable_mint @UxdError::InvalidRedeemableMint
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #5
    #[account(mut)]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// #6
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #7
    #[account(
        mut,
        constraint = user_redeemable.owner == user.key() @UxdError::InvalidOwner,
        constraint = user_redeemable.mint == redeemable_mint.key() @UxdError::InvalidRedeemableMint,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #8
    #[account(
        mut,
        constraint = user_collateral.owner == user.key() @UxdError::InvalidOwner,
        constraint = user_collateral.mint == collateral_mint.key() @UxdError::InvalidCollateralMint,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #4
    #[account(
        mut,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_NAMESPACE, mercurial_vault_depository_0.load()?.mercurial_vault.key().as_ref(), mercurial_vault_depository_0.load()?.collateral_mint.as_ref()],
        bump = mercurial_vault_depository_0.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        has_one = mercurial_vault_0 @UxdError::InvalidMercurialVault,
        has_one = mercurial_vault_0_lp_mint @UxdError::InvalidMercurialVaultLpMint,
        constraint = mercurial_vault_depository_0.load()?.lp_token_vault == mercurial_vault_depository_0_lp_token_vault.key() @UxdError::InvalidDepositoryLpTokenVault,
    )]
    pub mercurial_vault_depository_0: AccountLoader<'info, MercurialVaultDepository>,

    /// #9
    /// Token account holding the LP tokens minted by depositing collateral on mercurial vault
    #[account(
        mut,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE, mercurial_vault_0.key().as_ref(), collateral_mint.key().as_ref()],
        token::authority = mercurial_vault_depository_0,
        token::mint = mercurial_vault_0_lp_mint,
        bump = mercurial_vault_depository_0.load()?.lp_token_vault_bump,
    )]
    pub mercurial_vault_depository_0_lp_token_vault: Box<Account<'info, TokenAccount>>,

    /// #10
    #[account(
        mut,
        constraint = mercurial_vault_0.token_vault == mercurial_vault_0_collateral_token_safe.key() @UxdError::InvalidMercurialVaultCollateralTokenSafe,
    )]
    pub mercurial_vault_0: Box<Account<'info, mercurial_vault::state::Vault>>,

    /// #11
    #[account(mut)]
    pub mercurial_vault_0_lp_mint: Box<Account<'info, Mint>>,

    /// #12
    /// Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault.
    #[account(mut)]
    pub mercurial_vault_0_collateral_token_safe: Box<Account<'info, TokenAccount>>,

    /// #4
    #[account(
        mut,
        seeds = [CREDIX_LP_DEPOSITORY_NAMESPACE, credix_lp_depository_0.load()?.credix_global_market_state.key().as_ref(), credix_lp_depository_0.load()?.collateral_mint.as_ref()],
        bump = credix_lp_depository_0.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        has_one = credix_lp_depository_0_collateral @UxdError::InvalidDepositoryCollateral,
        has_one = credix_lp_depository_0_shares @UxdError::InvalidDepositoryShares,
        has_one = credix_global_market_state @UxdError::InvalidCredixGlobalMarketState,
        has_one = credix_signing_authority @UxdError::InvalidCredixSigningAuthority,
        has_one = credix_liquidity_collateral @UxdError::InvalidCredixLiquidityCollateral,
        has_one = credix_shares_mint @UxdError::InvalidCredixSharesMint,
    )]
    pub credix_lp_depository_0: AccountLoader<'info, CredixLpDepository>,

    /// #9
    #[account(mut)]
    pub credix_lp_depository_0_collateral: Box<Account<'info, TokenAccount>>,

    /// #10
    #[account(mut)]
    pub credix_lp_depository_0_shares: Box<Account<'info, TokenAccount>>,

    /// #15
    #[account(
        owner = credix_client::ID,
        seeds = [credix_global_market_state.key().as_ref(), credix_lp_depository_0.key().as_ref(), CREDIX_LP_EXTERNAL_PASS_NAMESPACE],
        bump,
        seeds::program = credix_client::ID,
        constraint = credix_pass_0.user == credix_lp_depository_0.key() @UxdError::InvalidCredixPass,
        constraint = credix_pass_0.disable_withdrawal_fee @UxdError::InvalidCredixPassNoFees,
    )]
    pub credix_pass_0: Account<'info, credix_client::CredixPass>,

    /// #11
    pub credix_global_market_state: Box<Account<'info, credix_client::GlobalMarketState>>,

    /// #12
    /// CHECK: unused by us, checked by credix
    pub credix_signing_authority: AccountInfo<'info>,

    /// #13
    #[account(mut)]
    pub credix_liquidity_collateral: Box<Account<'info, TokenAccount>>,

    /// #14
    #[account(mut)]
    pub credix_shares_mint: Box<Account<'info, Mint>>,

    /// #16
    pub system_program: Program<'info, System>,
    /// #17
    pub token_program: Program<'info, Token>,
    /// #18
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// #13
    pub mercurial_vault_program: Program<'info, mercurial_vault::program::Vault>,
    /// #19
    pub credix_program: Program<'info, credix_client::program::Credix>,
    /// #20
    pub rent: Sysvar<'info, Rent>,
}

pub(crate) fn handler(ctx: Context<MintBalanced>, collateral_amount: u64) -> Result<()> {
    crate::mint_with_credix_lp_depository(
        ctx.accounts.into_mint_with_credix_lp_depository_0(),
        collateral_amount,
    )?;

    // Done
    Ok(())
}

// Into functions
impl<'info> MintBalanced<'info> {
    pub fn into_mint_with_credix_lp_depository_0(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, crate::accounts::MintWithCredixLpDepository<'info>> {
        let cpi_accounts = crate::accounts::MintWithCredixLpDepository {
            user: self.user.to_account_info(),
            payer: self.payer.to_account_info(),
            controller: self.controller.to_account_info(),
            redeemable_mint: self.redeemable_mint.to_account_info(),
            collateral_mint: self.collateral_mint.to_account_info(),
            user_redeemable: self.user_redeemable.to_account_info(),
            user_collateral: self.user_collateral.to_account_info(),
            depository: self.credix_lp_depository_0.to_account_info(),
            depository_collateral: self.credix_lp_depository_0_collateral.to_account_info(),
            depository_shares: self.credix_lp_depository_0_shares.to_account_info(),
            credix_pass: self.credix_pass_0.to_account.info(),
            credix_global_market_state: self.credix_global_market_state.to_account.info(),
            credix_signing_authority: self.credix_signing_authority.to_account.info(),
            credix_liquidity_collateral: self.credix_liquidity_collateral.to_account.info(),
            credix_shares_mint: self.credix_shares_mint.to_account.info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            associated_token_program: self.associated_token_program.to_account_info(),
            credix_program: self.credix_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.credix_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_mint_with_mercurial_vault_depository_0(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, crate::accounts::MintWithCredixLpDepository<'info>> {
        let cpi_accounts = crate::accounts::MintWithCredixLpDepository {
            user: self.user.to_account_info(),
            payer: self.payer.to_account_info(),
            controller: self.controller.to_account_info(),
            redeemable_mint: self.redeemable_mint.to_account_info(),
            collateral_mint: self.collateral_mint.to_account_info(),
            user_redeemable: self.user_redeemable.to_account_info(),
            user_collateral: self.user_collateral.to_account_info(),
            depository: self.mercurial_vault_depository_0.to_account_info(),
            depository_collateral: self
                .mercurial_vault_depository_0_collateral
                .to_account_info(),
            depository_shares: self.mercurial_vault_depository_0_shares.to_account_info(),
            credix_pass: self.credix_pass_0.to_account.info(),
            credix_global_market_state: self.credix_global_market_state.to_account.info(),
            credix_signing_authority: self.credix_signing_authority.to_account.info(),
            credix_liquidity_collateral: self.credix_liquidity_collateral.to_account.info(),
            credix_shares_mint: self.credix_shares_mint.to_account.info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            associated_token_program: self.associated_token_program.to_account_info(),
            credix_program: self.credix_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.credix_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Validate
impl<'info> MintBalanced<'info> {
    pub(crate) fn validate(&self, collateral_amount: u64) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        validate_collateral_amount(&self.user_collateral, collateral_amount)?;
        require!(
            !&self.depository.load()?.minting_disabled,
            UxdError::MintingDisabled
        );
        Ok(())
    }
}
