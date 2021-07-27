use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};
use solana_program::{ system_instruction::create_account, program::invoke_signed };
use spl_token::instruction::{ initialize_account, initialize_mint };


const MINT_SPAN: u64 = 82;
const ACCOUNT_SPAN: u64 = 165;
const MINT_DECIMAL: u8 = 9;
const UXDSEEDWORD: &[u8] = b"STABLECOIN";
const PROXYSEEDWORD: &[u8] = b"PROXY";

#[program]
pub mod controller{
    use super::*;



    impl Controller{

        pub fn new(ctx: Context<New>) -> Result<Self, ProgramError> {
            let accounts = ctx.accounts.to_account_infos();

            let (dummy_addr, dummy_ctr) = Pubkey::find_program_address(&[], ctx.program_id);

            // create uxd mint
            let (uxd_addr, uxd_ctr) = Pubkey::find_program_address(&[UXDSEEDWORD], ctx.program_id);
            let uxd_seed: &[&[&[u8]]] = &[&[UXDSEEDWORD, &[uxd_ctr]]];
            let uxd_rent = ctx.accounts.rent.minimum_balance(MINT_SPAN as usize);
            let uxd_i1 = create_account(ctx.accounts.payer.key, &uxd_addr, uxd_rent, MINT_SPAN. ctx.accounts.tok.key);
            invoke_signed(&ix1, &accounts, uxd_seed)?;

            let uxd_i2 = initialize_mint(
                &spl_token::ID,
                &uxd_addr,
                &dummy_addr,
                Some(&dummy_addr),
                MINT_DECIMAL,
            )?;
            invoke_signed(&uxd_i2, &accounts, uxd_seed)?;

            // create proxy account
            let (proxy_addr, proxy_ctr) = Pubkey::find_program_address(&[PROXYSEEDWORD], ctx.program_id);
            let proxy_seed: &[&[&[u8]]] = &[&[PROXYSEEDWORD, &[proxy_ctr]]];
            let proxy_rent = ctx.accounts.rent.minimum_balance(ACCOUNT_SPAN as usize);
            let proxy_i1 = create_account(ctx.accounts.payer.key, &proxy_addr, proxy_rent, ACCOUNT_SPAN, ctx.tok.key);
            invoke_signed(&ix2, &accounts, dseed)?;

            //initialize proxy account
            let proxy_i2 = initialize_account(
                &spl_token::ID,
                &proxy_addr,
                ctx.accounts.proxy_mint.key,
                &dummy_addr,
            )?;

            // Don't use state because deprecated

            // initialize initialize mango or equivalent user account

        }

        // pub fn mint(ctx: Context<Mint>) -> Result<Self, ProgramError> {
        //     // accept depository redeemable token
        //     // validate user input
        //     // call depository proxytransfer or equivalent
        //     // with reciever being controller proxy address
        //     // transfer to controller's Mango user account
        //     //
        // }

        pub fn redeem(ctx: Context<Redeem>) -> Result<Self, ProgramError> {
            // validate user input
            // burn user uxd
            // exchange depository redeemable token
        }

    }

}
