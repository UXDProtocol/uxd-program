use anchor_lang::prelude::*;


 #[program]
 pub mod mango_tester {
     use super::*;
     pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
         Ok(())

    }

    pub fn init_mango_account(ctx: Context<InitMangoAccount>) -> ProgramResult {
        // Add test for account creation
        Ok(())
        // Accounts expected by this instruction (4):
        // 0. `[]` mango_group_ai - MangoGroup that this mango account is for
        // 1. `[writable]` mango_account_ai - the mango account data
        // 2. `[signer]` owner_ai - Solana account of owner of the mango account
        // 3. `[]` rent_ai - Rent sysvar account
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
        let mt = &mut ctx.accounts.mango_tester;
        mt.deposits += 1;
        Ok(())
        // Accounts expected by this instruction (8):
        // 0. `[]` mango_group_ai - MangoGroup that this mango account is for
        // 1. `[writable]` mango_account_ai - the mango account for this user
        // 2. `[signer]` owner_ai - Solana account of owner of the mango account
        // 3. `[]` mango_cache_ai - MangoCache
        // 4. `[]` root_bank_ai - RootBank owned by MangoGroup
        // 5. `[writable]` node_bank_ai - NodeBank owned by RootBank
        // 6. `[writable]` vault_ai - TokenAccount owned by MangoGroup
        // 7. `[]` token_prog_ai - acc pointed to by SPL token program id
        // 8. `[writable]` owner_token_account_ai - TokenAccount owned by user which will be sending the funds    }
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init)]
    pub mango_tester: ProgramAccount<'info, MangoTester>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitMangoAccount<'info> {
    #[account(mut)]
    pub mango_group: AccountInfo<'info>,
    pub mango_account: AccountInfo<'info>,
    #[account(signer)]
    pub owner_account: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub mango_tester: ProgramAccount<'info, MangoTester>,
}

#[account]
pub struct MangoTester {
    pub deposits: u64,
}
