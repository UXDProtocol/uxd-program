use crate::error::UxdError;
use crate::events::ZoMintEvent;
use crate::zo_utils::PerpInfo;
use crate::Controller;
use crate::ZoDepository;
use crate::COLLATERAL_PASSTHROUGH_NAMESPACE;
use crate::CONTROLLER_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use crate::ZO_DEPOSITORY_NAMESPACE;
use crate::ZO_MARGIN_ACCOUNT_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::CloseAccount;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use fixed::types::I80F48;
use zo::Cache;
use zo::Control;
use zo::Margin;
use zo::State;
use zo::ZO_DEX_PID;
use zo_abi as zo;

#[derive(Debug)]
pub struct DeltaNeutralPosition {
    // Quote native units
    pub size: i64,
    // Native units
    pub base_size: i64,
    // Quote native units
    pub realized_pnl: i64,
}

/// Takes 25 accounts - 8 used locally - 12 for Zo CPI - 4 Programs - 1 Sysvar
#[derive(Accounts)]
pub struct MintWithZoDepository<'info> {
    /// #1 Public call accessible to any user
    pub user: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #4 UXDProgram on chain account bound to a Controller instance.
    /// The `ZoDepository` manages a ZoAccount for a single Collateral.
    #[account(
        mut,
        seeds = [ZO_DEPOSITORY_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.bump,
        has_one = controller @UxdError::InvalidController,
        constraint = controller.registered_zo_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub depository: Box<Account<'info, ZoDepository>>,

    /// #5 The redeemable mint managed by the `controller` instance
    /// Tokens will be minted during this instruction
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.redeemable_mint_bump,
        constraint = redeemable_mint.key() == controller.redeemable_mint @UxdError::InvalidRedeemableMint
    )]
    pub redeemable_mint: Box<Account<'info, token::Mint>>,

    /// #6 The `user`'s ATA for the `depository` `collateral_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        seeds = [user.key.as_ref(), token_program.key.as_ref(), depository.collateral_mint.as_ref()],
        bump,
        seeds::program = AssociatedToken::id(),
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #7 The `user`'s ATA for the `controller`'s `redeemable_mint`
    /// Will be credited during this instruction
    #[account(
        init_if_needed,
        associated_token::mint = redeemable_mint,
        associated_token::authority = user,
        payer = payer,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #8 The `depository`'s TA for its `insurance_mint`
    /// ZoAccounts can only transact with the TAs owned by their authority
    /// and this only serves as a passthrough
    #[account(
        mut,
        seeds = [COLLATERAL_PASSTHROUGH_NAMESPACE, depository.key().as_ref()],
        bump = depository.collateral_passthrough_bump,
        constraint = depository.collateral_passthrough == depository_collateral_passthrough_account.key() @UxdError::InvalidCollateralPassthroughAccount
    )]
    pub depository_collateral_passthrough_account: Box<Account<'info, TokenAccount>>,

    /// #9 The Zo Dex Account (Margin) managed by the `depository`
    /// CHECK : Seeds checked. Depository registered
    #[account(
        mut,
        seeds = [depository.key().as_ref(), zo_state.key().as_ref(), ZO_MARGIN_ACCOUNT_NAMESPACE],
        seeds::program = zo_program.key(),
        constraint = depository.zo_account == depository_zo_account.key() @UxdError::InvalidZoAccount,
        bump
    )]
    pub depository_zo_account: AccountLoader<'info, Margin>,

    /// #10 [ZeroOne CPI]
    pub zo_state: AccountLoader<'info, State>,

    /// #11 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    #[account(mut)]
    pub zo_state_signer: UncheckedAccount<'info>,

    /// #12 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    #[account(mut)]
    pub zo_cache: AccountLoader<'info, Cache>,

    /// #13 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    #[account(mut)]
    pub zo_vault: UncheckedAccount<'info>,

    /// #14 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_control: AccountLoader<'info, Control>,

    /// #15 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_open_orders: UncheckedAccount<'info>,

    /// #16 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_dex_market: UncheckedAccount<'info>,

    /// #17 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_req_q: UncheckedAccount<'info>,

    /// #18 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_event_q: UncheckedAccount<'info>,

    /// #19 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_market_bids: UncheckedAccount<'info>,

    /// #20 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_market_asks: UncheckedAccount<'info>,

    /// #21 [ZeroOne CPI] Zo Dex program
    /// CHECK: ZeroOne CPI
    #[account(address = ZO_DEX_PID)]
    pub zo_dex_program: AccountInfo<'info>,

    /// #22 System Program
    pub system_program: Program<'info, System>,

    /// #23 Token Program
    pub token_program: Program<'info, Token>,

    /// #24 Associated Token Program
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// #25 ZeroOne Program
    pub zo_program: Program<'info, zo::program::ZoAbi>,

    /// #26 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<MintWithZoDepository>,
    collateral_amount: u64,
    limit_price: u64,
) -> Result<()> {
    let controller_pda_signer: &[&[&[u8]]] =
        &[&[CONTROLLER_NAMESPACE, &[ctx.accounts.controller.bump]]];
    let depository_pda_signer: &[&[&[u8]]] = &[&[
        ZO_DEPOSITORY_NAMESPACE,
        ctx.accounts.depository.collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];
    let perp_info = ctx.accounts.perp_info()?;

    // - 1 [DEPOSIT COLLATERAL ON DEPOSITORY] ---------------------------------
    let contract_size = I80F48::from_num(perp_info.quote_lot_size);

    // - [Get the amount of Base Lots for the perp order (odd lots won't be processed)]
    let max_base_quantity_precise = I80F48::from_num(collateral_amount)
        .checked_div(contract_size)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_floor()
        .ok_or_else(|| error!(UxdError::MathError))?;

    // - [Derive native units for transfer]
    let collateral_deposited_amount = max_base_quantity_precise
        .checked_mul(contract_size)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_to_num()
        .ok_or_else(|| error!(UxdError::MathError))?;

    // - [Transfers user collateral to the passthrough account]
    token::transfer(
        ctx.accounts
            .into_transfer_user_collateral_to_passthrough_context(),
        collateral_deposited_amount,
    )?;

    zo::cpi::deposit(
        ctx.accounts
            .into_deposit_collateral_context()
            .with_signer(depository_pda_signer),
        false,
        collateral_deposited_amount,
    )?;

    let dn_position = ctx
        .accounts
        .delta_neutral_position(perp_info.market_index)?;
    let max_base_quantity = max_base_quantity_precise
        .checked_to_num()
        .ok_or_else(|| error!(UxdError::MathError))?;

    // msg!("limit_price {:?}", limit_price);
    // msg!("max_base_quantity {:?}", max_base_quantity);

    zo::cpi::place_perp_order(
        ctx.accounts
            .into_place_short_perp_order_context()
            .with_signer(depository_pda_signer),
        false,
        limit_price,
        max_base_quantity,
        u64::MIN,
        zo::OrderType::FillOrKill,
        u16::MAX,
        u64::MIN,
    )?;

    let dn_position_post = ctx
        .accounts
        .delta_neutral_position(perp_info.market_index)?;

    // msg!("dn_position {:?}", dn_position);
    // msg!("dn_position_post {:?}", dn_position_post);

    // Additional backing (minus fees) in native units (base and quote) added to the DN position
    let collateral_short_amount = dist(dn_position.base_size, dn_position_post.base_size);
    let redeemable_mint_amount = dist(dn_position.size, dn_position_post.size);

    // msg!("collateral_short_amount {}", collateral_short_amount);
    // msg!("redeemable_mint_amount {}", redeemable_mint_amount);

    // validates that the collateral_amount matches the amount shorted
    if collateral_deposited_amount != collateral_short_amount {
        return Err(error!(UxdError::InvalidCollateralDelta));
    }

    // - 4 [MINTS THE HEDGED AMOUNT OF REDEEMABLE (minus fees)] ----------------
    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_pda_signer),
        redeemable_mint_amount,
    )?;

    // - [if ATA mint is WSOL, unwrap]
    if ctx.accounts.depository.collateral_mint == spl_token::native_mint::id() {
        token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;
    }

    // - 5 [UPDATE ACCOUNTING] ------------------------------------------------
    ctx.accounts.update_onchain_accounting(
        collateral_short_amount.into(),
        redeemable_mint_amount.into(),
    )?;

    // - 6 [CHECK GLOBAL REDEEMABLE SUPPLY CAP OVERFLOW] ----------------------
    ctx.accounts.check_redeemable_global_supply_cap_overflow()?;

    // emit!(ZoMintEvent {
    //     version: ctx.accounts.controller.version,
    //     controller: ctx.accounts.controller.key(),
    //     depository: ctx.accounts.depository.key(),
    //     user: ctx.accounts.user.key(),
    //     collateral_amount,
    //     collateral_deposited_amount,
    //     limit_price,
    //     minted_amount: redeemable_mint_amount
    // });

    Ok(())
}

impl<'info> MintWithZoDepository<'info> {
    pub fn into_transfer_user_collateral_to_passthrough_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_collateral.to_account_info(),
            to: self
                .depository_collateral_passthrough_account
                .to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_deposit_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, zo::cpi::accounts::Deposit<'info>> {
        let cpi_accounts = zo::cpi::accounts::Deposit {
            state: self.zo_state.to_account_info(),
            state_signer: self.zo_state_signer.to_account_info(),
            cache: self.zo_cache.to_account_info(),
            authority: self.depository.to_account_info(),
            margin: self.depository_zo_account.to_account_info(),
            token_account: self
                .depository_collateral_passthrough_account
                .to_account_info(),
            vault: self.zo_vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.zo_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_place_short_perp_order_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, zo::cpi::accounts::PlacePerpOrder<'info>> {
        let cpi_accounts = zo::cpi::accounts::PlacePerpOrder {
            state: self.zo_state.to_account_info(),
            state_signer: self.zo_state_signer.to_account_info(),
            cache: self.zo_cache.to_account_info(),
            authority: self.depository.to_account_info(),
            margin: self.depository_zo_account.to_account_info(),
            control: self.zo_control.to_account_info(),
            open_orders: self.zo_open_orders.to_account_info(),
            dex_market: self.zo_dex_market.to_account_info(),
            req_q: self.zo_req_q.to_account_info(),
            event_q: self.zo_event_q.to_account_info(),
            market_bids: self.zo_market_bids.to_account_info(),
            market_asks: self.zo_market_asks.to_account_info(),
            dex_program: self.zo_dex_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.zo_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_mint_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.redeemable_mint.to_account_info(),
            to: self.user_redeemable.to_account_info(),
            authority: self.controller.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_unwrap_wsol_by_closing_ata_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount {
            account: self.user_collateral.to_account_info(),
            destination: self.user.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Additional convenience methods related to the inputted accounts
impl<'info> MintWithZoDepository<'info> {
    fn perp_info(&self) -> Result<PerpInfo> {
        let state = self.zo_state.load()?;
        Ok(PerpInfo::new(&state, self.zo_dex_market.key)?)
    }

    fn delta_neutral_position(&self, index: usize) -> Result<DeltaNeutralPosition> {
        let control_account = self.zo_control.load()?;
        let open_orders_info = control_account
            .open_orders_agg
            .get(index)
            .ok_or_else(|| error!(UxdError::ZOOpenOrdersInfoNotFound))?;
        // Should never have pending orders nor a non short position
        if open_orders_info.order_count > 0 || open_orders_info.pos_size > 0 {
            return Err(error!(UxdError::ZOInvalidControlState));
        }
        Ok(DeltaNeutralPosition {
            size: open_orders_info.pos_size,
            base_size: open_orders_info.native_pc_total,
            realized_pnl: open_orders_info.realized_pnl,
        })
    }
    // Ensure that the minted amount does not raise the Redeemable supply beyond the Global Redeemable Supply Cap
    fn check_redeemable_global_supply_cap_overflow(&self) -> Result<()> {
        if self.controller.redeemable_circulating_supply
            > self.controller.redeemable_global_supply_cap
        {
            return Err(error!(UxdError::RedeemableGlobalSupplyCapReached));
        }
        Ok(())
    }

    // Update the accounting in the Depository and Controller Accounts to reflect changes
    fn update_onchain_accounting(
        &mut self,
        collateral_shorted_amount: u128,
        redeemable_minted_amount: u128,
    ) -> Result<()> {
        let depository = &mut self.depository;
        let controller = &mut self.controller;
        // Mango Depository
        depository.collateral_amount_deposited = depository
            .collateral_amount_deposited
            .checked_add(collateral_shorted_amount)
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
}

// Validate input arguments
impl<'info> MintWithZoDepository<'info> {
    pub fn validate(&self, collateral_amount: u64, limit_price: u64) -> Result<()> {
        if !self.depository.is_initialized {
            return Err(error!(UxdError::ZoDepositoryNotInitialized));
        }
        if limit_price <= 0 {
            return Err(error!(UxdError::InvalidLimitPrice));
        }
        if collateral_amount == 0 {
            return Err(error!(UxdError::InvalidCollateralAmount));
        }
        if self.user_collateral.amount < collateral_amount {
            return Err(error!(UxdError::InsufficientCollateralAmount));
        }
        Ok(())
    }
}

fn dist(a: i64, b: i64) -> u64 {
    if a < b {
        (b as u64).wrapping_sub(a as u64)
    } else {
        (a as u64).wrapping_sub(b as u64)
    }
}
