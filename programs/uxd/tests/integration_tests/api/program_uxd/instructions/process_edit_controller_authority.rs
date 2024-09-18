use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use uxd::state::Controller;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_edit_controller_authority(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    payer: &Keypair,
    authority: &Keypair,
    new_authority: &Pubkey,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let controller = program_uxd::accounts::find_controller_pda().0;

    // Read state before
    let controller_before =
        program_context::read_account_anchor::<Controller>(program_context, &controller).await?;

    // Execute IX
    let accounts = uxd::accounts::EditControllerAuthority {
        authority: authority.pubkey(),
        controller,
    };
    let payload = uxd::instruction::EditControllerAuthority {
        authority: *new_authority,
    };
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction_with_signer(
        program_context,
        instruction,
        payer,
        authority,
    )
    .await?;

    // Read state after
    let controller_after =
        program_context::read_account_anchor::<Controller>(program_context, &controller).await?;

    // Check behaviour
    let controller_authority_before = controller_before.authority;
    assert_eq!(controller_authority_before, authority.pubkey());

    let controller_authority_after = controller_after.authority;
    assert_eq!(controller_authority_after, *new_authority);

    // Done
    Ok(())
}
