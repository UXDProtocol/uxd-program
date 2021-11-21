# About Mango (Serum) lots size, native units, life and the universe

First some context:

A part is defined as BASE/QUOTE, base being the asset valued using quote
BASE and QUOTE are both SPL tokens, and have varying decimals.

`lot_size` are an abritrary amount, the minimum amount of `unit`, previously described, tradable
both QUOTE and BASE has a specific lot size, for BTC it's 10 and USDC it's 100.
`base_unit` and `quote_unit` are simply `10^respective_decimals`.
Meaning you cannot trade smaller chunks that 10 units.

So let's take BTC/USDC perp for instance :

BTC has 8 decimals
so 1BTC == to  100_000_000 BTC   native units (satoshis)

USDC has 6 decimals
so 1USDC == to   1_000_000 USDC  native units (tinycents idk)

Mango base lot size for BTC is 10 (arbitrary, probably from Serum)
That means that mango smallest amount for trades in BTC is 10 satoshis (0.00_000_010)
For USDC it's 100, meaning 0.00_0100

If you want to trade BTC with mango, you need to think in lot size,
 hence take your native units, and divide them by base_lot_size for that perp

I want to place a perp order long for 0.05 BTC :

First we calculate the quantity, that will be in [Base Lot]

- base_unit ==                  10 ** base_decimals         -> 100_000_000 (although it's 6 on solana iirc, for for the sake of this example doesn't matter)
- btc_amount ==                 0.05_000_000
- btc_amount_native_unit ==     btc_amount * base_unit      ->   5_000_000
- btc_amount_base_lot_unit ==   5_000_000 / base_lot_size   ->     500_000

Then we calculate the price, that will be in [Quote Lot]

 What we get from mango is the price of one `base unit` expressed in `quote units`
so for btc is how much quote unit for a satoshi

- perp_quote_price ==           mango_cache.price_cache[perp_market_index].price;

 Mango deal in lots (Serum actually), so you need to run some conversions

let base_lot_price_in_quote_unit = perp_price.checked_mul(base_lot_size)

let base_lot_order_quantity = order_amount_in_quote_unit.checked_div(base_lot_price_in_quote_unit)

let base_lot_price_in_quote_lot = base_lot_price_in_quote_unit.checked_div(quote_lot_size)

 === Now can call `place_perp_order(quantity: base_lot_order_quantity, price: base_lot_price_in_quote_lot);`

Let's say the order is filled 100%, then you bought

quantity_bought_in_btc_base_unit ==  perp_order.taker_base * base_lot_size
usdc spent                       ==  perp_order.taker_quote * quote_lot_size

And to that you can also calculate the fees
    let taker_fee = mango_group.perp_markets[perp_market_index].taker_fee;
then you do the calculation
