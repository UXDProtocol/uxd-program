use crate::error::check_assert;
use crate::error::SourceFileId;
use crate::error::UxdErrorCode;
use crate::error::UxdIdlErrorCode;
use crate::events::DepositInsuranceToMangoDepositoryEventV2;
use crate::mango_program;
use crate::AccountingEvent;
use crate::Controller;
use crate::MangoDepository;
use crate::UxdResult;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

declare_check_assert_macros!(SourceFileId::InstructionMangoDexDepositInsuranceToMangoDepository);

/// Takes 14 accounts - 7 used locally - 5 for MangoMarkets CPI - 2 Programs
#[derive(Accounts)]
pub struct DepositInsuranceToMangoDepository<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump,
        has_one = authority @UxdIdlErrorCode::InvalidAuthority,
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #3 UXDProgram on chain account bound to a Controller instance
    /// The `MangoDepository` manages a MangoAccount for a single Collateral
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.bump,
        has_one = controller @UxdIdlErrorCode::InvalidController,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdIdlErrorCode::InvalidDepository
    )]
    pub depository: Box<Account<'info, MangoDepository>>,

    /// #4 The collateral mint used by the `depository` instance
    #[account(
        constraint = collateral_mint.key() == depository.collateral_mint @UxdIdlErrorCode::InvalidCollateralMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #5 The quote mint used by the `depository` instance
    #[account(
        constraint = quote_mint.key() == depository.quote_mint @UxdIdlErrorCode::InvalidQuoteMint
    )]
    pub quote_mint: Box<Account<'info, Mint>>,

    /// #6 The `authority`'s ATA for the `quote_mint`
    /// Will be debited during this call
    #[account(
        mut,
        associated_token::mint = quote_mint,
        associated_token::authority = authority,
    )]
    pub authority_quote: Box<Account<'info, TokenAccount>>,

    /// #7 The MangoMarkets Account (MangoAccount) managed by the `depository`
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.mango_account_bump,
        constraint = depository.mango_account == depository_mango_account.key() @UxdIdlErrorCode::InvalidMangoAccount,
    )]
    pub depository_mango_account: AccountInfo<'info>,

    /// #8 [MangoMarkets CPI] Index grouping perp and spot markets
    pub mango_group: AccountInfo<'info>,

    /// #9 [MangoMarkets CPI] Cache
    pub mango_cache: AccountInfo<'info>,

    /// #10 [MangoMarkets CPI] Root Bank for the `depository`'s `quote_mint`
    pub mango_root_bank: AccountInfo<'info>,

    /// #11 [MangoMarkets CPI] Node Bank for the `depository`'s `quote_mint`
    #[account(mut)]
    pub mango_node_bank: AccountInfo<'info>,

    /// #12 [MangoMarkets CPI] Vault for the `depository`'s `quote_mint`
    #[account(mut)]
    pub mango_vault: Account<'info, TokenAccount>,

    /// #13 Token Program
    pub token_program: Program<'info, Token>,

    /// #14 MangoMarketv3 Program
    pub mango_program: Program<'info, mango_program::Mango>,
}

pub fn handler(
    ctx: Context<DepositInsuranceToMangoDepository>,
    amount: u64, // native units
) -> UxdResult {
    let collateral_mint = ctx.accounts.collateral_mint.key();

    let depository_signer_seeds: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];

    // - 1 [DEPOSIT INSURANCE TO MANGO] ---------------------------------------
    mango_program::deposit(
        ctx.accounts
            .into_deposit_to_mango_context()
            .with_signer(depository_signer_seeds),
        amount,
    )?;

    // - 2 [UPDATE ACCOUNTING] ------------------------------------------------
    ctx.accounts.update_accounting(amount)?;

    emit!(DepositInsuranceToMangoDepositoryEventV2 {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        quote_mint: ctx.accounts.depository.quote_mint,
        quote_mint_decimals: ctx.accounts.depository.quote_mint_decimals,
        deposited_amount: amount,
    });

    Ok(())
}

impl<'info> DepositInsuranceToMangoDepository<'info> {
    pub fn into_deposit_to_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::Deposit<'info>> {
        let cpi_accounts = mango_program::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.authority.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_root_bank: self.mango_root_bank.to_account_info(),
            mango_node_bank: self.mango_node_bank.to_account_info(),
            mango_vault: self.mango_vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
            owner_token_account: self.authority_quote.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Additional convenience methods related to the inputted accounts
impl<'info> DepositInsuranceToMangoDepository<'info> {
    fn update_accounting(&mut self, insurance_delta: u64) -> ProgramResult {
        self.depository
            .update_insurance_amount_deposited(&AccountingEvent::Deposit, insurance_delta)?;
        Ok(())
    }
}

// Validate input arguments
impl<'info> DepositInsuranceToMangoDepository<'info> {
    pub fn validate(&self, amount: u64) -> ProgramResult {
        check!(amount > 0, UxdErrorCode::InvalidInsuranceAmount)?;
        check!(
            self.authority_quote.amount >= amount,
            UxdErrorCode::InsufficientAuthorityInsuranceAmount
        )?;
        Ok(())
    }
}
