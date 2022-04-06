use crate::error::UxdError;
use crate::events::DepositInsuranceToDepositoryEvent;
use crate::Controller;
use crate::ZoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::ZO_DEPOSITORY_NAMESPACE;
use crate::ZO_MARGIN_ACCOUNT_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use zo::State;
use zo_abi as zo;

/// Takes 11 accounts - 4 used locally - 5 for ZoMarkets CPI - 2 Programs
#[derive(Accounts)]
pub struct DepositInsuranceToZoDepository<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump,
        has_one = authority @UxdError::InvalidAuthority,
        constraint = controller.registered_zo_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #3 UXDProgram on chain account bound to a Controller instance.
    /// The `ZoDepository` manages a ZoAccount for a single Collateral.
    #[account(
        mut,
        seeds = [ZO_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = zo_account @UxdError::InvalidZoAccount
    )]
    pub depository: AccountLoader<'info, ZoDepository>,

    /// #4 The `authority`'s ATA for the `quote_mint`
    /// Will be debited during this call
    #[account(
        mut,
        seeds = [authority.key.as_ref(), token_program.key.as_ref(), depository.load()?.quote_mint.as_ref()],
        bump,
        seeds::program = AssociatedToken::id(),
    )]
    pub authority_quote: Account<'info, TokenAccount>,

    /// #5 The Zo Dex Account (Margin) managed by the `depository`
    /// CHECK : Seeds checked. Depository registered
    #[account(
        mut,
        seeds = [depository.key().as_ref(), zo_state.key().as_ref(), ZO_MARGIN_ACCOUNT_NAMESPACE],
        bump = depository.load()?.zo_account_bump,
        seeds::program = zo_program.key()
    )]
    pub zo_account: AccountInfo<'info>,

    /// #6 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    pub zo_state: AccountLoader<'info, State>,

    /// #7 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    #[account(mut)]
    pub zo_state_signer: UncheckedAccount<'info>,

    /// #8 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    #[account(mut)]
    pub zo_cache: UncheckedAccount<'info>,

    /// #9 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    #[account(mut)]
    pub zo_vault: UncheckedAccount<'info>,

    /// #10 Token Program
    pub token_program: Program<'info, Token>,

    /// #11 ZeroOne Program
    pub zo_program: Program<'info, zo::program::ZoAbi>,
}

pub fn handler(ctx: Context<DepositInsuranceToZoDepository>, amount: u64) -> Result<()> {
    let depository = &mut ctx.accounts.depository.load_mut()?;
    let collateral_mint = depository.collateral_mint;

    let depository_pda_signer: &[&[&[u8]]] = &[&[
        ZO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[depository.bump],
    ]];

    // - 1 [DEPOSIT INSURANCE TO ZO] ------------------------------------------
    zo::cpi::deposit(
        ctx.accounts
            .into_deposit_insurance_context()
            .with_signer(depository_pda_signer),
        false,
        amount,
    )?;

    // - 2 [UPDATE ACCOUNTING] ------------------------------------------------
    depository.insurance_amount_deposited = depository
        .insurance_amount_deposited
        .checked_add(amount.into())
        .ok_or_else(|| error!(UxdError::MathError))?;

    emit!(DepositInsuranceToDepositoryEvent {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        quote_mint: depository.quote_mint,
        quote_mint_decimals: depository.quote_mint_decimals,
        deposited_amount: amount,
    });

    Ok(())
}

impl<'info> DepositInsuranceToZoDepository<'info> {
    pub fn into_deposit_insurance_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, zo::cpi::accounts::Deposit<'info>> {
        let cpi_accounts = zo::cpi::accounts::Deposit {
            state: self.zo_state.to_account_info(),
            state_signer: self.zo_state_signer.to_account_info(),
            cache: self.zo_cache.to_account_info(),
            authority: self.authority.to_account_info(),
            margin: self.zo_account.to_account_info(),
            token_account: self.authority_quote.to_account_info(),
            vault: self.zo_vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.zo_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Validate input arguments
impl<'info> DepositInsuranceToZoDepository<'info> {
    pub fn validate(&self, amount: u64) -> Result<()> {
        if amount == 0 {
            return Err(error!(UxdError::InvalidInsuranceAmount));
        }
        if self.authority_quote.amount < amount {
            return Err(error!(UxdError::InsufficientAuthorityQuoteAmount));
        }
        Ok(())
    }
}
