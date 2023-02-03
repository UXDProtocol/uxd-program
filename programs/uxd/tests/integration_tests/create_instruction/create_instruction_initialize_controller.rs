use anchor_lang::prelude::*;
use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

pub fn create_instruction_initialize_controller(
    authority: &Keypair,
    payer: &Keypair,
    redeemable_mint: &Pubkey,
    redeemable_mint_decimals: u8,
) -> solana_sdk::instruction::Instruction {
    let (controller, _) =
        Pubkey::find_program_address(&[uxd::CONTROLLER_NAMESPACE.as_ref()], &uxd::id());

    let accounts = uxd::accounts::InitializeController {
        authority: authority.pubkey(),
        payer: payer.pubkey(),
        controller,
        redeemable_mint: *redeemable_mint,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };
    let instruction = uxd::instruction::InitializeController {
        redeemable_mint_decimals,
    };
    solana_sdk::instruction::Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: instruction.data(),
    }
}
