use crate::events::MintWithIdentityDepositoryEvent;
use crate::state::identity_depository::IdentityDepository;
use crate::Controller;
use crate::UxdError;
use crate::CONTROLLER_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;

#[derive(Accounts)]
pub struct MintWithIdentityDepository<'info> {
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
        has_one = redeemable_mint @UxdError::InvalidRedeemableMint
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

    /// #6 The redeemable mint managed by the `controller` instance
    /// Tokens will be minted during this instruction
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.load()?.redeemable_mint_bump,
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// #7 The `user`'s TA for the `depository` `collateral_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        constraint = user_collateral.mint == depository.load()?.collateral_mint @UxdError::InvalidCollateralMint,
        constraint = &user_collateral.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #8 The `user`'s TA for the `controller`'s `redeemable_mint`
    /// Will be credited during this instruction
    #[account(
        mut,
        constraint = user_redeemable.mint == controller.load()?.redeemable_mint @UxdError::InvalidRedeemableMint,
        constraint = &user_redeemable.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #9
    pub system_program: Program<'info, System>,

    /// #10 Token Program
    pub token_program: Program<'info, Token>,
}

pub(crate) fn handler(
    ctx: Context<MintWithIdentityDepository>,
    collateral_amount: u64,
) -> Result<()> {
    // - 1 [TRANSFER USER'S COLLATERAL TO DEPOSITORY'S VAULT] -----------------
    token::transfer(
        ctx.accounts
            .to_transfer_collateral_from_user_to_depository_vault_context(),
        collateral_amount,
    )?;

    // - 2 [MINTS REDEEMABLES 1:1 FOR PROVIDED COLLATERAL] --------------------
    let redeemable_amount = collateral_amount;
    {
        let controller_bump = ctx.accounts.controller.load()?.bump;
        let controller_pda_signer: &[&[&[u8]]] = &[&[CONTROLLER_NAMESPACE, &[controller_bump]]];
        token::mint_to(
            ctx.accounts
                .to_mint_redeemable_context()
                .with_signer(controller_pda_signer),
            redeemable_amount,
        )?;
    }

    // - 3 [UPDATE ACCOUNTING] ------------------------------------------------
    {
        let depository = &mut ctx.accounts.depository.load_mut()?;
        let controller = &mut ctx.accounts.controller.load_mut()?;
        // Depository
        depository.collateral_amount_deposited = depository
            .collateral_amount_deposited
            .checked_add(collateral_amount.into())
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.redeemable_amount_under_management = depository
            .redeemable_amount_under_management
            .checked_add(redeemable_amount.into())
            .ok_or_else(|| error!(UxdError::MathError))?;
        // Controller
        controller.redeemable_circulating_supply = controller
            .redeemable_circulating_supply
            .checked_add(redeemable_amount.into())
            .ok_or_else(|| error!(UxdError::MathError))?;
    }

    // - 4 [SANITY CHECKS] ----------------------------------------------------
    let controller = ctx.accounts.controller.load()?;
    {
        // Check for Global supply cap limitation
        require!(
            controller.redeemable_circulating_supply <= controller.redeemable_global_supply_cap,
            UxdError::RedeemableGlobalSupplyCapReached
        );
    }

    ctx.accounts
        .check_redeemable_amount_under_management_cap_overflow()?;

    // - 5 [EVENT LOGGING] ----------------------------------------------------
    emit!(MintWithIdentityDepositoryEvent {
        version: controller.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        user: ctx.accounts.user.key(),
        collateral_amount,
    });

    Ok(())
}

impl<'info> MintWithIdentityDepository<'info> {
    fn to_mint_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.redeemable_mint.to_account_info(),
            to: self.user_redeemable.to_account_info(),
            authority: self.controller.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn to_transfer_collateral_from_user_to_depository_vault_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user_collateral.to_account_info(),
            to: self.collateral_vault.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> MintWithIdentityDepository<'info> {
    fn check_redeemable_amount_under_management_cap_overflow(&self) -> Result<()> {
        let depository = self.depository.load()?;
        require!(
            depository.redeemable_amount_under_management
                <= depository.redeemable_amount_under_management_cap,
            UxdError::RedeemableIdentityDepositoryAmountUnderManagementCap
        );
        Ok(())
    }
}

impl<'info> MintWithIdentityDepository<'info> {
    pub(crate) fn validate(&self, collateral_amount: u64) -> Result<()> {
        require!(collateral_amount != 0, UxdError::InvalidCollateralAmount);
        require!(
            self.user_collateral.amount >= collateral_amount,
            UxdError::InsufficientCollateralAmount
        );
        require!(
            !&self.depository.load()?.minting_disabled,
            UxdError::MintingDisabled
        );

        Ok(())
    }
}
