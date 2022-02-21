use super::anchor_mango::check_program_account;
use anchor_lang::prelude::*;
use solana_program::instruction::Instruction;

use solana_program::pubkey::Pubkey;

#[derive(Accounts)]
pub struct InitMangoAccount<'info> {
    pub mango_group: AccountInfo<'info>,
    pub mango_account: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
}

/// Note: in our case a user is a `MangoDepository`.
///
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
) -> Result<Instruction> {
    check_program_account(mango_program_id)?;
    let data = mango::instruction::MangoInstruction::InitMangoAccount.pack();
    let accounts = vec![
        AccountMeta::new_readonly(*mango_group_pubkey, false),
        AccountMeta::new(*mango_account_pubkey, false),
        AccountMeta::new_readonly(*owner_pubkey, true),
        AccountMeta::new_readonly(*rent_sysvar, false),
    ];
    Ok(Instruction {
        program_id: *mango_program_id,
        accounts,
        data,
    })
}

pub fn initialize_mango_account<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, InitMangoAccount<'info>>,
) -> Result<()> {
    let ix = initialize_mango_account_instruction(
        ctx.program.key,
        ctx.accounts.mango_group.key,
        ctx.accounts.mango_account.key,
        ctx.accounts.owner.key,
        ctx.accounts.rent.key,
    )?;
    Ok(solana_program::program::invoke_signed(
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
    .map_err(|me| ProgramError::from(me))?)
}
