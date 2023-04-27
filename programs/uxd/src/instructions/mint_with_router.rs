use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

use crate::error::UxdError;
use crate::state::controller::Controller;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::state::identity_depository::IdentityDepository;
use crate::state::mercurial_vault_depository::MercurialVaultDepository;
use crate::utils::calculate_depositories_mint_collateral_amount;
use crate::utils::calculate_depositories_mint_collateral_amount::DepositoryInfoForMintCollateralAmount;
use crate::utils::calculate_depositories_target_redeemable_amount;
use crate::utils::calculate_depositories_target_redeemable_amount::DepositoryInfoForTargetRedeemableAmount;
use crate::utils::validate_collateral_amount;
use crate::validate_is_program_frozen;
use crate::CONTROLLER_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_NAMESPACE;
use crate::CREDIX_LP_EXTERNAL_PASS_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_NAMESPACE;

#[derive(Accounts)]
pub struct MintWithRouter<'info> {
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

    /// #4
    #[account(mut)]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// #5
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #6
    #[account(
        mut,
        constraint = user_redeemable.owner == user.key() @UxdError::InvalidOwner,
        constraint = user_redeemable.mint == redeemable_mint.key() @UxdError::InvalidRedeemableMint,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #7
    #[account(
        mut,
        constraint = user_collateral.owner == user.key() @UxdError::InvalidOwner,
        constraint = user_collateral.mint == collateral_mint.key() @UxdError::InvalidCollateralMint,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #8 - UXDProgram on chain account bound to a Controller instance that represent the blank minting/redeeming
    #[account(
        mut,
        seeds = [IDENTITY_DEPOSITORY_NAMESPACE],
        bump = identity_depository.load()?.bump,
    )]
    pub identity_depository: AccountLoader<'info, IdentityDepository>,

    /// #9 - Token account holding the collateral from minting
    #[account(
        mut,
        seeds = [IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE],
        token::authority = identity_depository,
        token::mint = identity_depository.load()?.collateral_mint,
        bump = identity_depository.load()?.collateral_vault_bump,
    )]
    pub identity_depository_collateral_vault: Box<Account<'info, TokenAccount>>,

    /// #10
    #[account(
        mut,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_NAMESPACE, mercurial_vault_depository_0.load()?.mercurial_vault.key().as_ref(), mercurial_vault_depository_0.load()?.collateral_mint.as_ref()],
        bump = mercurial_vault_depository_0.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        constraint = mercurial_vault_depository_0.load()?.mercurial_vault == mercurial_vault_depository_0_vault.key() @UxdError::InvalidMercurialVault,
        constraint = mercurial_vault_depository_0.load()?.mercurial_vault_lp_mint == mercurial_vault_depository_0_vault_lp_mint.key() @UxdError::InvalidMercurialVaultLpMint,
        constraint = mercurial_vault_depository_0.load()?.lp_token_vault == mercurial_vault_depository_0_lp_token_vault.key() @UxdError::InvalidDepositoryLpTokenVault,
    )]
    pub mercurial_vault_depository_0: AccountLoader<'info, MercurialVaultDepository>,

    /// #11 - Token account holding the LP tokens minted by depositing collateral on mercurial vault
    #[account(
        mut,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE, mercurial_vault_depository_0_vault.key().as_ref(), collateral_mint.key().as_ref()],
        token::authority = mercurial_vault_depository_0,
        token::mint = mercurial_vault_depository_0_vault_lp_mint,
        bump = mercurial_vault_depository_0.load()?.lp_token_vault_bump,
    )]
    pub mercurial_vault_depository_0_lp_token_vault: Box<Account<'info, TokenAccount>>,

    /// #12
    #[account(
        mut,
        constraint = mercurial_vault_depository_0_vault.token_vault == mercurial_vault_depository_0_collateral_token_safe.key() @UxdError::InvalidMercurialVaultCollateralTokenSafe,
    )]
    pub mercurial_vault_depository_0_vault: Box<Account<'info, mercurial_vault::state::Vault>>,

    /// #13
    #[account(mut)]
    pub mercurial_vault_depository_0_vault_lp_mint: Box<Account<'info, Mint>>,

    /// #14 - Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault.
    #[account(mut)]
    pub mercurial_vault_depository_0_collateral_token_safe: Box<Account<'info, TokenAccount>>,

    /// #15
    #[account(
        mut,
        seeds = [CREDIX_LP_DEPOSITORY_NAMESPACE, credix_lp_depository_0.load()?.credix_global_market_state.key().as_ref(), credix_lp_depository_0.load()?.collateral_mint.as_ref()],
        bump = credix_lp_depository_0.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        constraint = credix_lp_depository_0.load()?.depository_collateral == credix_lp_depository_0_collateral.key() @UxdError::InvalidDepositoryCollateral,
        constraint = credix_lp_depository_0.load()?.depository_shares == credix_lp_depository_0_shares.key() @UxdError::InvalidDepositoryShares,
        constraint = credix_lp_depository_0.load()?.credix_global_market_state == credix_lp_depository_0_global_market_state.key() @UxdError::InvalidCredixGlobalMarketState,
        constraint = credix_lp_depository_0.load()?.credix_signing_authority == credix_lp_depository_0_signing_authority.key() @UxdError::InvalidCredixSigningAuthority,
        constraint = credix_lp_depository_0.load()?.credix_liquidity_collateral == credix_lp_depository_0_liquidity_collateral.key() @UxdError::InvalidCredixLiquidityCollateral,
        constraint = credix_lp_depository_0.load()?.credix_shares_mint == credix_lp_depository_0_shares_mint.key() @UxdError::InvalidCredixSharesMint,
    )]
    pub credix_lp_depository_0: AccountLoader<'info, CredixLpDepository>,

    /// #16
    #[account(mut)]
    pub credix_lp_depository_0_collateral: Box<Account<'info, TokenAccount>>,

    /// #17
    #[account(mut)]
    pub credix_lp_depository_0_shares: Box<Account<'info, TokenAccount>>,

    /// #18
    #[account(
        owner = credix_client::ID,
        seeds = [credix_lp_depository_0_global_market_state.key().as_ref(), credix_lp_depository_0.key().as_ref(), CREDIX_LP_EXTERNAL_PASS_NAMESPACE],
        bump,
        seeds::program = credix_client::ID,
        constraint = credix_lp_depository_0_pass.user == credix_lp_depository_0.key() @UxdError::InvalidCredixPass,
        constraint = credix_lp_depository_0_pass.disable_withdrawal_fee @UxdError::InvalidCredixPassNoFees,
    )]
    pub credix_lp_depository_0_pass: Account<'info, credix_client::CredixPass>,

    /// #19
    pub credix_lp_depository_0_global_market_state:
        Box<Account<'info, credix_client::GlobalMarketState>>,

    /// #20 - CHECK: unused by us, checked by credix
    pub credix_lp_depository_0_signing_authority: AccountInfo<'info>,

    /// #21
    #[account(mut)]
    pub credix_lp_depository_0_liquidity_collateral: Box<Account<'info, TokenAccount>>,

    /// #22
    #[account(mut)]
    pub credix_lp_depository_0_shares_mint: Box<Account<'info, Mint>>,

    /// #23
    pub system_program: Program<'info, System>,

    /// #24
    pub token_program: Program<'info, Token>,

    /// #25
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// #26
    pub mercurial_vault_program: Program<'info, mercurial_vault::program::Vault>,

    /// #27
    pub credix_program: Program<'info, credix_client::program::Credix>,

    /// #28
    pub uxd_program: Program<'info, crate::program::Uxd>,

    /// #29
    pub rent: Sysvar<'info, Rent>,
}

struct DepositoryInfoForMintWithRouter<'info> {
    pub weight_bps: u16,
    pub redeemable_amount_under_management: u128,
    pub redeemable_amount_under_management_cap: u128,
    pub mint_fn: Box<dyn Fn(u64) -> Result<()> + 'info>,
}

pub(crate) fn handler(ctx: Context<MintWithRouter>, collateral_amount: u64) -> Result<()> {
    // Gather all the onchain states we need (caps, weights and supplies)
    let controller = ctx.accounts.controller.load()?;
    let identity_depository = ctx.accounts.identity_depository.load()?;
    let mercurial_vault_depository_0 = ctx.accounts.mercurial_vault_depository_0.load()?;
    let credix_lp_depository_0 = ctx.accounts.credix_lp_depository_0.load()?;

    // The actual post-mint circulating supply might be slightly lower due to fees and precision loss
    // But the difference is negligible, and the difference will be taken into account
    // When the next mint/redeem IX recompute the new targets based on the new circulating supply
    let maximum_after_mint_circulating_supply = controller
        .redeemable_circulating_supply
        .checked_add(collateral_amount.into())
        .ok_or(UxdError::MathError)?;

    // Build the vector of all known depository participating in the routing system
    let depository_info = vec![
        // Identity depository details
        DepositoryInfoForMintWithRouter {
            weight_bps: controller.identity_depository_weight_bps,
            redeemable_amount_under_management: identity_depository
                .redeemable_amount_under_management,
            redeemable_amount_under_management_cap: identity_depository
                .redeemable_amount_under_management_cap,
            mint_fn: Box::new(|collateral_amount| {
                msg!(
                    "[mint_with_router:mint_with_identity_depository:{}]",
                    collateral_amount
                );
                if collateral_amount > 0 {
                    uxd_cpi::cpi::mint_with_identity_depository(
                        ctx.accounts.into_mint_with_identity_depository_context(),
                        collateral_amount,
                    )?;
                }
                Ok(())
            }),
        },
        // Mercurial Vault Depository 0 details
        DepositoryInfoForMintWithRouter {
            weight_bps: controller.mercurial_vault_depository_0_weight_bps,
            redeemable_amount_under_management: mercurial_vault_depository_0
                .redeemable_amount_under_management,
            redeemable_amount_under_management_cap: mercurial_vault_depository_0
                .redeemable_amount_under_management_cap,
            mint_fn: Box::new(|collateral_amount| {
                msg!(
                    "[mint_with_router:mint_with_mercurial_vault_depository:{}]",
                    collateral_amount
                );
                if collateral_amount > 0 {
                    uxd_cpi::cpi::mint_with_mercurial_vault_depository(
                        ctx.accounts
                            .into_mint_with_mercurial_vault_depository_0_context(),
                        collateral_amount,
                    )?;
                }
                Ok(())
            }),
        },
        // Credix Lp Depository 0 details
        DepositoryInfoForMintWithRouter {
            weight_bps: controller.credix_lp_depository_0_weight_bps,
            redeemable_amount_under_management: credix_lp_depository_0
                .redeemable_amount_under_management,
            redeemable_amount_under_management_cap: credix_lp_depository_0
                .redeemable_amount_under_management_cap,
            mint_fn: Box::new(|collateral_amount| {
                msg!(
                    "[mint_with_router:mint_with_credix_lp_depository:{}]",
                    collateral_amount
                );
                if collateral_amount > 0 {
                    uxd_cpi::cpi::mint_with_credix_lp_depository(
                        ctx.accounts.into_mint_with_credix_lp_depository_0_context(),
                        collateral_amount,
                    )?;
                }
                Ok(())
            }),
        },
    ];

    drop(controller);
    drop(identity_depository);
    drop(mercurial_vault_depository_0);
    drop(credix_lp_depository_0);

    // Compute the desired target amounts for each depository
    let depositories_target_redeemable_amount = calculate_depositories_target_redeemable_amount(
        maximum_after_mint_circulating_supply,
        &depository_info
            .iter()
            .map(|depository_info| DepositoryInfoForTargetRedeemableAmount {
                weight_bps: depository_info.weight_bps,
                redeemable_amount_under_management_cap: depository_info
                    .redeemable_amount_under_management_cap,
            })
            .collect::<Vec<DepositoryInfoForTargetRedeemableAmount>>(),
    )?;

    // Compute the desired mint amounts for each depository
    let depositories_mint_collateral_amount = calculate_depositories_mint_collateral_amount(
        collateral_amount,
        &std::iter::zip(
            depository_info.iter(),
            depositories_target_redeemable_amount.iter(),
        )
        .map(|(depository_info, depository_target_redeemable_amount)| {
            DepositoryInfoForMintCollateralAmount {
                target_redeemable_amount: *depository_target_redeemable_amount,
                redeemable_amount_under_management: depository_info
                    .redeemable_amount_under_management,
            }
        })
        .collect::<Vec<DepositoryInfoForMintCollateralAmount>>(),
    )?;

    // Call all the mint functions with the compute amounts
    std::iter::zip(
        depository_info.iter(),
        depositories_mint_collateral_amount.iter(),
    )
    .try_for_each(|(depository_info, depository_mint_collateral_amount)| {
        (depository_info.mint_fn)(*depository_mint_collateral_amount)
    })?;

    // Done
    Ok(())
}

// Into functions
impl<'info> MintWithRouter<'info> {
    pub fn into_mint_with_identity_depository_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, uxd_cpi::cpi::accounts::MintWithIdentityDepository<'info>>
    {
        let cpi_accounts = uxd_cpi::cpi::accounts::MintWithIdentityDepository {
            user: self.user.to_account_info(),
            payer: self.payer.to_account_info(),
            controller: self.controller.to_account_info(),
            redeemable_mint: self.redeemable_mint.to_account_info(),
            user_redeemable: self.user_redeemable.to_account_info(),
            user_collateral: self.user_collateral.to_account_info(),
            depository: self.identity_depository.to_account_info(),
            collateral_vault: self.identity_depository_collateral_vault.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.uxd_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_mint_with_mercurial_vault_depository_0_context(
        &self,
    ) -> CpiContext<
        '_,
        '_,
        '_,
        'info,
        uxd_cpi::cpi::accounts::MintWithMercurialVaultDepository<'info>,
    > {
        let cpi_accounts = uxd_cpi::cpi::accounts::MintWithMercurialVaultDepository {
            user: self.user.to_account_info(),
            payer: self.payer.to_account_info(),
            controller: self.controller.to_account_info(),
            redeemable_mint: self.redeemable_mint.to_account_info(),
            collateral_mint: self.collateral_mint.to_account_info(),
            user_redeemable: self.user_redeemable.to_account_info(),
            user_collateral: self.user_collateral.to_account_info(),
            depository: self.mercurial_vault_depository_0.to_account_info(),
            depository_lp_token_vault: self
                .mercurial_vault_depository_0_lp_token_vault
                .to_account_info(),
            mercurial_vault: self.mercurial_vault_depository_0_vault.to_account_info(),
            mercurial_vault_lp_mint: self
                .mercurial_vault_depository_0_vault_lp_mint
                .to_account_info(),
            mercurial_vault_collateral_token_safe: self
                .mercurial_vault_depository_0_collateral_token_safe
                .to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            mercurial_vault_program: self.mercurial_vault_program.to_account_info(),
        };
        let cpi_program = self.mercurial_vault_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_mint_with_credix_lp_depository_0_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, uxd_cpi::cpi::accounts::MintWithCredixLpDepository<'info>>
    {
        let cpi_accounts = uxd_cpi::cpi::accounts::MintWithCredixLpDepository {
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
            credix_pass: self.credix_lp_depository_0_pass.to_account_info(),
            credix_global_market_state: self
                .credix_lp_depository_0_global_market_state
                .to_account_info(),
            credix_signing_authority: self
                .credix_lp_depository_0_signing_authority
                .to_account_info(),
            credix_liquidity_collateral: self
                .credix_lp_depository_0_liquidity_collateral
                .to_account_info(),
            credix_shares_mint: self.credix_lp_depository_0_shares_mint.to_account_info(),
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
impl<'info> MintWithRouter<'info> {
    pub(crate) fn validate(&self, collateral_amount: u64) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        validate_collateral_amount(&self.user_collateral, collateral_amount)?;
        Ok(())
    }
}
