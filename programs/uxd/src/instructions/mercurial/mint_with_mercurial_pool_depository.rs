use crate::error::UxdError;
use crate::mercurial_utils::MercurialPoolInfos;
use crate::state::MercurialPoolToken;
use crate::utils;
use crate::Controller;
use crate::MercurialPoolDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MERCURIAL_POOL_DEPOSITORY_LP_VAULT_NAMESPACE;
use crate::MERCURIAL_POOL_DEPOSITORY_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;

#[derive(Accounts)]
pub struct MintWithMercurialPoolDepository<'info> {
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
        constraint = controller.load()?.registered_mercurial_pool_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
        has_one = redeemable_mint @UxdError::InvalidRedeemableMint
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4
    #[account(
        mut,
        seeds = [MERCURIAL_POOL_DEPOSITORY_NAMESPACE, depository.load()?.mercurial_pool.key().as_ref(), depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = mercurial_pool @UxdError::InvalidMercurialPool,
    )]
    pub depository: AccountLoader<'info, MercurialPoolDepository>,

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
    /// Unused by mercurial program (single side deposit), but required as parameter
    #[account(
        mut,
        constraint = &user_mercurial_pool_secondary_token.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_mercurial_pool_secondary_token: Box<Account<'info, TokenAccount>>,

    /// #10
    pub mercurial_pool_secondary_token_mint: Box<Account<'info, Mint>>,

    /// #11
    #[account(
        mut,
        seeds = [MERCURIAL_POOL_DEPOSITORY_LP_VAULT_NAMESPACE, mercurial_pool.key().as_ref(), collateral_mint.key().as_ref()],
        token::authority = depository,
        token::mint = mercurial_pool_lp_mint,
        bump = depository.load()?.pool_lp_token_vault_bump,
    )]
    pub depository_pool_lp_token_vault: Box<Account<'info, TokenAccount>>,

    /// #12
    #[account(mut)]
    pub mercurial_pool: Box<Account<'info, amm::state::Pool>>,

    /// #13
    #[account(mut)]
    pub mercurial_pool_lp_mint: Box<Account<'info, Mint>>,

    /// #14
    #[account(mut)]
    pub mercurial_vault_a: Box<Account<'info, mercurial_vault::state::Vault>>,

    /// #15
    #[account(
        mut,
        token::mint = mercurial_vault_a_lp_mint,
    )]
    pub mercurial_vault_a_lp: Box<Account<'info, TokenAccount>>,

    /// #16
    #[account(mut)]
    pub mercurial_vault_a_lp_mint: Box<Account<'info, Mint>>,

    /// #17
    #[account(mut)]
    pub mercurial_vault_a_token_vault: Box<Account<'info, TokenAccount>>,

    /// #18
    #[account(mut)]
    pub mercurial_vault_b: Box<Account<'info, mercurial_vault::state::Vault>>,

    /// #19
    #[account(mut)]
    pub mercurial_vault_b_lp_mint: Box<Account<'info, Mint>>,

    /// #20
    #[account(
        mut,
        token::mint = mercurial_vault_b_lp_mint,
    )]
    pub mercurial_vault_b_lp: Box<Account<'info, TokenAccount>>,

    /// #21
    #[account(mut)]
    pub mercurial_vault_b_token_vault: Box<Account<'info, TokenAccount>>,

    /// #22
    pub mercurial_vault_program: Program<'info, mercurial_vault::program::Vault>,

    /// #23
    pub system_program: Program<'info, System>,

    /// #24
    pub token_program: Program<'info, Token>,

    /// #25
    pub mercurial_pool_program: Program<'info, amm::program::Amm>,
}

// Context:
//
// Checks have been performed at depository creation. The following is considered true:
//
// - One of mercurial pool TokenA/TokenB is the same as collateral
// - The mercurial pool is stable. Theoric ratio for TokenA/TokenB is 1:1
pub fn handler(
    ctx: Context<MintWithMercurialPoolDepository>,
    collateral_amount: u64,
    minimum_redeemable_amount: u64,
) -> Result<()> {
    msg!(
        "collateral_amount: {}, minimum_redeemable_amount: {}",
        collateral_amount,
        minimum_redeemable_amount
    );

    let controller_bump = ctx.accounts.controller.load()?.bump;
    let controller_pda_signer: &[&[&[u8]]] = &[&[CONTROLLER_NAMESPACE, &[controller_bump]]];

    let depository = ctx.accounts.depository.load()?;

    let before_pool_lp_token_vault_balance = ctx.accounts.depository_pool_lp_token_vault.amount;

    msg!(
        "before_pool_lp_token_vault_balance: {}",
        before_pool_lp_token_vault_balance,
    );

    // 1 - Deposit collateral to mercurial pool and get lp tokens
    let token_a_amount = if depository
        .is_collateral_mercurial_pool_token_a_or_b
        .eq(&MercurialPoolToken::TokenA)
    {
        collateral_amount
    } else {
        0
    };

    let token_b_amount = if depository
        .is_collateral_mercurial_pool_token_a_or_b
        .eq(&MercurialPoolToken::TokenB)
    {
        collateral_amount
    } else {
        0
    };

    msg!(
        "Mercurial Pool CPI: Add imbalance liquidity, token_a_amount: {}, token_b_amount: {}",
        token_a_amount,
        token_b_amount
    );

    // One side deposit
    amm::cpi::add_imbalance_liquidity(
        ctx.accounts
            .into_deposit_collateral_to_mercurial_pool_context(&depository),
        // Do not handle slippage here
        0,
        token_a_amount,
        token_b_amount,
    )?;

    // 2 - Reload accounts impacted by the deposit (We need updated numbers for further calculation)
    ctx.accounts.mercurial_vault_a.reload()?;
    ctx.accounts.mercurial_vault_b.reload()?;
    ctx.accounts.mercurial_pool.reload()?;
    ctx.accounts.depository_pool_lp_token_vault.reload()?;
    ctx.accounts.mercurial_pool_lp_mint.reload()?;

    msg!(
        "depository.is_collateral_mercurial_pool_token_a_or_b: {}",
        depository.is_collateral_mercurial_pool_token_a_or_b
    );

    msg!(
        "ctx.accounts.collateral_mint.decimals: {}",
        ctx.accounts.collateral_mint.decimals
    );

    msg!(
        "ctx.accounts.mercurial_pool_secondary_token_mint.decimals: {}",
        ctx.accounts.mercurial_pool_secondary_token_mint.decimals
    );

    /*
    // 3 - Calculate the value of the minted lp tokens
    let mercurial_pool_infos = MercurialPoolInfos::new(
        *ctx.accounts.mercurial_vault_a.clone(),
        *ctx.accounts.mercurial_vault_b.clone(),
        *ctx.accounts.mercurial_vault_a_lp.clone(),
        *ctx.accounts.mercurial_vault_a_lp_mint.clone(),
        *ctx.accounts.mercurial_vault_b_lp.clone(),
        *ctx.accounts.mercurial_vault_b_lp_mint.clone(),
        *ctx.accounts.mercurial_pool_lp_mint.clone(),
        if depository
            .is_collateral_mercurial_pool_token_a_or_b
            .eq(&MercurialPoolToken::TokenA)
        {
            *ctx.accounts.collateral_mint.clone()
        } else {
            *ctx.accounts.mercurial_pool_secondary_token_mint.clone()
        },
        if depository
            .is_collateral_mercurial_pool_token_a_or_b
            .eq(&MercurialPoolToken::TokenB)
        {
            *ctx.accounts.collateral_mint.clone()
        } else {
            *ctx.accounts.mercurial_pool_secondary_token_mint.clone()
        },
    )?;

    msg!("Mercurial pool infos: {}", mercurial_pool_infos);

    let after_pool_lp_token_vault_balance = ctx.accounts.depository_pool_lp_token_vault.amount;

    msg!(
        "after_pool_lp_token_vault_balance: {}",
        after_pool_lp_token_vault_balance,
    );

    let lp_token_change = I80F48::checked_from_num(
        after_pool_lp_token_vault_balance
            .checked_sub(before_pool_lp_token_vault_balance)
            .ok_or_else(|| error!(UxdError::MathError))?,
    )
    .ok_or_else(|| error!(UxdError::MathError))?;

    msg!("lp_token_change: {}", lp_token_change);

    let minted_lp_token_base_value =
        mercurial_pool_infos.calculate_pool_lp_token_base_value(lp_token_change)?;

    msg!("minted_lp_token_base_value: {}", minted_lp_token_base_value);

    // 4 - Calculate the exact amount of redeemable to mint. Mint 1:1 for the lp token worth.
    let redeemable_amount = utils::base_to_native(
        minted_lp_token_base_value,
        ctx.accounts.redeemable_mint.decimals,
    )?
    // Remove the decimals from calculation imprecision
    // Use floor instead of ceil to always makes the program to win
    .checked_floor()
    .ok_or_else(|| error!(UxdError::MathError))?;

    msg!("redeemable_amount: {}", redeemable_amount);

    // 5 - Check if the redeemable amount fit the slippage
    require!(
        redeemable_amount >= minimum_redeemable_amount,
        UxdError::SlippageReached
    );

    // 6 - Mint
    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_pda_signer),
        redeemable_amount
            .checked_to_num()
            .ok_or_else(|| error!(UxdError::MathError))?,
    )?;

    // 7 - Update accounting
    // @TODO

    // 8 - Check that we don't mint more UXD than the fixed limit
    // @TODO
    // ctx.accounts.check_redeemable_global_supply_cap_overflow()?;
    */
    Ok(())
}

// Into functions
impl<'info> MintWithMercurialPoolDepository<'info> {
    pub fn into_deposit_collateral_to_mercurial_pool_context(
        &self,
        depository: &MercurialPoolDepository,
    ) -> CpiContext<'_, '_, '_, 'info, amm::cpi::accounts::AddOrRemoveBalanceLiquidity<'info>> {
        let user_a_token = if depository
            .is_collateral_mercurial_pool_token_a_or_b
            .eq(&MercurialPoolToken::TokenA)
        {
            self.user_collateral.to_account_info()
        } else {
            self.user_mercurial_pool_secondary_token.to_account_info()
        };

        let user_b_token = if depository
            .is_collateral_mercurial_pool_token_a_or_b
            .eq(&MercurialPoolToken::TokenB)
        {
            self.user_collateral.to_account_info()
        } else {
            self.user_mercurial_pool_secondary_token.to_account_info()
        };

        let cpi_accounts = amm::cpi::accounts::AddOrRemoveBalanceLiquidity {
            user: self.user.to_account_info(),
            user_pool_lp: self.depository_pool_lp_token_vault.to_account_info(),
            user_a_token,
            user_b_token,
            pool: self.mercurial_pool.to_account_info(),
            a_vault_lp: self.mercurial_vault_a_lp.to_account_info(),
            a_vault: self.mercurial_vault_a.to_account_info(),
            a_vault_lp_mint: self.mercurial_vault_a_lp_mint.to_account_info(),
            a_token_vault: self.mercurial_vault_a_token_vault.to_account_info(),
            b_vault_lp: self.mercurial_vault_b_lp.to_account_info(),
            b_vault: self.mercurial_vault_b.to_account_info(),
            b_vault_lp_mint: self.mercurial_vault_b_lp_mint.to_account_info(),
            b_token_vault: self.mercurial_vault_b_token_vault.to_account_info(),
            vault_program: self.mercurial_vault_program.to_account_info(),
            lp_mint: self.mercurial_pool_lp_mint.to_account_info(),
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

// Validate
impl<'info> MintWithMercurialPoolDepository<'info> {
    pub fn validate(&self, collateral_amount: u64, minimum_redeemable_amount: u64) -> Result<()> {
        require!(collateral_amount != 0, UxdError::InvalidCollateralAmount);

        Ok(())
    }
}
