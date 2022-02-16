use crate::declare_check_assert_macros;
use crate::error::check_assert;
use crate::error::SourceFileId;
use crate::error::UxdError;
use crate::error::UxdErrorCode;
use crate::UxdResult;
use mango::matching::Book;
use mango::matching::Side;

declare_check_assert_macros!(SourceFileId::MangoUtilsOrder);

pub struct Order {
    // The quantity, in base_lot
    pub quantity: i64,
    // Marginal Price, the price to place the order at, in quote (per base_lot)
    pub price: i64,
    // The resulting total amount that will be spent, in quote_lot (without fees)
    pub size: i64,
    pub side: Side,
}

/// Walk through the book and find the best quantity and price to spend a given amount of quote.
pub fn get_best_order_for_quote_lot_amount<'a>(
    book: &Book<'a>,
    side: Side,
    quote_lot_amount_to_spend: i64,
) -> UxdResult<Option<Order>> {
    let book_side = match side {
        Side::Bid => book.bids.iter(),
        Side::Ask => book.asks.iter(),
    };
    let mut cmlv_quantity: i64 = 0;
    let mut execution_price = 0; // Will update at each step, depending of how far it needs to go
    let mut quote_lot_left_to_spend = quote_lot_amount_to_spend;

    for order in book_side {
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
        //
        cmlv_quantity = cmlv_quantity
            .checked_add(quantity_matched)
            .ok_or(math_err!())?;
        quote_lot_left_to_spend = quote_lot_left_to_spend
            .checked_sub(spent)
            .ok_or(math_err!())?;

        // when the amount left to spend is inferior to the price of a base lot, or if we are fully filled
        if quote_lot_left_to_spend == 0 || spent == 0 {
            // success
            let quote_lot_spent = quote_lot_amount_to_spend
                .checked_sub(quote_lot_left_to_spend)
                .ok_or(math_err!())?;
            // Side is the matched side, invert for order side
            let order_side = match side {
                Side::Bid => Side::Ask,
                Side::Ask => Side::Bid,
            };
            return Ok(Some(Order {
                quantity: cmlv_quantity,
                price: execution_price,
                size: quote_lot_spent,
                side: order_side,
            }));
        }
    }
    Ok(None)
}

/// Walk through the book and find the price and total amount spent to order a given quantity of base_lot.
pub fn get_best_order_for_base_lot_quantity<'a>(
    book: &Book<'a>,
    side: Side,
    base_lot_quantity_to_order: i64,
) -> UxdResult<Option<Order>> {
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
        // What's the value of this purchase in quote_lot
        let spent = {
            if base_lot_quantity_left_to_order < order_size {
                // we can finish the operation by purchasing this order partially
                // find out how much we spend by doing so
                let spent = base_lot_quantity_left_to_order
                    .checked_mul(order.price())
                    .ok_or(math_err!())?;
                base_lot_quantity_left_to_order = 0;
                spent
            } else {
                // we eat this order
                let spent = order_size.checked_mul(order.price()).ok_or(math_err!())?;
                base_lot_quantity_left_to_order = base_lot_quantity_left_to_order
                    .checked_sub(order_size)
                    .ok_or(math_err!())?;
                spent
            }
        };
        if spent > 0 {
            // Update the current execution price
            execution_price = order.price();
            // Update how much we spent so far
            cmlv_quote_lot_amount_spent = cmlv_quote_lot_amount_spent
                .checked_add(spent)
                .ok_or(math_err!())?;
        }
        // Check if we need to go deeper in the book or if we'r done
        if base_lot_quantity_left_to_order == 0 || spent == 0 {
            // success
            let base_lot_quantity = base_lot_quantity_to_order
                .checked_sub(base_lot_quantity_left_to_order)
                .ok_or(math_err!())?;
            return Ok(Some(Order {
                quantity: base_lot_quantity,
                price: execution_price,
                size: cmlv_quote_lot_amount_spent,
                side,
            }));
        }
    }
    Ok(None)
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
    )?;
    Ok(())
}

// Unit Test
#[cfg(test)]
mod tests {

    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_check_perp_order_fully_filled(order_quantity in i64::MIN..i64::MAX, pre_position in i64::MIN..i64::MAX, post_position in i64::MIN..i64::MAX) {
            let res = check_perp_order_fully_filled(order_quantity, pre_position, post_position);
            // MangoMarket.place_perp_order take quantity as i64
            let order_quantity: u64 = order_quantity.abs().try_into().unwrap();

            match res {
                Ok(()) => {
                    prop_assert_eq!(order_quantity, pre_position.abs_diff(post_position));
                }
                Err(error) => {
                    match error {
                         UxdError::ProgramError(_) => prop_assert!(false),
                         UxdError::UxdErrorCode { uxd_error_code, line: _, source_file_id } => {
                            prop_assert_eq!(source_file_id, SourceFileId::MangoUtilsOrder);
                            match uxd_error_code {
                                UxdErrorCode::PerpOrderPartiallyFilled => prop_assert_ne!(order_quantity, pre_position.abs_diff(post_position)),
                                UxdErrorCode::MathError => prop_assert!(true),
                                _default => prop_assert!(false)
                            };
                         },
                    }
                }
            };
        }
    }
}
