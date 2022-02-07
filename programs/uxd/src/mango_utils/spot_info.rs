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

declare_check_assert_macros!(SourceFileId::MangoUtilsSpotInfo);

#[derive(Debug)]
pub struct SpotInfo {
    pub market_index: usize, // Mango
    //  price: I80F48 - native quote per native base - IMPORTANT - Equivalent to price per Lamport for SOL, or price per Satoshi
    pub price: I80F48,
    // How many native unit of base totalling for a base ui unit (i.e. how many Satoshi per BTC)
    pub base_unit: I80F48,
    // Size of trading lots in native unit (i.e. Satoshi for BTC)
    pub base_lot_size: I80F48,
    pub quote_unit: I80F48,
    pub quote_lot_size: I80F48,
    pub taker_fee: I80F48,
}

impl SpotInfo {
    // Make sure that this is called in an instruction where a Mango CPI that validate cache is also called, else the cache may be not up to date.
    pub fn new(
        mango_group_ai: &AccountInfo,
        mango_cache_ai: &AccountInfo,
        spot_market_ai: &AccountInfo,
        mango_program_key: &Pubkey,
        serum_dex_program_key: &Pubkey,
    ) -> UxdResult<Self> {
        let mango_group = MangoGroup::load_checked(mango_group_ai, mango_program_key)?;
        let mango_cache =
            MangoCache::load_checked(&mango_cache_ai, mango_program_key, &mango_group)?;
        let spot_market_index = mango_group
            .find_spot_market_index(spot_market_ai.key)
            .ok_or(throw_err!(UxdErrorCode::MangoSpotMarketIndexNotFound))?;
            // Load not checked?? CHECK
        let serum_spot_market =
            serum_dex::state::Market::load(spot_market_ai, serum_dex_program_key)?;

        Ok(SpotInfo::init(
            &mango_group,
            &mango_cache,
            spot_market_index,
            &serum_spot_market,
        )?)
    }

    pub fn init(
        mango_group: &MangoGroup,
        mango_cache: &MangoCache,
        spot_market_index: usize,
        serum_spot_market: &serum_dex::state::Market,
    ) -> UxdResult<Self> {
        let base_decimals = mango_group.tokens[spot_market_index].decimals;
        let quote_decimals = mango_group.tokens[mango::state::QUOTE_INDEX].decimals;
        let base_unit =
            I80F48::from_num(10u64.checked_pow(base_decimals.into()).ok_or(math_err!())?);
        let base_lot_size = I80F48::from_num(serum_spot_market.coin_lot_size);
        let quote_unit = I80F48::from_num(
            10u64
                .checked_pow(quote_decimals.into())
                .ok_or(math_err!())?,
        );
        let quote_lot_size = I80F48::from_num(serum_spot_market.pc_lot_size);
        Ok(SpotInfo {
            market_index: spot_market_index,
            price: mango_cache.price_cache[spot_market_index].price,
            base_unit,
            base_lot_size,
            quote_unit,
            quote_lot_size,
            taker_fee: I80F48::from_num(serum_spot_market.fee_rate_bps),
        })
    }
}
