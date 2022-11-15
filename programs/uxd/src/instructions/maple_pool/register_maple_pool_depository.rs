use crate::error::UxdError;
use crate::events::RegisterMaplePoolDepositoryEvent;
use crate::state::maple_pool_depository::MAPLE_POOL_DEPOSITORY_SPACE;
use crate::state::MaplePoolDepository;
use crate::utils::validate_collateral_mint_usdc;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::MAPLE_POOL_DEPOSITORY_ACCOUNT_VERSION;
use crate::MAPLE_POOL_DEPOSITORY_COLLATERAL_NAMESPACE;
use crate::MAPLE_POOL_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct RegisterMaplePoolDepository<'info> {
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
            MAPLE_POOL_DEPOSITORY_NAMESPACE,
            maple_pool.key().as_ref(),
            collateral_mint.key().as_ref()
        ],
        bump,
        payer = payer,
        space = MAPLE_POOL_DEPOSITORY_SPACE,
    )]
    pub depository: AccountLoader<'info, MaplePoolDepository>,

    /// #5
    #[account(
        init,
        seeds = [MAPLE_POOL_DEPOSITORY_COLLATERAL_NAMESPACE, depository.key().as_ref()],
        token::authority = depository,
        token::mint = collateral_mint,
        bump,
        payer = payer,
    )]
    pub depository_collateral: Box<Account<'info, TokenAccount>>,

    /// #6
    #[account(
        constraint = collateral_mint.key() == maple_pool.base_mint @UxdError::MaplePoolDoNotMatchCollateral,
        constraint = collateral_mint.key() != maple_pool.shares_mint @UxdError::CollateralMintEqualToRedeemableMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #7
    #[account(
        constraint = maple_pool.base_mint == collateral_mint.key() @UxdError::InvalidCollateralMint,
        constraint = maple_pool.locker == maple_pool_locker.key() @UxdError::InvalidMaplePoolLocker,
        constraint = maple_pool.globals == maple_globals.key() @UxdError::InvalidMapleGlobals,
        constraint = maple_pool.shares_mint == maple_shares_mint.key() @UxdError::InvalidMapleSharesMint,
    )]
    pub maple_pool: Box<Account<'info, syrup_cpi::Pool>>,
    /// #8
    #[account(mut, token::authority = maple_pool)]
    pub maple_pool_locker: Box<Account<'info, TokenAccount>>,
    /// #9
    pub maple_globals: Box<Account<'info, syrup_cpi::Globals>>,
    /// #10
    /// CHECK: Does not need an ownership check because it is initialised by syrup and checked by syrup.
    #[account(mut)]
    pub maple_lender: AccountInfo<'info>,
    /// #11
    #[account(mut)]
    pub maple_shares_mint: Box<Account<'info, Mint>>,
    /// #12
    /// CHECK: Does not need an ownership check because it is initialised by syrup and checked by syrup.
    #[account(mut)]
    pub maple_locked_shares: AccountInfo<'info>,
    /// #13
    /// CHECK: Does not need an ownership check because it is initialised by syrup and checked by syrup.
    #[account(mut)]
    pub maple_lender_shares: AccountInfo<'info>,

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
    ctx: Context<RegisterMaplePoolDepository>,
    minting_fee_in_bps: u8,
    redeeming_fee_in_bps: u8,
    redeemable_amount_under_management_cap: u128,
) -> Result<()> {
    // Create the maple lending accounts by calling maple's contract
    msg!("[register_maple_pool_depository:lender_initialize]");
    syrup_cpi::cpi::lender_initialize(ctx.accounts.into_initialize_lending_maple_pool_context())?;

    // Read some of the depositories required informations (before we start mutating it)
    let depository_bump = *ctx.bumps.get("depository").ok_or(UxdError::BumpError)?;
    let depository_collateral_bump = *ctx
        .bumps
        .get("depository_collateral")
        .ok_or(UxdError::BumpError)?;

    // Initialize the depository account
    msg!("[register_maple_pool_depository:init_depository]");
    let depository = &mut ctx.accounts.depository.load_init()?;

    // Initialize depository state
    depository.bump = depository_bump;
    depository.version = MAPLE_POOL_DEPOSITORY_ACCOUNT_VERSION;

    depository.controller = ctx.accounts.controller.key();
    depository.collateral_mint = ctx.accounts.collateral_mint.key();

    depository.depository_collateral = ctx.accounts.depository_collateral.key();
    depository.depository_collateral_bump = depository_collateral_bump;

    // We register all necessary maple accounts to facilitate other instructions safety checks
    depository.maple_pool = ctx.accounts.maple_pool.key();
    depository.maple_pool_locker = ctx.accounts.maple_pool_locker.key();
    depository.maple_globals = ctx.accounts.maple_globals.key();
    depository.maple_lender = ctx.accounts.maple_lender.key();
    depository.maple_shares_mint = ctx.accounts.maple_shares_mint.key();
    depository.maple_locked_shares = ctx.accounts.maple_locked_shares.key();
    depository.maple_lender_shares = ctx.accounts.maple_lender_shares.key();

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
    msg!("[register_maple_pool_depository:register_depository]");
    ctx.accounts
        .controller
        .load_mut()?
        .add_registered_maple_pool_depository_entry(ctx.accounts.depository.key())?;

    // Emit event
    msg!("[register_maple_pool_depository:emit_event]");
    emit!(RegisterMaplePoolDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: depository.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
        maple_pool: ctx.accounts.maple_pool.key(),
    });

    // Done
    Ok(())
}

// Into functions
impl<'info> RegisterMaplePoolDepository<'info> {
    pub fn into_initialize_lending_maple_pool_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, syrup_cpi::cpi::accounts::LenderInitialize<'info>> {
        let cpi_accounts = syrup_cpi::cpi::accounts::LenderInitialize {
            owner: self.depository.to_account_info(),
            payer: self.payer.to_account_info(),
            pool: self.maple_pool.to_account_info(),
            lender: self.maple_lender.to_account_info(),
            shares_mint: self.maple_shares_mint.to_account_info(),
            locked_shares: self.maple_locked_shares.to_account_info(),
            lender_shares: self.maple_lender_shares.to_account_info(),
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
impl<'info> RegisterMaplePoolDepository<'info> {
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
