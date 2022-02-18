use crate::declare_check_assert_macros;
use crate::error::check_assert;
use crate::error::SourceFileId;
use crate::error::UxdError;
use crate::error::UxdErrorCode;
use crate::UxdResult;
use mango::matching::BookSide;
use mango::matching::Side;
use std::cell::RefMut;

declare_check_assert_macros!(SourceFileId::MangoUtilsOrder);

/// This logic will eventually be deported back to MangoMarket repository.
/// 02/18/2022 - Spoke with Ckamm from mango market, they will implement this in the coming week.

pub struct Order {
    // The quantity, in base_lot
    pub quantity: i64,
    // Marginal Price, the price to place the order at, in quote (per base_lot)
    pub price: i64,
    pub taker_side: Side,
}

/// Walk through the maker side of the book and find the best quantity and price to spend a given amount of quote.
pub fn get_best_order_for_quote_lot_amount<'a>(
    book_side: RefMut<'a, BookSide>,
    taker_side: Side,
    quote_lot_amount_to_spend: i64,
) -> UxdResult<Order> {
    let mut cmlv_quantity: i64 = 0;
    let mut execution_price = 0; // Will update at each step, depending of how far it needs to go
    let mut quote_lot_left_to_spend = quote_lot_amount_to_spend;

    for order in book_side.iter() {
        // This order total value in quote lots
        let order_size = order
            .quantity
            .checked_mul(order.price())
            .ok_or(math_err!())?;
        // How much base_lot we can fill for this order size
        let quantity_matched = {
            if quote_lot_left_to_spend < order_size {
                // we can finish the operation by purchasing this order partially
                // find out how much quantity that is in base lots
                quote_lot_left_to_spend
                    .checked_div(order.price())
                    .ok_or(math_err!())?
            } else {
                // we eat this order
                order.quantity
            }
        };
        // How much quote_lot were spent
        let spent = quantity_matched
            .checked_mul(order.price())
            .ok_or(math_err!())?;
        if spent > 0 {
            // Current best execution price in quote_lot
            execution_price = order.price();
        }
        cmlv_quantity = cmlv_quantity
            .checked_add(quantity_matched)
            .ok_or(math_err!())?;
        quote_lot_left_to_spend = quote_lot_left_to_spend
            .checked_sub(spent)
            .ok_or(math_err!())?;

        // when the amount left to spend is inferior to the price of a base lot, or if we are fully filled
        if quote_lot_left_to_spend == 0 || spent == 0 {
            // failure
            if cmlv_quantity == 0 {
                return Err(throw_err!(UxdErrorCode::OrderSizeBelowMinLotSize));
            }
            // success
            return Ok(Order {
                quantity: cmlv_quantity,
                price: execution_price,
                taker_side,
            });
        }
    }
    Err(throw_err!(UxdErrorCode::InsufficientOrderBookDepth))
}

// Verify that the order quantity matches the base position delta
pub fn check_perp_order_fully_filled(
    order_quantity: i64,
    pre_position: i64,
    post_position: i64,
) -> UxdResult {
    let filled_amount = (post_position.checked_sub(pre_position).ok_or(math_err!())?)
        .checked_abs()
        .ok_or(math_err!())?;
    check_eq!(
        order_quantity,
        filled_amount,
        UxdErrorCode::PerpOrderPartiallyFilled
    )
}
