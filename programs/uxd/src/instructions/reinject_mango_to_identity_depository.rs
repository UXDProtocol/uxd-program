use crate::Controller;
use crate::IdentityDepository;
use crate::UxdError;
use crate::CONTROLLER_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_COLLATERAL_VAULT_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct ReinjectMangoToIdentityDepository<'info> {
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

    /// #7 The `user`'s TA for the `depository` `collateral_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        constraint = user_collateral.mint == depository.load()?.collateral_mint @UxdError::InvalidCollateralMint,
        constraint = &user_collateral.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #9
    pub system_program: Program<'info, System>,

    /// #10 Token Program
    pub token_program: Program<'info, Token>,
}

pub(crate) fn handler(ctx: Context<ReinjectMangoToIdentityDepository>) -> Result<()> {
    Ok(())
}

impl<'info> ReinjectMangoToIdentityDepository<'info> {
    pub(crate) fn validate(&self) -> Result<()> {
        Ok(())
    }
}
