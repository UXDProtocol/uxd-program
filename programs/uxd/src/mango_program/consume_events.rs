// use anchor_lang::prelude::{AccountInfo, AccountMeta, Accounts, ProgramResult};
// use anchor_lang::CpiContext;
// use solana_program::instruction::Instruction;
// use solana_program::program_error::ProgramError;
// use solana_program::pubkey::Pubkey;

// use super::anchor_mango::check_program_account;

// #[derive(Accounts)]
// pub struct ConsumeEvents<'info> {
//     pub mango_group: AccountInfo<'info>,
//     pub mango_cache: AccountInfo<'info>,
//     pub perp_market: AccountInfo<'info>,
//     pub event_queue: AccountInfo<'info>,
//     pub mango_account: AccountInfo<'info>,
// }

// /// Consume events from the EventQueue for a user
// /// <!> The real IX takes an array of accounts but we only use one in our case
// fn consume_events_instruction(
//     mango_program_id: &Pubkey,
//     mango_group_pk: &Pubkey,
//     mango_cache_pk: &Pubkey,
//     perp_market_pk: &Pubkey,
//     event_queue_pk: &Pubkey,
//     mango_acc_pk: &Pubkey,
//     limit: usize,
// ) -> Result<Instruction, ProgramError> {
//     check_program_account(mango_program_id)?;
//     let data = mango::instruction::MangoInstruction::ConsumeEvents { limit }.pack();

//     let mut accounts = Vec::with_capacity(5);
//     accounts.push(AccountMeta::new_readonly(*mango_group_pk, false));
//     accounts.push(AccountMeta::new_readonly(*mango_cache_pk, false));
//     accounts.push(AccountMeta::new(*perp_market_pk, false));
//     accounts.push(AccountMeta::new(*event_queue_pk, false));
//     accounts.push(AccountMeta::new(*mango_acc_pk, false));
//     Ok(Instruction {
//         program_id: *mango_program_id,
//         accounts,
//         data,
//     })
// }

// pub fn consume_events<'info>(
//     ctx: CpiContext<'_, '_, '_, 'info, ConsumeEvents<'info>>,
//     limit: usize,
// ) -> ProgramResult {
//     let ix = consume_events_instruction(
//         ctx.program.key,
//         ctx.accounts.mango_group.key,
//         ctx.accounts.mango_cache.key,
//         ctx.accounts.perp_market.key,
//         ctx.accounts.event_queue.key,
//         ctx.accounts.mango_account.key,
//         limit,
//     )?;
//     solana_program::program::invoke(
//         &ix,
//         &[
//             ctx.program.clone(),
//             ctx.accounts.mango_group.clone(),
//             ctx.accounts.mango_cache.clone(),
//             ctx.accounts.perp_market.clone(),
//             ctx.accounts.event_queue.clone(),
//             ctx.accounts.mango_account.clone(),
//         ],
//     )
// }
