use anchor_lang::prelude::*;


 #[program]
 pub mod mango_tester {
     use super::*;
     pub fn new<'info>(ctx: Context<New>) -> ProgramResult {
         Ok(())
    }

    pub fn init_mango_account<'a, 'b, 'c, 'info>(
            ctx:CpiContext<'a, 'b, 'c, 'info, InitMangoAccount<'info>>
    ) -> ProgramResult {
        // let ix =

        // Add test for account creation
        // Ok(())
        // Accounts expected by this instruction (4):
        // 0. `[]` mango_group_ai - MangoGroup that this mango account is for
        // 1. `[writable]` mango_account_ai - the mango account data
        // 2. `[signer]` owner_ai - Solana account of owner of the mango account
        // 3. `[]` rent_ai - Rent sysvar account
        let mango_cpi_program = ctx.accounts.mango_program.clone();
        let mango_cpi_accts = InitMangoAccount {
            mango_group: ctx.accounts.mango_group.to_account_info(),
            mango_account: ctx.accounts.mango_account.clone().into(),
            owner_account: ctx.accounts.proxy_account.clone().into(),
            rent: ctx.accounts.rent.clone(),
        };
        let mango_cpi_ctx = CpiContext::new(mango_cpi_program, mango_cpi_accts);
        // mango_tester::cpi::init_mango_account(mango_cpi_ctx);
        Ok(())

    }



}

#[derive(Accounts)]
pub struct New<'info> {
    // XXX need payer for 14 #[account(init)]
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

#[account]
pub struct MangoTester {
    pub deposits: u64,
}
