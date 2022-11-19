use crate::events::ReinjectMangoToIdentityDepositoryEvent;
use crate::state::MangoDepository;
use crate::Controller;
use crate::IdentityDepository;
use crate::UxdError;
use crate::CONTROLLER_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use std::str::FromStr;

pub const MANGO_DEPOSITORY_NAMESPACE: &[u8] = b"MANGODEPOSITORY";

#[cfg(feature = "development")]
pub const BTC_MINT: &str = "3UNBZ6o52WTWwjac2kPUb4FyodhU1vFkRJheu1Sh2TvU";
#[cfg(feature = "production")]
pub const BTC_MINT: &str = "9n4nbM75f5Ui33ZbPYXn59EwSgE8CGsHtAeTH5YFeJ9E";

#[cfg(feature = "development")]
pub const ETH_MINT: &str = "Cu84KB3tDL6SbFgToHMLYVDJJXdJjenNzSKikeAvzmkA";
#[cfg(feature = "production")]
pub const ETH_MINT: &str = "2FPyTwcZLUg1MDrwsyoP4D6s1tM7hAkHYRjkNb5w6Pxk";

#[derive(Accounts)]
pub struct ReinjectMangoToIdentityDepository<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.registered_mango_depositories.contains(&mango_depository.key()) @UxdError::InvalidDepository,
        has_one = authority @UxdError::InvalidAuthority,
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
        seeds = [IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE],
        token::authority = depository,
        token::mint = depository.load()?.collateral_mint,
        bump = depository.load()?.collateral_vault_bump,
    )]
    pub collateral_vault: Box<Account<'info, TokenAccount>>,

    /// #6 UXDProgram on chain account bound to a Controller instance.
    /// The `MangoDepository` manages a MangoAccount for a single Collateral.
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, mango_depository.load()?.collateral_mint.as_ref()],
        bump = mango_depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
    )]
    pub mango_depository: AccountLoader<'info, MangoDepository>,

    /// #7 The `user`'s TA for the `depository` `collateral_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        constraint = user_collateral.mint == depository.load()?.collateral_mint @UxdError::InvalidCollateralMint,
        constraint = &user_collateral.owner == authority.key @UxdError::InvalidOwner,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #8 System Program
    pub system_program: Program<'info, System>,

    /// #9 Token Program
    pub token_program: Program<'info, Token>,
}

pub(crate) fn handler(ctx: Context<ReinjectMangoToIdentityDepository>) -> Result<()> {
    // - 1 [GET REDEEMABLE AMOUNT OF THE MANGO DEPOSITORY ] -----------------
    let depository_redeemable: u128 = ctx
        .accounts
        .mango_depository
        .load()?
        .redeemable_amount_under_management;

    // - 2 [TRANSFER USER'S COLLATERAL TO DEPOSITORY'S VAULT ON 1:1 REDEEMABLES] -----------------
    let collateral_amount: u64 = depository_redeemable.try_into().unwrap();

    token::transfer(
        ctx.accounts
            .to_transfer_collateral_from_user_to_depository_vault_context(),
        collateral_amount,
    )?;

    // - 3 [UPDATE IDENTITY DEPOSITORY STATE] -----------------
    let depository = &mut ctx.accounts.depository.load_mut()?;
    depository.update_mango_collateral_reinjected(
        &ctx.accounts.mango_depository.load()?.collateral_mint,
    )?;

    // - 4 [UPDATE IDENTITY DEPOSITORY ACCOUNTING] -----------------
    depository.collateral_amount_deposited = depository
        .collateral_amount_deposited
        .checked_add(collateral_amount.into())
        .ok_or_else(|| error!(UxdError::MathError))?;
    depository.redeemable_amount_under_management = depository
        .redeemable_amount_under_management
        .checked_add(depository_redeemable)
        .ok_or_else(|| error!(UxdError::MathError))?;

    // - 5 [EVENT LOGGING] ----------------------------------------------------
    emit!(ReinjectMangoToIdentityDepositoryEvent {
        version: ctx.accounts.controller.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        mango_depository: ctx.accounts.mango_depository.key(),
        user: ctx.accounts.authority.key(),
        collateral_reinjected_amount: collateral_amount,
    });

    Ok(())
}

impl<'info> ReinjectMangoToIdentityDepository<'info> {
    fn to_transfer_collateral_from_user_to_depository_vault_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user_collateral.to_account_info(),
            to: self.collateral_vault.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
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

impl IdentityDepository {
    pub(crate) fn get_mango_collateral_reinjected(&self, collateral_mint: &Pubkey) -> Result<bool> {
        let wsol_mint: Pubkey = spl_token::native_mint::id();
        let btc_mint: Pubkey = Pubkey::from_str(BTC_MINT).unwrap();
        let eth_mint: Pubkey = Pubkey::from_str(ETH_MINT).unwrap();
        if collateral_mint.eq(&wsol_mint) {
            Ok(self.mango_collateral_reinjected_wsol)
        } else if collateral_mint.eq(&btc_mint) {
            Ok(self.mango_collateral_reinjected_btc)
        } else if collateral_mint.eq(&eth_mint) {
            Ok(self.mango_collateral_reinjected_eth)
        } else {
            Err(error!(UxdError::Default))
        }
    }

    pub(crate) fn update_mango_collateral_reinjected(
        &mut self,
        collateral_mint: &Pubkey,
    ) -> Result<()> {
        let wsol_mint: Pubkey = spl_token::native_mint::id();
        let btc_mint: Pubkey = Pubkey::from_str(BTC_MINT).unwrap();
        let eth_mint: Pubkey = Pubkey::from_str(ETH_MINT).unwrap();
        if collateral_mint.eq(&wsol_mint) {
            self.mango_collateral_reinjected_wsol = true;
        } else if collateral_mint.eq(&btc_mint) {
            self.mango_collateral_reinjected_btc = true;
        } else if collateral_mint.eq(&eth_mint) {
            self.mango_collateral_reinjected_eth = true;
        } else {
            return Err(error!(UxdError::Default));
        }
        Ok(())
    }
}
