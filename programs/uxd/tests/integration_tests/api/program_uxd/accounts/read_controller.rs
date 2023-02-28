use anchor_lang::AccountDeserialize;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;

use crate::integration_tests::api::program_test_context;

pub async fn read_controller(
    program_test_context: &mut ProgramTestContext,
    controller: &Pubkey,
) -> Result<uxd::state::Controller, String> {
    let data =
        program_test_context::read_account(program_test_context, controller, &uxd::id()).await?;
    let mut slice: &[u8] = &data;
    println!("SLICE.len: {:?}", slice.len());
    println!("SLICE.data: {:?}", slice);
    Ok(uxd::state::Controller::try_deserialize(&mut slice).map_err(|e| e.to_string())?)
}
