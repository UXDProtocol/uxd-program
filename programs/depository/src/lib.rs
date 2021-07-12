use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};
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

    #[state]
    pub struct Depository {
        pub dummy_signer: Pubkey,
        pub dummy_bump: u8,
        pub deposit_mint: Pubkey,
        pub redeemable_mint: Pubkey,
        pub deposit_account: Pubkey,
    }

    impl Depository {
        // creates a redeemable mint and a coin account
        pub fn new(ctx: Context<New>) -> Result<Self, ProgramError> {
            let accounts = ctx.accounts.to_account_infos();

            // XXX unless theres a backdoor function im missing, you cant sign for program_id
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
            // XXX anchor_spl does not have initialize_mint
            // TODO impl and pr it later
            // may as well use the normal thing for both so i dont have more needless clones
            let ix3 = initialize_mint(
                &spl_token::ID,
                &raddr,
                &dummy_addr,
                Some(&dummy_addr),
                MINT_DECIMAL,
            )?;
            invoke_signed(&ix3, &accounts, rseed)?;

            // and again
            let ix4 = initialize_account(
                &spl_token::ID,
                &daddr,
                ctx.accounts.deposit_mint.key,
                &dummy_addr,
            )?;
            invoke_signed(&ix4, &accounts, dseed)?;

            // we store raddr and daddr to avoid recalculating them
            Ok(Self {
                dummy_signer: dummy_addr,
                dummy_bump: dummy_ctr,
                deposit_mint: *ctx.accounts.deposit_mint.key,
                redeemable_mint: raddr,
                deposit_account: daddr,
            })
        }

        // transfer coin from user_coin to deposit_account
        // mint equivalent amount from redeemable_mint to user_redeemable
        pub fn deposit(&self, ctx: Context<Deposit>, amount: u64) -> ProgramResult {
            let transfer_accounts = Transfer {
                from: ctx.accounts.user_coin.clone(),
                to: ctx.accounts.deposit_account.clone(),
                authority: ctx.accounts.user.clone(),
            };

            let transfer_ctx = CpiContext::new(ctx.accounts.tok.clone(), transfer_accounts);
            token::transfer(transfer_ctx, amount)?;

            let mint_accounts = MintTo {
                mint: ctx.accounts.redeemable_mint.clone(),
                to: ctx.accounts.user_redeemable.clone(),
                authority: ctx.accounts.dummy.clone(),
            };

            let dummy_seed: &[&[&[u8]]] = &[&[&[self.dummy_bump]]];
            let mint_ctx = CpiContext::new_with_signer(ctx.accounts.tok.clone(), mint_accounts, dummy_seed);
            token::mint_to(mint_ctx, amount)?;

            Ok(())
        }

        // TODO withdraw. there is nothing novel its just an inversion of deposit with burn
    }
}

// TODO for all methods, need to do whatever sanity checks for account bullshit

#[derive(Accounts)]
pub struct New<'info> {
    #[account(signer, mut)]
    pub payer: AccountInfo<'info>,
    pub dummy: AccountInfo<'info>,
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
    pub dummy: AccountInfo<'info>,
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
    // FIXME ok seriously look up how to convert program id to a account info lol
    pub prog: AccountInfo<'info>,
}
