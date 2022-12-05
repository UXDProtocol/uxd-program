use crate::error::UxdError;
use crate::events::RegisterCredixLpDepositoryEvent;
use crate::state::credix_lp_depository::CREDIX_LP_DEPOSITORY_SPACE;
use crate::state::CredixLpDepository;
use crate::utils::validate_collateral_mint_usdc;
use crate::validate_is_program_frozen;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_ACCOUNT_VERSION;
use crate::CREDIX_LP_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct RegisterCredixLpDepository<'info> {
    /// #1
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

    /// #5
    #[account(
        constraint = collateral_mint.key() == credix_global_market_state.base_token_mint @UxdError::CredixLpDoNotMatchCollateral,
        constraint = collateral_mint.key() != credix_global_market_state.lp_token_mint @UxdError::CollateralMintEqualToRedeemableMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #6
    #[account(
        init,
        associated_token::mint = collateral_mint,
        associated_token::authority = depository,
        payer = payer,
    )]
    pub depository_collateral: Box<Account<'info, TokenAccount>>,

    /// #7
    #[account(
        init,
        associated_token::mint = credix_shares_mint,
        associated_token::authority = depository,
        payer = payer,
    )]
    pub depository_shares: Box<Account<'info, TokenAccount>>,

    /// #8
    pub credix_program_state: Box<Account<'info, credix_client::ProgramState>>,

    /// #9
    #[account(
        constraint = credix_global_market_state.base_token_mint == collateral_mint.key() @UxdError::InvalidCollateralMint,
        constraint = credix_global_market_state.lp_token_mint == credix_shares_mint.key() @UxdError::InvalidCredixSharesMint,
    )]
    pub credix_global_market_state: Box<Account<'info, credix_client::GlobalMarketState>>,

    /// #10
    /// CHECK: unused by us, checked by credix
    pub credix_signing_authority: AccountInfo<'info>,

    /// #11
    #[account(
        token::authority = credix_signing_authority,
        token::mint = collateral_mint,
    )]
    pub credix_liquidity_collateral: Box<Account<'info, TokenAccount>>,

    /// #12
    pub credix_shares_mint: Box<Account<'info, Mint>>,

    /// #13
    pub system_program: Program<'info, System>,
    /// #14
    pub token_program: Program<'info, Token>,
    /// #15
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// #16
    pub rent: Sysvar<'info, Rent>,
}

pub(crate) fn handler(
    ctx: Context<RegisterCredixLpDepository>,
    minting_fee_in_bps: u8,
    redeeming_fee_in_bps: u8,
    redeemable_amount_under_management_cap: u128,
) -> Result<()> {
    // Read some of the depositories required informations
    let depository_bump = *ctx.bumps.get("depository").ok_or(UxdError::BumpError)?;

    // Initialize the depository account
    msg!("[register_credix_lp_depository:init_depository]");
    let depository = &mut ctx.accounts.depository.load_init()?;

    // Initialize depository state
    depository.bump = depository_bump;
    depository.version = CREDIX_LP_DEPOSITORY_ACCOUNT_VERSION;

    depository.controller = ctx.accounts.controller.key();
    depository.collateral_mint = ctx.accounts.collateral_mint.key();

    depository.depository_collateral = ctx.accounts.depository_collateral.key();
    depository.depository_shares = ctx.accounts.depository_shares.key();

    // We register all necessary credix accounts to facilitate other instructions safety checks
    depository.credix_program_state = ctx.accounts.credix_program_state.key();
    depository.credix_global_market_state = ctx.accounts.credix_global_market_state.key();
    depository.credix_signing_authority = ctx.accounts.credix_signing_authority.key();
    depository.credix_liquidity_collateral = ctx.accounts.credix_liquidity_collateral.key();
    depository.credix_shares_mint = ctx.accounts.credix_shares_mint.key();

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

    // Profits collection
    depository.profits_total_collected = u128::MIN;

    // Add the depository to the controller
    ctx.accounts
        .controller
        .load_mut()?
        .add_registered_credix_lp_depository_entry(ctx.accounts.depository.key())?;

    // Emit event
    emit!(RegisterCredixLpDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: depository.version,
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
    pub(crate) fn validate(&self) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        validate_collateral_mint_usdc(&self.collateral_mint, &self.controller)?;
        Ok(())
    }
}
