use anchor_lang::prelude::Pubkey;

pub fn find_redeemable_mint_address() -> Pubkey {
    let (redeemable_mint, _) =
        Pubkey::find_program_address(&[uxd::REDEEMABLE_MINT_NAMESPACE.as_ref()], &uxd::id());
    assert_eq!(
        "7kbnvuGBxxj8AG9qp8Scn56muWGaRaFqxg1FsRp3PaFT",
        redeemable_mint.to_string()
    );
    return redeemable_mint;
}
