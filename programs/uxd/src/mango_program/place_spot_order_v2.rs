// use std::num::NonZeroU64;

// use anchor_lang::prelude::{AccountInfo, AccountMeta, Accounts, ProgramResult};
// use anchor_lang::CpiContext;
// use mango::state::MAX_PAIRS;
// use serum_dex::instruction::{NewOrderInstructionV3, SelfTradeBehavior};
// use serum_dex::matching::{OrderType, Side};
// use solana_program::instruction::Instruction;
// use solana_program::program_error::ProgramError;
// use solana_program::pubkey::Pubkey;

// use super::anchor_mango::check_program_account;

// #[derive(Accounts)]
// pub struct PlaceSpotOrderV2<'info> {
//     pub mango_group: AccountInfo<'info>,       // read
//     pub mango_account: AccountInfo<'info>,     // write
//     pub owner: AccountInfo<'info>,             // read & signer
//     pub mango_cache: AccountInfo<'info>,       // read
//     pub dex_prog: AccountInfo<'info>,          // read
//     pub spot_market: AccountInfo<'info>,       // write
//     pub bids: AccountInfo<'info>,              // write
//     pub asks: AccountInfo<'info>,              // write
//     pub dex_request_queue: AccountInfo<'info>, // write
//     pub dex_event_queue: AccountInfo<'info>,   // write
//     pub dex_base: AccountInfo<'info>,          // write
//     pub dex_quote: AccountInfo<'info>,         // write
//     pub base_root_bank: AccountInfo<'info>,    // read
//     pub base_node_bank: AccountInfo<'info>,    // write
//     pub base_vault: AccountInfo<'info>,        // write
//     pub quote_root_bank: AccountInfo<'info>,   // read
//     pub quote_node_bank: AccountInfo<'info>,   // write
//     pub quote_vault: AccountInfo<'info>,       // write
//     pub token_prog: AccountInfo<'info>,        // read
//     pub signer: AccountInfo<'info>,            // read
//     pub dex_signer: AccountInfo<'info>,        // read
//     pub msrm_or_srm_vault: AccountInfo<'info>, // read
// }

// /// Place an order on the Serum Dex using Mango account. Improved over PlaceSpotOrder
// /// by reducing the tx size
// ///
// /// Accounts expected by this instruction (22 + MAX_PAIRS):
// /// 0. `[]` mango_group_ai - MangoGroup
// /// 1. `[writable]` mango_account_ai - the MangoAccount of owner
// /// 2. `[signer]` owner_ai - owner of MangoAccount
// /// 3. `[]` mango_cache_ai - MangoCache for this MangoGroup
// /// 4. `[]` dex_prog_ai - serum dex program id
// /// 5. `[writable]` spot_market_ai - serum dex MarketState account
// /// 6. `[writable]` bids_ai - bids account for serum dex market
// /// 7. `[writable]` asks_ai - asks account for serum dex market
// /// 8. `[writable]` dex_request_queue_ai - request queue for serum dex market
// /// 9. `[writable]` dex_event_queue_ai - event queue for serum dex market
// /// 10. `[writable]` dex_base_ai - base currency serum dex market vault
// /// 11. `[writable]` dex_quote_ai - quote currency serum dex market vault
// /// 12. `[]` base_root_bank_ai - root bank of base currency
// /// 13. `[writable]` base_node_bank_ai - node bank of base currency
// /// 14. `[writable]` base_vault_ai - vault of the basenode bank
// /// 15. `[]` quote_root_bank_ai - root bank of quote currency
// /// 16. `[writable]` quote_node_bank_ai - node bank of quote currency
// /// 17. `[writable]` quote_vault_ai - vault of the quote node bank
// /// 18. `[]` token_prog_ai - SPL token program id
// /// 19. `[]` signer_ai - signer key for this MangoGroup
// /// 20. `[]` dex_signer_key - signer for serum dex
// /// 21. `[]` msrm_or_srm_vault_ai - the msrm or srm vault in this MangoGroup. Can be zero key
// /// 22+ `[writable]` open_orders_ais - An array of MAX_PAIRS. Only OpenOrders of current market
// ///         index needs to be writable. Only OpenOrders in_margin_basket needs to be correct;
// ///         remaining open orders can just be Pubkey::default() (the zero key)
// fn place_spot_order_v2_instruction(
//     mango_program_id: &Pubkey,
//     mango_group: &Pubkey,
//     mango_account: &Pubkey,
//     owner: &Pubkey,
//     mango_cache: &Pubkey,
//     dex_prog: &Pubkey,
//     spot_market: &Pubkey,
//     bids: &Pubkey,
//     asks: &Pubkey,
//     dex_request_queue: &Pubkey,
//     dex_event_queue: &Pubkey,
//     dex_base: &Pubkey,
//     dex_quote: &Pubkey,
//     base_root_bank: &Pubkey,
//     base_node_bank: &Pubkey,
//     base_vault: &Pubkey,
//     quote_root_bank: &Pubkey,
//     quote_node_bank: &Pubkey,
//     quote_vault: &Pubkey,
//     token_prog: &Pubkey,
//     signer: &Pubkey,
//     dex_signer: &Pubkey,
//     msrm_or_srm_vault: &Pubkey,
//     // open_orders_current_market: &[&Pubkey], // In our case we don't have open orders, only FoK - change for open sourcing
//     // open_orders_in_margin_basket: &[&Pubkey],
//     order: serum_dex::instruction::NewOrderInstructionV3,
// ) -> Result<Instruction, ProgramError> {
//     check_program_account(mango_program_id)?;
//     let data = mango::instruction::MangoInstruction::PlaceSpotOrder2 { order }.pack();

//     // use a vec directly cause it seems to take some computing?
//     // let mut accounts = Vec::with_capacity(8 + MAX_PAIRS + signer_pubkeys.len());
//     let mut accounts = vec![
//         AccountMeta::new_readonly(*mango_group, false),
//         AccountMeta::new(*mango_account, false),
//         AccountMeta::new_readonly(*owner, true),
//         AccountMeta::new_readonly(*mango_cache, false),
//         AccountMeta::new_readonly(*dex_prog, false),
//         AccountMeta::new(*spot_market, false),
//         AccountMeta::new(*bids, false),
//         AccountMeta::new(*asks, false),
//         AccountMeta::new(*dex_request_queue, false),
//         AccountMeta::new(*dex_event_queue, false),
//         AccountMeta::new(*dex_base, false),
//         AccountMeta::new(*dex_quote, false),
//         AccountMeta::new_readonly(*base_root_bank, false),
//         AccountMeta::new(*base_node_bank, false),
//         AccountMeta::new(*base_vault, false),
//         AccountMeta::new_readonly(*quote_root_bank, false),
//         AccountMeta::new(*quote_node_bank, false),
//         AccountMeta::new(*quote_vault, false),
//         AccountMeta::new_readonly(*token_prog, false),
//         AccountMeta::new_readonly(*signer, false),
//         AccountMeta::new_readonly(*dex_signer, false),
//         AccountMeta::new_readonly(*msrm_or_srm_vault, false),
//     ];
//     accounts.extend(
//         [Pubkey::default(); MAX_PAIRS]
//             .iter()
//             .map(|default_open_order_pubkey| {
//                 AccountMeta::new_readonly(*default_open_order_pubkey, false)
//             }),
//     );
//     Ok(Instruction {
//         program_id: *mango_program_id,
//         accounts,
//         data,
//     })
// }

// pub fn place_spot_order_v2<'info>(
//     ctx: CpiContext<'_, '_, '_, 'info, PlaceSpotOrderV2<'info>>,
//     side: Side,
//     limit_price: NonZeroU64,
//     max_coin_qty: NonZeroU64,
//     max_native_pc_qty_including_fees: NonZeroU64,
//     self_trade_behavior: SelfTradeBehavior,
//     order_type: OrderType,
//     client_order_id: u64,
//     limit: u16,
// ) -> ProgramResult {
//     let order = NewOrderInstructionV3 {
//         side,
//         limit_price,
//         max_coin_qty,
//         max_native_pc_qty_including_fees,
//         self_trade_behavior,
//         order_type,
//         client_order_id,
//         limit,
//     };
//     let ix = place_spot_order_v2_instruction(
//         ctx.program.key,
//         ctx.accounts.mango_group.key,
//         ctx.accounts.mango_account.key,
//         ctx.accounts.owner.key,
//         ctx.accounts.mango_cache.key,
//         ctx.accounts.dex_prog.key,
//         ctx.accounts.spot_market.key,
//         ctx.accounts.bids.key,
//         ctx.accounts.asks.key,
//         ctx.accounts.dex_request_queue.key,
//         ctx.accounts.dex_event_queue.key,
//         ctx.accounts.dex_base.key,
//         ctx.accounts.dex_quote.key,
//         ctx.accounts.base_root_bank.key,
//         ctx.accounts.base_node_bank.key,
//         ctx.accounts.base_vault.key,
//         ctx.accounts.quote_root_bank.key,
//         ctx.accounts.quote_node_bank.key,
//         ctx.accounts.quote_vault.key,
//         ctx.accounts.token_prog.key,
//         ctx.accounts.signer.key,
//         ctx.accounts.dex_signer.key,
//         ctx.accounts.msrm_or_srm_vault.key,
//         order,
//     )?;
//     solana_program::program::invoke_signed(
//         &ix,
//         &[
//             ctx.program.clone(),
//             ctx.accounts.mango_group.clone(),
//             ctx.accounts.mango_account.clone(),
//             ctx.accounts.owner.clone(),
//             ctx.accounts.mango_cache.clone(),
//             ctx.accounts.dex_prog.clone(),
//             ctx.accounts.spot_market.clone(),
//             ctx.accounts.bids.clone(),
//             ctx.accounts.asks.clone(),
//             ctx.accounts.dex_request_queue.clone(),
//             ctx.accounts.dex_event_queue.clone(),
//             ctx.accounts.dex_base.clone(),
//             ctx.accounts.dex_quote.clone(),
//             ctx.accounts.base_root_bank.clone(),
//             ctx.accounts.base_node_bank.clone(),
//             ctx.accounts.base_vault.clone(),
//             ctx.accounts.quote_root_bank.clone(),
//             ctx.accounts.quote_node_bank.clone(),
//             ctx.accounts.quote_vault.clone(),
//             ctx.accounts.token_prog.clone(),
//             ctx.accounts.signer.clone(),
//             ctx.accounts.dex_signer.clone(),
//             ctx.accounts.msrm_or_srm_vault.clone(),
//         ],
//         ctx.signer_seeds,
//     )
// }
