use crate::error::UxdError;
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
    /// Unused by mercurial program, but needed as parameter
    #[account(
        mut,
        constraint = &user_mercurial_pool_secondary_token.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_mercurial_pool_secondary_token: Box<Account<'info, TokenAccount>>,

    /// #10
    #[account(
        mut,
        seeds = [MERCURIAL_POOL_DEPOSITORY_LP_VAULT_NAMESPACE, mercurial_pool.key().as_ref(), collateral_mint.key().as_ref()],
        token::authority = depository,
        token::mint = mercurial_pool_lp_mint,
        bump = depository.load()?.pool_lp_token_vault_bump,
    )]
    pub depository_pool_lp_token_vault: Box<Account<'info, TokenAccount>>,

    /// #11
    #[account(mut)]
    pub mercurial_pool: Box<Account<'info, amm::state::Pool>>,

    /// #12
    #[account(mut)]
    pub mercurial_pool_lp_mint: Box<Account<'info, Mint>>,

    /// #13
    /// CHECK: Mercurial Amm CPI - checked Mercurial side
    #[account(mut)]
    pub mercurial_vault_a: UncheckedAccount<'info>,

    /// #14
    #[account(
        mut,
        token::mint = mercurial_vault_a_lp_mint,
    )]
    pub mercurial_vault_a_lp: Box<Account<'info, TokenAccount>>,

    /// #15
    #[account(mut)]
    pub mercurial_vault_a_lp_mint: Box<Account<'info, Mint>>,

    /// #16
    #[account(mut)]
    pub mercurial_vault_a_token_vault: Box<Account<'info, TokenAccount>>,

    /// #17
    /// CHECK: Mercurial Amm CPI - checked Mercurial side
    #[account(mut)]
    pub mercurial_vault_b: UncheckedAccount<'info>,

    /// #18
    #[account(mut)]
    pub mercurial_vault_b_lp_mint: Box<Account<'info, Mint>>,

    /// #19
    #[account(
        mut,
        token::mint = mercurial_vault_b_lp_mint,
    )]
    pub mercurial_vault_b_lp: Box<Account<'info, TokenAccount>>,

    /// #20
    #[account(mut)]
    pub mercurial_vault_b_token_vault: Box<Account<'info, TokenAccount>>,

    /// #21
    /// CHECK: Mercurial Amm CPI - checked Mercurial side
    pub mercurial_vault_program: UncheckedAccount<'info>,

    /// #22
    pub system_program: Program<'info, System>,

    /// #23
    pub token_program: Program<'info, Token>,

    /// #24
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
    let controller_bump = ctx.accounts.controller.load()?.bump;
    let controller_pda_signer: &[&[&[u8]]] = &[&[CONTROLLER_NAMESPACE, &[controller_bump]]];

    let depository = ctx.accounts.depository.load()?;

    let before_pool_lp_token_vault_balance = ctx.accounts.depository_pool_lp_token_vault.amount;

    msg!(
        "before_pool_lp_token_vault_balance {}",
        before_pool_lp_token_vault_balance
    );

    // 1 - Deposit collateral to mercurial pool and get lp tokens
    let token_a_amount = if depository.collateral_is_mercurial_pool_token_a {
        collateral_amount
    } else {
        0
    };

    let token_b_amount = if depository.collateral_is_mercurial_pool_token_b {
        collateral_amount
    } else {
        0
    };

    msg!(
        "Add imbalance liquidity token a {} token b {} ",
        token_a_amount,
        token_b_amount
    );

    // One side deposit
    amm::cpi::add_imbalance_liquidity(
        ctx.accounts
            .into_deposit_collateral_to_mercurial_pool_context(&depository),
        // Do not handle slippage here, later on we are going to check the lp token amount out with minimum_redeemable_amount
        // and throw an error if not enough
        0,
        token_a_amount,
        token_b_amount,
    )?;

    // 2 - Reload accounts impacted by the deposit (We need updated numbers for further calculation)
    ctx.accounts.mercurial_pool.reload()?;
    ctx.accounts.depository_pool_lp_token_vault.reload()?;
    ctx.accounts.mercurial_pool_lp_mint.reload()?;

    // 3 - Calculate the redeemable amount to mint, based on the minted LP tokens amount and the price of the pool
    let after_pool_lp_token_vault_balance = ctx.accounts.depository_pool_lp_token_vault.amount;

    msg!(
        "after_pool_lp_token_vault_balance {}",
        after_pool_lp_token_vault_balance
    );

    msg!(
        "ctx.accounts.mercurial_vault_lp_mint.supply {}",
        ctx.accounts.mercurial_pool_lp_mint.supply
    );

    let pool_lp_token_change_u64 = after_pool_lp_token_vault_balance
        .checked_sub(before_pool_lp_token_vault_balance)
        .ok_or_else(|| error!(UxdError::MathError))?;

    // let pool_lp_token_change = I80F48::from_num(pool_lp_token_change_u64);

    msg!("Pool Token Change {}", pool_lp_token_change_u64);

    /*
    let current_time = u64::try_from(Clock::get()?.unix_timestamp)
        .ok()
        .ok_or(UxdError::MathError)?;

    let unlocked_amount_u64 = ctx
        .accounts
        .mercurial_pool
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
        */

    // let lp_token_minted_value = 0;

    // 4 - Mint redeemable 1:1 - The only allowed mint for mercurial vault depository is USDC for now
    /*let redeemable_mint_amount = lp_token_minted_value
    .checked_to_num()
    .ok_or(UxdError::MathError)?;*/

    // For now mint 1:1 but should not be this
    let redeemable_mint_amount = collateral_amount;

    msg!("redeemable_mint_amount {}", redeemable_mint_amount);

    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_pda_signer),
        redeemable_mint_amount,
    )?;

    // 4 - Update accounting
    // @TODO

    // 6 - Check that we don't mint more UXD than the fixed limit
    // @TODO
    // ctx.accounts.check_redeemable_global_supply_cap_overflow()?;
    Ok(())
}

// Into functions
impl<'info> MintWithMercurialPoolDepository<'info> {
    pub fn into_deposit_collateral_to_mercurial_pool_context(
        &self,
        depository: &MercurialPoolDepository,
    ) -> CpiContext<'_, '_, '_, 'info, amm::cpi::accounts::AddOrRemoveBalanceLiquidity<'info>> {
        let user_a_token = if depository.collateral_is_mercurial_pool_token_a {
            self.user_collateral.to_account_info()
        } else {
            self.user_mercurial_pool_secondary_token.to_account_info()
        };

        let user_b_token = if depository.collateral_is_mercurial_pool_token_b {
            self.user_collateral.to_account_info()
        } else {
            self.user_mercurial_pool_secondary_token.to_account_info()
        };

        let cpi_accounts = amm::cpi::accounts::AddOrRemoveBalanceLiquidity {
            user: self.user.to_account_info(),
            user_pool_lp: self.depository_pool_lp_token_vault.to_account_info(),
            user_a_token: user_a_token,
            user_b_token: user_b_token,
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
