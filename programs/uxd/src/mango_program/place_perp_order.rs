use anchor_lang::prelude::AccountInfo;
use anchor_lang::prelude::AccountMeta;
use anchor_lang::prelude::Accounts;
use anchor_lang::prelude::CpiContext;
use anchor_lang::prelude::ProgramResult;
use mango::matching::OrderType;
use mango::matching::Side;
use mango::state::MAX_PAIRS;
use solana_program::instruction::Instruction;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use super::anchor_mango::check_program_account;

#[derive(Accounts)]
pub struct PlacePerpOrder<'info> {
    pub mango_group: AccountInfo<'info>,
    pub mango_account: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    pub mango_cache: AccountInfo<'info>,
    pub mango_perp_market: AccountInfo<'info>,
    pub mango_bids: AccountInfo<'info>,
    pub mango_asks: AccountInfo<'info>,
    pub mango_event_queue: AccountInfo<'info>,
}

/// Creates a `place_perp_order` instruction.
/// / Place an order on a perp market
/// Accounts expected by this instruction (8):
/// 0. `[]` mango_group_ai - MangoGroup
/// 1. `[writable]` mango_account_ai - the MangoAccount of owner
/// 2. `[signer]` owner_ai - owner of MangoAccount
/// 3. `[]` mango_cache_ai - MangoCache for this MangoGroup
/// 4. `[writable]` perp_market_ai
/// 5. `[writable]` bids_ai - bids account for this PerpMarket
/// 6. `[writable]` asks_ai - asks account for this PerpMarket
/// 7. `[writable]` event_queue_ai - EventQueue for this PerpMarket
fn place_perp_order_instruction(
    mango_program_id: &Pubkey,
    mango_group_pubkey: &Pubkey,
    mango_account_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    mango_cache_pubkey: &Pubkey,
    mango_perp_market_pubkey: &Pubkey,
    mango_bids_pubkey: &Pubkey,
    mango_asks_pubkey: &Pubkey,
    mango_event_queue_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    price: i64,
    quantity: i64,
    client_order_id: u64,
    side: Side,
    order_type: OrderType,
    reduce_only: bool,
) -> Result<Instruction, ProgramError> {
    check_program_account(mango_program_id)?;
    let data = mango::instruction::MangoInstruction::PlacePerpOrder {
        price,
        quantity,
        client_order_id,
        side,
        order_type,
        reduce_only,
    }
    .pack();

    // use a vec directly cause it seems to take some computing?
    // let mut accounts = Vec::with_capacity(8 + MAX_PAIRS + signer_pubkeys.len());
    let mut accounts = vec![
        AccountMeta::new_readonly(*mango_group_pubkey, false),
        AccountMeta::new(*mango_account_pubkey, false),
        AccountMeta::new_readonly(*owner_pubkey, signer_pubkeys.is_empty()),
        AccountMeta::new_readonly(*mango_cache_pubkey, false),
        AccountMeta::new(*mango_perp_market_pubkey, false),
        AccountMeta::new(*mango_bids_pubkey, false),
        AccountMeta::new(*mango_asks_pubkey, false),
        AccountMeta::new(*mango_event_queue_pubkey, false),
    ];
    accounts.extend(
        signer_pubkeys
            .iter()
            .map(|signer_pubkey| AccountMeta::new_readonly(**signer_pubkey, true)),
    );
    accounts.extend(
        [Pubkey::default(); MAX_PAIRS]
            .iter()
            .map(|default_open_order_pubkey| {
                AccountMeta::new_readonly(*default_open_order_pubkey, false)
            }),
    );
    Ok(Instruction {
        program_id: *mango_program_id,
        accounts,
        data,
    })
}

pub fn place_perp_order<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, PlacePerpOrder<'info>>,
    price: i64,
    quantity: i64,
    client_order_id: u64,
    side: Side,
    order_type: OrderType,
    reduce_only: bool,
) -> ProgramResult {
    let ix = place_perp_order_instruction(
        ctx.program.key,
        ctx.accounts.mango_group.key,
        ctx.accounts.mango_account.key,
        ctx.accounts.owner.key,
        ctx.accounts.mango_cache.key,
        ctx.accounts.mango_perp_market.key,
        ctx.accounts.mango_bids.key,
        ctx.accounts.mango_asks.key,
        ctx.accounts.mango_event_queue.key,
        &[ctx.accounts.owner.key],
        price,
        quantity,
        client_order_id,
        side,
        order_type,
        reduce_only,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.program.clone(),
            ctx.accounts.mango_group.clone(),
            ctx.accounts.mango_account.clone(),
            ctx.accounts.owner.clone(),
            ctx.accounts.mango_cache.clone(),
            ctx.accounts.mango_perp_market.clone(),
            ctx.accounts.mango_bids.clone(),
            ctx.accounts.mango_asks.clone(),
            ctx.accounts.mango_event_queue.clone(),
        ],
        ctx.signer_seeds,
    )
}
