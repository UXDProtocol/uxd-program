Stuff from mango that might be useful

// MARK: - Withdraw ----------------------------------------------------------

    /**
    Withdraw funds that were deposited earlier.

    Accounts expected by this instruction (10):

    0. `[read]` mango_group_ai,   -
    1. `[write]` mango_account_ai, -
    2. `[read]` owner_ai,         -
    3. `[read]` mango_cache_ai,   -
    4. `[read]` root_bank_ai,     -
    5. `[write]` node_bank_ai,     -
    6. `[write]` vault_ai,         -
    7. `[write]` token_account_ai, -
    8. `[read]` signer_ai,        -
    9. `[read]` token_prog_ai,    -
    10. `[read]` clock_ai,         -
    11..+ `[]` open_orders_accs - open orders for each of the spot market

    Withdraw {
        quantity: u64,
        allow_borrow: bool,
    },
     */

    /**
        Settle all funds from serum dex open orders

        Accounts expected by this instruction (18):

        0. `[]` mango_group_ai - MangoGroup that this mango account is for
        1. `[]` mango_cache_ai - MangoCache for this MangoGroup
        2. `[signer]` owner_ai - MangoAccount owner
        3. `[writable]` mango_account_ai - MangoAccount
        4. `[]` dex_prog_ai - program id of serum dex
        5.  `[writable]` spot_market_ai - dex MarketState account
        6.  `[writable]` open_orders_ai - open orders for this market for this MangoAccount
        7. `[]` signer_ai - MangoGroup signer key
        8. `[writable]` dex_base_ai - base vault for dex MarketState
        9. `[writable]` dex_quote_ai - quote vault for dex MarketState
        10. `[]` base_root_bank_ai - MangoGroup base vault acc
        11. `[writable]` base_node_bank_ai - MangoGroup quote vault acc
        12. `[]` quote_root_bank_ai - MangoGroup quote vault acc
        13. `[writable]` quote_node_bank_ai - MangoGroup quote vault acc
        14. `[writable]` base_vault_ai - MangoGroup base vault acc
        15. `[writable]` quote_vault_ai - MangoGroup quote vault acc
        16. `[]` dex_signer_ai - dex Market signer account
        17. `[]` spl token program
    SettleFunds,
        */

// MARK: - Extras for later ---------------------------------------------------

    /*
    XXX This can be used to hack some additional profits.
    When PNL is positive, we can settle it and touch USDC interest on it, on top.
    Take two MangoAccounts and settle profits and losses between them for a perp market

     Accounts expected (6):
    SettlePnl {
        market_index: usize,
    },
    */

    // XXX We might generate some Mangos, better use them/redistribute
    /**
    Redeem the mngo_accrued in a PerpAccount for MNGO in MangoAccount deposits

    Accounts expected by this instruction (11):
    0. `[]` mango_group_ai - MangoGroup that this mango account is for
    1. `[]` mango_cache_ai - MangoCache
    2. `[writable]` mango_account_ai - MangoAccount
    3. `[signer]` owner_ai - MangoAccount owner
    4. `[]` perp_market_ai - PerpMarket
    5. `[writable]` mngo_perp_vault_ai
    6. `[]` mngo_root_bank_ai
    7. `[writable]` mngo_node_bank_ai
    8. `[writable]` mngo_bank_vault_ai
    9. `[]` signer_ai - Group Signer Account
    10. `[]` token_prog_ai - SPL Token program id
    RedeemMngo,
    */

    // XXX Investigate ExecutePerpTriggerOrder or AddPerpTriggerOrder
