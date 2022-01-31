use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use crate::AccountingEvent;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::ZO_DEPOSITORY_NAMESPACE;
use crate::ZO_ACCOUNT_NAMESPACE;
use crate::INSURANCE_PASSTHROUGH_NAMESPACE;
use crate::ZODepository;
use crate::UxdResult;
use crate::zo_program;
use crate::error::check_assert;
use crate::error::UxdErrorCode;
use crate::error::SourceFileId;
use crate::error::UxdIdlErrorCode;
use crate::ZODepository;
use zo_abi::{self as zo, program::ZoAbi as Zo};
use crate::events::WithdrawInsuranceFromZODeposirotyEvent;

declare_check_assert_macros!(SourceFileId::InstructionMangoDexWithdrawInsuranceFromMangoDepository); //change

#[derive(Accounts)]
pub struct WithdrawInsuranceFromZODepository<'info> {
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
    // The account that will receive the funds withdrawn
    #[account(
        mut,
        constraint = authority_insurance.mint == depository.insurance_mint @UxdIdlErrorCode::InvalidAuthorityInsuranceATAMint
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
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    #[account(mut)]
    pub margin: AccountInfo<'info>,
    #[account(mut)]
    pub control: AccountInfo<'info>,
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub zo_program: Program<'info, zo_program::ZO>,
}

pub fn handler(
    ctx: Context<WithdrawInsuranceFromZODepository>,
    insurance_amount: u64, // native units
) -> UxdResult {
    let collateral_mint = ctx.accounts.collateral_mint.key();

    let depository_signer_seeds: [&[&[u8]]] = &[&[
        ZO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];

    // - 1 [WITHDRAW INSURANCE FROM ZO THEN RETURN TO USER] -------------------
    let perp_info = ctx.accounts.perpetual_info()?;

    // - zo withdraw insurance_amount
    zo::cpi::withdraw(
        ctx.accounts
            .into_withdraw_collateral_from_zo_context()
            .with_signer(depository_signer_seed),
        insurance_amount,
        false,
    )?;
    // zo_program::withdraw(
    //     ctx.accounts
    //         .into_withdraw_collateral_from_zo_context()
    //         .with_signer(depository_signer_seed),
    //     insurance_amount,
    //     false,
    // )?;

    // - Return insurance_amount back to authority
    token::transfer(
        ctx.accounts
            .into_transfer_insurance_to_authority_context()
            .with_signer(depository_signer_seeds),
        insurance_amount,
        false,
    )?;

    // - 2 [UPDATE ACCOUNTING] ------------------------------------------------
    ctx.account.update_accounting(insurance_amount)?;

    emit!(WithdrawInsuranceFromZODeposirotyEvent {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        insurance_mint: ctx.account.insurance_mint,
        insurance_mint_decimals: ctx.accounts.depository.insurance_mint_decimals,
        withdrawn_amount: insurance_amount,
    });
    

    Ok(())
}

impl<'info> WithdrawInsuranceFromZODepository<'info> {
    pub fn into_withdraw_collateral_from_zo_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, zo_program::Withdraw<'info>> {
        let cpi_accounts = zo_program::Withdraw {
            state: self.state.to_account_info(),
            state_signer: self.state_signer.to_account_info(),
            cache: self.cache.to_account_info(),
            authority: self.depository.to_account_info(),
            margin: self.margin.to_account_info(),
            control: self.control.to_account_info(),
            token_account: self
                .depository_insurance_passthrough_account
                .to_account_info(),
            vault: self.vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.zo_program.to_account_info();
        CpiContext::new(zo_program, zo_accounts)
    }

    pub fn into_transfer_insurance_to_authority_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = token::Transfer {
            from: self
                .depository_insurance_passthrough_account
                .to_account_info(),
            to: self.authority_insurance.to_account_info(),
            authority: self.depository.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Additional convenience methods related to the inputted accounts
impl<'info> WithdrawInsuranceFromZODepository<'info> {
    // Return general information about the perpetual related to the collateral in use
    fn perpetual_info(
        &self
    ) -> UxdResult<zo::types::PerpMarketInfo> { // is this right?
        let perp_info = zo::types::PerpMarketInfo { // how to get?
            symbol: None,
            oracle_symbol: None,
            perp_type: None,
            asset_decimals: None,
            asset_lot_size: None,
            quote_lot_size: None,
            strike: None,
            base_imf: None,
            liq_fee: None,
            dex_market: self.dex_market.key,
        };
        Ok(perp_info)
    }

    // Update the accounting in the Depository and Controller Accounts to reflect changes
    fn update_accounting(
        &mut self,
        insurance_delta: u64,
    ) -> ProgramResult {
        // ZO Depository
        self.depository
            .update_insurance_amount_deposited(&AccountingEvent::Withdraw, insurance_delta)?;
        Ok(())
    }
}

// Validate
impl<'info> WithdrawInsuranceFromZODepository<'info> {
    pub fn validate(
        &self,
        insurance_amount: u64,
    ) -> ProgramResult {
        check!(insurance_amount > 0, UxdErrorCode::InvalidInsuranceAmount)?;
        Ok(())
    }
}