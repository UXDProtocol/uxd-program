use fixed::types::I80F48;
use mango::state::{MangoCache, MangoGroup, PerpAccount};

// mngo not to collide with mango

#[derive(Debug)]
pub struct PerpInfo {
    pub market_index: usize,
    //  price: I80F48 - native quote per native base - THIS IS IMPORTANT - Equivalent to price per lamport for sol, or price per satoshi
    pub price: I80F48,
    // How many native unit of base totalling for a base ui unit (i.e. how many sat per BTC)
    pub base_unit: I80F48,
    // Size of trading lots in native unit (i.e. satoshi for btc)
    pub base_lot_size: I80F48,
    pub quote_unit: I80F48,
    pub quote_lot_size: I80F48,
    pub taker_fee: I80F48,
}

impl PerpInfo {
    pub fn init(
        mango_group: &MangoGroup,
        mango_cache: &MangoCache,
        perp_market_index: usize,
    ) -> Self {
        let base_decimals = mango_group.tokens[perp_market_index].decimals;
        let quote_decimals = mango_group.tokens[mango::state::QUOTE_INDEX].decimals;

        PerpInfo {
            market_index: perp_market_index,
            price: mango_cache.price_cache[perp_market_index].price,
            base_unit: I80F48::from_num(10u64.pow(base_decimals.into())),
            base_lot_size: I80F48::from_num(
                mango_group.perp_markets[perp_market_index].base_lot_size,
            ),
            quote_unit: I80F48::from_num(10u64.pow(quote_decimals.into())),
            quote_lot_size: I80F48::from_num(
                mango_group.perp_markets[perp_market_index].quote_lot_size,
            ),
            taker_fee: mango_group.perp_markets[perp_market_index].taker_fee,
        }
    }

    pub fn base_lot_price_in_quote_lot_unit(&self) -> I80F48 {
        self.price
            .checked_mul(self.quote_unit)
            .unwrap() // to quote native amount
            .checked_div(self.base_unit)
            .unwrap() // price for 1 decimal unit (1 satoshi for btc for instance)
            .checked_mul(self.base_lot_size)
            .unwrap() // price for a lot (100 sat for btc for instance)
            .checked_div(self.quote_lot_size)
            .unwrap() // price for a lot in quote_lot_unit
    }
}

// Return the current base position for a given PerpAccount
pub fn perp_base_position(perp_account: &PerpAccount) -> i64 {
    // msg!("  -----");
    // msg!("  base_position {}", perp_account.base_position);
    // msg!("  quote_position {}", perp_account.quote_position);
    // msg!("  taker_base {}", perp_account.taker_base);
    // msg!("  taker_quote {}", perp_account.taker_quote);
    // msg!("  -----");
    perp_account
        .base_position
        .checked_add(perp_account.taker_base)
        .unwrap()
}
