use crate::Controller;
use crate::UxdError;
use crate::CONTROLLER_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use anchor_comp::mango_markets_v3;
use anchor_comp::mango_markets_v3::MangoMarketV3;
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
        constraint = controller.load()?.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
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
        seeds = [IDENTITY_DEPOSITORY_COLLATERAL_VAULT_NAMESPACE],
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

    /// #9 System Program
    pub system_program: Program<'info, System>,

    /// #10 Token Program
    pub token_program: Program<'info, Token>,
}

pub(crate) fn handler(
    ctx: Context<MintWithIdentityDepository>,
    collateral_amount: u64,
) -> Result<()> {
    // - 1 [TRANSFER USER'S COLLATERAL TO DEPOSITORY'S VAULT] -----------------
    ctx.accounts
        .transfer_user_collateral_to_depository_vault(collateral_amount)?;

    // - 2 [MINTS REDEEMABLES 1:1 FOR PROVIDED COLLATERAL] --------------------
    let redeemable_amount = collateral_amount;
    ctx.accounts.mint_redeemable_to_user(redeemable_amount)?;

    // - 3 [UPDATE ACCOUNTING] ------------------------------------------------
    ctx.accounts
        .update_onchain_accounting(collateral_amount, redeemable_amount)?;

    // - 4 [SANITY CHECKS] ----------------------------------------------------
    ctx.accounts.sanity_checks()?;

    // - 5 [EVENT LOGGING] ----------------------------------------------------
    ctx.accounts.event_logging(collateral_amount)?;

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
    pub(crate) fn validate(&self, collateral_amount: u64) -> Result<()> {
        require!(collateral_amount != 0, UxdError::InvalidCollateralAmount);
        require!(
            self.user_collateral.amount >= collateral_amount,
            UxdError::InsufficientCollateralAmount
        );
        require!(
            !&self.depository.load()?.regular_minting_disabled,
            UxdError::MintingDisabled
        );

        Ok(())
    }

    pub(crate) fn transfer_user_collateral_to_depository_vault(&self, amount: u64) -> Result<()> {
        token::transfer(
            self.accounts
                .to_transfer_collateral_from_user_to_depository_vault_context(),
            amount,
        )?;
        Ok(())
    }

    pub(crate) fn mint_redeemable_to_user(&self, amount: u64) -> Result<()> {
        let controller_bump = self.accounts.controller.load()?.bump;
        let controller_pda_signer: &[&[&[u8]]] = &[&[CONTROLLER_NAMESPACE, &[controller_bump]]];
        token::mint_to(
            self.accounts
                .to_mint_redeemable_context()
                .with_signer(controller_pda_signer),
            amount,
        )?;
        Ok(())
    }

    fn update_onchain_accounting(
        &mut self,
        collateral_deposited_amount: u128,
        redeemable_minted_amount: u128,
    ) -> Result<()> {
        let depository = &mut self.depository.load_mut()?;
        let controller = &mut self.controller.load_mut()?;
        // Depository
        depository.collateral_amount_deposited = depository
            .collateral_amount_deposited
            .checked_add(collateral_deposited_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.redeemable_amount_under_management = depository
            .redeemable_amount_under_management
            .checked_add(redeemable_minted_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        // Controller
        controller.redeemable_circulating_supply = controller
            .redeemable_circulating_supply
            .checked_add(redeemable_minted_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        Ok(())
    }

    pub(crate) fn sanity_checks(&self) -> Result<()> {
        let controller = self.controller.load()?;
        // Check for Global supply cap limitation
        require!(
            controller.redeemable_circulating_supply <= controller.redeemable_global_supply_cap,
            UxdError::RedeemableGlobalSupplyCapReached
        );
        Ok(())
    }

    pub(crate) fn event_logging(&self, collateral_amount: u64) -> Result<()> {
        emit!(MintWithIdentityDepositoryEvent {
            version: self.controller.load()?.version,
            controller: ctx.accounts.controller.key(),
            depository: ctx.accounts.depository.key(),
            user: ctx.accounts.user.key(),
            collateral_amount,
        });
        Ok(())
    }
}
