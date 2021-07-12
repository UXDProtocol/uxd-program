use anchor_lang::prelude::*;
use anchor_spl::token::{self, InitializeAccount};
use solana_program::{ system_instruction::create_account, program::invoke_signed };
use spl_token::instruction::{ initialize_account, initialize_mint };

const RSEEDWORD: &[u8] = b"REDEEMABLE";
const DSEEDWORD: &[u8] = b"DEPOSIT";
const MINT_DECIMAL: u8 = 9;

// annoyingly the spl program does not expose these as constants
const MINT_SPAN: u64 = 82;
const ACCOUNT_SPAN: u64 = 165;

#[program]
pub mod depository {
    use super::*;

    // XXX oki i can make accounts and initialize them etc what *actually* do i need to do
    // talked to patrick and he wants to do these as singletons which makes things simple
    // * new: accept coin mint address. init a redeemable mint and a coin account
    // * deposit: stick the coins in the account, mint redeemables
    // * withdraw: take and burn the redeemables, return the coin
    // we dont need to track user balances or whatever, redeemables are bearer instruments
    // i think i need to approve the transfer for the program to do it?
    // or else it may just mean that i dont need the user as a signer

    // uhh ok so how tf does this work
    // mint and deposit need to be pdas obviously
    // we pass in the btc mint address in new
    // then both new accounts are derived from that...
    // man this is so confusing

    // their pda macros are fucking stupid the way i have to do this is
    // new: pass in btc mint address, store it in state
    // create a pda for the redeemable mint and a pda for the btc deposit address
    // this means we much perform four cpi calls. two creates two inits
    // we can still use the seed guards i thiiink to make sure our addresses are right
    // but for deposit and withdraw we cant check that say a btc address is btc
    // whatever. close enough

    #[state]
    pub struct Depository {
        pub deposit_mint: Pubkey,
        pub redeemable_mint: Pubkey,
        pub deposit_account: Pubkey,
    }

    impl Depository {
        // creates a redeemable mint and a coin account
        pub fn new(ctx: Context<New>) -> Result<Self, ProgramError> {
            let accounts = ctx.accounts.to_account_infos();

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
            // XXX anchor_spl does not have initialize_mint
            // and i may as well use the normal thing for both so i dont have more needless clones
            let ix3 = initialize_mint(
                &spl_token::ID,
                &raddr,
                ctx.program_id,
                Some(ctx.program_id),
                MINT_DECIMAL,
            )?;
            invoke_signed(&ix3, &accounts, rseed)?;

            // and again
            let ix4 = initialize_account(
                &spl_token::ID,
                &daddr,
                ctx.accounts.deposit_mint.key,
                ctx.program_id,
            )?;
            invoke_signed(&ix4, &accounts, dseed)?;

            // we store raddr and daddr to avoid recalculating them
            Ok(Self {
                deposit_mint: *ctx.accounts.deposit_mint.key,
                redeemable_mint: raddr,
                deposit_account: daddr,
            })
        }

        pub fn deposit(&self, ctx: Context<Deposit>) -> ProgramResult {
            Ok(())
        }
    }
}

#[derive(Accounts)]
pub struct New<'info> {
    #[account(signer, mut)]
    pub payer: AccountInfo<'info>,
    #[account(mut)]
    pub redeemable_mint: AccountInfo<'info>,
    #[account(mut)]
    pub deposit_account: AccountInfo<'info>,
    pub deposit_mint: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    // TODO can i enforce these are correct
    pub sys: AccountInfo<'info>,
    pub tok: AccountInfo<'info>,
    // XXX i hate including this but i need a full account info object for the program
    // and it seems more reasonable to let it do this than do it yourselves
    pub prog: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    // TODO i should use approval and xferfrom so user doesnt sign
    #[account(signer)]
    pub user: AccountInfo<'info>,
    #[account(mut)]
    pub deposit_account: AccountInfo<'info>,
    #[account(mut)]
    pub redeemable_mint: AccountInfo<'info>,
    #[account(mut)]
    pub user_coin: AccountInfo<'info>,
    #[account(mut)]
    pub user_redeemable: AccountInfo<'info>,
    // TODO enforce correct
    pub sys: AccountInfo<'info>,
    pub tok: AccountInfo<'info>,
}
