use anchor_lang::prelude::AccountInfo;
use anchor_lang::prelude::AccountMeta;
use anchor_lang::prelude::Accounts;
use anchor_lang::prelude::CpiContext;
use anchor_lang::prelude::ProgramResult;
use mango::matching::OrderType;
use mango::matching::Side;
use solana_program::instruction::Instruction;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use super::anchor_mango::check_program_account;

#[derive(Accounts)]
pub struct PlacePerpOrderV2<'info> {
    pub mango_group: AccountInfo<'info>,
    pub mango_account: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    pub mango_cache: AccountInfo<'info>,
    pub mango_perp_market: AccountInfo<'info>,
    pub mango_bids: AccountInfo<'info>,
    pub mango_asks: AccountInfo<'info>,
    pub mango_event_queue: AccountInfo<'info>,
}

fn place_perp_order_v2_instruction(
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

    limit_price: i64,
    max_base_quantity: i64,
    max_quote_quantity: i64,

    client_order_id: u64,
    side: Side,
    order_type: OrderType,
    reduce_only: bool,
) -> Result<Instruction, ProgramError> {
    check_program_account(mango_program_id)?;
    let data = mango::instruction::MangoInstruction::PlacePerpOrderV2 {
        limit_price,
        max_base_quantity,
        max_quote_quantity,
        client_order_id,
        side,
        order_type,
        reduce_only,
    }
    .pack();
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
    // accounts.extend(
    //     [Pubkey::default(); MAX_PAIRS]
    //         .iter()
    //         .map(|default_open_order_pubkey| {
    //             AccountMeta::new_readonly(*default_open_order_pubkey, false)
    //         }),
    // );
    Ok(Instruction {
        program_id: *mango_program_id,
        accounts,
        data,
    })
}

pub fn place_perp_order_v2<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, PlacePerpOrderV2<'info>>,
    limit_price: i64,
    max_base_quantity: i64,
    max_quote_quantity: i64,
    client_order_id: u64,
    side: Side,
    order_type: OrderType,
    reduce_only: bool,
) -> ProgramResult {
    let ix = place_perp_order_v2_instruction(
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
        limit_price,
        max_base_quantity,
        max_quote_quantity,
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
