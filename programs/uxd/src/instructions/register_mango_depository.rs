use crate::error::SourceFileId;
use crate::error::UxdError;
use crate::error::UxdErrorCode;
use crate::error::UxdIdlErrorCode;
use crate::events::RegisterMangoDepositoryEventV2;
use crate::mango_program;
use crate::Controller;
use crate::MangoDepository;
use crate::UxdResult;
use crate::COLLATERAL_PASSTHROUGH_NAMESPACE;
use crate::CONTROLLER_NAMESPACE;
use crate::INSURANCE_PASSTHROUGH_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_ACCOUNT_VERSION;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::QUOTE_PASSTHROUGH_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use mango::state::MangoAccount;
use std::mem::size_of;

const MANGO_ACCOUNT_SPAN: usize = size_of::<MangoAccount>();

declare_check_assert_macros!(SourceFileId::InstructionRegisterMangoDepository);

#[derive(Accounts)]
pub struct RegisterMangoDepository<'info> {
    /// Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump,
        has_one = authority @UxdIdlErrorCode::InvalidAuthority,
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// UXDProgram on chain account bound to a Controller instance
    /// The `MangoDepository` manager a MangoAccount for a single Collateral
    #[account(
        init,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump,
        payer = payer,
    )]
    pub depository: Box<Account<'info, MangoDepository>>,

    /// The collateral mint used by the `depository` instance
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// The insurance mint used by the `depository` instance
    pub insurance_mint: Box<Account<'info, Mint>>,

    /// The quote mint used by the `depository` instance
    pub quote_mint: Box<Account<'info, Mint>>,

    /// The `depository`'s TA for its `collateral_mint`
    /// MangoAccounts can only transact with the TAs owned by their authority
    /// and this only serves as a passthrough
    #[account(
        init,
        seeds = [COLLATERAL_PASSTHROUGH_NAMESPACE, collateral_mint.key().as_ref()],
        bump,
        token::mint = collateral_mint,
        token::authority = depository,
        payer = payer,
    )]
    pub depository_collateral_passthrough_account: Box<Account<'info, TokenAccount>>,

    /// The `depository`'s TA for its `insurance_mint`
    /// MangoAccounts can only transact with the TAs owned by their authority
    /// and this only serves as a passthrough
    #[account(
        init,
        seeds = [INSURANCE_PASSTHROUGH_NAMESPACE, collateral_mint.key().as_ref(), insurance_mint.key().as_ref()],
        bump,
        token::mint = insurance_mint,
        token::authority = depository,
        payer = payer,
    )]
    pub depository_insurance_passthrough_account: Box<Account<'info, TokenAccount>>,

    /// The `depository`'s TA for its `quote_mint`
    /// MangoAccounts can only transact with the TAs owned by their authority
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

    /// The MangoMarkets Account (MangoAccount) managed by the `depository`
    #[account(
        init,
        seeds = [MANGO_ACCOUNT_NAMESPACE, collateral_mint.key().as_ref()],
        bump,
        owner = mango_program::Mango::id(),
        payer = payer,
        space = MANGO_ACCOUNT_SPAN,
    )]
    pub depository_mango_account: AccountInfo<'info>,

    /// [MangoMarkets CPI] Index grouping perp and spot markets
    pub mango_group: AccountInfo<'info>,

    /// System Program
    pub system_program: Program<'info, System>,

    /// Token Program
    pub token_program: Program<'info, Token>,

    /// MangoMarketv3 Program
    pub mango_program: Program<'info, mango_program::Mango>,

    /// Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<RegisterMangoDepository>) -> UxdResult {
    let collateral_mint = ctx.accounts.collateral_mint.key();
    let insurance_mint = ctx.accounts.insurance_mint.key();
    let quote_mint = ctx.accounts.quote_mint.key();

    // - Initialize Mango Account
    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[*ctx.bumps.get("depository").ok_or(bump_err!())?],
    ]];
    mango_program::initialize_mango_account(
        ctx.accounts
            .into_mango_account_initialization_context()
            .with_signer(depository_signer_seed),
    )?;

    // - Initialize Depository state
    ctx.accounts.depository.bump = *ctx.bumps.get("depository").ok_or(bump_err!())?;
    ctx.accounts.depository.collateral_passthrough_bump = *ctx
        .bumps
        .get("depository_collateral_passthrough_account")
        .ok_or(bump_err!())?;
    ctx.accounts.depository.insurance_passthrough_bump = *ctx
        .bumps
        .get("depository_insurance_passthrough_account")
        .ok_or(bump_err!())?;
    ctx.accounts.depository.quote_passthrough_bump = *ctx
        .bumps
        .get("depository_quote_passthrough_account")
        .ok_or(bump_err!())?;
    ctx.accounts.depository.mango_account_bump = *ctx
        .bumps
        .get("depository_mango_account")
        .ok_or(bump_err!())?;
    ctx.accounts.depository.version = MANGO_DEPOSITORY_ACCOUNT_VERSION;
    ctx.accounts.depository.collateral_mint = collateral_mint;
    ctx.accounts.depository.collateral_mint_decimals = ctx.accounts.collateral_mint.decimals;
    ctx.accounts.depository.collateral_passthrough =
        ctx.accounts.depository_collateral_passthrough_account.key();
    ctx.accounts.depository.insurance_mint = insurance_mint;
    ctx.accounts.depository.insurance_mint_decimals = ctx.accounts.insurance_mint.decimals;
    ctx.accounts.depository.insurance_passthrough =
        ctx.accounts.depository_insurance_passthrough_account.key();
    ctx.accounts.depository.quote_mint = quote_mint;
    ctx.accounts.depository.quote_mint_decimals = ctx.accounts.quote_mint.decimals;
    ctx.accounts.depository.quote_passthrough =
        ctx.accounts.depository_quote_passthrough_account.key();
    ctx.accounts.depository.mango_account = ctx.accounts.depository_mango_account.key();
    ctx.accounts.depository.controller = ctx.accounts.controller.key();
    ctx.accounts.depository.insurance_amount_deposited = u128::MIN;
    ctx.accounts.depository.collateral_amount_deposited = u128::MIN;
    ctx.accounts.depository.redeemable_amount_under_management = u128::MIN;

    // - Update Controller state
    ctx.accounts
        .add_new_registered_mango_depository_entry_to_controller()?;

    emit!(RegisterMangoDepositoryEventV2 {
        version: ctx.accounts.controller.version,
        depository_version: ctx.accounts.depository.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
        insurance_mint: ctx.accounts.insurance_mint.key(),
        quote_mint: ctx.accounts.quote_mint.key(),
        mango_account: ctx.accounts.depository_mango_account.key(),
    });

    Ok(())
}

impl<'info> RegisterMangoDepository<'info> {
    pub fn into_mango_account_initialization_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::InitMangoAccount<'info>> {
        let cpi_accounts = mango_program::InitMangoAccount {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> RegisterMangoDepository<'info> {
    pub fn add_new_registered_mango_depository_entry_to_controller(&mut self) -> ProgramResult {
        let mango_depository_id = self.depository.key();
        self.controller
            .add_registered_mango_depository_entry(mango_depository_id)?;
        Ok(())
    }
}
