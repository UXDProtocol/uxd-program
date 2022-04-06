use crate::error::UxdError;
use crate::zo_utils::PerpInfo;
use crate::Controller;
use crate::ZoDepository;
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
use zo::Control;
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
        bump = controller.bump,
        has_one = redeemable_mint @UxdError::InvalidRedeemableMint,
        constraint = controller.registered_zo_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #4 UXDProgram on chain account bound to a Controller instance.
    /// The `ZoDepository` manages a ZoAccount for a single Collateral.
    #[account(
        mut,
        seeds = [ZO_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = zo_account @UxdError::InvalidZoAccount,
        has_one = zo_dex_market @UxdError::InvalidDexMarket
    )]
    pub depository: AccountLoader<'info, ZoDepository>,

    /// #5 The redeemable mint managed by the `controller` instance
    /// Tokens will be minted during this instruction
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.redeemable_mint_bump,
    )]
    pub redeemable_mint: Box<Account<'info, token::Mint>>,

    /// #6 The `user`'s ATA for the `depository` `collateral_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        seeds = [user.key.as_ref(), token_program.key.as_ref(), depository.load()?.collateral_mint.as_ref()],
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

    /// #8 The Zo Dex Account (Margin) managed by the `depository`
    /// CHECK : Seeds checked. Depository registered
    #[account(
        mut,
        seeds = [depository.key().as_ref(), zo_state.key().as_ref(), ZO_MARGIN_ACCOUNT_NAMESPACE],
        bump = depository.load()?.zo_account_bump,
        seeds::program = zo_program.key()
    )]
    pub zo_account: AccountInfo<'info>,

    /// #9 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    pub zo_state: AccountLoader<'info, State>,

    /// #10 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    #[account(mut)]
    pub zo_state_signer: UncheckedAccount<'info>,

    /// #11 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    #[account(mut)]
    pub zo_cache: UncheckedAccount<'info>,

    /// #12 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    #[account(mut)]
    pub zo_vault: UncheckedAccount<'info>,

    /// #13 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_control: AccountLoader<'info, Control>,

    /// #14 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_open_orders: UncheckedAccount<'info>,

    /// #15 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_dex_market: UncheckedAccount<'info>,

    /// #16 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_req_q: UncheckedAccount<'info>,

    /// #17 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_event_q: UncheckedAccount<'info>,

    /// #18 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_market_bids: UncheckedAccount<'info>,

    /// #19 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_market_asks: UncheckedAccount<'info>,

    /// #20 [ZeroOne CPI] Zo Dex program
    /// CHECK: ZeroOne CPI
    #[account(address = ZO_DEX_PID)]
    pub zo_dex_program: AccountInfo<'info>,

    /// #21 System Program
    pub system_program: Program<'info, System>,

    /// #22 Token Program
    pub token_program: Program<'info, Token>,

    /// #23 Associated Token Program
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// #24 ZeroOne Program
    pub zo_program: Program<'info, zo::program::ZoAbi>,

    /// #25 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<MintWithZoDepository>,
    max_base_quantity: u64,
    max_quote_quantity: u64,
    limit_price: u64,
) -> Result<()> {
    let depository = ctx.accounts.depository.load()?;

    let controller_pda_signer: &[&[&[u8]]] =
        &[&[CONTROLLER_NAMESPACE, &[ctx.accounts.controller.bump]]];
    let depository_pda_signer: &[&[&[u8]]] = &[&[
        ZO_DEPOSITORY_NAMESPACE,
        depository.collateral_mint.as_ref(),
        &[depository.bump],
    ]];
    let perp_info = ctx.accounts.perp_info()?;

    // - 1 [DEPOSIT COLLATERAL ON DEPOSITORY] ---------------------------------

    // - [Converts lots back to native amount]
    let collateral_amount = max_base_quantity
        .checked_mul(perp_info.base_lot_size)
        .ok_or_else(|| error!(UxdError::MathError))?;

    msg!("max_base_quantity {}", max_base_quantity);
    msg!("max_quote_quantity {}", max_quote_quantity);
    msg!("collateral_amount {}", collateral_amount);

    // - [Get the amount of Base Lots for the perp order (odd lots won't be processed)]
    // let max_base_quantity_precise = I80F48::from_num(collateral_amount)
    //     .checked_div(contract_size)
    //     .ok_or_else(|| error!(UxdError::MathError))?
    //     .checked_floor()
    //     .ok_or_else(|| error!(UxdError::MathError))?;

    // // - [Derive native units for transfer]
    // let collateral_deposited_amount = max_base_quantity_precise
    //     .checked_mul(contract_size)
    //     .ok_or_else(|| error!(UxdError::MathError))?
    //     .checked_to_num()
    //     .ok_or_else(|| error!(UxdError::MathError))?;

    // - [Transfers user collateral]
    zo::cpi::deposit(
        ctx.accounts
            .into_deposit_collateral_context()
            .with_signer(depository_pda_signer),
        false,
        collateral_amount,
    )?;

    let dn_position = ctx
        .accounts
        .delta_neutral_position(perp_info.market_index)?;
    // let max_base_quantity = max_base_quantity_precise
    //     .checked_to_num()
    //     .ok_or_else(|| error!(UxdError::MathError))?;

    // msg!("limit_price {:?}", limit_price);
    // msg!("max_base_quantity {:?}", max_base_quantity);

    msg!("dn_position {:?}", dn_position);

    zo::cpi::place_perp_order(
        ctx.accounts
            .into_place_short_perp_order_context()
            .with_signer(depository_pda_signer),
        false,
        limit_price,
        max_base_quantity,
        max_quote_quantity,
        zo::OrderType::FillOrKill,
        u16::MAX,
        u64::MIN,
    )?;

    let dn_position_post = ctx
        .accounts
        .delta_neutral_position(perp_info.market_index)?;

    msg!("dn_position_post {:?}", dn_position_post);

    // Additional backing (minus fees) in native units (base and quote) added to the DN position
    let collateral_short_amount = dist(dn_position.base_size, dn_position_post.base_size);
    let redeemable_mint_amount = dist(dn_position.size, dn_position_post.size);

    // validates that the collateral_amount matches the amount shorted
    if collateral_amount != collateral_short_amount {
        return Err(error!(UxdError::InvalidCollateralDelta));
    }

    // - 4 [MINTS THE HEDGED AMOUNT OF REDEEMABLE (minus fees already accounted for by 01)] --
    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_pda_signer),
        redeemable_mint_amount,
    )?;

    // - [if ATA mint is WSOL, unwrap]
    if depository.collateral_mint == spl_token::native_mint::id() {
        token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;
    }

    // - 5 [UPDATE ACCOUNTING] ------------------------------------------------
    drop(depository);
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
    pub fn into_deposit_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, zo::cpi::accounts::Deposit<'info>> {
        let cpi_accounts = zo::cpi::accounts::Deposit {
            state: self.zo_state.to_account_info(),
            state_signer: self.zo_state_signer.to_account_info(),
            cache: self.zo_cache.to_account_info(),
            authority: self.user.to_account_info(),
            margin: self.zo_account.to_account_info(),
            token_account: self.user_collateral.to_account_info(),
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
            margin: self.zo_account.to_account_info(),
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
        // if open_orders_info.order_count > 0 {
        //     return Err(error!(UxdError::ZOInvalidControlState));
        // }
        Ok(DeltaNeutralPosition {
            size: open_orders_info.native_pc_total,
            base_size: open_orders_info.pos_size,
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
        let depository = &mut self.depository.load_mut()?;
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
    pub fn validate(
        &self,
        max_base_quantity: u64,
        max_quote_quantity: u64,
        limit_price: u64,
    ) -> Result<()> {
        let depository = self.depository.load()?;
        if !depository.is_initialized {
            return Err(error!(UxdError::ZoDepositoryNotInitialized));
        }
        if limit_price <= 0 {
            return Err(error!(UxdError::InvalidLimitPrice));
        }
        if max_base_quantity == 0 || max_quote_quantity == 0 {
            return Err(error!(UxdError::InvalidCollateralAmount));
        }
        // Will error "naturally"
        // if self.user_collateral.amount < collateral_amount {
        //     return Err(error!(UxdError::InsufficientCollateralAmount));
        // }
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
