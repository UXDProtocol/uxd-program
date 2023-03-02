use solana_program::pubkey::Pubkey;

pub struct IdentityDepositoryInfo {
    pub depository: Pubkey,
    pub collateral_vault: Pubkey,
}

pub fn create_identity_depository_info() -> IdentityDepositoryInfo {
    let depository =
        Pubkey::find_program_address(&[uxd::IDENTITY_DEPOSITORY_NAMESPACE.as_ref()], &uxd::id()).0;

    let collateral_vault = Pubkey::find_program_address(
        &[uxd::IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE.as_ref()],
        &uxd::id(),
    )
    .0;

    IdentityDepositoryInfo {
        depository,
        collateral_vault,
    }
}
