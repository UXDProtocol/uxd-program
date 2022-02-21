use anchor_lang::prelude::AccountInfo;
use anchor_lang::prelude::AccountMeta;
use anchor_lang::prelude::Accounts;
use anchor_lang::prelude::CpiContext;
use solana_program::instruction::Instruction;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use super::anchor_mango::check_program_account;

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
/// Accounts expected by this instruction (9):
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
#[allow(clippy::too_many_arguments)]
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
    quantity: u64,
) -> Result<Instruction, ProgramError> {
    check_program_account(mango_program_id)?;
    let data = mango::instruction::MangoInstruction::Deposit { quantity }.pack();
    let accounts = vec![
        AccountMeta::new_readonly(*mango_group_pubkey, false),
        AccountMeta::new(*mango_account_pubkey, false),
        AccountMeta::new_readonly(*owner_pubkey, true),
        AccountMeta::new_readonly(*mango_cache_pubkey, false),
        AccountMeta::new_readonly(*mango_root_bank_pubkey, false),
        AccountMeta::new(*mango_node_bank_pubkey, false),
        AccountMeta::new(*mango_vault_pubkey, false),
        AccountMeta::new_readonly(*token_program_id, false),
        AccountMeta::new(*owner_token_account_pubkey, false),
    ];
    Ok(Instruction {
        program_id: *mango_program_id,
        accounts,
        data,
    })
}

pub fn deposit<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Deposit<'info>>,
    quantity: u64,
) -> Result<()> {
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
