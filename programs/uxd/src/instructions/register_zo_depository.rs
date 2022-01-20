    // let client = Client::new_with_options(
    //     cluster.clone(),
    //     payer,
    //     CommitmentConfig::confirmed(),
    // );

    // let program = client.program(zo_abi::ID);
    // let rpc = program.rpc();
    // let zo_state: zo_abi::State = program.account(zo_state_pubkey).unwrap();
    // let zo_cache: zo_abi::Cache = program.account(zo_state.cache).unwrap();

use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use crate::error::SourceFileId;
use zo_abi::CreateMargin;
use crate::PROGRAM_VERSION;
use crate::UxdResult;
use crate::ZODepository;
use crate::Controller;
use crate::error::UxdError;
use crate::error::UxdErrorCode;
use crate::error::UxdIdlErrorCode;
use crate::CONTROLLER_NAMESPACE;
use crate::ZO_DEPOSITORY_NAMESPACE;
use crate::ZO_MARGIN_ACCOUNT_NAMESPACE;
use crate::events::RegisterZODepositoryEvent;
use crate::zo_program;

declare_check_assert_macros!(SourceFileId::InstructionsRegisterZODepository);

// https://github.com/01protocol/zo-abi/blob/3c9ee1f4ca2177fa61cccaea718ae79977848fd2/src/lib.rs#L261
const ZO_CONTROL_SPAN: usize = 8 + 4482;

#[derive(Accounts)]
#[instruction(
    bump: u8,
    zo_account_bump: u8,
)]
pub struct RegisterZODepository<'info> {
    pub authority: Signer<'info>,
    // In order to use with governance program, as the PDA cannot be the payer in nested TX.
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
    pub depository: Box<Account<'info, ZODepository>>,
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
    pub zo_program: Program<'info, zo_program::ZO>,
    // sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<RegisterZODepository>,
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
    let instruction = zo_abi::zo_abi::create_margin(cx, margin_nonce)
    zo_abi::zo_abi::create_margin(
        ctx.accounts
        .into_zo_create_margin_context()?
        .with_signer(depository_signer_seed).into(),
         zo_account_bump)?;

    // - Initialize Depository state
    ctx.accounts.depository.bump = bump;
    // ctx.accounts.depository.collateral_passthrough_bump = collateral_passthrough_bump;
    // ctx.accounts.depository.insurance_passthrough_bump = insurance_passthrough_bump;
    ctx.accounts.depository.zo_account_bump = zo_account_bump;
    ctx.accounts.depository.version = PROGRAM_VERSION;
    ctx.accounts.depository.collateral_mint = collateral_mint;
    ctx.accounts.depository.collateral_mint_decimals = ctx.accounts.collateral_mint.decimals;
    // ctx.accounts.depository.collateral_passthrough = ctx.accounts.depository_collateral_passthrough_account.key();
    ctx.accounts.depository.insurance_mint = insurance_mint;
    ctx.accounts.depository.insurance_mint_decimals = ctx.accounts.insurance_mint.decimals;
    // ctx.accounts.depository.insurance_passthrough = ctx.accounts.depository_insurance_passthrough_account.key();
    ctx.accounts.depository.zo_account = ctx.accounts.depository_zo_account.key();
    ctx.accounts.depository.controller = ctx.accounts.controller.key();
    ctx.accounts.depository.insurance_amount_deposited = u128::MIN;
    ctx.accounts.depository.collateral_amount_deposited = u128::MIN;
    ctx.accounts.depository.redeemable_amount_under_management = u128::MIN;

    // - Update Controller state
    ctx.accounts.add_new_registered_zo_depository_entry_to_controller()?;

    emit!(RegisterZODepositoryEvent {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
        insurance_mint: ctx.accounts.insurance_mint.key(),
        zo_account: ctx.accounts.depository_zo_account.key(),
    });

    Ok(())
}

impl<'info> RegisterZODepository<'info> {
    pub fn into_zo_create_margin_context(&self) -> UxdResult<CpiContext<'_, '_, '_, 'info, CreateMargin<'info>>> {
        let authority = Signer::try_from(&self.depository.to_account_info())
        .ok_or(throw_err!(UxdErrorCode::InsufficientOrderBookDepth))?;
        let cpi_program = self.zo_program.to_account_info();
        let cpi_accounts = CreateMargin {
            state: self.zo_state.to_account_info(),
            authority: authority,
            margin: self.depository_zo_account.to_account_info(),
            control: self.zo_control.to_account_info(),
            rent: self.rent.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        Ok(CpiContext::new(cpi_program, cpi_accounts))
    }
}

impl<'info> RegisterZODepository<'info> {
    pub fn add_new_registered_zo_depository_entry_to_controller(
        &mut self,
    ) -> ProgramResult {
        let zo_depository_id = self.depository.key();
        self.controller.add_registered_zo_depository_entry(zo_depository_id)?;
        Ok(())
    }
}
