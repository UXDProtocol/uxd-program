use crate::error::UxdError;
use crate::Controller;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use ::mango_v3_reimbursement::program::MangoV3Reimbursement;
use ::mango_v3_reimbursement::state::Group;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use mango_v3_reimbursement::state::ReimbursementAccount;

#[derive(Accounts)]
#[instruction(token_index: usize)]
pub struct MangoReimburse<'info> {
    /// #1
    pub authority: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = mango_account @UxdError::InvalidMangoAccount,
    )]
    pub depository: AccountLoader<'info, MangoDepository>,

    /// #5 The mint of the token received from mango reimbursement program
    pub token_mint: Box<Account<'info, Mint>>,

    /// #6 Authority's token account which will receives the funds
    #[account(
        mut,
        constraint = authority_token_account.mint == token_mint.key(),
        constraint = &authority_token_account.owner == authority.key @UxdError::InvalidOwner,
    )]
    pub authority_token_account: Box<Account<'info, TokenAccount>>,

    /// #7 Depository's token account which will temporarily receives the funds from mango
    /// before transferring it to authority_token_account
    #[account(
        mut,
        constraint = authority_token_account.mint == token_mint.key(),
        constraint = &authority_token_account.owner == authority.key @UxdError::InvalidOwner,
    )]
    pub depository_token_account: Box<Account<'info, TokenAccount>>,

    /// #8
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.mango_account_bump,
    )]
    pub mango_account: AccountInfo<'info>,

    /// #9
    #[account(mut)]
    pub mango_reimbursement_group: AccountLoader<'info, Group>,

    /// #10
    #[account(
        mut,
        address = mango_reimbursement_group.load()?.vaults[token_index]
    )]
    pub mango_reimbursement_vault: Account<'info, TokenAccount>,

    /// #11
    #[account(
        mut,
        seeds = [b"ReimbursementAccount".as_ref(), mango_reimbursement_group.key().as_ref(), depository.key().as_ref()],
        bump,
    )]
    pub mango_reimbursement_account: AccountLoader<'info, ReimbursementAccount>,

    /// #12
    #[account(
        mut,
        associated_token::mint = mango_reimbursement_claim_mint,
        associated_token::authority = mango_reimbursement_group.load()?.claim_transfer_destination,
    )]
    pub mango_reimbursement_claim_mint_token_account: Box<Account<'info, TokenAccount>>,

    /// #13
    #[account(
        mut,
        address = mango_reimbursement_group.load()?.claim_mints[token_index]
    )]
    pub mango_reimbursement_claim_mint: Box<Account<'info, Mint>>,

    /// #14
    pub mango_reimbursement_table: UncheckedAccount<'info>,

    /// #15
    pub system_program: Program<'info, System>,

    /// #16
    pub token_program: Program<'info, Token>,

    /// #17
    pub mango_reimbursement_program: Program<'info, MangoV3Reimbursement>,
}

pub(crate) fn handler(ctx: Context<MangoReimburse>) -> Result<()> {
    Ok(())
}

impl<'info> MangoReimburse<'info> {}
