use solana_program::program_pack::IsInitialized;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;

use crate::integration_tests::api::program_test_context;

pub async fn read_packed_account<T: Pack + IsInitialized>(
    program_test_context: &mut ProgramTestContext,
    address: &Pubkey,
) -> Result<T, String> {
    let raw_account_data =
        program_test_context::read_account_data(program_test_context, address).await?;
    let mut raw_account_slice: &[u8] = &raw_account_data;
    T::unpack(&mut raw_account_slice).map_err(|e| e.to_string())
}
