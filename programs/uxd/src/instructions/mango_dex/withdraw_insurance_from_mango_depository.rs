use crate::error::check_assert;
use crate::error::SourceFileId;
use crate::error::UxdErrorCode;
use crate::error::UxdIdlErrorCode;
use crate::events::WithdrawInsuranceFromMangoDepositoryEvent;
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

declare_check_assert_macros!(SourceFileId::InstructionMangoDexWithdrawInsuranceFromMangoDepository);

#[derive(Accounts)]
pub struct WithdrawInsuranceFromMangoDepository<'info> {
    /// Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump,
        has_one = authority @UxdIdlErrorCode::InvalidAuthority,
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// UXDProgram on chain account bound to a Controller instance
    /// The `MangoDepository` manager a MangoAccount for a single Collateral
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.bump,
        has_one = controller @UxdIdlErrorCode::InvalidController,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdIdlErrorCode::InvalidDepository
    )]
    pub depository: Box<Account<'info, MangoDepository>>,

    /// The collateral mint used by the `depository` instance
    #[account(
        constraint = collateral_mint.key() == depository.collateral_mint @UxdIdlErrorCode::InvalidCollateralMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// The insurance mint used by the `depository` instance
    #[account(
        constraint = insurance_mint.key() == depository.insurance_mint @UxdIdlErrorCode::InvalidInsuranceMint
    )]
    pub insurance_mint: Box<Account<'info, Mint>>,

    /// The `user`'s ATA for the `controller`'s `redeemable_mint`
    /// Will be credited during this instruction
    #[account(
        mut,
        constraint = authority_insurance.mint == depository.insurance_mint @UxdIdlErrorCode::InvalidAuthorityInsuranceATAMint
    )]
    pub authority_insurance: Box<Account<'info, TokenAccount>>,

    /// The `depository`'s TA for its `insurance_mint`
    /// MangoAccounts can only transact with the TAs owned by their authority
    /// and this only serves as a passthrough
    #[account(
        mut,
        seeds = [INSURANCE_PASSTHROUGH_NAMESPACE, collateral_mint.key().as_ref(), insurance_mint.key().as_ref()],
        bump = depository.insurance_passthrough_bump,
        constraint = depository.insurance_passthrough == depository_insurance_passthrough_account.key() @UxdIdlErrorCode::InvalidInsurancePassthroughAccount,
        constraint = depository_insurance_passthrough_account.mint == insurance_mint.key() @UxdIdlErrorCode::InvalidInsurancePassthroughATAMint,
    )]
    pub depository_insurance_passthrough_account: Box<Account<'info, TokenAccount>>,

    /// The MangoMarkets Account (MangoAccount) managed by the `depository`
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.mango_account_bump,
        constraint = depository.mango_account == depository_mango_account.key() @UxdIdlErrorCode::InvalidMangoAccount,
    )]
    pub depository_mango_account: AccountInfo<'info>,

    /// [MangoMarkets CPI] Index grouping perp and spot markets
    pub mango_group: AccountInfo<'info>,

    /// [MangoMarkets CPI] Cache
    pub mango_cache: AccountInfo<'info>,

    /// [MangoMarkets CPI] Signer PDA
    pub mango_signer: AccountInfo<'info>,

    /// [MangoMarkets CPI] Root Bank for the `depository`'s `insurance_mint`
    pub mango_root_bank: AccountInfo<'info>,

    /// [MangoMarkets CPI] Node Bank for the `depository`'s `insurance_mint`
    #[account(mut)]
    pub mango_node_bank: AccountInfo<'info>,

    /// [MangoMarkets CPI] Vault for the `depository`'s `insurance_mint`
    #[account(mut)]
    pub mango_vault: Account<'info, TokenAccount>,

    /// System Program
    pub system_program: Program<'info, System>,

    /// Token Program
    pub token_program: Program<'info, Token>,

    /// MangoMarketv3 Program
    pub mango_program: Program<'info, mango_program::Mango>,
}

pub fn handler(
    ctx: Context<WithdrawInsuranceFromMangoDepository>,
    insurance_amount: u64, // native units
) -> UxdResult {
    let collateral_mint = ctx.accounts.collateral_mint.key();

    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];

    // - 1 [WITHDRAW INSURANCE FROM MANGO THEN RETURN TO USER] ---------------

    // - mango withdraw insurance_amount
    mango_program::withdraw(
        ctx.accounts
            .into_withdraw_insurance_from_mango_context()
            .with_signer(depository_signer_seed),
        insurance_amount,
        false,
    )?;

    // - Return insurance_amount back to authority
    token::transfer(
        ctx.accounts
            .into_transfer_insurance_to_authority_context()
            .with_signer(depository_signer_seed),
        insurance_amount,
    )?;

    // - 2 [UPDATE ACCOUNTING] ------------------------------------------------
    ctx.accounts.update_accounting(insurance_amount)?;

    emit!(WithdrawInsuranceFromMangoDepositoryEvent {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        insurance_mint: ctx.accounts.depository.insurance_mint,
        insurance_mint_decimals: ctx.accounts.depository.insurance_mint_decimals,
        withdrawn_amount: insurance_amount,
    });

    Ok(())
}

impl<'info> WithdrawInsuranceFromMangoDepository<'info> {
    pub fn into_withdraw_insurance_from_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::Withdraw<'info>> {
        let cpi_accounts = mango_program::Withdraw {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_root_bank: self.mango_root_bank.to_account_info(),
            mango_node_bank: self.mango_node_bank.to_account_info(),
            mango_vault: self.mango_vault.to_account_info(),
            token_account: self
                .depository_insurance_passthrough_account
                .to_account_info(),
            mango_signer: self.mango_signer.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_insurance_to_authority_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::Transfer {
            from: self
                .depository_insurance_passthrough_account
                .to_account_info(),
            to: self.authority_insurance.to_account_info(),
            authority: self.depository.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Additional convenience methods related to the inputted accounts
impl<'info> WithdrawInsuranceFromMangoDepository<'info> {
    fn update_accounting(&mut self, insurance_delta: u64) -> ProgramResult {
        // Mango Depository
        self.depository
            .update_insurance_amount_deposited(&AccountingEvent::Withdraw, insurance_delta)?;
        Ok(())
    }
}

// Validate input arguments
impl<'info> WithdrawInsuranceFromMangoDepository<'info> {
    pub fn validate(&self, insurance_amount: u64) -> ProgramResult {
        check!(insurance_amount > 0, UxdErrorCode::InvalidInsuranceAmount)?;
        // Mango withdraw will fail with proper error thanks to  `disabled borrow` set to true if the balance is not enough.
        Ok(())
    }
}
