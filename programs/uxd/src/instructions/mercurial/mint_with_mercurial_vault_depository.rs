use crate::error::UxdError;
use crate::mercurial_utils;
use crate::utils;
use crate::Controller;
use crate::MercurialVaultDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;

#[derive(Accounts)]
pub struct MintWithMercurialVaultDepository<'info> {
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
    ctx: Context<MintWithMercurialVaultDepository>,
    collateral_amount: u64,
) -> Result<()> {
    let controller_bump = ctx.accounts.controller.load()?.bump;
    let controller_pda_signer: &[&[&[u8]]] = &[&[CONTROLLER_NAMESPACE, &[controller_bump]]];

    let before_lp_token_vault_balance = ctx.accounts.depository_lp_token_vault.amount;

    // 1 - Deposit collateral to mercurial vault and get lp tokens
    // Precision loss may occur on transferred LP token amounts, calculate the possible loss and check it later
    let possible_lp_token_precision_loss_collateral_value =
        mercurial_utils::calculate_possible_lp_token_precision_loss_collateral_value(
            &ctx.accounts.mercurial_vault,
            ctx.accounts.mercurial_vault_lp_mint.supply,
        )?;

    mercurial_vault::cpi::deposit(
        ctx.accounts
            .into_deposit_collateral_to_mercurial_vault_context(),
        collateral_amount,
        // Do not handle slippage here
        0,
    )?;

    // 2 - Reload accounts impacted by the deposit (We need updated numbers for further calculation)
    ctx.accounts.mercurial_vault.reload()?;
    ctx.accounts.depository_lp_token_vault.reload()?;
    ctx.accounts.mercurial_vault_lp_mint.reload()?;

    // 3 - Calculate the value of minted lp tokens
    let after_lp_token_vault_balance = ctx.accounts.depository_lp_token_vault.amount;

    let lp_token_change = I80F48::checked_from_num(
        after_lp_token_vault_balance
            .checked_sub(before_lp_token_vault_balance)
            .ok_or_else(|| error!(UxdError::MathError))?,
    )
    .ok_or_else(|| error!(UxdError::MathError))?;

    let minted_lp_token_value =
        mercurial_utils::calculate_lp_tokens_value::calculate_lp_tokens_value(
            &ctx.accounts.mercurial_vault,
            ctx.accounts.mercurial_vault_lp_mint.supply,
            lp_token_change
                .checked_to_num()
                .ok_or_else(|| error!(UxdError::MathError))?,
        )?;

    // 4 - Check that the minted lp token value matches the collateral value.
    // When manipulating LP tokens/collateral numbers, precision loss may occur.
    // The maximum allowed precision loss is 1 (native unit).
    // Plus the possible LP token precision loss that may have occurred in deposit
    MintWithMercurialVaultDepository::check_minted_lp_token_value_to_match_collateral_value(
        minted_lp_token_value,
        collateral_amount,
        possible_lp_token_precision_loss_collateral_value,
    )?;

    // 5 - Calculate the redeemable amount to send back to the user.
    // Mint redeemable 1:1 with provided collateral minus fees minus minting precision loss
    let base_redeemable_amount = minted_lp_token_value;

    let redeemable_amount_less_fees = utils::calculate_amount_less_fees(
        base_redeemable_amount,
        ctx.accounts.depository.load()?.minting_fee_in_bps,
    )?;

    let total_paid_fees = base_redeemable_amount
        .checked_sub(redeemable_amount_less_fees)
        .ok_or_else(|| error!(UxdError::MathError))?;

    // 6 - Redeemable amount should be positive
    require!(
        redeemable_amount_less_fees > 0,
        UxdError::MinimumMintedRedeemableAmountError
    );

    // 7 - Mint redeemable
    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_pda_signer),
        redeemable_amount_less_fees,
    )?;

    // 8 - Update Onchain accounting to reflect the changes
    ctx.accounts
        .controller
        .load_mut()?
        .update_onchain_accounting_following_mint_or_redeem(redeemable_amount_less_fees.into())?;

    ctx.accounts
        .depository
        .load_mut()?
        .update_onchain_accounting_following_mint_or_redeem(
            collateral_amount.into(),
            redeemable_amount_less_fees.into(),
            total_paid_fees.into(),
            0,
        )?;

    // 9 - Check that we don't mint more UXD than the fixed limit
    ctx.accounts.check_redeemable_global_supply_cap_overflow()?;

    ctx.accounts
        .check_redeemable_amount_under_management_cap_overflow()?;

    Ok(())
}

// Into functions
impl<'info> MintWithMercurialVaultDepository<'info> {
    pub fn into_deposit_collateral_to_mercurial_vault_context(
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
            user: self.user.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };

        let cpi_program = self.mercurial_vault_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_mint_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.redeemable_mint.to_account_info(),
            to: self.user_redeemable.to_account_info(),
            authority: self.controller.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Additional convenience methods related to the inputted accounts
impl<'info> MintWithMercurialVaultDepository<'info> {
    // Ensure that the minted amount does not raise the Redeemable supply beyond the Global Redeemable Supply Cap
    fn check_redeemable_global_supply_cap_overflow(&self) -> Result<()> {
        let controller = self.controller.load()?;

        require!(
            controller.redeemable_circulating_supply <= controller.redeemable_global_supply_cap,
            UxdError::RedeemableGlobalSupplyCapReached
        );

        Ok(())
    }

    // Accept precision loss diff
    fn check_minted_lp_token_value_to_match_collateral_value(
        minted_lp_token_value: u64,
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
            (target_minimal_allowed_value..target).contains(&minted_lp_token_value),
            UxdError::SlippageReached,
        );

        Ok(())
    }

    fn check_redeemable_amount_under_management_cap_overflow(&self) -> Result<()> {
        let depository = self.depository.load()?;
        require!(
            depository.redeemable_amount_under_management
                <= depository.redeemable_amount_under_management_cap,
            UxdError::RedeemableMercurialVaultAmountUnderManagementCap
        );
        Ok(())
    }
}

// Validate
impl<'info> MintWithMercurialVaultDepository<'info> {
    pub fn validate(&self, collateral_amount: u64) -> Result<()> {
        require!(collateral_amount != 0, UxdError::InvalidCollateralAmount);

        require!(
            !&self.depository.load()?.minting_disabled,
            UxdError::MintingDisabled
        );

        Ok(())
    }
}
