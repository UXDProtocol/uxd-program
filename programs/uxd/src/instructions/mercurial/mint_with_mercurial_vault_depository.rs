use crate::error::UxdError;
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

    let minted_lp_token_value = ctx.accounts.calculate_lp_tokens_value(
        lp_token_change
            .checked_to_num()
            .ok_or_else(|| error!(UxdError::MathError))?,
    )?;

    // 4 - Check that the minted lp token value matches the collateral value.
    // When manipulating LP tokens/collateral numbers, precision loss may occur.
    // The maximum allowed precision loss is 1 (native unit).
    MintWithMercurialVaultDepository::check_minted_lp_token_value_to_match_collateral_value(
        minted_lp_token_value,
        collateral_amount,
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
    ctx.accounts.update_onchain_accounting(
        collateral_amount.into(),
        redeemable_amount_less_fees.into(),
        total_paid_fees.into(),
    )?;

    // 9 - Check that we don't mint more UXD than the fixed limit
    ctx.accounts.check_redeemable_global_supply_cap_overflow()?;

    msg!("collateral_amount: {}, base_redeemable_amount: {}, redeemable_amount_less_fees: {}, total_paid_fees: {}, lp_token_change: {}", collateral_amount, base_redeemable_amount, redeemable_amount_less_fees, total_paid_fees, lp_token_change);

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
    // Update the accounting in the Depository and Controller Accounts to reflect changes
    fn update_onchain_accounting(
        &mut self,
        collateral_amount_deposited: u128,
        redeemable_minted_amount: u128,
        total_paid_fees: u128,
    ) -> Result<()> {
        let mut depository = self.depository.load_mut()?;
        let mut controller = self.controller.load_mut()?;

        // Depository
        depository.collateral_amount_deposited = depository
            .collateral_amount_deposited
            .checked_add(collateral_amount_deposited)
            .ok_or_else(|| error!(UxdError::MathError))?;

        depository.minted_redeemable_amount = depository
            .minted_redeemable_amount
            .checked_add(redeemable_minted_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;

        depository.total_paid_mint_fees = depository
            .total_paid_mint_fees
            .checked_add(total_paid_fees)
            .ok_or_else(|| error!(UxdError::MathError))?;

        // Controller
        controller.redeemable_circulating_supply = controller
            .redeemable_circulating_supply
            .checked_add(redeemable_minted_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;

        Ok(())
    }

    // Ensure that the minted amount does not raise the Redeemable supply beyond the Global Redeemable Supply Cap
    fn check_redeemable_global_supply_cap_overflow(&self) -> Result<()> {
        let controller = self.controller.load()?;

        require!(
            controller.redeemable_circulating_supply <= controller.redeemable_global_supply_cap,
            UxdError::RedeemableGlobalSupplyCapReached
        );

        Ok(())
    }

    fn calculate_lp_tokens_value(&self, lp_token_amount: u64) -> Result<u64> {
        let current_time = u64::try_from(Clock::get()?.unix_timestamp)
            .ok()
            .ok_or_else(|| error!(UxdError::MathError))?;

        self.mercurial_vault
            .get_amount_by_share(
                current_time,
                lp_token_amount,
                self.mercurial_vault_lp_mint.supply,
            )
            .ok_or_else(|| error!(UxdError::MathError))
    }

    // Accept precision loss diff
    fn check_minted_lp_token_value_to_match_collateral_value(
        minted_lp_token_value: u64,
        collateral_amount: u64,
    ) -> Result<()> {
        let collateral_amount_minus_precision_loss = collateral_amount
            .checked_sub(1)
            .ok_or_else(|| error!(UxdError::MathError))?;

        require!(
            // Without precision loss
            minted_lp_token_value == collateral_amount
            ||
            // With precision loss 
            minted_lp_token_value == collateral_amount_minus_precision_loss,
            UxdError::SlippageReached,
        );

        Ok(())
    }
}

// Validate
impl<'info> MintWithMercurialVaultDepository<'info> {
    pub fn validate(&self, collateral_amount: u64) -> Result<()> {
        require!(collateral_amount != 0, UxdError::InvalidCollateralAmount);

        Ok(())
    }
}
