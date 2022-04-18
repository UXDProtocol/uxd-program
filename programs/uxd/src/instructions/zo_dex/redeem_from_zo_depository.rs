use crate::error::UxdError;
use crate::zo_utils::dist;
use crate::zo_utils::DeltaNeutralPosition;
use crate::zo_utils::PerpInfo;
use crate::Controller;
use crate::ZoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use crate::ZO_DEPOSITORY_NAMESPACE;
use crate::ZO_MARGIN_ACCOUNT_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_lang::require;
use anchor_spl::token;
use anchor_spl::token::Burn;
use anchor_spl::token::CloseAccount;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;
use zo::Control;
use zo::FeeTier;
use zo::PerpType;
use zo::State;
use zo_abi as zo;

/// Takes 25 accounts - 9 used locally - 11 for ZoMarkets CPI - 4 Programs - 1 Sysvar
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
        bump = controller.load()?.bump,
        has_one = redeemable_mint @UxdError::InvalidRedeemableMint,
        constraint = controller.load()?.registered_zo_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4 UXDProgram on chain account bound to a Controller instance.
    /// The `ZoDepository` manages a ZoAccount for a single Collateral.
    #[account(
        mut,
        seeds = [ZO_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = zo_account @UxdError::InvalidZoAccount,
        has_one = zo_dex_market @UxdError::InvalidDexMarket,
    )]
    pub depository: AccountLoader<'info, ZoDepository>,

    /// #5 The redeemable mint managed by the `controller` instance
    /// Tokens will be burnt during this instruction
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.load()?.redeemable_mint_bump,
    )]
    pub redeemable_mint: Account<'info, token::Mint>,

    /// #6 The `user`'s ATA for the `depository`'s `collateral_mint`
    /// Will be credited during this instruction
    #[account(
        mut,
        constraint = user_collateral.mint == depository.load()?.collateral_mint @UxdError::InvalidCollateralMint,
        constraint = &user_collateral.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #7 The `user`'s ATA for the `controller`'s `redeemable_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        constraint = user_redeemable.mint == redeemable_mint.key() @UxdError::InvalidCollateralMint,
        constraint = &user_redeemable.owner == user.key @UxdError::InvalidOwner,
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
    #[account(mut)]
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
    // #[account(address = ZO_DEX_PID)]
    pub zo_dex_program: AccountInfo<'info>,

    /// #21 System Program
    pub system_program: Program<'info, System>,

    /// #22 Token Program
    pub token_program: Program<'info, Token>,

    /// #23 ZeroOne Program
    pub zo_program: Program<'info, zo::program::ZoAbi>,

    /// #24 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<RedeemFromZoDepository>,
    max_base_quantity: u64,
    max_quote_quantity: u64,
    limit_price: u64,
) -> Result<()> {
    let depository = ctx.accounts.depository.load()?;

    let collateral_mint = depository.collateral_mint;
    let depository_pda_signer: &[&[&[u8]]] = &[&[
        ZO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[depository.bump],
    ]];
    let perp_info = PerpInfo::new(
        ctx.accounts.zo_state.load()?,
        ctx.accounts.zo_dex_market.key,
    )?;
    drop(depository);

    // - 1 [CLOSE THE EQUIVALENT PERP SHORT ON ZO] -------------------------

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

    // msg!("dn_position {:?}", dn_position);

    // - [Place perp order to close a part of the short perp position]
    solana_program::log::sol_log_compute_units();
    zo::cpi::place_perp_order_lite(
        ctx.accounts
            .into_close_short_perp_position_context()
            .with_signer(depository_pda_signer),
        true,
        limit_price,
        max_base_quantity,
        max_quote_quantity,
        zo::OrderType::FillOrKill,
        u16::MAX,
        u64::MIN,
    )?;
    solana_program::log::sol_log_compute_units();

    // - [Delta neutral position POST perp order]
    let dn_position_post = ctx
        .accounts
        .delta_neutral_position(perp_info.market_index)?;

    // msg!("dn_position_post {:?}", dn_position_post);

    // Amount of DN position unwinded
    let collateral_short_closed_amount = dist(dn_position.base_size, dn_position_post.base_size);
    // The fees are factored in, the position is reduced (+), minus fees on top.
    let perp_order_notional_size =
        I80F48::checked_from_num(dist(dn_position.size, dn_position_post.size)).unwrap();

    // Position is -100 quote notional size pre instruction
    //      (because the position is short, hence negative. Redeem is an actual +, and fee are -, they go in
    //       opposite directions hence need to do some accounting manually to decide how much to burn)
    // Fees are 1% for the example, and assuming 0 slippage.
    //
    // WRONG | user redeem 50, client sends 50 as redeem value
    //      - position becomes -50.5 (-100 + 50 - 0.5, as fees were -0.5)
    //      - we burn 50 UXD, and user receives base equivalent of 50 quote.
    //      => we gave the user 50 when he should have gotten 49.5 and 0.5 should have been burnt to capture the fees in the DN position
    // RIGHT | user redeem 50, client sends 49.5 as redeem value,
    //      - position becomes -50.995 (-100 + 49.5 - 0.495, as fees were -0.495) (this is quote value of the position, meld with funding for instance. Base value always equal to spot collateral deposited (minus interests))
    //      - we burn 49.995 UXD and user receive base equivalent to 49.5 quote with 0.005 UXD dust leftover on his initial 50
    //      => out of the initial 50 UXD, 49.995 are burnt, 0.005 back to user.
    //          => Out of the burnt one, 49.5 have been converted to collateral, and 0.495 are still in the DN position as fee, and will get rebalanced later
    //               (as the accounting for uxd circulating has been reduced by 49.995 and not just 49.5)
    //      => we gave the user 49.5 value of collateral, he paid 0.495 fees for that out of his 50 UXD, ends up with max amount possible + UXD dust leftover to be square, without over drafting on the initial UXD amount.

    // Initial perp position is -100$ (the short part of the DN pos).
    // Redeem 50 UXD is about closing -50$ notional size of short position and seeing how much that moves the base_size of the short position, then return that base_size diff to the user.
    // When closing -50$ notional size of short position, it does a +(50$ - fees$) on the initial position of -100$
    // So the distance between short_position_pre_close and short_position_post_close if less than 50, and represent the amount - fees

    // Calculate the reverse % to find the amount of fee paid (it has been "billed" on the short position quote)
    // so we now need to burn that extra amount)
    let taker_rate = I80F48::from(zo::taker_rate(PerpType::Future, FeeTier::MSRM));
    let taker_rate_base = I80F48::from(zo::taker_rate(PerpType::Future, FeeTier::Base));
    // Fee ratio is 99.996% as the order size is +x and fees are -y (different directions, so the diff is amount - fees)
    let fee_ratio = I80F48::ONE - (taker_rate / taker_rate_base);
    // perp_order_notional_size represent <100% of the amount, it's the absolute value of [-position + (amount - 0.004%)],
    //   dividing it by 0.996 bring it back to original value with fees.
    let amount_with_fees = perp_order_notional_size / fee_ratio;
    // Then we can single out how much the fee amount is
    let fee_amount: u64 = amount_with_fees
        .checked_sub(perp_order_notional_size)
        .unwrap()
        .checked_to_num()
        .unwrap();
    // And derive what is the amount to burn.
    // Burning an amount superior to the base returned effectively capture the fees cost in the delta neutral position, to be harvested later by the protocol. In the end it's unaccounted for in the depository.redeemable_under_management, so it gets rebalanced and end up in the USDC balance
    //  If the user want to redeem 100, client must do 100 - (100 * fee_percentage) and send us this value, so that we are sure there is enough UXD to burn. That also create the issue of dust, to fix later.
    let redeemable_burn_amount = amount_with_fees.checked_to_num().unwrap(); // or perp_order_notional_size + fee_amount;

    // validates that the collateral_amount matches the amount shorted
    require!(
        collateral_amount == collateral_short_closed_amount,
        UxdError::InvalidCollateralDelta
    );

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
    if collateral_mint == spl_token::native_mint::id() {
        token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;
    }

    // - 4 [UPDATE ACCOUNTING] ------------------------------------------------
    let controller = &mut ctx.accounts.controller.load_mut()?;
    let depository = &mut ctx.accounts.depository.load_mut()?;

    depository.collateral_amount_deposited = depository
        .collateral_amount_deposited
        .checked_sub(collateral_amount.into())
        .ok_or_else(|| error!(UxdError::MathError))?;
    depository.redeemable_amount_under_management = depository
        .redeemable_amount_under_management
        .checked_add(redeemable_burn_amount.into())
        .ok_or_else(|| error!(UxdError::MathError))?;
    depository.total_amount_paid_taker_fee = depository
        .total_amount_paid_taker_fee
        .checked_add(fee_amount.into())
        .ok_or_else(|| error!(UxdError::MathError))?;
    // Controller
    controller.redeemable_circulating_supply = controller
        .redeemable_circulating_supply
        .checked_add(redeemable_burn_amount.into())
        .ok_or_else(|| error!(UxdError::MathError))?;

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
    pub fn into_close_short_perp_position_context(
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
    fn delta_neutral_position(&self, index: usize) -> Result<DeltaNeutralPosition> {
        let control_account = self.zo_control.load()?;
        let open_orders_info = control_account
            .open_orders_agg
            .get(index)
            .ok_or_else(|| error!(UxdError::ZOOpenOrdersInfoNotFound))?;
        Ok(DeltaNeutralPosition::try_from(open_orders_info)?)
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

        require!(
            depository.is_initialized,
            UxdError::ZoDepositoryNotInitialized
        );
        require!(limit_price > 0, UxdError::InvalidLimitPrice);
        require!(max_base_quantity > 0, UxdError::InvalidMaxBaseQuantity);
        require!(max_quote_quantity > 0, UxdError::InvalidMaxQuoteQuantity);
        Ok(())
    }
}
