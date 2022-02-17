use crate::error::check_assert;
use crate::error::SourceFileId;
use crate::error::UxdErrorCode;
use crate::error::UxdIdlErrorCode;
use crate::events::DepositInsuranceToMangoDepositoryEvent;
use crate::mango_program;
use crate::AccountingEvent;
use crate::Controller;
use crate::MangoDepository;
use crate::UxdResult;
use crate::CONTROLLER_NAMESPACE;
use crate::INSURANCE_PASSTHROUGH_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;

declare_check_assert_macros!(SourceFileId::InstructionMangoDexDepositInsuranceToMangoDepository);

#[derive(Accounts)]
pub struct DepositInsuranceToMangoDepository<'info> {
    pub authority: Signer<'info>,
    #[account(
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump,
        has_one = authority @UxdIdlErrorCode::InvalidAuthority,
    )]
    pub controller: Box<Account<'info, Controller>>,
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.bump,
        has_one = controller @UxdIdlErrorCode::InvalidController,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdIdlErrorCode::InvalidDepository
    )]
    pub depository: Box<Account<'info, MangoDepository>>,
    #[account(
        constraint = collateral_mint.key() == depository.collateral_mint @UxdIdlErrorCode::InvalidCollateralMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,
    #[account(
        constraint = insurance_mint.key() == depository.insurance_mint @UxdIdlErrorCode::InvalidInsuranceMint
    )]
    pub insurance_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = insurance_mint, // @UxdIdlErrorCode::InvalidAuthorityInsuranceATAMint
        associated_token::authority = authority,
    )]
    pub authority_insurance: Box<Account<'info, TokenAccount>>,
    // Passthrough accounts as only mangoAccount's Owner Owned accounts can transact w/ the mangoAccount
    // (In the case of a deposit it can be bypassed, but for the sake of a coherent interface we use it)
    #[account(
        mut,
        seeds = [INSURANCE_PASSTHROUGH_NAMESPACE, collateral_mint.key().as_ref(), insurance_mint.key().as_ref()],
        bump = depository.insurance_passthrough_bump,
        constraint = depository.insurance_passthrough == depository_insurance_passthrough_account.key() @UxdIdlErrorCode::InvalidInsurancePassthroughAccount,
        constraint = depository_insurance_passthrough_account.mint == insurance_mint.key() @UxdIdlErrorCode::InvalidInsurancePassthroughATAMint,
    )]
    pub depository_insurance_passthrough_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.mango_account_bump,
        constraint = depository.mango_account == depository_mango_account.key() @UxdIdlErrorCode::InvalidMangoAccount,
    )]
    pub depository_mango_account: AccountInfo<'info>,
    // Mango CPI accounts
    pub mango_group: AccountInfo<'info>,
    pub mango_cache: AccountInfo<'info>,
    pub mango_root_bank: AccountInfo<'info>,
    #[account(mut)]
    pub mango_node_bank: AccountInfo<'info>,
    #[account(mut)]
    pub mango_vault: Account<'info, TokenAccount>,
    // programs
    pub token_program: Program<'info, Token>,
    pub mango_program: Program<'info, mango_program::Mango>,
}

pub fn handler(
    ctx: Context<DepositInsuranceToMangoDepository>,
    insurance_amount: u64, // native units
) -> UxdResult {
    let collateral_mint = ctx.accounts.collateral_mint.key();

    let depository_signer_seeds: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];

    // - 1 [TRANSFER INSURANCE TO MANGO] --------------------------------------

    // - Transfers insurance to the passthrough account
    token::transfer(
        ctx.accounts.into_transfer_to_passthrough_context(),
        insurance_amount,
    )?;

    // - Deposit Insurance to Mango Account
    mango_program::deposit(
        ctx.accounts
            .into_deposit_to_mango_context()
            .with_signer(depository_signer_seeds),
        insurance_amount,
    )?;

    // - 2 [UPDATE ACCOUNTING] ------------------------------------------------
    ctx.accounts.update_accounting(insurance_amount)?;

    emit!(DepositInsuranceToMangoDepositoryEvent {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        insurance_mint: ctx.accounts.depository.insurance_mint,
        insurance_mint_decimals: ctx.accounts.depository.insurance_mint_decimals,
        deposited_amount: insurance_amount,
    });

    Ok(())
}

impl<'info> DepositInsuranceToMangoDepository<'info> {
    pub fn into_transfer_to_passthrough_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.authority_insurance.to_account_info(),
            to: self
                .depository_insurance_passthrough_account
                .to_account_info(),
            authority: self.authority.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_deposit_to_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::Deposit<'info>> {
        let cpi_accounts = mango_program::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_root_bank: self.mango_root_bank.to_account_info(),
            mango_node_bank: self.mango_node_bank.to_account_info(),
            mango_vault: self.mango_vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
            owner_token_account: self
                .depository_insurance_passthrough_account
                .to_account_info(),
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
    pub fn validate(&self, insurance_amount: u64) -> ProgramResult {
        check!(insurance_amount > 0, UxdErrorCode::InvalidInsuranceAmount)?;
        check!(
            self.authority_insurance.amount >= insurance_amount,
            UxdErrorCode::InsufficientAuthorityInsuranceAmount
        )?;
        Ok(())
    }
}
