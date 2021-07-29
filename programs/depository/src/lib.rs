use anchor_lang::prelude::*;
use anchor_lang::Key;
use anchor_spl::token::{self, Mint, TokenAccount, MintTo, Transfer, Burn};
use solana_program::{ system_program as system, system_instruction::create_account, program::invoke_signed };
use spl_token::instruction::{ initialize_account, initialize_mint };

const STATE_SEED: &[u8] = b"STATE";
const RSEEDWORD: &[u8] = b"REDEEMABLE";
const DSEEDWORD: &[u8] = b"DEPOSIT";

// annoyingly the spl program does not expose these as constants
const MINT_SPAN: u64 = 82;
const ACCOUNT_SPAN: u64 = 165;

#[program]
pub mod depository {
    use super::*;

    // XXX OK TODO i wanted to impose some safety on this
    // * switch token shit to cpiaccount
    // * use seed if state in scope
    // * impose token mint etc if in scope

    // XXX TODO next thing actually is convert state to a normal account
    // then bring it in scope for imposing constraints
    // remember to check that the state struct is owned by us...
    // its our source of truth so an attacker could swap it wholesale otherwise

    // creates a redeemable mint and a coin account
    pub fn new(ctx: Context<New>) -> ProgramResult {
        let accounts = ctx.accounts.to_account_infos();

        let (state_addr, state_ctr) = Pubkey::find_program_address(&[STATE_SEED], ctx.program_id);

        // create redeemable token mint
        // XXX anchor pda abstraction forces you to associate pdas with user wallets
        // so its do it by hand or hardcode a fake value zzz
        // XXX anchor also doesnt let you init accounts without a struct i think
        // regardless they force the discriminator in it so its unsuable
        // XXX because they want you using their macros they also dont expose create_account
        let (raddr, rctr) = Pubkey::find_program_address(&[RSEEDWORD], ctx.program_id);
        let rseed: &[&[&[u8]]] = &[&[RSEEDWORD, &[rctr]]];
        let rrent = ctx.accounts.rent.minimum_balance(MINT_SPAN as usize);
        let ix1 = create_account(ctx.accounts.payer.key, &raddr, rrent, MINT_SPAN, ctx.accounts.token_program.key);
        invoke_signed(&ix1, &accounts, rseed)?;

        // now do the same for our account
        let (daddr, dctr) = Pubkey::find_program_address(&[DSEEDWORD], ctx.program_id);
        let dseed: &[&[&[u8]]] = &[&[DSEEDWORD, &[dctr]]];
        let drent = ctx.accounts.rent.minimum_balance(ACCOUNT_SPAN as usize);
        let ix2 = create_account(ctx.accounts.payer.key, &daddr, drent, ACCOUNT_SPAN, ctx.accounts.token_program.key);
        invoke_signed(&ix2, &accounts, dseed)?;

        // now we initialize them
        // TODO anchor-spl implemented its own initialize_mint but it's not in a release (as of 7/22)
        // swap impls when it drops? idk im gonna be honest its more verbose and more copies for no real gain
        let ix3 = initialize_mint(
            &spl_token::ID,
            &raddr,
            &state_addr,
            // XXX it may be desirable to repudiate freeze
            Some(&state_addr),
            ctx.accounts.coin_mint.decimals,
        )?;
        invoke_signed(&ix3, &accounts, rseed)?;

        // and again
        let ix4 = initialize_account(
            &spl_token::ID,
            &daddr,
            &ctx.accounts.coin_mint.key(),
            &state_addr,
        )?;
        invoke_signed(&ix4, &accounts, dseed)?;

        // store stuff in our state account now
        ctx.accounts.state.bump = state_ctr;
        ctx.accounts.state.coin_mint_key = ctx.accounts.coin_mint.key();
        ctx.accounts.state.redeemable_mint_key = raddr;
        ctx.accounts.state.program_coin_key = daddr;

        Ok(())
    }

    // transfer coin from user_coin to program_coin
    // mint equivalent amount from redeemable_mint to user_redeemable
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
        let transfer_accounts = Transfer {
            from: ctx.accounts.user_coin.to_account_info(),
            to: ctx.accounts.program_coin.to_account_info(),
            authority: ctx.accounts.user.clone(),
        };

        let transfer_ctx = CpiContext::new(ctx.accounts.token_program.clone(), transfer_accounts);
        token::transfer(transfer_ctx, amount)?;

        let mint_accounts = MintTo {
            mint: ctx.accounts.redeemable_mint.to_account_info(),
            to: ctx.accounts.user_redeemable.to_account_info(),
            authority: ctx.accounts.state.to_account_info(),
        };

        let state_seed: &[&[&[u8]]] = &[&[STATE_SEED, &[ctx.accounts.state.bump]]];
        let mint_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.clone(), mint_accounts, state_seed);
        token::mint_to(mint_ctx, amount)?;

        Ok(())
    }

    // burn an amount of redeemable in exchange for a withdrawl of coin
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let burn_accounts = Burn {
            mint: ctx.accounts.redeemable_mint.to_account_info(),
            to: ctx.accounts.user_redeemable.to_account_info(),
            authority: ctx.accounts.user.clone(),
        };

        let burn_ctx = CpiContext::new(ctx.accounts.token_program.clone(), burn_accounts);
        token::burn(burn_ctx, amount)?;

        let transfer_accounts = Transfer {
            from: ctx.accounts.program_coin.to_account_info(),
            to: ctx.accounts.user_coin.to_account_info(),
            authority: ctx.accounts.state.to_account_info(),
        };

        let state_seed: &[&[&[u8]]] = &[&[STATE_SEED, &[ctx.accounts.state.bump]]];
        let transfer_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.clone(), transfer_accounts, state_seed);
        token::transfer(transfer_ctx, amount)?;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct New<'info> {
    // account paying for allocations
    #[account(signer, mut)]
    pub payer: AccountInfo<'info>,
    // stores program config, authority of the redeemable mint and deposit account
    #[account(
        init,
        seeds = [STATE_SEED.as_ref()],
        bump = Pubkey::find_program_address(&[STATE_SEED], program_id).1,
        payer = payer,
    )]
    pub state: ProgramAccount<'info, State>,
    // mint for bearer tokens representing deposited balances
    #[account(mut)]
    pub redeemable_mint: AccountInfo<'info>,
    // program account that coins are deposited into
    #[account(mut)]
    pub program_coin: AccountInfo<'info>,
    // mint for coins this depository accepts
    pub coin_mint: CpiAccount<'info, Mint>,
    // rent sysvar
    pub rent: Sysvar<'info, Rent>,
    // system program
    #[account(constraint = system_program.key() == system::ID)]
    pub system_program: AccountInfo<'info>,
    // spl token program
    #[account(constraint = token_program.key() == spl_token::ID)]
    pub token_program: AccountInfo<'info>,
    // this program
    #[account(constraint = program.key() == *program_id)]
    pub program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Deposit<'info> {
    // the user depositing funds
    // TODO i should use approval and xferfrom so user doesnt sign
    #[account(signer)]
    pub user: AccountInfo<'info>,
    // this program signing and state account
    #[account(
        seeds = [STATE_SEED.as_ref()],
        bump = Pubkey::find_program_address(&[STATE_SEED], program_id).1,
        payer = payer,
    )]
    pub state: ProgramAccount<'info, State>,
    // program account for coin deposit
    #[account(mut, constraint = program_coin.key() == state.program_coin_key)]
    pub program_coin: CpiAccount<'info, TokenAccount>,
    // mint for redeemable tokens
    #[account(mut, constraint = redeemable_mint.key() == state.redeemable_mint_key)]
    pub redeemable_mint: CpiAccount<'info, Mint>,
    // user account depositing coins
    #[account(
        mut,
        constraint = user_coin.mint == state.coin_mint_key,
        constraint = amount > 0,
        constraint = user_coin.amount >= amount,
    )]
    pub user_coin: CpiAccount<'info, TokenAccount>,
    // user account to receive redeemables
    #[account(mut, constraint = user_redeemable.mint == state.redeemable_mint_key)]
    pub user_redeemable: CpiAccount<'info, TokenAccount>,
    // system program
    #[account(constraint = system_program.key() == system::ID)]
    pub system_program: AccountInfo<'info>,
    // spl token program
    #[account(constraint = token_program.key() == spl_token::ID)]
    pub token_program: AccountInfo<'info>,
    // this program
    #[account(constraint = program.key() == *program_id)]
    pub program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Withdraw<'info> {
    // the user withdrawing funds
    // TODO i should use approval and xferfrom so user doesnt sign
    #[account(signer)]
    pub user: AccountInfo<'info>,
    // this program signing and state account
    #[account(
        seeds = [STATE_SEED.as_ref()],
        bump = Pubkey::find_program_address(&[STATE_SEED], program_id).1,
        payer = payer,
    )]
    pub state: ProgramAccount<'info, State>,
    // program account withdrawing coins from
    #[account(
        mut,
        constraint = program_coin.key() == state.program_coin_key,
        constraint = program_coin.amount >= amount
    )]
    pub program_coin: CpiAccount<'info, TokenAccount>,
    // mint for redeemable tokens
    #[account(mut, constraint = redeemable_mint.key() == state.redeemable_mint_key)]
    pub redeemable_mint: CpiAccount<'info, Mint>,
    // user account for coin withdrawal
    #[account(mut, constraint = user_coin.mint == state.coin_mint_key)]
    pub user_coin: CpiAccount<'info, TokenAccount>,
    // user account sending redeemables
    #[account(
        mut,
        constraint = user_redeemable.mint == state.redeemable_mint_key,
        constraint = amount > 0,
        constraint = user_redeemable.amount >= amount,
    )]
    pub user_redeemable: CpiAccount<'info, TokenAccount>,
    // system program
    #[account(constraint = system_program.key() == system::ID)]
    pub system_program: AccountInfo<'info>,
    // spl token program
    #[account(constraint = token_program.key() == spl_token::ID)]
    pub token_program: AccountInfo<'info>,
    // this program
    #[account(constraint = program.key() == *program_id)]
    pub program: AccountInfo<'info>,
}

#[account]
#[derive(Default)]
pub struct State {
    pub bump: u8,
    pub coin_mint_key: Pubkey,
    pub redeemable_mint_key: Pubkey,
    pub program_coin_key: Pubkey,
}
