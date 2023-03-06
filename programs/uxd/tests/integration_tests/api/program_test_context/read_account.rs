use anchor_lang::AccountDeserialize;
use solana_program::program_pack::IsInitialized;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;

use crate::integration_tests::api::program_test_context;

pub async fn read_account_data(
    program_test_context: &mut ProgramTestContext,
    address: &Pubkey,
) -> Result<Vec<u8>, program_test_context::ProgramTestError> {
    let raw_account = program_test_context
        .banks_client
        .get_account(*address)
        .await
        .map_err(program_test_context::ProgramTestError::BanksClient)?
        .ok_or(program_test_context::ProgramTestError::Custom(
            "AccountDoesNotExist",
        ))?;
    Ok(raw_account.data)
}

pub async fn read_account_anchor<T: AccountDeserialize>(
    program_test_context: &mut ProgramTestContext,
    address: &Pubkey,
) -> Result<T, program_test_context::ProgramTestError> {
    let raw_account_data =
        program_test_context::read_account_data(program_test_context, address).await?;
    let mut raw_account_slice: &[u8] = &raw_account_data;
    T::try_deserialize(&mut raw_account_slice)
        .map_err(program_test_context::ProgramTestError::Anchor)
}

pub async fn read_account_packed<T: Pack + IsInitialized>(
    program_test_context: &mut ProgramTestContext,
    address: &Pubkey,
) -> Result<T, program_test_context::ProgramTestError> {
    let raw_account_data =
        program_test_context::read_account_data(program_test_context, address).await?;
    let raw_account_slice: &[u8] = &raw_account_data;
    T::unpack(raw_account_slice).map_err(program_test_context::ProgramTestError::Program)
}
