pub fn ui_amount_to_native_amount(ui_amount: u64, mint_decimals: u8) -> u64 {
    ui_amount * 10u64.pow(mint_decimals.into())
}
