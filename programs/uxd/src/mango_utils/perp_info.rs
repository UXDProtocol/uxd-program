use crate::error::UxdError;
use crate::mango_utils::MangoInfo;
use anchor_lang::prelude::*;
use fixed::types::I80F48;
use mango::ids::mngo_token;
use mango::state::CENTIBPS_PER_UNIT;

#[derive(Debug)]
pub struct PerpInfo {
    pub market_index: usize,
    //  price: I80F48 - native quote per native base - IMPORTANT - Equivalent to price per Lamport for SOL, or price per Satoshi
    pub price: I80F48,
    // Size of trading lots in native unit (i.e. Satoshi for BTC)
    pub base_lot_size: I80F48,
    pub quote_lot_size: I80F48,
    // taker_fee + ref_fee if any
    // ref_fee : 1bps or 0 with 10K MNGO on the account(as of 02/20/2022)
    // taker_fee : 0.004 = 4bps on most perp markets (as of 02/20/2022)
    pub effective_fee: I80F48,
}

impl PerpInfo {
    // Make sure that this is called in an instruction where a Mango CPI that validate cache is also called, else the cache may be not up to date.
    pub fn new(
        mango_group_ai: &AccountInfo,
        mango_cache_ai: &AccountInfo,
        mango_account_ai: &AccountInfo,
        perp_market_key: &Pubkey,
        mango_group_key: &Pubkey,
        mango_program_key: &Pubkey,
    ) -> Result<Self> {
        let mango_info = MangoInfo::init(
            mango_group_ai,
            mango_cache_ai,
            mango_account_ai,
            mango_group_key,
            mango_program_key,
        )?;
        let perp_market_index = mango_info
            .mango_group
            .find_perp_market_index(perp_market_key)
            .ok_or_else(|| error!(UxdError::MangoPerpMarketIndexNotFound))?;
        PerpInfo::init(&mango_info, perp_market_index)
    }

    pub fn init(mango_info: &MangoInfo, perp_market_index: usize) -> Result<Self> {
        let ref_fee = determine_ref_fee(&mango_info)?;
        let taker_fee = mango_info.mango_group.perp_markets[perp_market_index].taker_fee;
        Ok(PerpInfo {
            market_index: perp_market_index,
            price: mango_info.mango_cache.price_cache[perp_market_index].price,
            base_lot_size: I80F48::from_num(
                mango_info.mango_group.perp_markets[perp_market_index].base_lot_size,
            ),
            quote_lot_size: I80F48::from_num(
                mango_info.mango_group.perp_markets[perp_market_index].quote_lot_size,
            ),
            effective_fee: ref_fee + taker_fee,
        })
    }
}

// Check for Ref fees (fees added on mango v3.3.5)
// Referral fees
// If you hold 10k MNGO or more in your MangoAccount as of 02/20/2022, you skip this fee.
fn determine_ref_fee(mango_info: &MangoInfo) -> Result<I80F48> {
    let mngo_deposits = mango_info
        .get_native_deposit_of_mint(&mngo_token::id())
        .map_err(|me| ProgramError::from(me))?;
    let ref_mngo_req = I80F48::from_num(mango_info.mango_group.ref_mngo_required);
    if mngo_deposits >= ref_mngo_req {
        return Ok(I80F48::ZERO);
    }
    Ok(I80F48::from_num(mango_info.mango_group.ref_share_centibps) / CENTIBPS_PER_UNIT)
}

// Convert price into a quote lot per base lot price.
// Price is the value of 1 native base unit expressed in native quote.
pub fn price_to_lot_price(price: I80F48, perp_info: &PerpInfo) -> Result<I80F48> {
    price
        .checked_mul(perp_info.base_lot_size)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_div(perp_info.quote_lot_size)
        .ok_or_else(|| error!(UxdError::MathError))
}
