use anchor_lang::prelude::{AccountInfo, Pubkey};
use fixed::types::I80F48;
use mango::{
    matching::{Book, Side},
    state::{MangoCache, MangoGroup, PerpAccount},
};

use crate::{ErrorCode, UxdResult};

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

// Return the current uncommitted base position for a given PerpAccount
pub fn uncommitted_perp_base_position(perp_account: &PerpAccount) -> i64 {
    perp_account
        .base_position
        .checked_add(perp_account.taker_base)
        .unwrap()
}

pub struct Order {
    // The quantity, in base_lot
    pub quantity: i64,
    // The price to place the order at, in quote (per base_lot)
    pub price: i64,
    // The resulting total amount that will be spent, in quote_lot (without fees)
    pub size: i64,
}

/// Walk through the book and find the best quantity and price to spend a given amount of quote.
pub fn get_best_order_for_quote_lot_amount<'a>(
    book: &Book<'a>,
    side: Side,
    quote_lot_amount_to_spend: i64,
) -> Option<Order> {
    let book_side = match side {
        Side::Bid => book.bids.iter(),
        Side::Ask => book.asks.iter(),
    };
    let mut cmlv_quantity: i64 = 0;
    let mut execution_price = 0; // Will update at each step, depending of how far it needs to go
    let mut quote_lot_left_to_spend = quote_lot_amount_to_spend;

    for order in book_side {
        // This order total value in quote lots
        let order_size = order.quantity.checked_mul(order.price()).unwrap();
        // How much base_lot we can fill for this order size
        let quantity_matched = {
            if quote_lot_left_to_spend < order_size {
                // we can finish the operation by purchasing this order partially
                // find out how much quantity that is in base lots
                quote_lot_left_to_spend.checked_div(order.price()).unwrap()
            } else {
                // we eat this order
                order.quantity
            }
        };
        // How much quote_lot were spent
        let spent = quantity_matched.checked_mul(order.price()).unwrap();
        if spent > 0 {
            // Current best execution price in quote_lot
            execution_price = order.price();
        }
        //
        cmlv_quantity = cmlv_quantity.checked_add(quantity_matched).unwrap();
        quote_lot_left_to_spend = quote_lot_left_to_spend.checked_sub(spent).unwrap();

        // when the amount left to spend is inferior to the price of a base lot, or if we are fully filled
        if quote_lot_left_to_spend == 0 || spent == 0 {
            // success
            let quote_lot_spent = quote_lot_amount_to_spend
                .checked_sub(quote_lot_left_to_spend)
                .unwrap();
            return Some(Order {
                quantity: cmlv_quantity,
                price: execution_price,
                size: quote_lot_spent,
            });
        }
    }
    None
}

/// Walk through the book and find the price and total amount spent to order a given quantity of base_lot.
pub fn get_best_order_for_base_lot_quantity<'a>(
    book: &Book<'a>,
    side: Side,
    base_lot_quantity_to_order: i64,
) -> Option<Order> {
    let book_side = match side {
        Side::Bid => book.bids.iter(),
        Side::Ask => book.asks.iter(),
    };
    let mut cmlv_quote_lot_amount_spent: i64 = 0;
    let mut execution_price = 0; // Will update at each step, depending of how far it needs to go
    let mut base_lot_quantity_left_to_order = base_lot_quantity_to_order;

    for order in book_side {
        // This current order size
        let order_size = order.quantity;
        // What's the value of this purchsase in quote_lot
        let spent = {
            if base_lot_quantity_left_to_order < order_size {
                // we can finish the operation by purchasing this order partially
                // find out how much we spend by doing so
                let spent = base_lot_quantity_left_to_order
                    .checked_mul(order.price())
                    .unwrap();
                base_lot_quantity_left_to_order = 0;
                spent
            } else {
                // we eat this order
                let spent = order_size.checked_mul(order.price()).unwrap();
                base_lot_quantity_left_to_order = base_lot_quantity_left_to_order
                    .checked_sub(order_size)
                    .unwrap();
                spent
            }
        };
        if spent > 0 {
            // Update the current execution price
            execution_price = order.price();
            // Update how much we spent so far
            cmlv_quote_lot_amount_spent = cmlv_quote_lot_amount_spent.checked_add(spent).unwrap();
        }
        // Check if we need to go deeper in the book or if we'r done
        if base_lot_quantity_left_to_order == 0 || spent == 0 {
            // success
            let base_lot_quantity = base_lot_quantity_to_order
                .checked_sub(base_lot_quantity_left_to_order)
                .unwrap();
            return Some(Order {
                quantity: base_lot_quantity,
                price: execution_price,
                size: cmlv_quote_lot_amount_spent,
            });
        }
    }
    None
}
