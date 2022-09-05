use crate::error::UxdError;
use crate::events::RegisterMercurialPoolDepositoryEvent;
use crate::Controller;
use crate::MercurialPoolDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MERCURIAL_POOL_DEPOSITORY_ACCOUNT_VERSION;
use crate::MERCURIAL_POOL_DEPOSITORY_LP_VAULT_NAMESPACE;
use crate::MERCURIAL_POOL_DEPOSITORY_NAMESPACE;
use crate::MERCURIAL_POOL_DEPOSITORY_SPACE;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct RegisterMercurialPoolDepository<'info> {
    pub authority: Signer<'info>,

    // In order to use with governance program, as the PDA cannot be the payer in nested TX.
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: AccountLoader<'info, Controller>,

    #[account(
        init,
        seeds = [MERCURIAL_POOL_DEPOSITORY_NAMESPACE, mercurial_pool.key().as_ref(), collateral_mint.key().as_ref()],
        bump,
        payer = payer,
        space = MERCURIAL_POOL_DEPOSITORY_SPACE,
    )]
    pub depository: AccountLoader<'info, MercurialPoolDepository>,

    pub collateral_mint: Box<Account<'info, Mint>>,

    pub mercurial_pool: Box<Account<'info, amm::state::Pool>>,

    pub mercurial_pool_lp_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [MERCURIAL_POOL_DEPOSITORY_LP_VAULT_NAMESPACE, mercurial_pool.key().as_ref(), collateral_mint.key().as_ref()],
        token::authority = depository,
        token::mint = mercurial_pool_lp_mint,
        bump,
        payer = payer,
    )]
    pub depository_pool_lp_token_vault: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<RegisterMercurialPoolDepository>) -> Result<()> {
    let collateral_mint = ctx.accounts.collateral_mint.key();

    let depository_bump = *ctx
        .bumps
        .get("depository")
        .ok_or_else(|| error!(UxdError::BumpError))?;

    let depository_pool_lp_token_vault_bump = *ctx
        .bumps
        .get("depository_pool_lp_token_vault")
        .ok_or_else(|| error!(UxdError::BumpError))?;

    let depository = &mut ctx.accounts.depository.load_init()?;

    // 1 - Initialize Depository state
    depository.bump = depository_bump;

    depository.version = MERCURIAL_POOL_DEPOSITORY_ACCOUNT_VERSION;
    depository.collateral_mint = collateral_mint;
    depository.collateral_mint_decimals = ctx.accounts.collateral_mint.decimals;
    depository.controller = ctx.accounts.controller.key();
    depository.collateral_amount_deposited = u128::MIN;

    depository.pool_lp_token_vault = ctx.accounts.depository_pool_lp_token_vault.key();
    depository.pool_lp_token_vault_bump = depository_pool_lp_token_vault_bump;

    depository.pool_lp_mint = ctx.accounts.mercurial_pool_lp_mint.key();
    depository.pool_lp_mint_decimals = ctx.accounts.mercurial_pool_lp_mint.decimals;

    depository.mercurial_pool = ctx.accounts.mercurial_pool.key();

    if ctx
        .accounts
        .mercurial_pool
        .token_a_mint
        .eq(&ctx.accounts.collateral_mint.key())
    {
        depository.collateral_is_mercurial_pool_token_a = true;
        depository.collateral_is_mercurial_pool_token_b = false;
    } else {
        depository.collateral_is_mercurial_pool_token_a = false;
        depository.collateral_is_mercurial_pool_token_b = true;
    }

    // 2 - Update Controller state
    ctx.accounts
        .controller
        .load_mut()?
        .add_registered_mercurial_pool_depository_entry(ctx.accounts.depository.key())?;

    // 3 - Emit event
    emit!(RegisterMercurialPoolDepositoryEvent {
        version: ctx.accounts.controller.load()?.version,
        depository_version: depository.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
    });

    Ok(())
}

// Validate
impl<'info> RegisterMercurialPoolDepository<'info> {
    pub fn validate(&self) -> Result<()> {
        // Collateral mint should be either the Token A or Token B of the provided mercurial pool
        let pool_token_a_mint_match_collateral_mint = self
            .mercurial_pool
            .token_a_mint
            .eq(&self.collateral_mint.key());

        let pool_token_b_mint_match_collateral_mint = self
            .mercurial_pool
            .token_b_mint
            .eq(&self.collateral_mint.key());

        require!(
            pool_token_a_mint_match_collateral_mint || pool_token_b_mint_match_collateral_mint,
            UxdError::MercurialPoolDoNotMatchCollateral,
        );

        // Collateral mint should be different than redeemable mint
        require!(
            self.collateral_mint
                .key()
                .ne(&self.controller.load()?.redeemable_mint),
            UxdError::MercurialPoolDoNotMatchCollateral,
        );

        // Only accept stable pools with ratio 1:1, like USDT/USDC, USDC/UXD etc.
        let stable_curve_type = amm::curve::curve_type::CurveType::default();

        require!(
            self.mercurial_pool
                .curve_type
                .is_same_type(&stable_curve_type),
            UxdError::MercurialPoolIsNotStable
        );

        // Do not validate this for now as devnet doesn't work with UXD pool for now
        /* let usdc_mint: Pubkey = Pubkey::new(b"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

        // Only accept USDC mint for now, as other collaterals will cause logic errors on mint/redeem with mercurial vault
        require!(
            self.collateral_mint.key().eq(&usdc_mint),
            UxdError::InvalidCollateralAmount
        );*/

        Ok(())
    }
}
