use crate::error::UxdError;
use crate::zo_utils::DeltaNeutralPosition;
use crate::zo_utils::PerpInfo;
use crate::Controller;
use crate::ZoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use crate::ZO_DEPOSITORY_NAMESPACE;
use crate::ZO_MARGIN_ACCOUNT_NAMESPACE;
use crate::zo_utils::dist;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Burn;
use anchor_spl::token::CloseAccount;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use zo::Control;
use zo::State;
use zo::ZO_DEX_PID;
use zo_abi as zo;

/// Takes 26 accounts - 9 used locally - 11 for ZoMarkets CPI - 5 Programs - 1 Sysvar
#[derive(Accounts)]
pub struct RedeemFromZoDepository<'info> {
    /// #1 Public call accessible to any user
    /// Note - Mut required for WSOL unwrapping
    #[account(mut)]
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
        has_one = zo_dex_market @UxdError::InvalidDexMarket,
        has_one = collateral_mint @UxdError::InvalidCollateralMint
    )]
    pub depository: AccountLoader<'info, ZoDepository>,

    /// #5 The collateral mint used by the `depository` instance
    /// Required to create the user_collateral ATA if needed
    pub collateral_mint: Box<Account<'info, token::Mint>>,

    /// #6 The redeemable mint managed by the `controller` instance
    /// Tokens will be burnt during this instruction
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.redeemable_mint_bump,
    )]
    pub redeemable_mint: Box<Account<'info, token::Mint>>,

    /// #7 The `user`'s ATA for the `depository`'s `collateral_mint`
    /// Will be credited during this instruction
    #[account(
        init_if_needed,
        associated_token::mint = collateral_mint,
        associated_token::authority = user,
        payer = payer,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #8 The `user`'s ATA for the `controller`'s `redeemable_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        seeds = [user.key.as_ref(), token_program.key.as_ref(), controller.redeemable_mint.as_ref()],
        bump,
        seeds::program = AssociatedToken::id(),
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #9 The Zo Dex Account (Margin) managed by the `depository`
    /// CHECK : Seeds checked. Depository registered
    #[account(
        mut,
        seeds = [depository.key().as_ref(), zo_state.key().as_ref(), ZO_MARGIN_ACCOUNT_NAMESPACE],
        bump = depository.load()?.zo_account_bump,
        seeds::program = zo_program.key()
    )]
    pub zo_account: AccountInfo<'info>,

    /// #10 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    pub zo_state: AccountLoader<'info, State>,

    /// #11 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    #[account(mut)]
    pub zo_state_signer: UncheckedAccount<'info>,

    /// #12 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    #[account(mut)]
    pub zo_cache: UncheckedAccount<'info>,

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
    ctx: Context<RedeemFromZoDepository>,
    max_base_quantity: u64,
    max_quote_quantity: u64,
    limit_price: u64,
) -> Result<()> {
    let depository = ctx.accounts.depository.load()?;
    let depository_pda_signer: &[&[&[u8]]] = &[&[
        ZO_DEPOSITORY_NAMESPACE,
        depository.collateral_mint.as_ref(),
        &[depository.bump],
    ]];
    let perp_info = ctx.accounts.perp_info()?;

    // - 1 [CLOSE THE EQUIVALENT PERP SHORT ON ZO] -------------------------

    // NEED TO OFFSET FOR THE FEES here.
    // User wants to redeem 100 UXD
    // We will burn 100 UXD, but will redeem 100 - fees (and slippage will be in already)
    // This way the fees are on the user and will be extracted during rebalancing

    // // - [Calculates the quantity of short to close]
    // let quote_exposure_delta = I80F48::from_num(redeemable_amount);

    // // - [Find the max taker fees mango will take on the perp order and remove it from the exposure delta to be sure the amount order + fees don't overflow the redeemed amount]
    // let max_fee_amount = quote_exposure_delta
    //     .checked_mul(perp_info.effective_fee)
    //     .ok_or_else(|| error!(UxdError::MathError))?
    //     .checked_ceil()
    //     .ok_or_else(|| error!(UxdError::MathError))?;
    // let quote_exposure_delta_minus_fees = quote_exposure_delta
    //     .checked_sub(max_fee_amount)
    //     .ok_or_else(|| error!(UxdError::MathError))?;

    // - [Converts lots back to native amount]
    let collateral_amount = max_base_quantity
        .checked_mul(perp_info.base_lot_size)
        .ok_or_else(|| error!(UxdError::MathError))?;

    // msg!("max_base_quantity {}", max_base_quantity);
    // msg!("max_quote_quantity {}", max_quote_quantity);
    // msg!("collateral_amount {}", collateral_amount);

    // - [Delta neutral position PRE perp order]
    let dn_position = ctx
        .accounts
        .delta_neutral_position(perp_info.market_index)?;

    msg!("dn_position {:?}", dn_position);

    // - [Place perp order to close a part of the short perp position]
    zo::cpi::place_perp_order(
        ctx.accounts
            .into_close_short_perp_context()
            .with_signer(depository_pda_signer),
        true,
        limit_price,
        max_base_quantity,
        max_quote_quantity,
        zo::OrderType::FillOrKill,
        u16::MAX,
        u64::MIN,
    )?;

    // - [Delta neutral position POST perp order]
    let dn_position_post = ctx
        .accounts
        .delta_neutral_position(perp_info.market_index)?;

    msg!("dn_position_post {:?}", dn_position_post);

    // Additional backing (minus fees) in native units (base and quote) added to the DN position
    let collateral_short_closed_amount = dist(dn_position.base_size, dn_position_post.base_size);
    // NEED to be more, edit
    let redeemable_burn_amount = dist(dn_position.size, dn_position_post.size);

    // validates that the collateral_amount matches the amount shorted
    if collateral_amount != collateral_short_closed_amount {
        return Err(error!(UxdError::InvalidCollateralDelta));
    }

    // - 2 [BURN REDEEMABLES] -------------------------------------------------
    token::burn(
        ctx.accounts.into_burn_redeemable_context(),
        redeemable_burn_amount,
    )?;

    // - 3 [WITHDRAW COLLATERAL FROM ZO] --------------------------------------
    // - [Transfers depository collateral to user]
    zo::cpi::withdraw(
        ctx.accounts
            .into_withdraw_collateral_context()
            .with_signer(depository_pda_signer),
        false,
        collateral_amount,
    )?;

    // - [If ATA mint is WSOL, unwrap]
    if depository.collateral_mint == spl_token::native_mint::id() {
        token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;
    }

    // - 4 [UPDATE ACCOUNTING] ------------------------------------------------
    drop(depository);
    ctx.accounts
        .update_onchain_accounting(collateral_amount.into(), redeemable_burn_amount.into())?;

    // emit!(RedeemFromDepositoryEvent {
    //     version: controller.version,
    //     controller: controller.key(),
    //     depository: depository.key(),
    //     user: ctx.accounts.user.key(),
    //     redeemable_amount,
    //     limit_price,
    //     base_delta: order_delta.base.to_num(),
    //     quote_delta: order_delta.quote.to_num(),
    //     fee_delta: order_delta.fee.to_num(),
    // });

    Ok(())
}

// MARK: - Contexts -----

impl<'info> RedeemFromZoDepository<'info> {
    pub fn into_close_short_perp_context(
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

    pub fn into_burn_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.redeemable_mint.to_account_info(),
            to: self.user_redeemable.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_withdraw_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, zo::cpi::accounts::Withdraw<'info>> {
        let cpi_accounts = zo::cpi::accounts::Withdraw {
            state: self.zo_state.to_account_info(),
            state_signer: self.zo_state_signer.to_account_info(),
            cache: self.zo_cache.to_account_info(),
            authority: self.depository.to_account_info(),
            margin: self.zo_account.to_account_info(),
            control: self.zo_control.to_account_info(),
            token_account: self.user_collateral.to_account_info(),
            vault: self.zo_vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.zo_program.to_account_info();
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
impl<'info> RedeemFromZoDepository<'info> {
    // Return general information about the perpetual related to the collateral in use
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

    // Update the accounting in the Depository and Controller Accounts to reflect changes
    fn update_onchain_accounting(
        &mut self,
        collateral_withdrawn_amount: u128,
        redeemable_burnt_amount: u128,
    ) -> Result<()> {
        let depository = &mut self.depository.load_mut()?;
        let controller = &mut self.controller;
        // Mango Depository
        depository.collateral_amount_deposited = depository
            .collateral_amount_deposited
            .checked_sub(collateral_withdrawn_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.redeemable_amount_under_management = depository
            .redeemable_amount_under_management
            .checked_add(redeemable_burnt_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        // Controller
        controller.redeemable_circulating_supply = controller
            .redeemable_circulating_supply
            .checked_add(redeemable_burnt_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        Ok(())
    }
}

// Validate input arguments
impl<'info> RedeemFromZoDepository<'info> {
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
        if limit_price == 0 {
            return Err(error!(UxdError::InvalidLimitPrice));
        }
        if max_base_quantity == 0 || max_quote_quantity == 0 {
            return Err(error!(UxdError::InvalidCollateralAmount));
        }
        Ok(())
    }
}
