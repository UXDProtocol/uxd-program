use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use mango::state::MangoAccount;
use std::mem::size_of;

use crate::mango_program;
use crate::Depository;
use crate::State;
use crate::DEPOSITORY_SEED;
use crate::MANGO_SEED;
use crate::PASSTHROUGH_SEED;
use crate::STATE_SEED;

const MANGO_ACCOUNT_SPAN: usize = size_of::<MangoAccount>();

#[derive(Accounts)]
pub struct RegisterDepository<'info> {
    #[account(mut, constraint = authority.key() == state.authority_key)]
    pub authority: Signer<'info>,
    #[account(seeds = [STATE_SEED], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(
        init,
        seeds = [DEPOSITORY_SEED, coin_mint.key().as_ref()],
        bump,
        payer = authority,
    )]
    pub depository: Box<Account<'info, Depository>>,
    pub coin_mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        seeds = [PASSTHROUGH_SEED, coin_mint.key().as_ref()],
        bump,
        token::mint = coin_mint,
        token::authority = depository,
        payer = authority,
    )]
    pub coin_passthrough: Account<'info, TokenAccount>,
    // The mango group for the mango_account
    pub mango_group: AccountInfo<'info>,
    // The mango PDA
    #[account(
        init,
        seeds = [MANGO_SEED, coin_mint.key().as_ref()],
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

pub fn handler(ctx: Context<RegisterDepository>) -> ProgramResult {
    let coin_mint_key = ctx.accounts.coin_mint.key();

    // - Initialize Mango Account

    let depository_bump =
        Pubkey::find_program_address(&[DEPOSITORY_SEED, coin_mint_key.as_ref()], ctx.program_id).1;
    let depository_signer_seed: &[&[&[u8]]] =
        &[&[DEPOSITORY_SEED, coin_mint_key.as_ref(), &[depository_bump]]];
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
    ctx.accounts.depository.coin_mint_key = coin_mint_key;
    ctx.accounts.depository.coin_passthrough_key = ctx.accounts.coin_passthrough.key();
    ctx.accounts.depository.mango_account_key = ctx.accounts.mango_account.key();

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
