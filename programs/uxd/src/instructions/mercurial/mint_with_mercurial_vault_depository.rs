use crate::error::UxdError;
use crate::Controller;
use crate::MercurialVaultDepository;
use crate::CONTROLLER_NAMESPACE;
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
        seeds = [MERCURIAL_VAULT_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
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
        constraint = user_collateral.mint == depository.load()?.collateral_mint @UxdError::InvalidCollateralMint,
        constraint = &user_collateral.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #7
    #[account(
        mut,
        constraint = user_redeemable.mint == controller.load()?.redeemable_mint @UxdError::InvalidRedeemableMint,
        constraint = &user_redeemable.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #8
    #[account(mut)]
    pub mercurial_vault: Box<Account<'info, mercurial_vault::state::Vault>>,

    /// #9
    #[account(mut)]
    pub mercurial_vault_lp_mint: Box<Account<'info, Mint>>,

    /// #10
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #11
    #[account(
        mut,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref(), mercurial_vault_lp_mint.key().as_ref()],
        token::authority = depository,
        token::mint = mercurial_vault_lp_mint,
        bump = depository.load()?.lp_tokens_vault_bump,
    )]
    pub depository_lp_token_vault: Box<Account<'info, TokenAccount>>,

    /// #12
    #[account(
        mut,
        token::mint = collateral_mint,
    )]
    pub mercurial_vault_program_collateral_token_vault: Box<Account<'info, TokenAccount>>,

    /// #13
    pub system_program: Program<'info, System>,

    /// #14
    pub token_program: Program<'info, Token>,

    /// #15
    pub mercurial_vault_program: Program<'info, mercurial_vault::program::Vault>,
}

pub fn handler(
    ctx: Context<MintWithMercurialVaultDepository>,
    collateral_amount: u64,
    minimum_lp_token_amount: u64,
) -> Result<()> {
    let controller_bump = ctx.accounts.controller.load()?.bump;
    let controller_pda_signer: &[&[&[u8]]] = &[&[CONTROLLER_NAMESPACE, &[controller_bump]]];

    // Deposit the collateral tokens on mercurial vault and mint uxd to user redeemable account
    let before_lp_token_vault_balance = ctx.accounts.depository_lp_token_vault.amount;

    // 1 - Deposit collateral to mercurial vault and get lp tokens
    mercurial_vault::cpi::deposit(
        ctx.accounts
            .into_deposit_collateral_to_mercurial_vault_context(),
        collateral_amount,
        minimum_lp_token_amount,
    )?;

    // 2 - Reload accounts impacted by the deposit
    ctx.accounts.mercurial_vault.reload()?;
    ctx.accounts.depository_lp_token_vault.reload()?;
    ctx.accounts.mercurial_vault_lp_mint.reload()?;

    // 3 - Calculate the redeemable amount to mint, based on the minted LP tokens amount and the price of the pool
    let after_lp_token_vault_balance = ctx.accounts.depository_lp_token_vault.amount;

    let lp_token_change_u64 = after_lp_token_vault_balance
        .checked_sub(before_lp_token_vault_balance)
        .ok_or_else(|| error!(UxdError::MathError))?;

    let lp_token_change = I80F48::from_num(lp_token_change_u64);

    let current_time = u64::try_from(Clock::get()?.unix_timestamp)
        .ok()
        .ok_or(UxdError::MathError)?;

    let unlocked_amount_u64 = ctx
        .accounts
        .mercurial_vault
        .get_unlocked_amount(current_time)
        .ok_or(UxdError::MathError)?;

    let unlocked_amount = I80F48::from_num(unlocked_amount_u64);
    let lp_token_total_supply = I80F48::from_num(ctx.accounts.mercurial_vault_lp_mint.supply);

    let vault_virtual_price = unlocked_amount
        .checked_div(lp_token_total_supply)
        .ok_or(UxdError::MathError)?;

    let lp_token_minted_value = vault_virtual_price
        .checked_mul(lp_token_change)
        .ok_or(UxdError::MathError)?;

    // 4 - Mint redeemable 1:1 - The only allowed mint for mercurial vault depository is USDC for now
    let redeemable_mint_amount = lp_token_minted_value
        .checked_to_num()
        .ok_or(UxdError::MathError)?;

    msg!(
        "before_lp_token_vault_balance {}",
        before_lp_token_vault_balance
    );

    msg!(
        "after_lp_token_vault_balance {}",
        after_lp_token_vault_balance
    );

    msg!("current_time {}", current_time);
    msg!("unlocked_amount {}", unlocked_amount);
    msg!(
        "ctx.accounts.mercurial_vault_lp_mint.supply {}",
        ctx.accounts.mercurial_vault_lp_mint.supply
    );
    msg!("vault_virtual_price {}", vault_virtual_price);
    msg!("lp_token_change {}", lp_token_change);

    msg!("redeemable_mint_amount {}", redeemable_mint_amount);

    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_pda_signer),
        redeemable_mint_amount,
    )?;

    // @TODO => Maybe remove? Imagine if we want to deposit funds with DAO, then we lose ours WSOL and kaboom
    // It means also that we can't mint or redeem with SOL with actual UXD Program using DAO
    // if ATA mint is WSOL => unwrap
    //if ctx.accounts.collateral_mint.key() == spl_token::native_mint::id() {
    //    token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;
    //}

    // 4 - Update accounting
    // @TODO

    // 6 - Check that we don't mint more UXD than the fixed limit
    // @TODO
    // ctx.accounts.check_redeemable_global_supply_cap_overflow()?;
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
            lp_mint: self.mercurial_vault_lp_mint.to_account_info(),
            token_program: self.token_program.to_account_info(),
            user_lp: self.depository_lp_token_vault.to_account_info(),
            token_vault: self
                .mercurial_vault_program_collateral_token_vault
                .to_account_info(),
            user: self.user.to_account_info(),
            user_token: self.user_collateral.to_account_info(),
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

// Validate
impl<'info> MintWithMercurialVaultDepository<'info> {
    pub fn validate(&self, collateral_amount: u64, minimum_lp_token_amount: u64) -> Result<()> {
        require!(collateral_amount != 0, UxdError::InvalidCollateralAmount);

        Ok(())
    }
}
