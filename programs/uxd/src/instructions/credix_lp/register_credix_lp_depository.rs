use crate::error::UxdError;
use crate::events::RegisterCredixLpDepositoryEvent;
use crate::state::credix_lp_depository::CREDIX_LP_DEPOSITORY_SPACE;
use crate::state::CredixLpDepository;
use crate::utils::validate_collateral_mint_usdc;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_ACCOUNT_VERSION;
use crate::CREDIX_LP_DEPOSITORY_COLLATERAL_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_LP_SHARES_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct RegisterCredixLpDepository<'info> {
    /// #1
    #[account()]
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
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4
    #[account(
        init,
        seeds = [
            CREDIX_LP_DEPOSITORY_NAMESPACE,
            credix_global_market_state.key().as_ref(),
            collateral_mint.key().as_ref()
        ],
        bump,
        payer = payer,
        space = CREDIX_LP_DEPOSITORY_SPACE,
    )]
    pub depository: AccountLoader<'info, CredixLpDepository>,

    /// #6
    #[account(
        constraint = collateral_mint.key() == credix_global_market_state.base_token_mint @UxdError::CredixLpDoNotMatchCollateral,
        constraint = collateral_mint.key() != credix_global_market_state.lp_token_mint @UxdError::CollateralMintEqualToRedeemableMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #5
    #[account(
        init,
        seeds = [CREDIX_LP_DEPOSITORY_COLLATERAL_NAMESPACE, depository.key().as_ref()],
        token::authority = depository,
        token::mint = collateral_mint,
        bump,
        payer = payer,
    )]
    pub depository_collateral: Box<Account<'info, TokenAccount>>,

    /// #5
    #[account(
        init,
        seeds = [CREDIX_LP_DEPOSITORY_LP_SHARES_NAMESPACE, depository.key().as_ref()],
        token::authority = depository,
        token::mint = credix_lp_shares_mint,
        bump,
        payer = payer,
    )]
    pub depository_lp_shares: Box<Account<'info, TokenAccount>>,

    /// #7
    #[account(
        constraint = credix_global_market_state.base_token_mint == collateral_mint.key() @UxdError::InvalidCollateralMint,
        constraint = credix_global_market_state.lp_token_mint == credix_lp_shares_mint.key() @UxdError::InvalidCredixLpSharesMint,
        constraint = credix_global_market_state.treasury_pool_token_account == credix_treasury_collateral.key() @UxdError::InvalidCredixTreasuryCollateral,
    )]
    pub credix_global_market_state: Box<Account<'info, credix_client::GlobalMarketState>>,
    /// #8
    #[account()] // TODO - check
    pub credix_signing_authority: AccountInfo<'info>,
    /// #9
    #[account(token::mint = collateral_mint)]
    pub credix_treasury_collateral: Box<Account<'info, TokenAccount>>,
    /// #10
    #[account(token::mint = collateral_mint)]
    pub credix_liquidity_collateral: Box<Account<'info, TokenAccount>>,
    /// #11
    #[account()]
    pub credix_lp_shares_mint: Box<Account<'info, Mint>>,
    /// #13
    #[account()] // TODO - check
    pub credix_pass: AccountInfo<'info>,

    /// #14
    pub system_program: Program<'info, System>,
    /// #15
    pub token_program: Program<'info, Token>,
    /// #16
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// #17
    pub credix_program: Program<'info, credix_client::program::Credix>,
    /// #18
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<RegisterCredixLpDepository>,
    minting_fee_in_bps: u8,
    redeeming_fee_in_bps: u8,
    redeemable_amount_under_management_cap: u128,
) -> Result<()> {
    // Read some of the depositories required informations
    let depository_bump = *ctx.bumps.get("depository").ok_or(UxdError::BumpError)?;

    let depository_collateral_bump = *ctx
        .bumps
        .get("depository_collateral")
        .ok_or(UxdError::BumpError)?;
    let depository_lp_shares_bump = *ctx
        .bumps
        .get("depository_lp_shares")
        .ok_or(UxdError::BumpError)?;

    // Initialize the depository account
    msg!("[register_credix_lp_depository:init_depository]");
    let depository = &mut ctx.accounts.depository.load_init()?;

    // Initialize depository state
    depository.bump = depository_bump;
    depository.version = CREDIX_LP_DEPOSITORY_ACCOUNT_VERSION;

    depository.controller = ctx.accounts.controller.key();
    depository.collateral_mint = ctx.accounts.collateral_mint.key();

    depository.depository_collateral = ctx.accounts.depository_collateral.key();
    depository.depository_collateral_bump = depository_collateral_bump;

    depository.depository_lp_shares = ctx.accounts.depository_lp_shares.key();
    depository.depository_lp_shares_bump = depository_lp_shares_bump;

    // We register all necessary credix accounts to facilitate other instructions safety checks
    depository.credix_global_market_state = ctx.accounts.credix_global_market_state.key();
    depository.credix_signing_authority = ctx.accounts.credix_signing_authority.key();
    depository.credix_treasury_collateral = ctx.accounts.credix_treasury_collateral.key();
    depository.credix_liquidity_collateral = ctx.accounts.credix_liquidity_collateral.key();
    depository.credix_lp_shares_mint = ctx.accounts.credix_lp_shares_mint.key();
    depository.credix_pass = ctx.accounts.credix_pass.key();

    // Depository configuration
    depository.redeemable_amount_under_management_cap = redeemable_amount_under_management_cap;
    depository.minting_fee_in_bps = minting_fee_in_bps;
    depository.redeeming_fee_in_bps = redeeming_fee_in_bps;
    depository.minting_disabled = false;

    // Depository accounting
    depository.collateral_amount_deposited = u128::MIN;
    depository.redeemable_amount_under_management = u128::MIN;
    depository.minting_fee_total_accrued = u128::MIN;
    depository.redeeming_fee_total_accrued = u128::MIN;

    // Add the depository to the controller
    msg!("[register_credix_lp_depository:register_depository]");
    ctx.accounts
        .controller
        .load_mut()?
        .add_registered_credix_lp_depository_entry(ctx.accounts.depository.key())?;

    // Emit event
    msg!("[register_credix_lp_depository:emit_event]");
    emit!(RegisterCredixLpDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: ctx.accounts.depository.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
        credix_global_market_state: ctx.accounts.credix_global_market_state.key(),
    });

    // Done
    Ok(())
}

// Validate
impl<'info> RegisterCredixLpDepository<'info> {
    pub fn validate(
        &self,
        _minting_fee_in_bps: u8,
        _redeeming_fee_in_bps: u8,
        _redeemable_amount_under_management_cap: u128,
    ) -> Result<()> {
        validate_collateral_mint_usdc(&self.collateral_mint, &self.controller)?;
        Ok(())
    }
}
