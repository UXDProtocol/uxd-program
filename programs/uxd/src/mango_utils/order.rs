use mango::matching::Book;
use mango::matching::Side;

use crate::ErrorCode;
use crate::UxdResult;

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
            // Side is the matched side, invert for order side
            let order_side = match side {
                Side::Bid => Side::Ask,
                Side::Ask => Side::Bid,
            };
            return Some(Order {
                quantity: cmlv_quantity,
                price: execution_price,
                size: quote_lot_spent,
                side: order_side,
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
        // What's the value of this purchase in quote_lot
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
                side,
            });
        }
    }
    None
}

// Verify that the order quantity matches the base position delta
pub fn check_short_perp_order_fully_filled(
    order_quantity: i64,
    pre_position: i64,
    post_position: i64,
) -> UxdResult {
    let filled_amount = (post_position.checked_sub(pre_position).unwrap())
        .checked_abs()
        .unwrap();
    if !(order_quantity == filled_amount) {
        return Err(ErrorCode::PerpOrderPartiallyFilled);
    }
    Ok(())
}
