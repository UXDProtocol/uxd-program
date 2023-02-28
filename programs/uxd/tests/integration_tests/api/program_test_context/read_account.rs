use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;

pub async fn read_account(
    program_test_context: &mut ProgramTestContext,
    address: &Pubkey,
    owner: &Pubkey,
) -> Result<Vec<u8>, String> {
    println!(" [[[[ READING ACCOUNT {:?} ]]]]", address);
    let raw_account = program_test_context
        .banks_client
        .get_account(*address)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("AccountDoesNotExist")?;
    if raw_account.owner != *owner {
        return Err(String::from("InvalidAccountOwner"));
    }
    println!(" --- len: {:?}", raw_account.data.len());
    println!(" --- data: {:?}", raw_account.data);
    Ok(raw_account.data)
}
