use crate::error::UxdError;
use crate::events::RegisterCredixLpDepositoryEvent;
use crate::state::credix_lp_depository::CREDIX_LP_DEPOSITORY_SPACE;
use crate::state::CredixLpDepository;
use crate::utils::validate_collateral_mint_usdc;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_ACCOUNT_VERSION;
use crate::CREDIX_LP_DEPOSITORY_COLLATERAL_NAMESPACE;
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
        constraint = collateral_mint.key() == credix_lp.base_mint @UxdError::CredixLpDoNotMatchCollateral,
        constraint = collateral_mint.key() != credix_lp.shares_mint @UxdError::CollateralMintEqualToRedeemableMint
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

    /// #7
    #[account(
        constraint = credix_lp.base_mint == collateral_mint.key() @UxdError::InvalidCollateralMint,
        constraint = credix_lp.locker == credix_lp_locker.key() @UxdError::InvalidCredixLpLocker,
        constraint = credix_lp.globals == credix_globals.key() @UxdError::InvalidCredixGlobals,
        constraint = credix_lp.shares_mint == credix_shares_mint.key() @UxdError::InvalidCredixSharesMint,
    )]
    pub credix_lp: Box<Account<'info, syrup_cpi::Pool>>,
    /// #8
    #[account(mut, token::authority = credix_lp)]
    pub credix_lp_locker: Box<Account<'info, TokenAccount>>,
    /// #9
    #[account()]
    pub credix_globals: Box<Account<'info, syrup_cpi::Globals>>,
    /// #10
    /// CHECK: Does not need an ownership check because it is initialised by syrup and checked by syrup.
    #[account(mut)]
    pub credix_lender: AccountInfo<'info>,
    /// #11
    #[account(mut)]
    pub credix_shares_mint: Box<Account<'info, Mint>>,
    /// #12
    /// CHECK: Does not need an ownership check because it is initialised by syrup and checked by syrup.
    #[account(mut)]
    pub credix_locked_shares: AccountInfo<'info>,
    /// #13
    /// CHECK: Does not need an ownership check because it is initialised by syrup and checked by syrup.
    #[account(mut)]
    pub credix_lender_shares: AccountInfo<'info>,

    /// #14
    pub system_program: Program<'info, System>,
    /// #15
    pub token_program: Program<'info, Token>,
    /// #16
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// #17
    pub syrup_program: Program<'info, syrup_cpi::program::Syrup>,
    /// #18
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<RegisterCredixLpDepository>,
    redeemable_amount_under_management_cap: u128,
    minting_fee_in_bps: u8,
    redeeming_fee_in_bps: u8,
) -> Result<()> {
    // Create the credix lending accounts by calling credix's contract
    msg!("[register_credix_lp_depository:lender_initialize]");
    syrup_cpi::cpi::lender_initialize(ctx.accounts.into_initialize_lending_credix_lp_context())?;

    // Read some of the depositories required informations
    let depository_key = ctx.accounts.depository.key();
    let depository_bump = *ctx.bumps.get("depository").ok_or(UxdError::BumpError)?;

    let depository_collateral_bump = *ctx
        .bumps
        .get("depository_collateral")
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

    // We register all necessary credix accounts to facilitate other instructions safety checks
    depository.credix_lp = ctx.accounts.credix_lp.key();
    depository.credix_lp_locker = ctx.accounts.credix_lp_locker.key();
    depository.credix_globals = ctx.accounts.credix_globals.key();
    depository.credix_lender = ctx.accounts.credix_lender.key();
    depository.credix_shares_mint = ctx.accounts.credix_shares_mint.key();
    depository.credix_locked_shares = ctx.accounts.credix_locked_shares.key();
    depository.credix_lender_shares = ctx.accounts.credix_lender_shares.key();

    // Depository configuration
    depository.redeemable_amount_under_management_cap = redeemable_amount_under_management_cap;
    depository.minting_fee_in_bps = minting_fee_in_bps;
    depository.redeeming_fee_in_bps = redeeming_fee_in_bps;

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
        .add_registered_credix_lp_depository_entry(depository_key)?;

    // Emit event
    msg!("[register_credix_lp_depository:emit_event]");
    emit!(RegisterCredixLpDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: depository.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
        credix_lp: ctx.accounts.credix_lp.key(),
    });

    // Done
    Ok(())
}

// Into functions
impl<'info> RegisterCredixLpDepository<'info> {
    pub fn into_initialize_lending_credix_lp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, syrup_cpi::cpi::accounts::LenderInitialize<'info>> {
        let cpi_accounts = syrup_cpi::cpi::accounts::LenderInitialize {
            owner: self.depository.to_account_info(),
            payer: self.payer.to_account_info(),
            pool: self.credix_lp.to_account_info(),
            lender: self.credix_lender.to_account_info(),
            shares_mint: self.credix_shares_mint.to_account_info(),
            locked_shares: self.credix_locked_shares.to_account_info(),
            lender_shares: self.credix_lender_shares.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            associated_token_program: self.associated_token_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.syrup_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Validate
impl<'info> RegisterCredixLpDepository<'info> {
    pub fn validate(
        &self,
        _redeemable_amount_under_management_cap: u128,
        _minting_fee_in_bps: u8,
        _redeeming_fee_in_bps: u8,
    ) -> Result<()> {
        validate_collateral_mint_usdc(&self.collateral_mint, &self.controller)?;
        Ok(())
    }
}
