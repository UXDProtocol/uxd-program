use anchor_lang::prelude::*;
use anchor_lang::Key;
use anchor_spl::token::{self, Mint, TokenAccount, MintTo, Transfer};
use solana_program::{ system_program, system_instruction::create_account, program::invoke_signed };
use spl_token::instruction::{ initialize_account, initialize_mint };

const RSEEDWORD: &[u8] = b"REDEEMABLE";
const DSEEDWORD: &[u8] = b"DEPOSIT";

// annoyingly the spl program does not expose these as constants
const MINT_SPAN: u64 = 82;
const ACCOUNT_SPAN: u64 = 165;

#[program]
pub mod depository {
    use super::*;

    #[state]
    pub struct Depository {
        pub dummy_signer: Pubkey,
        pub dummy_bump: u8,
        pub deposit_mint: Pubkey,
        pub redeemable_mint: Pubkey,
        pub deposit_account: Pubkey,
    }

    // XXX OK TODO i wanted to impose some safety on this
    // * switch token shit to cpiaccount
    // * use seed if state in scope
    // * impose token mint etc if in scope

    impl Depository {
        // creates a redeemable mint and a coin account
        pub fn new(ctx: Context<New>) -> Result<Self, ProgramError> {
            let accounts = ctx.accounts.to_account_infos();

            // generate an address we can sign for to own the accounts
            let (dummy_addr, dummy_ctr) = Pubkey::find_program_address(&[], ctx.program_id);

            // create redeemable token mint
            // XXX anchor pda abstraction forces you to associate pdas with user wallets
            // so its do it by hand or hardcode a fake value zzz
            // XXX anchor also doesnt let you init accounts without a struct i think
            // regardless they force the discriminator in it so its unsuable
            // XXX because they want you using their macros they also dont expose create_account
            let (raddr, rctr) = Pubkey::find_program_address(&[RSEEDWORD], ctx.program_id);
            let rseed: &[&[&[u8]]] = &[&[RSEEDWORD, &[rctr]]];
            let rrent = ctx.accounts.rent.minimum_balance(MINT_SPAN as usize);
            let ix1 = create_account(ctx.accounts.payer.key, &raddr, rrent, MINT_SPAN, ctx.accounts.tok.key);
            invoke_signed(&ix1, &accounts, rseed)?;

            // now do the same for our account
            let (daddr, dctr) = Pubkey::find_program_address(&[DSEEDWORD], ctx.program_id);
            let dseed: &[&[&[u8]]] = &[&[DSEEDWORD, &[dctr]]];
            let drent = ctx.accounts.rent.minimum_balance(ACCOUNT_SPAN as usize);
            let ix2 = create_account(ctx.accounts.payer.key, &daddr, drent, ACCOUNT_SPAN, ctx.accounts.tok.key);
            invoke_signed(&ix2, &accounts, dseed)?;

            // now we initialize them
            // TODO anchor-spl implemented its own initialize_mint but it's not in a release (as of 7/22)
            // swap impls when it drops? idk im gonna be honest its more verbose and more copies for no real gain
            let ix3 = initialize_mint(
                &spl_token::ID,
                &raddr,
                &dummy_addr,
                Some(&dummy_addr),
                ctx.accounts.deposit_mint.decimals,
            )?;
            invoke_signed(&ix3, &accounts, rseed)?;

            // and again
            let ix4 = initialize_account(
                &spl_token::ID,
                &daddr,
                &ctx.accounts.deposit_mint.key(),
                &dummy_addr,
            )?;
            invoke_signed(&ix4, &accounts, dseed)?;

            // we store raddr and daddr to avoid recalculating them
            Ok(Self {
                dummy_signer: dummy_addr,
                dummy_bump: dummy_ctr,
                deposit_mint: ctx.accounts.deposit_mint.key(),
                redeemable_mint: raddr,
                deposit_account: daddr,
            })
        }

        // transfer coin from user_coin to deposit_account
        // mint equivalent amount from redeemable_mint to user_redeemable
        pub fn deposit(&self, ctx: Context<Deposit>, amount: u64) -> ProgramResult {
            let transfer_accounts = Transfer {
                from: ctx.accounts.user_coin.to_account_info(),
                to: ctx.accounts.deposit_account.to_account_info(),
                authority: ctx.accounts.user.clone(),
            };

            let transfer_ctx = CpiContext::new(ctx.accounts.tok.clone(), transfer_accounts);
            token::transfer(transfer_ctx, amount)?;

            let mint_accounts = MintTo {
                mint: ctx.accounts.redeemable_mint.to_account_info(),
                to: ctx.accounts.user_redeemable.to_account_info(),
                authority: ctx.accounts.dummy.clone(),
            };

            let dummy_seed: &[&[&[u8]]] = &[&[&[self.dummy_bump]]];
            let mint_ctx = CpiContext::new_with_signer(ctx.accounts.tok.clone(), mint_accounts, dummy_seed);
            token::mint_to(mint_ctx, amount)?;

            Ok(())
        }

        // burn an amount of 
        //pub fn withdraw(&self, ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        //}

    }
}

#[derive(Accounts)]
pub struct New<'info> {
    // account paying for allocations
    #[account(signer, mut)]
    pub payer: AccountInfo<'info>,
    // the empty seed account, owner of the redeemable mint and deposit account
    pub dummy: AccountInfo<'info>,
    // mint for bearer tokens representing deposited balances
    #[account(mut)]
    pub redeemable_mint: AccountInfo<'info>,
    // program account that coins are deposited into
    #[account(mut)]
    pub deposit_account: AccountInfo<'info>,
    // mint for coins this depository accepts
    pub deposit_mint: CpiAccount<'info, Mint>,
    // rent sysvar
    pub rent: Sysvar<'info, Rent>,
    // system program
    #[account(constraint = sys.key() == system_program::ID)]
    pub sys: AccountInfo<'info>,
    // spl token program
    #[account(constraint = tok.key() == spl_token::ID)]
    pub tok: AccountInfo<'info>,
    // this program
    #[account(constraint = prog.key() == *program_id)]
    pub prog: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    // the user depositing funds
    // TODO i should use approval and xferfrom so user doesnt sign
    #[account(signer)]
    pub user: AccountInfo<'info>,
    // this program signing account
    pub dummy: AccountInfo<'info>,
    // program account for coin deposit
    #[account(mut)]
    pub deposit_account: CpiAccount<'info, TokenAccount>,
    // mint for redeemable tokens
    #[account(mut)]
    pub redeemable_mint: CpiAccount<'info, Mint>,
    // user account depositing coins
    #[account(mut)]
    pub user_coin: CpiAccount<'info, TokenAccount>,
    // user account to receive redeemables
    #[account(mut, constraint = user_redeemable.mint == redeemable_mint.key())]
    pub user_redeemable: CpiAccount<'info, TokenAccount>,
    // system program
    #[account(constraint = sys.key() == system_program::ID)]
    pub sys: AccountInfo<'info>,
    // spl token program
    #[account(constraint = tok.key() == spl_token::ID)]
    pub tok: AccountInfo<'info>,
    // this program
    #[account(constraint = prog.key() == *program_id)]
    pub prog: AccountInfo<'info>,
}
