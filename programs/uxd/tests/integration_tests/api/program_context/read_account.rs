use crate::integration_tests::api::program_context;
use anchor_lang::AccountDeserialize;
use solana_sdk::program_pack::IsInitialized;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;

pub async fn read_account_exists(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    address: &Pubkey,
) -> Result<bool, program_context::ProgramError> {
    Ok(program_context.get_account(address).await?.is_some())
}

pub async fn read_account_data(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    address: &Pubkey,
) -> Result<Vec<u8>, program_context::ProgramError> {
    let raw_account = program_context
        .get_account(address)
        .await?
        .ok_or(program_context::ProgramError::Custom("AccountDoesNotExist"))?;
    Ok(raw_account.data)
}

pub async fn read_account_anchor<T: AccountDeserialize>(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    address: &Pubkey,
) -> Result<T, program_context::ProgramError> {
    let raw_account_data = program_context::read_account_data(program_context, address).await?;
    let mut raw_account_slice: &[u8] = &raw_account_data;
    T::try_deserialize(&mut raw_account_slice).map_err(program_context::ProgramError::Anchor)
}

pub async fn read_account_packed<T: Pack + IsInitialized>(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    address: &Pubkey,
) -> Result<T, program_context::ProgramError> {
    let raw_account_data = program_context::read_account_data(program_context, address).await?;
    let raw_account_slice: &[u8] = &raw_account_data;
    T::unpack(raw_account_slice).map_err(program_context::ProgramError::Program)
}
