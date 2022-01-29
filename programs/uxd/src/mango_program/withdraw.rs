use super::anchor_mango::check_program_account;
use anchor_lang::prelude::AccountInfo;
use anchor_lang::prelude::AccountMeta;
use anchor_lang::prelude::Accounts;
use anchor_lang::prelude::CpiContext;
use anchor_lang::prelude::ProgramResult;
use mango::state::MAX_PAIRS;
use solana_program::instruction::Instruction;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub mango_group: AccountInfo<'info>,
    pub mango_account: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    pub mango_cache: AccountInfo<'info>,
    pub mango_root_bank: AccountInfo<'info>,
    pub mango_node_bank: AccountInfo<'info>,
    pub mango_vault: AccountInfo<'info>,
    pub token_account: AccountInfo<'info>,
    pub mango_signer: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

/// Withdraw funds that were deposited earlier.
///
/// Accounts expected by this instruction (10):
///
/// 0. `[read]` mango_group_ai,   -
/// 1. `[write]` mango_account_ai, -
/// 2. `[read]` owner_ai,         -
/// 3. `[read]` mango_cache_ai,   -
/// 4. `[read]` root_bank_ai,     -
/// 5. `[write]` node_bank_ai,     -
/// 6. `[write]` vault_ai,         -
/// 7. `[write]` token_account_ai, -
/// 8. `[read]` signer_ai,        -
/// 9. `[read]` token_prog_ai,    -
/// 10. `[read]` clock_ai,         -
/// 11..+ `[]` open_orders_accs - open orders for each of the spot market
fn withdraw_instruction(
    mango_program_id: &Pubkey,
    mango_group_pubkey: &Pubkey,
    mango_account_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    mango_cache_pubkey: &Pubkey,
    mango_root_bank_pubkey: &Pubkey,
    mango_node_bank_pubkey: &Pubkey,
    mango_vault_pubkey: &Pubkey,
    token_account_pubkey: &Pubkey,
    mango_signer_pubkey: &Pubkey,
    token_program_id: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    quantity: u64,
    allow_borrow: bool,
) -> Result<Instruction, ProgramError> {
    check_program_account(mango_program_id)?;
    let data = mango::instruction::MangoInstruction::Withdraw {
        quantity,
        allow_borrow,
    }
    .pack();

    let mut accounts = vec![
        AccountMeta::new_readonly(*mango_group_pubkey, false),
        AccountMeta::new(*mango_account_pubkey, false),
        AccountMeta::new_readonly(*owner_pubkey, false),
        AccountMeta::new_readonly(*mango_cache_pubkey, false),
        AccountMeta::new_readonly(*mango_root_bank_pubkey, false),
        AccountMeta::new(*mango_node_bank_pubkey, false),
        AccountMeta::new(*mango_vault_pubkey, false),
        AccountMeta::new(*token_account_pubkey, false),
        AccountMeta::new_readonly(*mango_signer_pubkey, false),
        AccountMeta::new_readonly(*token_program_id, false),
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

pub fn withdraw<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Withdraw<'info>>,
    quantity: u64,
    allow_borrow: bool,
) -> ProgramResult {
    let ix = withdraw_instruction(
        ctx.program.key,
        ctx.accounts.mango_group.key,
        ctx.accounts.mango_account.key,
        ctx.accounts.owner.key,
        ctx.accounts.mango_cache.key,
        ctx.accounts.mango_root_bank.key,
        ctx.accounts.mango_node_bank.key,
        ctx.accounts.mango_vault.key,
        ctx.accounts.token_account.key,
        ctx.accounts.mango_signer.key,
        ctx.accounts.token_program.key,
        &[ctx.accounts.owner.key],
        quantity,
        allow_borrow,
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
            ctx.accounts.token_account.clone(),
            ctx.accounts.mango_signer.clone(),
            ctx.accounts.token_program.clone(),
        ],
        ctx.signer_seeds,
    )
}
