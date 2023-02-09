use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

pub async fn process_initialize_program_state(
    program_test_context: &mut ProgramTestContext,
    owner: &Keypair,
    multisig_key: &Pubkey,
) -> Result<(), String> {
    let program_state =
        crate::integration_tests::program_credix::accounts::find_program_state_address();

    let accounts = credix_client::accounts::InitializeProgramState {
        owner: owner.pubkey(),
        program_state,
        system_program: anchor_lang::system_program::ID,
        //associated_token_program: anchor_spl::associated_token::ID,
        //token_program: anchor_spl::token::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };
    let payload = credix_client::instruction::InitializeProgramState {
        _credix_managers: [
            owner.pubkey(),
            owner.pubkey(),
            owner.pubkey(),
            owner.pubkey(),
            owner.pubkey(),
            owner.pubkey(),
            owner.pubkey(),
            owner.pubkey(),
            owner.pubkey(),
            owner.pubkey(),
        ],
        _credix_multisig_key: *multisig_key,
        _credix_service_fee_percentage: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        _credix_performance_fee_percentage: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
    };
    let instruction = solana_sdk::instruction::Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    crate::integration_tests::program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction,
        owner,
        owner,
    )
    .await
}
