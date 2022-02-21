use crate::error::UxdError;
use crate::events::DepositInsuranceToMangoDepositoryEvent;
use crate::mango_program;
use crate::AccountingEvent;
use crate::Controller;
use crate::MangoDepository;
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

/// Takes 15 accounts - 8 used locally - 5 for MangoMarkets CPI - 2 Programs
#[derive(Accounts)]
pub struct DepositInsuranceToMangoDepository<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump,
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #3 UXDProgram on chain account bound to a Controller instance
    /// The `MangoDepository` manages a MangoAccount for a single Collateral
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.bump,
        has_one = controller @UxdError::InvalidController,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub depository: Box<Account<'info, MangoDepository>>,

    /// #4 The collateral mint used by the `depository` instance
    #[account(
        constraint = collateral_mint.key() == depository.collateral_mint @UxdError::InvalidCollateralMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #5 The insurance mint used by the `depository` instance
    #[account(
        constraint = insurance_mint.key() == depository.insurance_mint @UxdError::InvalidInsuranceMint
    )]
    pub insurance_mint: Box<Account<'info, Mint>>,

    /// #6 The `authority`'s ATA for the `insurance_mint`
    /// Will be debited during this call
    #[account(
        mut,
        associated_token::mint = insurance_mint,
        associated_token::authority = authority,
    )]
    pub authority_insurance: Box<Account<'info, TokenAccount>>,

    /// #7 The `depository`'s TA for its `insurance_mint`
    /// MangoAccounts can only transact with the TAs owned by their authority
    /// and this only serves as a passthrough
    #[account(
        mut,
        seeds = [INSURANCE_PASSTHROUGH_NAMESPACE, collateral_mint.key().as_ref(), insurance_mint.key().as_ref()],
        bump = depository.insurance_passthrough_bump,
        constraint = depository.insurance_passthrough == depository_insurance_passthrough_account.key() @UxdError::InvalidInsurancePassthroughAccount,
        constraint = depository_insurance_passthrough_account.mint == insurance_mint.key() @UxdError::InvalidInsurancePassthroughATAMint,
    )]
    pub depository_insurance_passthrough_account: Box<Account<'info, TokenAccount>>,

    /// #8 The MangoMarkets Account (MangoAccount) managed by the `depository`
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.mango_account_bump,
        constraint = depository.mango_account == depository_mango_account.key() @UxdError::InvalidMangoAccount,
    )]
    pub depository_mango_account: AccountInfo<'info>,

    /// #9 [MangoMarkets CPI] Index grouping perp and spot markets
    pub mango_group: AccountInfo<'info>,

    /// #10 [MangoMarkets CPI] Cache
    pub mango_cache: AccountInfo<'info>,

    /// #11 [MangoMarkets CPI] Root Bank for the `depository`'s `insurance_mint`
    pub mango_root_bank: AccountInfo<'info>,

    /// #12 [MangoMarkets CPI] Node Bank for the `depository`'s `insurance_mint`
    #[account(mut)]
    pub mango_node_bank: AccountInfo<'info>,

    /// #13 [MangoMarkets CPI] Vault for the `depository`'s `insurance_mint`
    #[account(mut)]
    pub mango_vault: Account<'info, TokenAccount>,

    /// #14 Token Program
    pub token_program: Program<'info, Token>,

    /// #15 MangoMarketv3 Program
    pub mango_program: Program<'info, mango_program::Mango>,
}

pub fn handler(
    ctx: Context<DepositInsuranceToMangoDepository>,
    insurance_amount: u64, // native units
) -> Result<()> {
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
    fn update_accounting(&mut self, insurance_delta: u64) -> Result<()> {
        self.depository
            .update_insurance_amount_deposited(&AccountingEvent::Deposit, insurance_delta)?;
        Ok(())
    }
}

// Validate input arguments
impl<'info> DepositInsuranceToMangoDepository<'info> {
    pub fn validate(&self, insurance_amount: u64) -> Result<()> {
        if insurance_amount > 0 {
            error!(UxdError::InvalidInsuranceAmount);
        }
        if self.authority_insurance.amount >= insurance_amount {
            error!(UxdError::InsufficientAuthorityInsuranceAmount);
        }
        Ok(())
    }
}
