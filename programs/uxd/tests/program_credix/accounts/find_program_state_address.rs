use solana_sdk::pubkey::Pubkey;

pub fn find_program_state_address() -> Pubkey {
    let (program_state, _) = credix_client::ProgramState::generate_pda();
    assert_eq!(
        "9DyuYBKDGMfy72sH9TtVfyydpQ4RNSEprobHrAhMX4Yt",
        program_state.to_string()
    );
    return program_state;
}
