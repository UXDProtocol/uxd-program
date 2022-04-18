use zo_abi::OpenOrdersInfo;

use crate::error::UxdError;

#[derive(Debug)]
pub struct DeltaNeutralPosition {
    // Quote native units
    pub size: i64,
    // Native units
    pub base_size: i64,
    // Quote native units
    pub realized_pnl: i64,
}

impl TryFrom<&OpenOrdersInfo> for DeltaNeutralPosition {
    type Error = UxdError;

    fn try_from(open_orders_info: &OpenOrdersInfo) -> Result<Self, Self::Error> {
        // Should at max have an open position for the DN short, 0 if no UXD has been minted.
        if open_orders_info.order_count > 1 {
            return Err(UxdError::ZOInvalidControlState);
        }
        // Should be a short position, or 0 if no UXD has been minted.
        if open_orders_info.pos_size > 0 {
            return Err(UxdError::ZOInvalidControlState);
        }
        Ok(DeltaNeutralPosition {
            size: open_orders_info.native_pc_total,
            base_size: open_orders_info.pos_size,
            realized_pnl: open_orders_info.realized_pnl,
        })
    }
}
