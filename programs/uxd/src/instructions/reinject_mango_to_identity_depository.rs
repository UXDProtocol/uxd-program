use crate::state::MangoDepository;
use crate::Controller;
use crate::IdentityDepository;
use crate::UxdError;
use crate::CONTROLLER_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_COLLATERAL_VAULT_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use std::str::FromStr;

pub const MANGO_DEPOSITORY_NAMESPACE: &[u8] = b"MANGODEPOSITORY";

#[derive(Accounts)]
pub struct ReinjectMangoToIdentityDepository<'info> {
    /// #1 Public call accessible to any user
    pub user: Signer<'info>,
    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.registered_mango_depositories.contains(&mango_depository.key()) @UxdError::InvalidDepository,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4 UXDProgram on chain account bound to a Controller instance that represent the blank minting/redeeming
    #[account(
        mut,
        seeds = [IDENTITY_DEPOSITORY_NAMESPACE],
        bump = depository.load()?.bump,
    )]
    pub depository: AccountLoader<'info, IdentityDepository>,

    /// #5
    /// Token account holding the collateral from minting
    #[account(
        mut,
        seeds = [IDENTITY_DEPOSITORY_COLLATERAL_VAULT_NAMESPACE],
        token::authority = depository,
        token::mint = depository.load()?.collateral_mint,
        bump = depository.load()?.collateral_vault_bump,
    )]
    pub collateral_vault: Box<Account<'info, TokenAccount>>,

    /// #4 UXDProgram on chain account bound to a Controller instance.
    /// The `MangoDepository` manages a MangoAccount for a single Collateral.
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
    )]
    pub mango_depository: AccountLoader<'info, MangoDepository>,

    /// #7 The `user`'s TA for the `depository` `collateral_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        constraint = user_collateral.mint == depository.load()?.collateral_mint @UxdError::InvalidCollateralMint,
        constraint = &user_collateral.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #9
    pub system_program: Program<'info, System>,

    /// #10 Token Program
    pub token_program: Program<'info, Token>,
}

pub(crate) fn handler(ctx: Context<ReinjectMangoToIdentityDepository>) -> Result<()> {
    Ok(())
}

impl<'info> ReinjectMangoToIdentityDepository<'info> {
    pub(crate) fn validate(&self) -> Result<()> {
        let mango_collateral_reinjected: bool = self
            .depository
            .load()?
            .get_mango_collateral_reinjected(&self.mango_depository.load()?.collateral_mint)?;

        require!(
            !mango_collateral_reinjected,
            UxdError::MangoCollateralReinjected
        );
        Ok(())
    }
}

#[cfg(feature = "development")]
pub const BTC_MINT: &str = "3UNBZ6o52WTWwjac2kPUb4FyodhU1vFkRJheu1Sh2TvU";
#[cfg(feature = "production")]
pub const BTC_MINT: &str = "9n4nbM75f5Ui33ZbPYXn59EwSgE8CGsHtAeTH5YFeJ9E";

#[cfg(feature = "development")]
pub const ETH_MINT: &str = "Cu84KB3tDL6SbFgToHMLYVDJJXdJjenNzSKikeAvzmkA";
#[cfg(feature = "production")]
pub const ETH_MINT: &str = "2FPyTwcZLUg1MDrwsyoP4D6s1tM7hAkHYRjkNb5w6Pxk";

impl IdentityDepository {
    pub(crate) fn get_mango_collateral_reinjected(&self, collateral_mint: &Pubkey) -> Result<bool> {
        let wsol_mint: Pubkey = spl_token::native_mint::id();
        let btc_mint: Pubkey = Pubkey::from_str(BTC_MINT).unwrap();
        let eth_mint: Pubkey = Pubkey::from_str(ETH_MINT).unwrap();
        if collateral_mint.eq(&wsol_mint) {
            return Ok(self.mango_collateral_reinjected_wsol);
        } else if collateral_mint.eq(&btc_mint) {
            return Ok(self.mango_collateral_reinjected_btc);
        } else if collateral_mint.eq(&eth_mint) {
            return Ok(self.mango_collateral_reinjected_eth);
        } else {
            return Err(error!(UxdError::Default));
        }
    }
}
