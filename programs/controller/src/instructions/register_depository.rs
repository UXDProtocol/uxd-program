use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use mango::state::MangoAccount;
use std::mem::size_of;
use anchor_lang::Discriminator;

use crate::mango_program;
use crate::Depository;
use crate::State;
use crate::UXDError;
use crate::COLLATERAL_PASSTHROUGH_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;

const MANGO_ACCOUNT_SPAN: usize = size_of::<MangoAccount>();

#[derive(Accounts)]
#[instruction(
    depository_bump: u8,
    collateral_passthrough_bump: u8
)]
pub struct RegisterDepository<'info> {
    #[account(
        mut, 
        constraint = authority.key() == state.authority @UXDError::InvalidAuthority
    )]
    pub authority: Signer<'info>,
    #[account(
        seeds = [&State::discriminator()[..]],
        bump,
        has_one = authority,
        )]
    pub state: Box<Account<'info, State>>,
    #[account(
        init,
        seeds = [&Depository::discriminator()[..], collateral_mint.key().as_ref()],
        bump = depository_bump,
        payer = authority,
    )]
    pub depository: Box<Account<'info, Depository>>,
    pub collateral_mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        seeds = [COLLATERAL_PASSTHROUGH_NAMESPACE, collateral_mint.key().as_ref()],
        bump = collateral_passthrough_bump,
        token::mint = collateral_mint,
        token::authority = depository,
        payer = authority,
    )]
    pub collateral_passthrough: Account<'info, TokenAccount>,
    // The mango group for the mango_account
    pub mango_group: AccountInfo<'info>,
    // The mango PDA
    #[account(
        init,
        seeds = [MANGO_ACCOUNT_NAMESPACE, collateral_mint.key().as_ref()],
        bump,
        owner = mango_program::Mango::id(),
        payer = authority,
        space = MANGO_ACCOUNT_SPAN,
    )]
    pub mango_account: AccountInfo<'info>,
    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub mango_program: Program<'info, mango_program::Mango>,
    // sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<RegisterDepository>, depository_bump: u8, collateral_passthrough_bump: u8) -> ProgramResult {
    let collateral_mint = ctx.accounts.collateral_mint.key();

    // - Initialize Mango Account
    let depository_signer_seed: &[&[&[u8]]] = &[&[
        &Depository::discriminator()[..],
        collateral_mint.as_ref(),
        &[depository_bump],
    ]];
    mango_program::initialize_mango_account(
        ctx.accounts
            .into_mango_account_initialization_context()
            .with_signer(depository_signer_seed),
    )?;

    // - Set our depo record up
    // this later acts as proof we trust a given depository
    // we also use this to derive the depository state key, from which we get mint and account keys
    // creating a hierarchy of trust rooted at the authority key that instantiated the controller
    ctx.accounts.depository.bump = depository_bump;
    ctx.accounts.depository.collateral_mint = collateral_mint;
    ctx.accounts.depository.collateral_passthrough = ctx.accounts.collateral_passthrough.key();
    ctx.accounts.depository.collateral_passthrough_bump = collateral_passthrough_bump;
    ctx.accounts.depository.mango_account = ctx.accounts.mango_account.key();

    Ok(())
}

impl<'info> RegisterDepository<'info> {
    pub fn into_mango_account_initialization_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::InitMangoAccount<'info>> {
        let cpi_accounts = mango_program::InitMangoAccount {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
