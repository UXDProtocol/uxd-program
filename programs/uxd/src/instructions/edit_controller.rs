use crate::error::UxdError;
use crate::events::SetMangoDepositoryRedeemableSoftCapEvent;
use crate::events::SetRedeemableGlobalSupplyCapEvent;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::MAX_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP;
use crate::MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EditControllerAccounts<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,
    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: AccountLoader<'info, Controller>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct EditControllerFields {
    quote_mint_and_redeem_soft_cap: Option<u64>,
    redeemable_soft_cap: Option<u64>,
    redeemable_global_supply_cap: Option<u128>,
}

pub fn handler(ctx: Context<EditControllerAccounts>, fields: &EditControllerFields) -> Result<()> {
    let controller = &mut ctx.accounts.controller.load_mut()?;
    // Optionally edit "quote_mint_and_redeem_soft_cap"
    if let Some(quote_mint_and_redeem_soft_cap) = fields.quote_mint_and_redeem_soft_cap {
        msg!("[set_mango_depository_quote_mint_and_redeem_soft_cap]");
        controller.mango_depositories_quote_redeemable_soft_cap = quote_mint_and_redeem_soft_cap;
    }
    // Optionally edit "redeemable_soft_cap"
    if let Some(redeemable_soft_cap) = fields.redeemable_soft_cap {
        msg!("[set_mango_depositories_redeemable_soft_cap]");
        controller.mango_depositories_redeemable_soft_cap = redeemable_soft_cap;
        emit!(SetMangoDepositoryRedeemableSoftCapEvent {
            version: controller.version,
            controller: ctx.accounts.controller.key(),
            redeemable_mint_decimals: controller.redeemable_mint_decimals,
            redeemable_mint: controller.redeemable_mint,
            redeemable_soft_cap
        });
    }
    // Optionally edit "redeemable_global_supply_cap"
    if let Some(redeemable_global_supply_cap) = fields.redeemable_global_supply_cap {
        msg!("[set_redeemable_global_supply_cap]");
        controller.redeemable_global_supply_cap = redeemable_global_supply_cap;
        emit!(SetRedeemableGlobalSupplyCapEvent {
            version: controller.version,
            controller: ctx.accounts.controller.key(),
            redeemable_global_supply_cap
        });
    }
    Ok(())
}

#[allow(clippy::absurd_extreme_comparisons)]
impl<'info> EditControllerAccounts<'info> {
    pub fn validate(&self, fields: &EditControllerFields) -> Result<()> {
        if let Some(redeemable_soft_cap) = fields.redeemable_soft_cap {
            require!(
                redeemable_soft_cap <= MAX_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP,
                UxdError::InvalidMangoDepositoriesRedeemableSoftCap
            );
        }
        // Asserts that the Mango Depositories redeemable soft cap is between 0 and MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP.
        if let Some(redeemable_global_supply_cap) = fields.redeemable_global_supply_cap {
            require!(
                redeemable_global_supply_cap <= MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP,
                UxdError::InvalidRedeemableGlobalSupplyCap
            );
        }
        Ok(())
    }
}
