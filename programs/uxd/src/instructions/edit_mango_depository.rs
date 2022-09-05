use crate::error::UxdError;
use crate::state::MangoDepository;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EditMangoDepository<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,
    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
        constraint = controller.load()?.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub controller: AccountLoader<'info, Controller>,
    /// #3 UXDProgram on chain account bound to a Controller instance.
    /// The `MangoDepository` manages a MangoAccount for a single Collateral.
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
    )]
    pub depository: AccountLoader<'info, MangoDepository>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EditMangoDepositoryFields {
    quote_mint_and_redeem_fee: Option<u8>, // in bps
}

pub fn handler(
    ctx: Context<EditMangoDepository>,
    fields: &EditMangoDepositoryFields,
) -> Result<()> {
    let depository = &mut ctx.accounts.depository.load_mut()?;
    // optional: quote_mint_and_redeem_fee
    if let Some(quote_mint_and_redeem_fee) = fields.quote_mint_and_redeem_fee {
        msg!(
            "[set_mango_depository_quote_mint_and_redeem_fee] quote_fee {}",
            quote_mint_and_redeem_fee
        );
        depository.quote_mint_and_redeem_fee = quote_mint_and_redeem_fee
    }
    Ok(())
}
