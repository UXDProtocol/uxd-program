use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use crate::AccountingEvent;
use crate::Controller;
use crate::UxdResult;
use crate::error::check_assert;
use crate::error::UxdErrorCode;
use crate::error::SourceFileId;
use crate::error::UxdIdlErrorCode;
use crate::CONTROLLER_NAMESPACE;
use crate::ZO_DEPOSITORY_NAMESPACE;
use crate::ZO_ACCOUNT_NAMESPACE;
use crate::INSURANCE_PASSTHROUGH_NAMESPACE;
use crate::ZODepository;
use zo_abi::{self as zo, program::ZoAbi as Zo};
use crate::events::DepositInsuranceToZODepositoryEvent;

declare_check_assert_macros!(SourceFileId::InstructionMangoDexDepositInsuranceToMangoDepository); // change

#[derive(Accounts)]
pub struct DepositInsuranceToZODepository<'info> {
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE], 
        bump = controller.bump,
        has_one = authority @UxdIdlErrorCode::InvalidAuthority,
    )]
    pub controller: Box<Account<'info, Controller>>,
    #[account(
        mut,
        seeds = [ZO_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.bump,
        has_one = controller @UxdIdlErrorCode::InvalidController,
        constraint = controller.registered_zo_depositories.contains(&depository.key()) @UxdIdlErrorCode::InvalidDepository
    )]
    pub depository: Box<Account<'info, ZODepository>>,
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
        seeds = [ZO_ACCOUNT_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.zo_account_bump,
        constraint = depository.zo_account == depository_zo_account.key() @UxdIdlErrorCode::InvalidZOAccount,
    )]
    pub depository_zo_account: AccountInfo<'info>,
    // ZO CPI accounts
    pub state: AccountInfo<'info>,
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    #[account(mut)]
    pub margin: AccountInfo<'info>,
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    // programs
    pub token_program: Program<'info, Token>,
    pub zo_program: Program<'info, zo_program::ZO>,
}

pub fn handler(
    ctx: Context<DepositInsuranceToZODepository>,
    insurance_amount: u64, // native units
) -> UxdResult {
    let collateral_mint = ctx.accounts.collateral_mint.key();

    let depository_signer_seeds: &[&[&[u8]]] = &[&[
        ZO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];

    // - 1 [TRANSFER INSURANCE TO ZO] -----------------------------------------

    // - Transfers insurance to the passthrough account
    token::transfer(
        ctx.accounts
            .into_transfer_to_passthrough_context(),
        insurance_amount,
    )?;

    // - Deposit Insurance to ZO Account
    zo::cpi::deposit(
        ctx.accounts
            .into_deposit_to_zo_context()
            .with_signer(depository_signer_seed),
        insurance_amount,
    )?;

    // zo_program::deposit(
    //     ctx.accounts
    //         .into_deposit_to_zo_context()
    //         .with_signer(depository_signer_seeds),
    //     insurance_amount,
    // )?;

    // - 2 [UPDATE ACCOUNTING] ------------------------------------------------
    ctx.accounts.update_accounting(insurance_amount)?;

    emit!(DepositInsuranceToZODepositoryEvent {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        insurance_mint: ctx.accounts.depository.insurance_mint,
        insurance_mint_decimals: ctx.accounts.depository.insurance_mint_decimals,
        deposited_amount: insurance_amount,
    });

    Ok(())
}

impl<'info> DepositInsuranceToZODepository<'info> {
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

    pub fn into_deposit_to_zo_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, zo_program::Deposit<'info>> {
        let cpi_accounts = zo_program::Deposit {
            state: self.state.to_account_info(),
            state_signer: self.state_signer.to_account_info(), //?
            cache: self.cache.to_account_info(),
            authority: self.depository.to_account_info(),
            margin: self.depository_zo_account.to_account_info(),
            token_account: self
                .depository_insurance_passthrough_account
                .to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.zo_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Additional convenience methods related to the inputted accounts
impl<'info> DepositInsuranceToZODepository<'info> {
    fn update_accounting(
        &mut self,
        insurance_delta: u64,
    ) -> ProgramResult {
        self.depository
            .update_insurance_amount_deposited(&AccountingEvent::Deposit, insurance_delta)?;
        Ok(())
    }
}

// Validate
impl<'info> DepositInsuranceToZODepository<'info> {
    pub fn validate(
        &self,
        insurance_amount: u64,
    ) -> ProgramResult {
        check!(insurance_amount > 0, UxdErrorCode::InvalidInsuranceAmount)?;
        check!(
            self.authority_insurance.amount >= insurance_amount,
            UxdErrorCode::InsufficientAuthorityInsuranceAmount
        )?;
        Ok(())
    }
}
