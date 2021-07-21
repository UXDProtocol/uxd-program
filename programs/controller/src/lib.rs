use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};
use solana_program::{ system_instruction::create_account, program::invoke_signed };
use spl_token::instruction::{ initialize_account, initialize_mint };


const MINT_SPAN: u64 = 82;
const ACCOUNT_SPAN: u64 = 165;
const MINT_DECIMAL: u8 = 9;
const SSEEDWORD: &[u8] = b"STABLECOIN";

#[program]
pub mod controller{
    use super::*;



    impl Controller{

        pub fn new(ctx: Context<New>) -> Result<Self, ProgramError> {
            let accounts = ctx.accounts.to_account_infos();

            let (dummy_addr, dummy_ctr) = Pubkey::find_program_address(&[], ctx.program_id);


        }

    }

}
