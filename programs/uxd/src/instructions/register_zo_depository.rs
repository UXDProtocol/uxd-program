use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use crate::error::SourceFileId;
use crate::PROGRAM_VERSION;
use crate::UxdResult;
use crate::ZoDepository;
use crate::Controller;
use crate::error::UxdError;
use crate::error::UxdErrorCode;
use crate::error::UxdIdlErrorCode;
use crate::CONTROLLER_NAMESPACE;
use crate::ZO_DEPOSITORY_NAMESPACE;
use crate::ZO_MARGIN_ACCOUNT_NAMESPACE;
use crate::events::RegisterZoDepositoryEvent;
use zo_abi::{self as zo, program::ZoAbi as Zo};

declare_check_assert_macros!(SourceFileId::InstructionsRegisterZoDepository);

// https://github.com/01protocol/zo-abi/blob/3c9ee1f4ca2177fa61cccaea718ae79977848fd2/src/lib.rs#L261
const ZO_CONTROL_SPAN: usize = 8 + 4482;

#[derive(Accounts)]
#[instruction(
    bump: u8,
    zo_account_bump: u8,
)]
pub struct RegisterZoDepository<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE], 
        bump = controller.bump,
        has_one = authority @UxdIdlErrorCode::InvalidAuthority,
    )]
    pub controller: Box<Account<'info, Controller>>,
    #[account(
        init,
        seeds = [ZO_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump = bump,
        payer = payer,
    )]
    pub depository: Box<Account<'info, ZoDepository>>,
    pub collateral_mint: Box<Account<'info, Mint>>,
    pub insurance_mint: Box<Account<'info, Mint>>,

    // ZO CPI
    #[account(
        mut,
        seeds = [authority.key.as_ref(), zo_state.key().as_ref(), ZO_MARGIN_ACCOUNT_NAMESPACE],
        bump = zo_account_bump
    )]
    pub depository_zo_account: AccountInfo<'info>,
    pub zo_state: AccountInfo<'info>,
    #[account(
        init,
        owner = zo_abi::ID,
        payer = payer,
        space = ZO_CONTROL_SPAN,
    )]
    pub zo_control: AccountInfo<'info>,
    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub zo_program: Program<'info, Zo>,
    // sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<RegisterZoDepository>,
    bump: u8, 
    zo_account_bump: u8
) -> UxdResult {
    let collateral_mint = ctx.accounts.collateral_mint.key();
    let insurance_mint = ctx.accounts.insurance_mint.key();

    // - Initialize ZO Margin Account
    let depository_signer_seed: &[&[&[u8]]] = &[&[
        ZO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[bump],
    ]];
    zo::cpi::create_margin(
        ctx.accounts
        .into_zo_create_margin_context()
        .with_signer(depository_signer_seed).into(),
         zo_account_bump)?;

    // - Initialize Depository state
    ctx.accounts.depository.bump = bump;
    ctx.accounts.depository.zo_account_bump = zo_account_bump;
    ctx.accounts.depository.version = PROGRAM_VERSION;
    ctx.accounts.depository.collateral_mint = collateral_mint;
    ctx.accounts.depository.collateral_mint_decimals = ctx.accounts.collateral_mint.decimals;
    ctx.accounts.depository.insurance_mint = insurance_mint;
    ctx.accounts.depository.insurance_mint_decimals = ctx.accounts.insurance_mint.decimals;
    ctx.accounts.depository.zo_account = ctx.accounts.depository_zo_account.key();
    ctx.accounts.depository.controller = ctx.accounts.controller.key();
    ctx.accounts.depository.insurance_amount_deposited = u128::MIN;
    ctx.accounts.depository.collateral_amount_deposited = u128::MIN;
    ctx.accounts.depository.redeemable_amount_under_management = u128::MIN;

    // - Update Controller state
    ctx.accounts.add_new_registered_zo_depository_entry_to_controller()?;

    emit!(RegisterZoDepositoryEvent {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
        insurance_mint: ctx.accounts.insurance_mint.key(),
        zo_account: ctx.accounts.depository_zo_account.key(),
    });

    Ok(())
}

impl<'info> RegisterZoDepository<'info> {
    pub fn into_zo_create_margin_context(&self) -> CpiContext<'_, '_, '_, 'info, zo::cpi::accounts::CreateMargin<'info>> {
        let cpi_program = self.zo_program.to_account_info();
        let cpi_accounts = zo::cpi::accounts::CreateMargin {
            authority: self.authority.to_account_info(),
            control: self.zo_control.to_account_info(),
            margin: self.depository_zo_account.to_account_info(),
            state: self.zo_state.to_account_info(),
            payer: self.payer.to_account_info(),
            rent: self.rent.to_account_info(),
            system_program: self.system_program.to_account_info(),   
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> RegisterZoDepository<'info> {
    pub fn add_new_registered_zo_depository_entry_to_controller(
        &mut self,
    ) -> ProgramResult {
        let zo_depository_id = self.depository.key();
        self.controller.add_registered_zo_depository_entry(zo_depository_id)?;
        Ok(())
    }
}
