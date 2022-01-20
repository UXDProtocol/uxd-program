use crate::error::UxdError;
use anchor_lang::prelude::*;
use fixed::types::I80F48;
use zo_abi::State;

#[derive(Debug)]
pub struct PerpInfo {
    pub market_index: usize,
    // Size of trading lots in native unit (i.e. Satoshi for BTC)
    pub base_lot_size: I80F48,
    pub quote_lot_size: I80F48,
}

impl PerpInfo {
    pub fn new(zo_state: &State, zo_dex_market_key: &Pubkey) -> Result<Self> {
        let perp_market_index = zo_state
            .perp_markets
            .iter()
            .position(|market| market.dex_market == *zo_dex_market_key)
            .ok_or_else(|| error!(UxdError::ZOPerpMarketNotFound))?;
        let perp_market_info = zo_state
            .perp_markets
            .get(perp_market_index)
            .ok_or_else(|| error!(UxdError::ZOPerpMarketInfoNotFound))?;

        Ok(PerpInfo {
            market_index: perp_market_index,
            base_lot_size: I80F48::from_num(perp_market_info.asset_lot_size),
            quote_lot_size: I80F48::from_num(perp_market_info.quote_lot_size),
        })
    }
}
