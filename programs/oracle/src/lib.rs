use anchor_lang::prelude::*;
use pyth_client::Price;

const SEED: &[u8] = b"BTCUSD";

solana_program::declare_id!("UXDGB2YeFbSL72cAxYtfQCQXzyyWW2xYPCJ1uSPtNiP");

#[program]
#[deny(unused_must_use)]
pub mod oracle {
    use super::*;

    pub fn init(_ctx: Context<Init>) -> ProgramResult {
        Ok(())
    }

    pub fn get(ctx: Context<Get>) -> ProgramResult {
        let price_data = ctx.accounts.oracle.try_borrow_data()?;
        let price = pyth_client::cast::<Price>(&price_data);

        msg!("price: {}", price.agg.price);
        msg!("expo: {}", price.expo);

        Ok(())
    }

    pub fn put(ctx: Context<Put>, offset: u64, src: Vec<u8>) -> ProgramResult {
        let mut dst = ctx.accounts.buffer.try_borrow_mut_data()?;

        dst[offset as usize..(src.len() + offset as usize)].copy_from_slice(&src);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub wallet: Signer<'info>,
    #[account(
        init,
        seeds = [SEED],
        bump,
        space = 3312,
        payer = wallet,
        owner = *program_id,
    )]
    pub buffer: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Get<'info> {
    pub oracle: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Put<'info> {
    #[account(mut)]
    pub buffer: UncheckedAccount<'info>,
}
