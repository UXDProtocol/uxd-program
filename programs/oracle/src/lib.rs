use anchor_lang::prelude::*;
use pyth_client::Price;

solana_program::declare_id!("DzmGB2YeeFbSL72cAxYtfQCQXzyyWW2xYPCJ1uSPtNiP");

#[program]
#[deny(unused_must_use)]
pub mod oracle {
    use super::*;

    pub fn lol(ctx: Context<Lol>) -> ProgramResult {
        let price_data = ctx.accounts.btc_price.try_borrow_data()?;
        let price = pyth_client::cast::<Price>(&price_data);

        msg!("price: {}", price.agg.price);
        msg!("expo: {}", price.expo);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Lol<'info> {
    pub btc_price: AccountInfo<'info>,
}
