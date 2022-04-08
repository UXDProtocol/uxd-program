use crate::error::UxdError;
use crate::events::RegisterMangoDepositoryEventV2;
use crate::Controller;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_ACCOUNT_VERSION;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::state::controller::MAX_STABLE_MINTS;
use anchor_comp::mango_markets_v3;
use anchor_comp::mango_markets_v3::MangoMarketV3;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use mango::state::MangoAccount;
use std::mem::size_of;

#[derive(Accounts)]
pub struct RegisterStable<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,
    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump,
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #4 The stable mint to be registered with the controller
    pub stable_mint: Box<Account<'info, Mint>>,
}

pub fn handler(
    ctx: Context<RegisterStable>,
) -> Result<()> {
    let new_entry_index = usize::from(ctx.accounts.controller.registered_stable_mints_count);
    ctx.accounts.controller.registered_stable_mints_count = ctx.accounts.controller
        .registered_stable_mints_count
        .checked_add(1u8)
        .ok_or_else(|| error!(UxdError::MathError))?;
    
    ctx.accounts.controller.registered_stable_mints[new_entry_index] = ctx.accounts.stable_mint.key();

    Ok(())
}

impl<'info> RegisterStable<'info> {
    pub fn validate(&self) -> Result<()> {
        let registered_stable_mint_count = usize::from(self.controller.registered_stable_mints_count);
        if registered_stable_mint_count
            .checked_add(1usize)
            .ok_or_else(|| error!(UxdError::MathError))? 
            > MAX_STABLE_MINTS {
            return Err(error!(UxdError::MaxNumberOfStableMintsReached));
        }

        Ok(())
    }
}
