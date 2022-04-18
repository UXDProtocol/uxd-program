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
use anchor_spl::token::CloseAccount;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;
use zo::Control;
use zo::FeeTier;
use zo::PerpType;
use zo::State;
use zo::ZO_DEX_PID;
use zo_abi as zo;

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
        has_one = zo_dex_market @UxdError::InvalidDexMarket
    )]
    pub depository: AccountLoader<'info, ZoDepository>,

    /// #5 The redeemable mint managed by the `controller` instance
    /// Tokens will be minted during this instruction
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.load()?.redeemable_mint_bump,
    )]
    pub redeemable_mint: Account<'info, token::Mint>,

    /// #6 The `user`'s ATA for the `depository` `collateral_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        constraint = user_collateral.mint == depository.load()?.collateral_mint @UxdError::InvalidCollateralMint,
        constraint = &user_collateral.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #7 The `user`'s ATA for the `controller`'s `redeemable_mint`
    /// Will be credited during this instruction
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
        seeds::program = zo_program.key
    )]
    pub zo_account: AccountInfo<'info>,

    /// #9 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    pub zo_state: AccountLoader<'info, State>,

    /// #10 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    #[account(mut)]
    pub zo_state_signer: AccountInfo<'info>,

    /// #11 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    #[account(mut)]
    pub zo_cache: AccountInfo<'info>,

    /// #12 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    #[account(mut)]
    pub zo_vault: AccountInfo<'info>,

    /// #13 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_control: AccountLoader<'info, Control>,

    /// #14 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_open_orders: AccountInfo<'info>,

    /// #15 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_dex_market: AccountInfo<'info>,

    /// #16 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_req_q: AccountInfo<'info>,

    /// #17 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_event_q: AccountInfo<'info>,

    /// #18 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_market_bids: AccountInfo<'info>,

    /// #19 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide
    #[account(mut)]
    pub zo_market_asks: AccountInfo<'info>,

    /// #20 [ZeroOne CPI] Zo Dex program
    /// CHECK: ZeroOne CPI
    #[account(address = ZO_DEX_PID)]
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
    ctx: Context<MintWithZoDepository>,
    max_base_quantity: u64,
    max_quote_quantity: u64,
    limit_price: u64,
) -> Result<()> {
    let depository = ctx.accounts.depository.load()?;

    let collateral_mint = depository.collateral_mint;
    let controller_bump = ctx.accounts.controller.load()?.bump;
    let controller_pda_signer: &[&[&[u8]]] = &[&[CONTROLLER_NAMESPACE, &[controller_bump]]];
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
    // - 1 [DEPOSIT COLLATERAL ON DEPOSITORY] ---------------------------------

    // - [Converts lots back to native amount]
    let collateral_amount = max_base_quantity
        .checked_mul(perp_info.base_lot_size)
        .ok_or_else(|| error!(UxdError::MathError))?;

    // msg!("max_base_quantity {}", max_base_quantity);
    // msg!("max_quote_quantity {}", max_quote_quantity);
    // msg!("collateral_amount {}", collateral_amount);

    // - [Transfers user collateral]
    zo::cpi::deposit(
        ctx.accounts
            .into_deposit_collateral_context()
            .with_signer(depository_pda_signer),
        false,
        collateral_amount,
    )?;

    // - [Delta neutral position PRE perp order]
    let dn_position = ctx
        .accounts
        .delta_neutral_position(perp_info.market_index)?;

    // msg!("dn_position {:?}", dn_position);

    // - [Place perp order to increase the short perp position]
    zo::cpi::place_perp_order_lite(
        ctx.accounts
            .into_open_short_perp_position_context()
            .with_signer(depository_pda_signer),
        false,
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

    // msg!("dn_position_post {:?}", dn_position_post);

    // Additional backing (minus fees) in native units (base and quote) added to the DN position
    let collateral_short_amount = dist(dn_position.base_size, dn_position_post.base_size);
    // Fees are already deducted unlike mango
    let perp_order_notional_size =
        I80F48::checked_from_num(dist(dn_position.size, dn_position_post.size)).unwrap();

    // Calculate the reverse % to find the amount of fee paid (it has been "billed" on the short position quote)
    let taker_rate = I80F48::from(zo::taker_rate(PerpType::Future, FeeTier::MSRM));
    let taker_rate_base = I80F48::from(zo::taker_rate(PerpType::Future, FeeTier::Base));
    // Fee ratio is 100.004% as the order size is -x and fees are -y (same direction, so the diff is amount + fees)
    let fee_ratio = I80F48::ONE + (taker_rate / taker_rate_base);
    // perp_order_notional_size represent >100% of the amount, it's the absolute value of [-position + (-amount - 0.004%)],
    //   dividing it by 1.004 give us the value without fees
    let amount_without_fees = perp_order_notional_size / fee_ratio;
    // Then we can single out how much the fee amount is
    let fee_amount = perp_order_notional_size
        .checked_sub(amount_without_fees)
        .unwrap()
        .checked_to_num()
        .unwrap();
    // And derive what is the amount to mint.
    //
    // Minting an amount inferior to the base deposited effectively capture the fees cost in the delta neutral position, that meld in protocol PnL. In the end it's unaccounted for in the depository.redeemable_under_management, so it gets rebalanced and end up in the USDC balance
    let redeemable_mint_amount = amount_without_fees.checked_to_num().unwrap(); // or perp_order_notional_size + fee_amount;

    // validates that the collateral_amount matches the amount shorted
    require!(
        collateral_amount == collateral_short_amount,
        UxdError::InvalidCollateralDelta
    );

    // - 4 [MINTS THE HEDGED AMOUNT OF REDEEMABLE (minus fees already accounted for by 01)] --
    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_pda_signer),
        redeemable_mint_amount,
    )?;

    // - [if ATA mint is WSOL, unwrap]
    if collateral_mint == spl_token::native_mint::id() {
        token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;
    }

    // - 5 [UPDATE ACCOUNTING] ------------------------------------------------
    let depository = &mut ctx.accounts.depository.load_mut()?;
    let controller = &mut ctx.accounts.controller.load_mut()?;

    // Mango Depository
    depository.collateral_amount_deposited = depository
        .collateral_amount_deposited
        .checked_add(collateral_short_amount.into())
        .ok_or_else(|| error!(UxdError::MathError))?;
    depository.redeemable_amount_under_management = depository
        .redeemable_amount_under_management
        .checked_add(redeemable_mint_amount.into())
        .ok_or_else(|| error!(UxdError::MathError))?;
    depository.total_amount_paid_taker_fee = depository
        .total_amount_paid_taker_fee
        .checked_add(fee_amount)
        .ok_or_else(|| error!(UxdError::MathError))?;
    // Controller
    controller.redeemable_circulating_supply = controller
        .redeemable_circulating_supply
        .checked_add(redeemable_mint_amount.into())
        .ok_or_else(|| error!(UxdError::MathError))?;

    // - 6 [CHECK GLOBAL REDEEMABLE SUPPLY CAP OVERFLOW] ----------------------
    require!(
        controller.redeemable_circulating_supply <= controller.redeemable_global_supply_cap,
        UxdError::RedeemableGlobalSupplyCapReached
    );

    // emit!(ZoMintEvent {
    //     version: ctx.accounts.controller.version,
    //     controller: ctx.accounts.controller.key(),
    //     depository: ctx.accounts.depository.key(),
    //     user: ctx.accounts.user.key(),
    //     collateral_amount,
    //     collateral_deposited_amount,
    //     limit_price,
    //     minted_amount: redeemable_amount
    // });

    Ok(())
}

impl<'info> MintWithZoDepository<'info> {
    pub fn into_deposit_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, zo::cpi::accounts::Deposit<'info>> {
        let cpi_accounts = zo::cpi::accounts::Deposit {
            state: self.zo_state.to_account_info(),
            state_signer: self.zo_state_signer.clone(),
            cache: self.zo_cache.clone(),
            authority: self.user.to_account_info(),
            margin: self.zo_account.clone(),
            token_account: self.user_collateral.to_account_info(),
            vault: self.zo_vault.clone(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.zo_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_open_short_perp_position_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, zo::cpi::accounts::PlacePerpOrder<'info>> {
        let cpi_accounts = zo::cpi::accounts::PlacePerpOrder {
            state: self.zo_state.to_account_info(),
            state_signer: self.zo_state_signer.clone(),
            cache: self.zo_cache.clone(),
            authority: self.depository.to_account_info(),
            margin: self.zo_account.clone(),
            control: self.zo_control.to_account_info(),
            open_orders: self.zo_open_orders.clone(),
            dex_market: self.zo_dex_market.clone(),
            req_q: self.zo_req_q.clone(),
            event_q: self.zo_event_q.clone(),
            market_bids: self.zo_market_bids.clone(),
            market_asks: self.zo_market_asks.clone(),
            dex_program: self.zo_dex_program.clone(),
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
impl<'info> MintWithZoDepository<'info> {
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
