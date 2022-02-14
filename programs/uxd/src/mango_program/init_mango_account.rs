use anchor_lang::prelude::AccountInfo;
use anchor_lang::prelude::AccountMeta;
use anchor_lang::prelude::Accounts;
use anchor_lang::prelude::CpiContext;
use anchor_lang::prelude::ProgramResult;
use mango::state::MAX_PAIRS;
use solana_program::instruction::Instruction;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use super::anchor_mango::check_program_account;

#[derive(Accounts)]
pub struct InitMangoAccount<'info> {
    pub mango_group: AccountInfo<'info>,
    pub mango_account: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
}

/// Initialize a mango account for a user
///
/// Accounts expected by this instruction (4):
///
/// 0. `[]` mango_group_ai - MangoGroup that this mango account is for
/// 1. `[writable]` mango_account_ai - the mango account data
/// 2. `[signer]` owner_ai - Solana account of owner of the mango account
/// 3. `[]` rent_ai - Rent sysvar account
fn initialize_mango_account_instruction(
    mango_program_id: &Pubkey,
    mango_group_pubkey: &Pubkey,
    mango_account_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    rent_sysvar: &Pubkey,
) -> Result<Instruction, ProgramError> {
    check_program_account(mango_program_id)?;
    let data = mango::instruction::MangoInstruction::InitMangoAccount.pack();

    let mut accounts = Vec::with_capacity(8 + MAX_PAIRS);
    accounts.push(AccountMeta::new_readonly(*mango_group_pubkey, false));
    accounts.push(AccountMeta::new(*mango_account_pubkey, false));
    accounts.push(AccountMeta::new_readonly(*owner_pubkey, true));
    accounts.push(AccountMeta::new_readonly(*rent_sysvar, false));
    Ok(Instruction {
        program_id: *mango_program_id,
        accounts,
        data,
    })
}

pub fn initialize_mango_account<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, InitMangoAccount<'info>>,
) -> ProgramResult {
    let ix = initialize_mango_account_instruction(
        ctx.program.key,
        ctx.accounts.mango_group.key,
        ctx.accounts.mango_account.key,
        ctx.accounts.owner.key,
        ctx.accounts.rent.key,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.program.clone(),
            ctx.accounts.mango_group.clone(),
            ctx.accounts.mango_account.clone(),
            ctx.accounts.owner.clone(),
            ctx.accounts.rent.clone(),
        ],
        ctx.signer_seeds,
    )
}
