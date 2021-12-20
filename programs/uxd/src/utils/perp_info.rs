use crate::ErrorCode;
use crate::UxdResult;
use anchor_lang::prelude::AccountInfo;
use anchor_lang::prelude::Pubkey;
use fixed::types::I80F48;
use mango::state::MangoCache;
use mango::state::MangoGroup;

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
    // Make sure that this is called in an instruction where a Mango CPI that validate cache is also called, else the cache may be not up to date.
    pub fn new(
        mango_group_ai: &AccountInfo,
        mango_cache_ai: &AccountInfo,
        perp_market_key: &Pubkey,
        mango_program_key: &Pubkey,
    ) -> UxdResult<Self> {
        let mango_group = match MangoGroup::load_checked(mango_group_ai, mango_program_key) {
            Ok(it) => it,
            Err(_err) => return Err(ErrorCode::MangoGroupLoading),
        };
        let mango_cache =
            match MangoCache::load_checked(&mango_cache_ai, mango_program_key, &mango_group) {
                Ok(it) => it,
                Err(_err) => return Err(ErrorCode::MangoCacheLoading),
            };
        let perp_market_index = match mango_group.find_perp_market_index(perp_market_key) {
            Some(it) => it,
            None => return Err(ErrorCode::MangoPerpMarketIndexNotFound),
        };

        Ok(PerpInfo::init(
            &mango_group,
            &mango_cache,
            perp_market_index,
        ))
    }
    pub fn init(
        mango_group: &MangoGroup,
        mango_cache: &MangoCache,
        perp_market_index: usize,
    ) -> Self {
        let base_decimals = mango_group.tokens[perp_market_index].decimals;
        let quote_decimals = mango_group.tokens[mango::state::QUOTE_INDEX].decimals;
        let base_unit = I80F48::checked_from_num(10u64.pow(base_decimals.into())).unwrap();
        let base_lot_size =
            I80F48::checked_from_num(mango_group.perp_markets[perp_market_index].base_lot_size)
                .unwrap();
        let quote_unit = I80F48::checked_from_num(10u64.pow(quote_decimals.into())).unwrap();
        let quote_lot_size =
            I80F48::checked_from_num(mango_group.perp_markets[perp_market_index].quote_lot_size)
                .unwrap();
        PerpInfo {
            market_index: perp_market_index,
            price: mango_cache.price_cache[perp_market_index].price,
            base_unit,
            base_lot_size,
            quote_unit,
            quote_lot_size,
            taker_fee: mango_group.perp_markets[perp_market_index].taker_fee,
        }
    }
}
