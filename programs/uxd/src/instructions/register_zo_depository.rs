use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use crate::ZO_DEPOSITORY_ACCOUNT_VERSION;
use crate::ZoDepository;
use crate::Controller;
use crate::error::UxdError;
use crate::CONTROLLER_NAMESPACE;
use crate::ZO_DEPOSITORY_NAMESPACE;
use crate::ZO_DEPOSITORY_SPACE;
use crate::events::RegisterZoDepositoryEvent;

/// Takes 10 accounts - 7 used locally - 0 for CPI - 2 Programs - 1 Sysvar
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
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4 UXDProgram on chain account bound to a Controller instance (ZO)
    /// The `ZoDepository` manages a ZeroOne account for a single Collateral
    #[account(
        init,
        seeds = [ZO_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump,
        payer = payer,
        space = ZO_DEPOSITORY_SPACE
    )]
    pub depository: AccountLoader<'info, ZoDepository>,

    /// #5 The collateral mint used by the `depository` instance
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #6 The insurance mint used by the `depository` instance
    pub quote_mint: Box<Account<'info, Mint>>,

    /// #7 System Program
    pub system_program: Program<'info, System>,

    /// #8 Token Program
    pub token_program: Program<'info, Token>,

    /// #9 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<RegisterZoDepository>
) -> Result<()>  {
    let depository = &mut ctx.accounts.depository.load_init()?;

    // - Initialize Depository state
    depository.bump = *ctx
        .bumps
        .get("depository")
        .ok_or_else(|| error!(UxdError::BumpError))?;


    // This will need to be called in `initialize_open_orders_account` as we are limited by the stack size issues
    // Can be refactored later on when more stack space in later versions.
    depository.zo_account_bump = 0;
    depository.is_initialized = false;
    depository.zo_account = Pubkey::default();

    depository.collateral_mint = ctx.accounts.collateral_mint.key();
    depository.collateral_mint_decimals = ctx.accounts.collateral_mint.decimals;
    depository.quote_mint = ctx.accounts.quote_mint.key();
    depository.quote_mint_decimals = ctx.accounts.quote_mint.decimals;

    depository.version = ZO_DEPOSITORY_ACCOUNT_VERSION;
    depository.controller = ctx.accounts.controller.key();
    depository.insurance_amount_deposited = u128::MIN;
    depository.collateral_amount_deposited = u128::MIN;
    depository.redeemable_amount_under_management = u128::MIN;
    depository.total_amount_rebalanced = u128::MIN;
    
    // - Update Controller state
    ctx.accounts.controller.load_mut()?.add_registered_zo_depository_entry(ctx.accounts.depository.key())?;

    emit!(RegisterZoDepositoryEvent {
        version: ctx.accounts.controller.load()?.version,
        depository_version: depository.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
        quote_mint: ctx.accounts.quote_mint.key(),
    });

    Ok(())
}