use crate::error::UxdError;
use crate::events::RegisterMangoDepositoryEventV2;
use crate::Controller;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_ACCOUNT_VERSION;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::MANGO_DEPOSITORY_SPACE;
use anchor_comp::mango_markets_v3;
use anchor_comp::mango_markets_v3::MangoMarketV3;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use mango::state::MangoAccount;
use std::mem::size_of;

const MANGO_ACCOUNT_SPAN: usize = size_of::<MangoAccount>();

/// Takes 12 accounts - 8 used locally - 1 for CPI - 3 Programs - 1 Sysvar
#[derive(Accounts)]
pub struct RegisterMangoDepository<'info> {
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

    /// #4 UXDProgram on chain account bound to a Controller instance
    /// The `MangoDepository` manages a MangoAccount for a single Collateral
    #[account(
        init,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump,
        payer = payer,
        space = MANGO_DEPOSITORY_SPACE,
    )]
    pub depository: AccountLoader<'info, MangoDepository>,

    /// #5 The collateral mint used by the `depository` instance
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #6 The insurance mint used by the `depository` instance
    pub quote_mint: Box<Account<'info, Mint>>,

    /// #7 The MangoMarkets Account (MangoAccount) managed by the `depository`
    /// CHECK : Seeds checked. Depository registered
    #[account(
        init,
        seeds = [MANGO_ACCOUNT_NAMESPACE, collateral_mint.key().as_ref()],
        bump,
        owner = MangoMarketV3::id(),
        payer = payer,
        space = MANGO_ACCOUNT_SPAN,
    )]
    pub mango_account: AccountInfo<'info>,

    /// #8 [MangoMarkets CPI] Index grouping perp and spot markets
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_group: UncheckedAccount<'info>,

    /// #9 System Program
    pub system_program: Program<'info, System>,

    /// #10 MangoMarketv3 Program
    pub mango_program: Program<'info, MangoMarketV3>,

    /// #11 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub(crate) fn handler(ctx: Context<RegisterMangoDepository>) -> Result<()> {
    let collateral_mint = ctx.accounts.collateral_mint.key();
    let quote_mint = ctx.accounts.quote_mint.key();

    let depository_bump = *ctx
        .bumps
        .get("depository")
        .ok_or_else(|| error!(UxdError::BumpError))?;

    // - Initialize Mango Account
    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[depository_bump],
    ]];
    mango_markets_v3::init_mango_account(
        ctx.accounts
            .to_mango_init_account_context()
            .with_signer(depository_signer_seed),
    )?;

    // - Initialize Depository state
    let depository = &mut ctx.accounts.depository.load_init()?;
    let mango_account_bump = *ctx
        .bumps
        .get("mango_account")
        .ok_or_else(|| error!(UxdError::BumpError))?;
    depository.bump = depository_bump;
    depository.mango_account_bump = mango_account_bump;
    depository.version = MANGO_DEPOSITORY_ACCOUNT_VERSION;
    depository.collateral_mint = collateral_mint;
    depository.collateral_mint_decimals = ctx.accounts.collateral_mint.decimals;
    depository.quote_mint = quote_mint;
    depository.quote_mint_decimals = ctx.accounts.quote_mint.decimals;
    depository.mango_account = ctx.accounts.mango_account.key();
    depository.controller = ctx.accounts.controller.key();
    depository.insurance_amount_deposited = u128::MIN;
    depository.collateral_amount_deposited = u128::MIN;
    depository.redeemable_amount_under_management = u128::MIN;
    depository.total_amount_paid_taker_fee = u128::MIN;
    depository.total_amount_rebalanced = u128::MIN;
    depository.regular_minting_disabled = false; // enable minting by default

    // - Update Controller state
    ctx.accounts
        .controller
        .load_mut()?
        .add_registered_mango_depository_entry(ctx.accounts.depository.key())?;

    emit!(RegisterMangoDepositoryEventV2 {
        version: ctx.accounts.controller.load()?.version,
        depository_version: depository.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
        quote_mint: ctx.accounts.quote_mint.key(),
        mango_account: ctx.accounts.mango_account.key(),
    });

    Ok(())
}

impl<'info> RegisterMangoDepository<'info> {
    fn to_mango_init_account_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::InitMangoAccount<'info>> {
        let cpi_accounts = mango_markets_v3::InitMangoAccount {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
