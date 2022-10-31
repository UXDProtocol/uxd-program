use crate::error::UxdError;
use crate::events::RedeemFromIdentityDepositoryEvent;
use crate::state::identity_depository::IdentityDepository;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_COLLATERAL_VAULT_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Burn;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;

#[derive(Accounts)]
pub struct RedeemFromIdentityDepository<'info> {
    /// #1 Public call accessible to any user
    pub user: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = redeemable_mint @UxdError::InvalidRedeemableMint
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4 UXDProgram on chain account bound to a Controller instance that represent the blank minting/redeeming
    #[account(
        mut,
        seeds = [IDENTITY_DEPOSITORY_NAMESPACE],
        bump = depository.load()?.bump,
    )]
    pub depository: AccountLoader<'info, IdentityDepository>,

    /// #5
    /// Token account holding the collateral from minting
    #[account(
        mut,
        seeds = [IDENTITY_DEPOSITORY_COLLATERAL_VAULT_NAMESPACE],
        token::authority = depository,
        token::mint = depository.load()?.collateral_mint,
        bump = depository.load()?.collateral_vault_bump,
    )]
    pub collateral_vault: Box<Account<'info, TokenAccount>>,

    /// #6 The collateral mint used by the `depository` instance
    /// Required to create the user_collateral ATA if needed
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #7 The redeemable mint managed by the `controller` instance
    /// Tokens will be burnt during this instruction
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.load()?.redeemable_mint_bump,
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// #8 The `user`'s ATA for the `depository`'s `collateral_mint`
    /// Will be credited during this instruction
    #[account(
        mut,
        constraint = user_collateral.mint == depository.load()?.collateral_mint @UxdError::InvalidCollateralMint,
        constraint = &user_collateral.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #9 The `user`'s ATA for the `controller`'s `redeemable_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        constraint = user_redeemable.mint == controller.load()?.redeemable_mint @UxdError::InvalidRedeemableMint,
        constraint = &user_redeemable.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #10 System Program
    pub system_program: Program<'info, System>,

    /// #11 Token Program
    pub token_program: Program<'info, Token>,
}

pub(crate) fn handler(
    ctx: Context<RedeemFromIdentityDepository>,
    redeemable_amount: u64,
) -> Result<()> {
    // - 1 [TRANSFER COLLATERAL FROM DEPOSITORY'S VAULT TO USER]
    let collateral_amount = redeemable_amount;
    token::transfer(
        ctx.accounts
            .to_transfer_collateral_from_depository_vault_to_user_context(),
        collateral_amount,
    )?;

    // - 2 [BURN EQUIVALENT UXD] ----------------------------------------------
    token::burn(ctx.accounts.to_burn_redeemable_context(), redeemable_amount)?;

    // - 3 [UPDATE ACCOUNTING] ------------------------------------------------
    {
        let depository = &mut ctx.accounts.depository.load_mut()?;
        let controller = &mut ctx.accounts.controller.load_mut()?;
        // Depository
        depository.collateral_amount_deposited = depository
            .collateral_amount_deposited
            .checked_sub(collateral_amount.into())
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.redeemable_amount_under_management = depository
            .redeemable_amount_under_management
            .checked_sub(redeemable_amount.into())
            .ok_or_else(|| error!(UxdError::MathError))?;
        // Controller
        controller.redeemable_circulating_supply = controller
            .redeemable_circulating_supply
            .checked_sub(redeemable_amount.into())
            .ok_or_else(|| error!(UxdError::MathError))?;
    }

    // - 5 [EVENT LOGGING] ----------------------------------------------------
    emit!(RedeemFromIdentityDepositoryEvent {
        version: ctx.accounts.controller.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        user: ctx.accounts.user.key(),
        redeemable_amount,
    });

    Ok(())
}

impl<'info> RedeemFromIdentityDepository<'info> {
    fn to_burn_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.redeemable_mint.to_account_info(),
            from: self.user_redeemable.to_account_info(),
            authority: self.controller.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn to_transfer_collateral_from_depository_vault_to_user_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.collateral_vault.to_account_info(),
            to: self.user_collateral.to_account_info(),
            authority: self.controller.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Validate input arguments
impl<'info> RedeemFromIdentityDepository<'info> {
    pub(crate) fn validate(&self, redeemable_amount: u64) -> Result<()> {
        require!(redeemable_amount != 0, UxdError::InvalidRedeemableAmount);
        require!(
            self.user_redeemable.amount >= redeemable_amount,
            UxdError::InsufficientRedeemableAmount
        );

        Ok(())
    }
}
