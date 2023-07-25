use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;

use crate::BPS_POWER;
use crate::LSD_USER_LIQUIDATION_THREAD_AUTHORITY_NAMESPACE;
use crate::error::UxdError;
use crate::state::LsdDepository;
use crate::state::LsdPosition;
use crate::state::OraclePrice;
use crate::utils::maths;
use crate::utils::validate_collateral_amount;
use crate::utils::validate_loan_to_value_bps;
use crate::validate_is_program_frozen;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::LSD_DEPOSITORY_NAMESPACE;
use crate::LSD_POSITION_SPACE;
use crate::LSD_PROFITS_TOKEN_ACCOUNT_NAMESPACE;

// clockwork automation cost per action
pub const CLOCKWORK_AUTOMATION_FEE: u64 = 1_000;

#[derive(Accounts)]
pub struct BorrowFromLsdDepository<'info> {
    /// #1
    pub user: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = redeemable_mint @UxdError::InvalidRedeemableMint
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4
    #[account(
        mut,
        seeds = [LSD_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.load()?.bump,
        has_one = collateral_mint @UxdError::InvalidCollateralMint
    )]
    pub depository: AccountLoader<'info, LsdDepository>,

    /// #5
    #[account(
        mut,
        seeds = [LSD_PROFITS_TOKEN_ACCOUNT_NAMESPACE, depository.key().as_ref()],
        token::authority = depository,
        token::mint = liquidation_mint,
        bump = depository.load()?.profits_token_account_bump,
    )]
    pub profits_token_account: Account<'info, TokenAccount>,

    /// #6
    #[account(
        mut,
        constraint = user_collateral.owner == user.key() @UxdError::InvalidOwner,
        constraint = user_collateral.mint == collateral_mint.key() @UxdError::InvalidCollateralMint,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #
    #[account(
        mut,
        constraint = user_redeemable.owner == user.key() @UxdError::InvalidOwner,
        constraint = user_redeemable.mint == redeemable_mint.key() @UxdError::InvalidRedeemableMint,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #
    /// Tracks the user overall position in the LSD Depository (one per user, per LSD Depository).
    #[account(
        init_if_needed,
        seeds = [depository.key().as_ref(), user.key().as_ref()],
        space = LSD_POSITION_SPACE,       
        payer = payer,
        bump,
    )]
    pub lsd_position: AccountLoader<'info, LsdPosition>,

    /// #
    /// PDA holding the deposited collateral on behalf of the lsd_position
    #[account(
        init_if_needed,
        seeds = [lsd_position.key().as_ref()],
        token::authority = lsd_position,
        token::mint = collateral_mint,
        payer = payer,
        bump,
    )]
    pub lsd_position_collateral: Box<Account<'info, TokenAccount>>,

    /// # price oracle account for the collateral
    #[account(
        constraint = collateral_oracle_account.key() == depository.load()?.collateral_oracle_params.oracle_account
    )]
    pub collateral_oracle_account: AccountInfo<'info>,

    /// #
    #[account(mut)]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// #
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #
    pub liquidation_mint: Box<Account<'info, Mint>>,

    /// #
    pub system_program: Program<'info, System>,

    /// #
    pub token_program: Program<'info, Token>,

    /// #
    pub uxd_program: Program<'info, crate::program::Uxd>,

    /// #
    pub rent: Sysvar<'info, Rent>,
}

pub(crate) fn handler(ctx: Context<BorrowFromLsdDepository>, collateral_amount: u64, loan_to_value_bps: u16) -> Result<()> {
    // - 0 [CHECKS] ------------------------------------------------------------
    {
        // - 0.1 verify that borrows are enabled
        let lsd_depository = ctx.accounts.depository.load()?;
        require!(lsd_depository.borrowing_disabled == false, UxdError::BorrowingDisabled);

    }

    // - 1 [TRANSFER USER'S COLLATERAL] ----------------------------------------
    {
        msg!("[transfer] {} collateral from user to his lsd_position",
            collateral_amount,
        );
        token::transfer(
            ctx.accounts
                .to_transfer_collateral_from_user_to_lsd_position_collateral_ta_context(),
            collateral_amount,
        )?;
    }

    // - 2 [MINT REDEEMABLES] --------------------------------------------------
    let redeemable_borrow_amount = collateral_amount / u64::from(loan_to_value_bps) * u64::from(BPS_POWER);
    {
        msg!("[mint] {} redeemable to user (ltv {})",
            collateral_amount,
            u64::from(loan_to_value_bps) / BPS_POWER,
        );
        let controller_bump = ctx.accounts.controller.load()?.bump;
        let controller_pda_signer: &[&[&[u8]]] = &[&[CONTROLLER_NAMESPACE, &[controller_bump]]];
        token::mint_to(
            ctx.accounts
                .to_mint_redeemable_context()
                .with_signer(controller_pda_signer),
            redeemable_borrow_amount,
        )?;
    }

    // - 3 [SETUP LIQUIDATION CRON] --------------------------------------------
    let liquidation_price: u64;
    {
        let controller = &mut ctx.accounts.controller.load_mut()?;
        let depository = &mut ctx.accounts.depository.load_mut()?;
        let curtime = controller.get_time()?;
        let collateral_oracle_params = depository.collateral_oracle_params;
        let lsd_price = OraclePrice::new_from_oracle(
            &ctx.accounts
                .collateral_oracle_account
                .to_account_info(),
            &collateral_oracle_params,
            curtime,
            false,
        )?;
        let lsd_price_usd = lsd_price.get_asset_amount_usd(collateral_amount, depository.collateral_mint_decimals)?;
        liquidation_price = maths::checked_mul(redeemable_borrow_amount, maths::checked_div(u64::from(depository.max_loan_to_value_bps), u64::from(BPS_POWER))?)?;

        // Create a clockwork thread to auto-liquidate when the price is reached
        {}
        // - - setup/update either another cron or in previous cron: liquidate when running out of funding
    }

    // - 3.1 [INITIALIZE LSD POSITION if needed] -------------------------------
    {
        let lsd_position = &mut ctx.accounts.lsd_position.load_mut()?;
        if lsd_position.is_initialized == false {
            lsd_position.is_initialized = true;
            lsd_position.bump = *ctx
                .bumps
                .get("lsd_position")
                .ok_or_else(|| error!(UxdError::BumpError))?;
            lsd_position.user_liquidation_thread_authority_bump = *ctx
                .bumps
                .get("user_liquidation_thread_authority")
                .ok_or_else(|| error!(UxdError::BumpError))?;
            lsd_position.depository = ctx.accounts.depository.key();
        }
    }

    // - 4 [UPDATE STATS] ------------------------------------------------------
    {
        let controller = &mut ctx.accounts.controller.load_mut()?;
        let depository = &mut ctx.accounts.depository.load_mut()?;
        let lsd_position = &mut ctx.accounts.lsd_position.load_mut()?;
        // Depository
        depository.collateral_amount_deposits = depository
            .collateral_amount_deposits
            .checked_add(collateral_amount)
            .ok_or(UxdError::MathOverflow)?;
        depository.redeemable_amount_under_management = depository
            .redeemable_amount_under_management
            .checked_add(redeemable_borrow_amount)
            .ok_or(UxdError::MathOverflow)?;
        // Controller
        controller.redeemable_circulating_supply = controller
            .redeemable_circulating_supply
            .checked_add(redeemable_borrow_amount.into())
            .ok_or(UxdError::MathOverflow)?;
        // LSD Position
        lsd_position.collateral_amount = lsd_position
            .collateral_amount
            .checked_add(collateral_amount)
            .ok_or(UxdError::MathOverflow)?;
        lsd_position.redeemable_amount = lsd_position
            .redeemable_amount
            .checked_add(redeemable_borrow_amount)
            .ok_or(UxdError::MathOverflow)?;
        lsd_position.liquidation_price = liquidation_price;
    }

    // - 5 [POST CHECKS]--------------------------------------------------------
    {
        let depository = ctx.accounts.depository.load()?;
        // - 6.1 check that the depository is not over the redeemable amount cap
        require!(depository.redeemable_amount_under_management <= depository.redeemable_amount_under_management_cap, UxdError::MathOverflow);
    }

    Ok(())
}

impl<'info> BorrowFromLsdDepository<'info> {
    fn to_transfer_collateral_from_user_to_lsd_position_collateral_ta_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user_collateral.to_account_info(),
            to: self.lsd_position_collateral.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn to_mint_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.redeemable_mint.to_account_info(),
            to: self.user_redeemable.to_account_info(),
            authority: self.controller.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Validate
impl<'info> BorrowFromLsdDepository<'info> {
    pub(crate) fn validate(&self, collateral_amount: u64, loan_to_value_bps: u16) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        validate_collateral_amount(&self.user_collateral, collateral_amount)?;
        validate_loan_to_value_bps(loan_to_value_bps, self.depository.load()?.max_loan_to_value_bps)?;
        Ok(())
    }
}
