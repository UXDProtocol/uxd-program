use crate::error::UxdError;
use crate::events::DepositInsuranceToMangoDepositoryEventV2;
use crate::Controller;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use anchor_comp::mango_markets_v3;
use anchor_comp::mango_markets_v3::MangoMarketV3;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

/// Takes 12 accounts - 5 used locally - 5 for MangoMarkets CPI - 2 Programs
#[derive(Accounts)]
pub struct DepositInsuranceToMangoDepository<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump,
        has_one = authority @UxdError::InvalidAuthority,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #3 UXDProgram on chain account bound to a Controller instance
    /// The `MangoDepository` manages a MangoAccount for a single Collateral
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = mango_account @UxdError::InvalidMangoAccount,
    )]
    pub depository: Box<Account<'info, MangoDepository>>,

    /// #4 The `authority`'s ATA for the `quote_mint`
    /// Will be debited during this call
    #[account(
        mut,
        seeds = [authority.key.as_ref(), token_program.key.as_ref(), depository.quote_mint.as_ref()],
        bump,
        seeds::program = AssociatedToken::id(),
    )]
    pub authority_quote: Box<Account<'info, TokenAccount>>,

    /// #5 The MangoMarkets Account (MangoAccount) managed by the `depository`
    /// CHECK : Seeds checked. Depository registered
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.mango_account_bump,
    )]
    pub mango_account: AccountInfo<'info>,

    /// #6 [MangoMarkets CPI] Index grouping perp and spot markets
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_group: UncheckedAccount<'info>,

    /// #7 [MangoMarkets CPI] Cache
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_cache: UncheckedAccount<'info>,

    /// #8 [MangoMarkets CPI] Root Bank for the `depository`'s `quote_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_root_bank: UncheckedAccount<'info>,

    /// #9 [MangoMarkets CPI] Node Bank for the `depository`'s `quote_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_node_bank: UncheckedAccount<'info>,

    /// #10 [MangoMarkets CPI] Vault for the `depository`'s `quote_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_vault: UncheckedAccount<'info>,

    /// #11 Token Program
    pub token_program: Program<'info, Token>,

    /// #12 MangoMarketv3 Program
    pub mango_program: Program<'info, MangoMarketV3>,
}

pub fn handler(ctx: Context<DepositInsuranceToMangoDepository>, amount: u64) -> Result<()> {
    let collateral_mint = ctx.accounts.depository.collateral_mint;

    let depository_signer_seeds: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];

    // - 1 [DEPOSIT INSURANCE TO MANGO] ---------------------------------------
    mango_markets_v3::deposit(
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
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Deposit<'info>> {
        let cpi_accounts = mango_markets_v3::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.authority.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_root_bank.to_account_info(),
            node_bank: self.mango_node_bank.to_account_info(),
            vault: self.mango_vault.to_account_info(),
            owner_token_account: self.authority_quote.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Additional convenience methods related to the inputted accounts
impl<'info> DepositInsuranceToMangoDepository<'info> {
    fn update_accounting(&mut self, amount: u64) -> Result<()> {
        self.depository.insurance_amount_deposited = self
            .depository
            .insurance_amount_deposited
            .checked_add(amount.into())
            .ok_or_else(|| error!(UxdError::MathError))?;
        Ok(())
    }
}

// Validate input arguments
impl<'info> DepositInsuranceToMangoDepository<'info> {
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
