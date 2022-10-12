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
    // Account 1
    #[account()]
    pub authority: Signer<'info>,

    // Account 2
    #[account(mut)]
    pub payer: Signer<'info>,

    // Account 3
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: AccountLoader<'info, Controller>,

    // Account 4
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

    // Account 5
    #[account(
        init,
        seeds = [MAPLE_POOL_DEPOSITORY_COLLATERAL_NAMESPACE, depository.key().as_ref(), collateral_mint.key().as_ref()],
        token::authority = depository,
        token::mint = collateral_mint,
        bump,
        payer = payer,
    )]
    pub depository_collateral: Box<Account<'info, TokenAccount>>,

    // Account 6
    #[account(
        constraint = collateral_mint.key() == maple_pool.base_mint @UxdError::MaplePoolDoNotMatchCollateral,
        constraint = collateral_mint.key() != maple_pool.shares_mint @UxdError::CollateralMintEqualToRedeemableMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,

    // Account 7
    #[account()]
    pub maple_pool: Box<Account<'info, syrup_cpi::Pool>>,

    // Account 8
    /// CHECK: Does not need an ownership check because it is initialised by syrup and checked by syrup.
    #[account(mut)]
    pub maple_lender: AccountInfo<'info>,

    // Account 9
    #[account(mut, address = maple_pool.shares_mint)]
    pub maple_shares_mint: Box<Account<'info, Mint>>,

    // Account 10
    /// CHECK: Does not need an ownership check because it is initialised by syrup and checked by syrup.
    #[account(mut)]
    pub maple_locked_shares: AccountInfo<'info>,

    // Account 11
    /// CHECK: Does not need an ownership check because it is initialised by syrup and checked by syrup.
    #[account(mut)]
    pub maple_lender_shares: AccountInfo<'info>,

    // Account 12
    pub system_program: Program<'info, System>,
    // Account 13
    pub token_program: Program<'info, Token>,
    // Account 14
    pub associated_token_program: Program<'info, AssociatedToken>,
    // Account 15
    pub syrup_program: Program<'info, syrup_cpi::program::Syrup>,
    // Account 16
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<RegisterMaplePoolDepository>,
    minted_redeemable_soft_cap: u128,
    minting_fees_in_bps: u8,
    redeeming_fees_in_bps: u8,
) -> Result<()> {
    // Create the maple lending accounts by calling maple's contract
    syrup_cpi::cpi::lender_initialize(ctx.accounts.into_initialize_lending_maple_pool_context())?;

    // Read some of the depositories required informations
    let depository_key = ctx.accounts.depository.key();
    let depository_bump = *ctx.bumps.get("depository").ok_or(UxdError::BumpError)?;

    let depository_collateral_bump = *ctx
        .bumps
        .get("depository_collateral")
        .ok_or(UxdError::BumpError)?;

    let depository = &mut ctx.accounts.depository.load_init()?;

    // Initialize depository state
    depository.bump = depository_bump;
    depository.version = MAPLE_POOL_DEPOSITORY_ACCOUNT_VERSION;

    depository.controller = ctx.accounts.controller.key();
    depository.collateral_mint = ctx.accounts.collateral_mint.key();

    depository.depository_collateral = ctx.accounts.depository_collateral.key();
    depository.depository_collateral_bump = depository_collateral_bump;

    depository.maple_pool = ctx.accounts.maple_pool.key();
    depository.maple_lender = ctx.accounts.maple_lender.key();
    depository.maple_shares_mint = ctx.accounts.maple_shares_mint.key();
    depository.maple_locked_shares = ctx.accounts.maple_locked_shares.key();
    depository.maple_lender_shares = ctx.accounts.maple_lender_shares.key();

    // Depository configuration
    depository.minted_redeemable_soft_cap = minted_redeemable_soft_cap;
    depository.minting_fees_in_bps = minting_fees_in_bps;
    depository.redeeming_fees_in_bps = redeeming_fees_in_bps;

    // Depository accounting
    depository.deposited_collateral_amount = u128::MIN;
    depository.minted_redeemable_amount = u128::MIN;
    depository.minting_fees_total_paid = u128::MIN;
    depository.redeeming_fees_total_paid = u128::MIN;

    // Add the depository to the controller
    ctx.accounts
        .controller
        .load_mut()?
        .add_registered_maple_pool_depository_entry(depository_key)?;

    // Emit event
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
        _minted_redeemable_soft_cap: u128,
        _minting_fees_in_bps: u8,
        _redeeming_fees_in_bps: u8,
    ) -> Result<()> {
        validate_collateral_mint_usdc(&self.collateral_mint, &self.controller)?;
        Ok(())
    }
}
