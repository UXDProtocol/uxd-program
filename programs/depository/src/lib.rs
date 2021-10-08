use anchor_lang::prelude::*;
use anchor_lang::Key;
use anchor_spl::token::Token;
use anchor_spl::token::{self, Burn, Mint, MintTo, TokenAccount, Transfer};
use solana_program::program::invoke_signed;
use spl_token::instruction::{initialize_account, initialize_mint};

pub const STATE_SEED: &[u8] = b"STATE";
const REDEEMABLE_MINT_SEED: &[u8] = b"REDEEMABLE";
const PROGRAM_COIN_SEED: &[u8] = b"DEPOSIT";

// annoyingly the spl program does not expose these as constants
const MINT_SPAN: usize = 82;
const ACCOUNT_SPAN: usize = 165;

solana_program::declare_id!("UXDDepTysvnvAhFyY7tfG793iQAJA8T4ZpyAZyrCLQ7");

// To expose the Depository like a Token or System in anchor `Program<'info, Depository>`
#[derive(Clone)]
pub struct Depository;
impl anchor_lang::AccountDeserialize for Depository {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self, ProgramError> {
        Depository::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(_buf: &mut &[u8]) -> Result<Self, ProgramError> {
        Ok(Depository)
    }
}
impl anchor_lang::Id for Depository {
    fn id() -> Pubkey {
        ID
    }
}

#[program]
#[deny(unused_must_use)]
pub mod depository {
    use super::*;

    // creates a redeemable mint and a coin account
    // also registers the controller as an authority to authenticate proxy transfers
    pub fn new(ctx: Context<New>, controller_key: Pubkey) -> ProgramResult {
        msg!("depository: new");
        let accounts = ctx.accounts.to_account_infos();
        let coin_mint_key = &ctx.accounts.coin_mint.key();

        // build the seeds to sign for account initializations
        let state_ctr =
            Pubkey::find_program_address(&[STATE_SEED, coin_mint_key.as_ref()], ctx.program_id).1;
        let mint_ctr = Pubkey::find_program_address(
            &[REDEEMABLE_MINT_SEED, coin_mint_key.as_ref()],
            ctx.program_id,
        )
        .1;
        let account_ctr = Pubkey::find_program_address(
            &[PROGRAM_COIN_SEED, coin_mint_key.as_ref()],
            ctx.program_id,
        )
        .1;

        let mint_seed: &[&[&[u8]]] =
            &[&[REDEEMABLE_MINT_SEED, coin_mint_key.as_ref(), &[mint_ctr]]];
        let account_seed: &[&[&[u8]]] =
            &[&[PROGRAM_COIN_SEED, coin_mint_key.as_ref(), &[account_ctr]]];

        // now initialize them
        // TODO anchor-spl implemented its own initialize_mint but it's not in a release (as of 7/22)
        // swap impls when it drops? idk im gonna be honest its more verbose and more copies for no real gain
        let ix = initialize_mint(
            &spl_token::ID,
            &ctx.accounts.redeemable_mint.key(),
            &ctx.accounts.state.key(),
            // XXX it may be desirable to repudiate freeze
            Some(&ctx.accounts.state.key()),
            ctx.accounts.coin_mint.decimals,
        )?;
        invoke_signed(&ix, &accounts, mint_seed)?;

        // and again
        let ix = initialize_account(
            &spl_token::ID,
            &ctx.accounts.program_coin.key(),
            &ctx.accounts.coin_mint.key(),
            &ctx.accounts.state.key(),
        )?;
        invoke_signed(&ix, &accounts, account_seed)?;

        // store stuff in our state account now
        ctx.accounts.state.bump = state_ctr;
        ctx.accounts.state.controller_key = controller_key;
        ctx.accounts.state.coin_mint_key = ctx.accounts.coin_mint.key();
        ctx.accounts.state.redeemable_mint_key = ctx.accounts.redeemable_mint.key();
        ctx.accounts.state.program_coin_key = ctx.accounts.program_coin.key();

        Ok(())
    }

    // transfer coin from user_coin to program_coin
    // mint equivalent amount from redeemable_mint to user_redeemable
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
        msg!("depository: deposit");

        let transfer_accounts = Transfer {
            from: ctx.accounts.user_coin.to_account_info(),
            to: ctx.accounts.program_coin.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
        );
        token::transfer(transfer_ctx, amount)?;

        let mint_accounts = MintTo {
            mint: ctx.accounts.redeemable_mint.to_account_info(),
            to: ctx.accounts.user_redeemable.to_account_info(),
            authority: ctx.accounts.state.to_account_info(),
        };

        let state_seed: &[&[&[u8]]] = &[&[
            STATE_SEED,
            ctx.accounts.state.coin_mint_key.as_ref(),
            &[ctx.accounts.state.bump],
        ]];
        let mint_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            mint_accounts,
            state_seed,
        );
        token::mint_to(mint_ctx, amount)?;

        Ok(())
    }

    // burn held redeemables in exchange for a withdrawl of coin
    pub fn withdraw(ctx: Context<Withdraw>, maybe_amount: Option<u64>) -> ProgramResult {
        msg!("depository: withdraw");

        let amount = match maybe_amount {
            Some(n) => n,
            _ => ctx.accounts.user_redeemable.amount,
        };

        let burn_accounts = Burn {
            mint: ctx.accounts.redeemable_mint.to_account_info(),
            to: ctx.accounts.user_redeemable.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let burn_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), burn_accounts);
        token::burn(burn_ctx, amount)?;

        let transfer_accounts = Transfer {
            from: ctx.accounts.program_coin.to_account_info(),
            to: ctx.accounts.user_coin.to_account_info(),
            authority: ctx.accounts.state.to_account_info(),
        };

        let state_seed: &[&[&[u8]]] = &[&[
            STATE_SEED,
            ctx.accounts.state.coin_mint_key.as_ref(),
            &[ctx.accounts.state.bump],
        ]];
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
            state_seed,
        );
        token::transfer(transfer_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct New<'info> {
    // account paying for allocations
    #[account(mut)]
    pub payer: Signer<'info>,
    // stores program config, authority of the redeemable mint and deposit account
    #[account(
        init,
        seeds = [STATE_SEED, coin_mint.key().as_ref()],
        bump = Pubkey::find_program_address(&[STATE_SEED, coin_mint.key().as_ref()], program_id).1,
        payer = payer,
    )]
    pub state: Box<Account<'info, State>>,
    // mint for bearer tokens representing deposited balances
    #[account(
        init,
        seeds = [REDEEMABLE_MINT_SEED, coin_mint.key().as_ref()],
        bump = Pubkey::find_program_address(&[REDEEMABLE_MINT_SEED, coin_mint.key().as_ref()], program_id).1,
        payer = payer,
        owner = spl_token::ID,
        space = MINT_SPAN,
    )]
    pub redeemable_mint: AccountInfo<'info>,
    // program account that coins are deposited into
    #[account(
        init,
        seeds = [PROGRAM_COIN_SEED, coin_mint.key().as_ref()],
        bump = Pubkey::find_program_address(&[PROGRAM_COIN_SEED, coin_mint.key().as_ref()], program_id).1,
        payer = payer,
        owner = spl_token::ID,
        space = ACCOUNT_SPAN,
    )]
    pub program_coin: AccountInfo<'info>,
    // mint for coins this depository accepts
    pub coin_mint: Box<Account<'info, Mint>>,
    // rent sysvar
    pub rent: Sysvar<'info, Rent>,
    // system program
    pub system_program: Program<'info, System>,
    // spl token program
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Deposit<'info> {
    // the user depositing funds
    // TODO i should use approval and xferfrom so user doesnt sign
    #[account(signer)]
    pub user: AccountInfo<'info>,
    // this program signing and state account
    #[account(seeds = [STATE_SEED, state.coin_mint_key.as_ref()], bump)]
    pub state: Box<Account<'info, State>>,
    // program account for coin deposit
    // XXX this one would rename `collateral_deposit_pda`
    #[account(mut, constraint = program_coin.key() == state.program_coin_key)]
    pub program_coin: Box<Account<'info, TokenAccount>>,
    // mint for redeemable tokens
    #[account(mut, constraint = redeemable_mint.key() == state.redeemable_mint_key)]
    pub redeemable_mint: Box<Account<'info, Mint>>,
    // user account depositing coins
    // XXX this one would call user_collateral_deposit
    #[account(
        mut,
        constraint = user_coin.mint == state.coin_mint_key,
        constraint = amount > 0,
        constraint = user_coin.amount >= amount,
    )]
    pub user_coin: Box<Account<'info, TokenAccount>>,
    // user account to receive redeemables
    // XXX user_redeemable_receive
    #[account(mut, constraint = user_redeemable.mint == state.redeemable_mint_key)]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,
    // system program
    pub system_program: Program<'info, System>,
    // spl token program
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(maybe_amount: Option<u64>)]
pub struct Withdraw<'info> {
    // the user withdrawing funds
    // TODO i should use approval and xferfrom so user doesnt sign
    pub user: Signer<'info>,
    // this program signing and state account
    #[account(seeds = [STATE_SEED, state.coin_mint_key.as_ref()], bump)]
    pub state: Box<Account<'info, State>>,
    // program account withdrawing coins from
    #[account(
        mut,
        constraint = program_coin.key() == state.program_coin_key,
        constraint = program_coin.amount >= user_redeemable.amount
    )]
    pub program_coin: Box<Account<'info, TokenAccount>>,
    // mint for redeemable tokens
    #[account(mut, constraint = redeemable_mint.key() == state.redeemable_mint_key)]
    pub redeemable_mint: Box<Account<'info, Mint>>,
    // user account for coin withdrawal
    #[account(mut, constraint = user_coin.mint == state.coin_mint_key)]
    pub user_coin: Box<Account<'info, TokenAccount>>,
    // user account sending redeemables
    #[account(
        mut,
        constraint = user_redeemable.mint == state.redeemable_mint_key,
        constraint =
            if let Some(n) = maybe_amount {
                n > 0 && user_redeemable.amount >= n
            } else {
                user_redeemable.amount > 0
            },
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,
    // system program
    pub system_program: Program<'info, System>,
    // spl token program
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(Default)]
pub struct State {
    pub bump: u8,
    pub controller_key: Pubkey,
    pub coin_mint_key: Pubkey,
    pub redeemable_mint_key: Pubkey,
    pub program_coin_key: Pubkey,
}
