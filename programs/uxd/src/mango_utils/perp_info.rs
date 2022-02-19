use crate::declare_check_assert_macros;
use crate::error::SourceFileId;
use crate::error::UxdError;
use crate::error::UxdErrorCode;
use crate::UxdResult;
use anchor_lang::prelude::AccountInfo;
use anchor_lang::prelude::Pubkey;
use fixed::types::I80F48;
use mango::state::MangoCache;
use mango::state::MangoGroup;

declare_check_assert_macros!(SourceFileId::MangoUtilsPerpInfo);

#[derive(Debug)]
pub struct PerpInfo {
    pub market_index: usize,
    //  price: I80F48 - native quote per native base - IMPORTANT - Equivalent to price per Lamport for SOL, or price per Satoshi
    pub price: I80F48,
    // Size of trading lots in native unit (i.e. Satoshi for BTC)
    pub base_lot_size: I80F48,
    pub quote_lot_size: I80F48,
    pub taker_fee: I80F48, // 0.005 = 5bps
}

impl PerpInfo {
    // Make sure that this is called in an instruction where a Mango CPI that validate cache is also called, else the cache may be not up to date.
    pub fn new(
        mango_group_ai: &AccountInfo,
        mango_cache_ai: &AccountInfo,
        perp_market_key: &Pubkey,
        mango_program_key: &Pubkey,
    ) -> UxdResult<Self> {
        let mango_group = MangoGroup::load_checked(mango_group_ai, mango_program_key)?;
        let mango_cache =
            MangoCache::load_checked(mango_cache_ai, mango_program_key, &mango_group)?;
        let perp_market_index = mango_group
            .find_perp_market_index(perp_market_key)
            .ok_or(throw_err!(UxdErrorCode::MangoPerpMarketIndexNotFound))?;
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
        PerpInfo {
            market_index: perp_market_index,
            price: mango_cache.price_cache[perp_market_index].price,
            base_lot_size: I80F48::from_num(
                mango_group.perp_markets[perp_market_index].base_lot_size,
            ),
            quote_lot_size: I80F48::from_num(
                mango_group.perp_markets[perp_market_index].quote_lot_size,
            ),
            taker_fee: mango_group.perp_markets[perp_market_index].taker_fee,
        }
    }
}
