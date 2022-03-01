use crate::error::UxdError;
use crate::events::WithdrawInsuranceFromMangoDepositoryEvent;
use crate::Controller;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::INSURANCE_PASSTHROUGH_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use anchor_comp::mango_markets_v3;
use anchor_comp::mango_markets_v3::MangoMarketV3;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

/// Takes 15 accounts - 6 used locally - 6 for MangoMarkets CPI - 3 Programs
#[derive(Accounts)]
pub struct WithdrawInsuranceFromMangoDepository<'info> {
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
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.bump,
        has_one = controller @UxdError::InvalidController,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub depository: Box<Account<'info, MangoDepository>>,

    /// #4 The `user`'s ATA for the `controller`'s `redeemable_mint`
    /// Will be credited during this instruction
    #[account(
        mut,
        constraint = authority_insurance.mint == depository.insurance_mint @UxdError::InvalidAuthorityInsuranceATAMint
    )]
    pub authority_insurance: Box<Account<'info, TokenAccount>>,

    /// #5 The `depository`'s TA for its `insurance_mint`
    /// MangoAccounts can only transact with the TAs owned by their authority
    /// and this only serves as a passthrough
    #[account(
        mut,
        seeds = [INSURANCE_PASSTHROUGH_NAMESPACE, depository.collateral_mint.as_ref(), depository.insurance_mint.as_ref()],
        bump = depository.insurance_passthrough_bump,
        constraint = depository.insurance_passthrough == depository_insurance_passthrough_account.key() @UxdError::InvalidInsurancePassthroughAccount,
        constraint = depository_insurance_passthrough_account.mint == depository.insurance_mint @UxdError::InvalidInsurancePassthroughATAMint,
    )]
    pub depository_insurance_passthrough_account: Box<Account<'info, TokenAccount>>,

    /// #6 The MangoMarkets Account (MangoAccount) managed by the `depository`
    /// CHECK : Seeds checked. Depository registered
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.mango_account_bump,
        constraint = depository.mango_account == depository_mango_account.key() @UxdError::InvalidMangoAccount,
    )]
    pub depository_mango_account: AccountInfo<'info>,

    /// #7 [MangoMarkets CPI] Index grouping perp and spot markets
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_group: UncheckedAccount<'info>,

    /// #8 [MangoMarkets CPI] Cache
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_cache: UncheckedAccount<'info>,

    /// #9 [MangoMarkets CPI] Signer PDA
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_signer: UncheckedAccount<'info>,

    /// #10 [MangoMarkets CPI] Root Bank for the `depository`'s `insurance_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_root_bank: UncheckedAccount<'info>,

    /// #11 [MangoMarkets CPI] Node Bank for the `depository`'s `insurance_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_node_bank: UncheckedAccount<'info>,

    /// #12 [MangoMarkets CPI] Vault for the `depository`'s `insurance_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_vault: UncheckedAccount<'info>,

    /// #13 System Program
    pub system_program: Program<'info, System>,

    /// #14 Token Program
    pub token_program: Program<'info, Token>,

    /// #15 MangoMarketv3 Program
    pub mango_program: Program<'info, MangoMarketV3>,
}

pub fn handler(ctx: Context<WithdrawInsuranceFromMangoDepository>, amount: u64) -> Result<()> {
    let collateral_mint = ctx.accounts.depository.collateral_mint;

    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];

    // - 1 [WITHDRAW INSURANCE FROM MANGO THEN RETURN TO USER] ---------------

    // - mango withdraw insurance_amount
    mango_markets_v3::withdraw(
        ctx.accounts
            .into_withdraw_insurance_from_mango_context()
            .with_signer(depository_signer_seed),
        amount,
        false,
    )?;

    // - Return insurance_amount back to authority
    token::transfer(
        ctx.accounts
            .into_transfer_insurance_to_authority_context()
            .with_signer(depository_signer_seed),
        amount,
    )?;

    // - 2 [UPDATE ACCOUNTING] ------------------------------------------------
    ctx.accounts.update_accounting(amount)?;

    emit!(WithdrawInsuranceFromMangoDepositoryEvent {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        insurance_mint: ctx.accounts.depository.insurance_mint,
        insurance_mint_decimals: ctx.accounts.depository.insurance_mint_decimals,
        withdrawn_amount: amount,
    });

    Ok(())
}

impl<'info> WithdrawInsuranceFromMangoDepository<'info> {
    pub fn into_withdraw_insurance_from_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Withdraw<'info>> {
        let cpi_accounts = mango_markets_v3::Withdraw {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_root_bank.to_account_info(),
            node_bank: self.mango_node_bank.to_account_info(),
            vault: self.mango_vault.to_account_info(),
            token_account: self
                .depository_insurance_passthrough_account
                .to_account_info(),
            signer: self.mango_signer.to_account_info(),
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
    fn update_accounting(&mut self, amount: u64) -> Result<()> {
        self.depository.insurance_amount_deposited = self
            .depository
            .insurance_amount_deposited
            .checked_sub(amount.into())
            .ok_or_else(|| error!(UxdError::MathError))?;
        Ok(())
    }
}

// Validate input arguments
impl<'info> WithdrawInsuranceFromMangoDepository<'info> {
    pub fn validate(&self, insurance_amount: u64) -> Result<()> {
        if insurance_amount == 0 {
            return Err(error!(UxdError::InvalidInsuranceAmount));
        };
        // Mango withdraw will fail with proper error thanks to  `disabled borrow` set to true if the balance is not enough.
        Ok(())
    }
}
