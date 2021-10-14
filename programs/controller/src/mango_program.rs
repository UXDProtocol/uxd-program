use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

// Temporary, the one I opened PR for when merged https://github.com/blockworks-foundation/mango-v3/pull/67
#[derive(Clone)]
pub struct Mango;

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
        return Pubkey::from_str("4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA").unwrap();
    }
}

///////////
/// Might be usefull for other calls
/// Thing is this PR is light but I tried many differents things, up to rewritting part of mango.. to make it more anchor compatible.
/// But it doesn't matter, let's just start raw and improve later.

// pub fn initialize_mango_account<'a, 'b, 'c, 'info>(
//     ctx: CpiContext<'a, 'b, 'c, 'info, InitializeMangoAccount<'info>>,
// ) -> ProgramResult {
//     let ix = mango::instruction::init_mango_account(
//         ctx.program.key,
//         ctx.accounts.group.key,
//         ctx.accounts.account.key,
//         ctx.accounts.owner.key,
//     )?;
//     solana_program::program::invoke(
//         &ix,
//         &[
//             ctx.accounts.group.to_account_info(),
//             ctx.accounts.account.clone(),
//             ctx.accounts.owner.clone(),
//             ctx.accounts.rent.to_account_info(),
//             ctx.program.clone(),
//         ],
//     )
// }

// // Accounts expected by this instruction (4):
// //
// // 0. `[]` mango_group_ai - MangoGroup that this mango account is for
// // 1. `[writable]` mango_account_ai - the mango account data
// // 2. `[signer]` owner_ai - Solana account of owner of the mango account
// // 3. `[]` rent_ai - Rent sysvar account
// #[derive(Accounts)]
// pub struct InitializeMangoAccount<'info> {
//     group: AccountInfo<'info>,
//     account: AccountInfo<'info>,
//     owner: AccountInfo<'info>,
//     rent: AccountInfo<'info>,
// }

// ///// MANGOGROUP
// #[derive(Clone)]
// pub struct MangoGroup(mango::state::MangoGroup);

// impl MangoGroup {
//     pub const LEN: usize = std::mem::size_of::<mango::state::MangoGroup>();
// }

// impl anchor_lang::AccountDeserialize for MangoGroup {
//     fn try_deserialize(buf: &mut &[u8]) -> Result<Self, ProgramError> {
//         MangoGroup::try_deserialize_unchecked(buf)
//     }

//     fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self, ProgramError> {
//         match mango::state::MangoGroup::unpack() {
//             Some(group) => Ok(group),
//             None => Err(ProgramError::InvalidAccountData),
//         }
//     }
// }

// impl anchor_lang::AccountSerialize for MangoGroup {
//     fn try_serialize<W: std::io::Write>(&self, _writer: &mut W) -> Result<(), ProgramError> {
//         // no-op
//         Ok(())
//     }
// }

// impl anchor_lang::Owner for MangoGroup {
//     fn owner() -> Pubkey {
//         Mango::ID
//         // &self.0.admin // Not sure about this
//     }
// }

// impl std::ops::Deref for MangoGroup {
//     type Target = mango::state::MangoGroup;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// ///// MANGOACCOUNT
// #[derive(Clone)]
// pub struct MangoAccount(mango::state::MangoAccount);

// impl MangoAccount {
//     pub const LEN: usize = std::mem::size_of::<mango::state::MangoAccount>();
// }

// impl anchor_lang::AccountDeserialize for MangoAccount {
//     fn try_deserialize(buf: &mut &[u8]) -> Result<Self, ProgramError> {
//         MangoAccount::try_deserialize_unchecked(buf)
//     }

//     fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self, ProgramError> {
//         mango::state::MangoAccount()
//     }
// }

// impl anchor_lang::AccountSerialize for MangoAccount {
//     fn try_serialize<W: std::io::Write>(&self, _writer: &mut W) -> Result<(), ProgramError> {
//         // no-op
//         Ok(())
//     }
// }

// impl anchor_lang::Owner for MangoAccount {
//     fn owner() -> Pubkey {
//         Mango::ID // &self.0.owner
//     }
// }

// impl std::ops::Deref for MangoAccount {
//     type Target = mango::state::MangoAccount;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
