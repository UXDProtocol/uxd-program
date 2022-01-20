use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use crate::ZO_DEPOSITORY_ACCOUNT_VERSION;
use crate::ZoDepository;
use crate::Controller;
use crate::error::UxdError;
use crate::CONTROLLER_NAMESPACE;
use crate::ZO_DEPOSITORY_NAMESPACE;
use crate::COLLATERAL_PASSTHROUGH_NAMESPACE;
use crate::INSURANCE_PASSTHROUGH_NAMESPACE;
use crate::QUOTE_PASSTHROUGH_NAMESPACE;
use crate::events::RegisterZoDepositoryEvent;

/// Takes 13 accounts - 10 used locally - 0 for CPI - 2 Programs - 1 Sysvar
#[derive(Accounts)]
pub struct RegisterZoDepository<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE], 
        bump = controller.bump,
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #4 UXDProgram on chain account bound to a Controller instance (ZO)
    /// The `ZoDepository` manages a ZeroOne account for a single Collateral
    #[account(
        init,
        seeds = [ZO_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump,
        payer = payer,
    )]
    pub depository: Box<Account<'info, ZoDepository>>,

    /// #5 The collateral mint used by the `depository` instance
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #6 The insurance mint used by the `depository` instance
    pub insurance_mint: Box<Account<'info, Mint>>,

    /// #7 The insurance mint used by the `depository` instance
    pub quote_mint: Box<Account<'info, Mint>>,

    /// #8 The `depository`'s TA for its `collateral_mint`
    /// ZoAccounts can only transact with the TAs owned by their authority
    /// and this only serves as a passthrough
    #[account(
        init,
        seeds = [COLLATERAL_PASSTHROUGH_NAMESPACE, depository.key().as_ref()],
        bump,
        token::mint = collateral_mint,
        token::authority = depository,
        payer = payer,
    )]
    pub depository_collateral_passthrough_account: Box<Account<'info, TokenAccount>>,

    /// #9 The `depository`'s TA for its `insurance_mint`
    /// ZoAccounts can only transact with the TAs owned by their authority
    /// and this only serves as a passthrough
    #[account(
        init,
        seeds = [INSURANCE_PASSTHROUGH_NAMESPACE, depository.key().as_ref()],
        bump,
        token::mint = insurance_mint,
        token::authority = depository,
        payer = payer,
    )]
    pub depository_insurance_passthrough_account: Box<Account<'info, TokenAccount>>,

    /// #10 The `depository`'s TA for its `quote_mint`
    /// ZoAccounts can only transact with the TAs owned by their authority
    /// and this only serves as a passthrough
    #[account(
        init,
        seeds = [QUOTE_PASSTHROUGH_NAMESPACE, depository.key().as_ref()],
        bump,
        token::mint = quote_mint,
        token::authority = depository,
        payer = payer,
    )]
    pub depository_quote_passthrough_account: Box<Account<'info, TokenAccount>>,

    /// #11 System Program
    pub system_program: Program<'info, System>,

    /// #12 Token Program
    pub token_program: Program<'info, Token>,

    /// #13 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<RegisterZoDepository>
) -> Result<()>  {
    // - Initialize Depository state
    ctx.accounts.depository.bump = *ctx
        .bumps
        .get("depository")
        .ok_or_else(|| error!(UxdError::BumpError))?;
    ctx.accounts.depository.zo_account_bump = 0;
    ctx.accounts.depository.collateral_passthrough_bump = *ctx
        .bumps
        .get("depository_collateral_passthrough_account")
        .ok_or_else(|| error!(UxdError::BumpError))?;
    ctx.accounts.depository.insurance_passthrough_bump = *ctx
        .bumps
        .get("depository_insurance_passthrough_account")
        .ok_or_else(|| error!(UxdError::BumpError))?;
    ctx.accounts.depository.quote_passthrough_bump = *ctx
        .bumps
        .get("depository_quote_passthrough_account")
        .ok_or_else(|| error!(UxdError::BumpError))?;

    // This will need to be called in `initialize_open_orders_account` as we are limited by the stack size issues
    // Can be refactored later on when more stack space in later versions.
    ctx.accounts.depository.is_initialized = false;
    ctx.accounts.depository.zo_account = Pubkey::default();

    ctx.accounts.depository.collateral_mint = ctx.accounts.collateral_mint.key();
    ctx.accounts.depository.collateral_mint_decimals = ctx.accounts.collateral_mint.decimals;
    ctx.accounts.depository.collateral_passthrough =
        ctx.accounts.depository_collateral_passthrough_account.key();
    ctx.accounts.depository.insurance_mint =  ctx.accounts.insurance_mint.key();
    ctx.accounts.depository.insurance_mint_decimals = ctx.accounts.insurance_mint.decimals;
    ctx.accounts.depository.insurance_passthrough =
        ctx.accounts.depository_insurance_passthrough_account.key();
    ctx.accounts.depository.quote_mint = ctx.accounts.quote_mint.key();
    ctx.accounts.depository.quote_mint_decimals = ctx.accounts.quote_mint.decimals;
    ctx.accounts.depository.quote_passthrough =
        ctx.accounts.depository_quote_passthrough_account.key();

    ctx.accounts.depository.version = ZO_DEPOSITORY_ACCOUNT_VERSION;
    ctx.accounts.depository.controller = ctx.accounts.controller.key();
    ctx.accounts.depository.insurance_amount_deposited = u128::MIN;
    ctx.accounts.depository.collateral_amount_deposited = u128::MIN;
    ctx.accounts.depository.redeemable_amount_under_management = u128::MIN;
    ctx.accounts.depository.total_amount_rebalanced = u128::MIN;

    // - Update Controller state
    ctx.accounts.controller.add_registered_zo_depository_entry(ctx.accounts.depository.key())?;

    emit!(RegisterZoDepositoryEvent {
        version: ctx.accounts.controller.version,
        depository_version: ctx.accounts.depository.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
        insurance_mint: ctx.accounts.insurance_mint.key(),
        quote_mint: ctx.accounts.quote_mint.key(),
    });

    Ok(())
}