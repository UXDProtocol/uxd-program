#[derive(Debug)]
pub struct DeltaNeutralPosition {
    // Quote native units
    pub size: i64,
    // Native units
    pub base_size: i64,
    // Quote native units
    pub realized_pnl: i64,
}
