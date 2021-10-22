use anchor_lang::prelude::{AccountInfo, AccountMeta, Accounts, ProgramResult};
use anchor_lang::CpiContext;
use mango::matching::{OrderType, Side};
use mango::state::MAX_PAIRS;
use solana_program::instruction::Instruction;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

// Temporary, the one I opened PR for when merged https://github.com/blockworks-foundation/mango-v3/pull/67
#[derive(Clone)]
pub struct Mango;

// if the mango program use declare_id we can get ride of that
const MANGO_ID: &str = "4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA";

impl anchor_lang::AccountDeserialize for Mango {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self, ProgramError> {
        Mango::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(_buf: &mut &[u8]) -> Result<Self, ProgramError> {
        Ok(Mango)
    }
}

impl anchor_lang::Id for Mango {
    fn id() -> Pubkey {
        return Pubkey::from_str(MANGO_ID).unwrap();
    }
}

// Anchorization for Mango

/// Checks that the supplied program ID is the correct one
pub fn check_program_account(mango_program_id: &Pubkey) -> ProgramResult {
    if mango_program_id != &Pubkey::from_str(MANGO_ID).unwrap() {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}

// MARK: -  PlacePerpOrder ----------------------------------------------------
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

    let mut accounts = Vec::with_capacity(8 + MAX_PAIRS + signer_pubkeys.len());
    accounts.push(AccountMeta::new_readonly(*mango_group_pubkey, false));
    accounts.push(AccountMeta::new(*mango_account_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    accounts.push(AccountMeta::new_readonly(*mango_cache_pubkey, false));
    accounts.push(AccountMeta::new(*mango_perp_market_pubkey, false));
    accounts.push(AccountMeta::new(*mango_bids_pubkey, false));
    accounts.push(AccountMeta::new(*mango_asks_pubkey, false));
    accounts.push(AccountMeta::new(*mango_event_queue_pubkey, false));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    }
    // Default fill - We never have ongoing order, only immediates
    [Pubkey::default(); MAX_PAIRS]
        .iter()
        .for_each(|ai| accounts.push(AccountMeta::new_readonly(*ai, false)));

    Ok(Instruction {
        program_id: *mango_program_id,
        accounts,
        data,
    })
}

pub fn place_perp_order<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, PlacePerpOrder<'info>>,
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

// MARK: -  Deposit -----------------------------------------------------------
#[derive(Accounts)]
pub struct Deposit<'info> {
    pub mango_group: AccountInfo<'info>,
    pub mango_account: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    pub mango_cache: AccountInfo<'info>,
    pub mango_root_bank: AccountInfo<'info>,
    pub mango_node_bank: AccountInfo<'info>,
    pub mango_vault: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub owner_token_account: AccountInfo<'info>,
}

/// Deposit funds into mango account
///
/// Accounts expected by this instruction (8):
///
/// 0. `[]` mango_group_ai - MangoGroup that this mango account is for
/// 1. `[writable]` mango_account_ai - the mango account for this user
/// 2. `[signer]` owner_ai - Solana account of owner of the mango account
/// 3. `[]` mango_cache_ai - MangoCache
/// 4. `[]` root_bank_ai - RootBank owned by MangoGroup
/// 5. `[writable]` node_bank_ai - NodeBank owned by RootBank
/// 6. `[writable]` vault_ai - TokenAccount owned by MangoGroup
/// 7. `[]` token_prog_ai - acc pointed to by SPL token program id
/// 8. `[writable]` owner_token_account_ai - TokenAccount owned by user which will be sending the funds
fn deposit_instruction(
    mango_program_id: &Pubkey,
    mango_group_pubkey: &Pubkey,
    mango_account_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    mango_cache_pubkey: &Pubkey,
    mango_root_bank_pubkey: &Pubkey,
    mango_node_bank_pubkey: &Pubkey,
    mango_vault_pubkey: &Pubkey,
    token_program_id: &Pubkey,
    owner_token_account_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    quantity: u64,
) -> Result<Instruction, ProgramError> {
    check_program_account(mango_program_id)?;
    let data = mango::instruction::MangoInstruction::Deposit { quantity }.pack();

    let mut accounts = Vec::with_capacity(8 + MAX_PAIRS + signer_pubkeys.len());
    accounts.push(AccountMeta::new_readonly(*mango_group_pubkey, false));
    accounts.push(AccountMeta::new(*mango_account_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    accounts.push(AccountMeta::new_readonly(*mango_cache_pubkey, false));
    accounts.push(AccountMeta::new_readonly(*mango_root_bank_pubkey, false));
    accounts.push(AccountMeta::new(*mango_node_bank_pubkey, false));
    accounts.push(AccountMeta::new(*mango_vault_pubkey, false));
    accounts.push(AccountMeta::new_readonly(*token_program_id, false));
    accounts.push(AccountMeta::new(*owner_token_account_pubkey, false));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *mango_program_id,
        accounts,
        data,
    })
}

pub fn deposit<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Deposit<'info>>,
    quantity: u64,
) -> ProgramResult {
    let ix = deposit_instruction(
        ctx.program.key,
        ctx.accounts.mango_group.key,
        ctx.accounts.mango_account.key,
        ctx.accounts.owner.key,
        ctx.accounts.mango_cache.key,
        ctx.accounts.mango_root_bank.key,
        ctx.accounts.mango_node_bank.key,
        ctx.accounts.mango_vault.key,
        ctx.accounts.token_program.key,
        ctx.accounts.owner_token_account.key,
        &[ctx.accounts.owner.key],
        quantity,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.program.clone(),
            ctx.accounts.mango_group.clone(),
            ctx.accounts.mango_account.clone(),
            ctx.accounts.owner.clone(),
            ctx.accounts.mango_cache.clone(),
            ctx.accounts.mango_root_bank.clone(),
            ctx.accounts.mango_node_bank.clone(),
            ctx.accounts.mango_vault.clone(),
            ctx.accounts.token_program.clone(),
            ctx.accounts.owner_token_account.clone(),
        ],
        ctx.signer_seeds,
    )
}
