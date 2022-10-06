use crate::events::MintWithMangoDepositoryEvent;
use crate::mango_utils::derive_order_delta;
use crate::mango_utils::price_to_lot_price;
use crate::mango_utils::total_perp_base_lot_position;
use crate::mango_utils::PerpInfo;
use crate::validate_mango_group;
use crate::validate_perp_market_mints_matches_depository_mints;
use crate::Controller;
use crate::MangoDepository;
use crate::UxdError;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::MANGO_PERP_MAX_FILL_EVENTS;
use crate::REDEEMABLE_MINT_NAMESPACE;
use anchor_comp::mango_markets_v3;
use anchor_comp::mango_markets_v3::MangoMarketV3;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;
use mango::matching::OrderType;
use mango::matching::Side;
use mango::state::MangoAccount;
use mango::state::PerpAccount;

/// Takes 20 accounts
#[derive(Accounts)]
pub struct MintWithMangoDepository<'info> {
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

    /// #4 UXDProgram on chain account bound to a Controller instance.
    /// The `MangoDepository` manages a MangoAccount for a single Collateral.
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = mango_account @UxdError::InvalidMangoAccount,
    )]
    pub depository: AccountLoader<'info, MangoDepository>,

    /// #5 The redeemable mint managed by the `controller` instance
    /// Tokens will be minted during this instruction
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.load()?.redeemable_mint_bump,
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// #6 The `user`'s TA for the `depository` `collateral_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        constraint = user_collateral.mint == depository.load()?.collateral_mint @UxdError::InvalidCollateralMint,
        constraint = &user_collateral.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #7 The `user`'s TA for the `controller`'s `redeemable_mint`
    /// Will be credited during this instruction
    #[account(
        mut,
        constraint = user_redeemable.mint == controller.load()?.redeemable_mint @UxdError::InvalidRedeemableMint,
        constraint = &user_redeemable.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #8 The MangoMarkets Account (MangoAccount) managed by the `depository`
    /// CHECK : Seeds checked. Depository registered
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.mango_account_bump,
    )]
    pub mango_account: AccountInfo<'info>,

    /// #9 [MangoMarkets CPI] Index grouping perp and spot markets
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_group: UncheckedAccount<'info>,

    /// #10 [MangoMarkets CPI] Cache
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_cache: UncheckedAccount<'info>,

    /// #11 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_root_bank: UncheckedAccount<'info>,

    /// #12 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_node_bank: UncheckedAccount<'info>,

    /// #13 [MangoMarkets CPI] Vault for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_vault: UncheckedAccount<'info>,

    /// #14 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_perp_market: UncheckedAccount<'info>,

    /// #15 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook bids
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_bids: UncheckedAccount<'info>,

    /// #16 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook asks
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_asks: UncheckedAccount<'info>,

    /// #17 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market event queue
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_event_queue: UncheckedAccount<'info>,

    /// #18 System Program
    pub system_program: Program<'info, System>,

    /// #19 Token Program
    pub token_program: Program<'info, Token>,

    /// #20 MangoMarketv3 Program
    pub mango_program: Program<'info, MangoMarketV3>,
}

pub(crate) fn handler(
    ctx: Context<MintWithMangoDepository>,
    collateral_amount: u64,
    limit_price: f32,
) -> Result<()> {
    let depository = ctx.accounts.depository.load()?;
    let collateral_mint = depository.collateral_mint;
    let depository_bump = depository.bump;
    drop(depository);

    let depository_pda_signer: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[depository_bump],
    ]];
    let controller_bump = ctx.accounts.controller.load()?.bump;
    let controller_pda_signer: &[&[&[u8]]] = &[&[CONTROLLER_NAMESPACE, &[controller_bump]]];

    // - 1 [FIND BEST ORDER FOR SHORT PERP POSITION] --------------------------

    // - [Get MangoMarkets  Collateral-Perp information]
    let perp_info = ctx.accounts.perpetual_info()?;
    let contract_size = perp_info.base_lot_size;

    // - [Get the amount of Base Lots for the perp order (odd lots won't be processed)]
    let max_base_quantity = I80F48::from_num(collateral_amount)
        .checked_div(contract_size)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_floor()
        .ok_or_else(|| error!(UxdError::MathError))?;

    require!(
        !max_base_quantity.is_zero(),
        UxdError::QuantityBelowContractSize
    );

    // - 2 [TRANSFER COLLATERAL TO MANGO (LONG)] ------------------------------
    // It's the amount we are depositing on the MangoAccount and that will be used as collateral
    // to open the short perp
    let collateral_deposited_amount = max_base_quantity
        .checked_mul(contract_size)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_to_num()
        .ok_or_else(|| error!(UxdError::MathError))?;

    // - [MangoMarkets CPI - Deposit collateral to Depository MangoAccount]
    mango_markets_v3::deposit(
        ctx.accounts
            .to_deposit_collateral_to_mango_context()
            .with_signer(depository_pda_signer),
        collateral_deposited_amount,
    )?;

    // - 3 [OPEN SHORT PERP] --------------------------------------------------

    // - [Perp account state PRE perp order]
    let pre_pa = ctx.accounts.perp_account(&perp_info)?;

    // Note : Augment the delta neutral position, increasing short exposure, by selling perp.
    //        [BID: maker | ASK: taker (us, the caller)]
    let taker_side = Side::Ask;
    let limit_price_fixed =
        I80F48::checked_from_num(limit_price).ok_or_else(|| error!(UxdError::MathError))?;
    let limit_price_lot = price_to_lot_price(limit_price_fixed, &perp_info)?;
    let max_base_quantity_num = max_base_quantity.to_num();
    // - [MangoMarkets CPI - Place perp order]
    mango_markets_v3::place_perp_order2(
        ctx.accounts
            .to_open_mango_short_perp_context()
            .with_signer(depository_pda_signer),
        taker_side,
        limit_price_lot.to_num(),
        max_base_quantity_num,
        i64::MAX,
        0,
        OrderType::ImmediateOrCancel,
        false,
        None,
        MANGO_PERP_MAX_FILL_EVENTS,
    )?;

    // - [Perp account state POST perp order]
    let post_pa = ctx.accounts.perp_account(&perp_info)?;

    // - [Checks that the order was fully filled (FoK)]
    check_perp_order_fully_filled(max_base_quantity_num, &pre_pa, &post_pa)?;

    // - 3 [CHECK REDEEMABLE SOFT CAP OVERFLOW] -------------------------------

    // ensure current context is valid as the derive_order_delta is generic
    require!(
        pre_pa.taker_quote <= post_pa.taker_quote,
        UxdError::InvalidOrderDirection
    );
    let order_delta = derive_order_delta(&pre_pa, &post_pa, &perp_info)?;
    msg!("order_delta {:?}", order_delta);

    // The resulting UXD amount is equal to the quote delta minus the fees.
    // By sending the amount less the fees, the user is paying them.
    let redeemable_delta = order_delta
        .quote
        .checked_sub(order_delta.fee)
        .ok_or_else(|| error!(UxdError::MathError))?;
    let redeemable_mint_amount = redeemable_delta
        .checked_to_num()
        .ok_or_else(|| error!(UxdError::MathError))?;

    ctx.accounts
        .check_mango_depositories_redeemable_soft_cap_overflow(redeemable_mint_amount)?;

    let collateral_shorted_amount: u64 = order_delta
        .base
        .unsigned_abs()
        .checked_to_num()
        .ok_or_else(|| error!(UxdError::MathError))?;
    // validate that the deposited_collateral matches the amount shorted
    require!(
        collateral_deposited_amount == collateral_shorted_amount,
        UxdError::InvalidCollateralDelta
    );

    // - 4 [MINTS THE HEDGED AMOUNT OF REDEEMABLE (minus fees)] ----------------
    token::mint_to(
        ctx.accounts
            .to_mint_redeemable_context()
            .with_signer(controller_pda_signer),
        redeemable_mint_amount,
    )?;

    // - [if ATA mint is WSOL, unwrap]
    if collateral_mint == spl_token::native_mint::id() {
        token::close_account(ctx.accounts.to_unwrap_wsol_by_closing_ata_context())?;
    }

    // - 5 [UPDATE ACCOUNTING] ------------------------------------------------
    ctx.accounts.update_onchain_accounting(
        collateral_shorted_amount.into(),
        redeemable_mint_amount.into(),
        order_delta.fee.to_num(),
    )?;

    // - 6 [CHECK REDEEMABLE SUPPLY CAPS OVERFLOW] ----------------------
    ctx.accounts.check_redeemable_global_supply_cap_overflow()?;

    ctx.accounts
        .check_redeemable_depository_supply_cap_overflow()?;

    emit!(MintWithMangoDepositoryEvent {
        version: ctx.accounts.controller.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        user: ctx.accounts.user.key(),
        collateral_amount,
        limit_price,
        base_delta: order_delta.base.to_num(),
        quote_delta: order_delta.quote.to_num(),
        fee_delta: order_delta.fee.to_num(),
    });

    Ok(())
}

impl<'info> MintWithMangoDepository<'info> {
    fn to_deposit_collateral_to_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Deposit<'info>> {
        let cpi_accounts = mango_markets_v3::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.user.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_root_bank.to_account_info(),
            node_bank: self.mango_node_bank.to_account_info(),
            vault: self.mango_vault.to_account_info(),
            owner_token_account: self.user_collateral.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn to_open_mango_short_perp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::PlacePerpOrder2<'info>> {
        let cpi_accounts = mango_markets_v3::PlacePerpOrder2 {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            perp_market: self.mango_perp_market.to_account_info(),
            bids: self.mango_bids.to_account_info(),
            asks: self.mango_asks.to_account_info(),
            event_queue: self.mango_event_queue.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
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

    fn to_unwrap_wsol_by_closing_ata_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::CloseAccount<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::CloseAccount {
            account: self.user_collateral.to_account_info(),
            destination: self.user.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Additional convenience methods related to the inputted accounts
impl<'info> MintWithMangoDepository<'info> {
    // Return general information about the perpetual related to the collateral in use
    fn perpetual_info(&self) -> Result<PerpInfo> {
        let perp_info = PerpInfo::new(
            &self.mango_group,
            &self.mango_cache,
            &self.mango_account,
            self.mango_perp_market.key,
            self.mango_group.key,
            self.mango_program.key,
        )?;
        msg!("perp_info {:?}", perp_info);
        Ok(perp_info)
    }

    // Return the PerpAccount that represent the account balances (Quote and Taker, Taker is the part that is waiting settlement)
    fn perp_account(&self, perp_info: &PerpInfo) -> Result<PerpAccount> {
        // - loads Mango's accounts
        let mango_account = MangoAccount::load_checked(
            &self.mango_account,
            self.mango_program.key,
            self.mango_group.key,
        )
        .map_err(ProgramError::from)?;
        Ok(mango_account.perp_accounts[perp_info.market_index])
    }

    // Ensure that the minted amount does not raise the Redeemable supply beyond the Global Redeemable Supply Cap
    fn check_redeemable_global_supply_cap_overflow(&self) -> Result<()> {
        let controller = self.controller.load()?;
        require!(
            controller.redeemable_circulating_supply <= controller.redeemable_global_supply_cap,
            UxdError::RedeemableGlobalSupplyCapReached
        );
        Ok(())
    }

    fn check_redeemable_depository_supply_cap_overflow(&self) -> Result<()> {
        let depository = self.depository.load()?;
        require!(
            depository.redeemable_amount_under_management
                <= depository.redeemable_depository_supply_cap,
            UxdError::RedeemableMangoDepositorySupplyCapReached
        );
        Ok(())
    }

    fn check_mango_depositories_redeemable_soft_cap_overflow(
        &self,
        redeemable_delta: u64,
    ) -> Result<()> {
        let controller = self.controller.load()?;
        require!(
            redeemable_delta <= controller.mango_depositories_redeemable_soft_cap,
            UxdError::MangoDepositoriesSoftCapOverflow
        );
        Ok(())
    }

    // Update the accounting in the Depository and Controller Accounts to reflect changes
    fn update_onchain_accounting(
        &mut self,
        collateral_shorted_amount: u128,
        redeemable_minted_amount: u128,
        fee_amount: u128,
    ) -> Result<()> {
        let depository = &mut self.depository.load_mut()?;
        let controller = &mut self.controller.load_mut()?;
        // Mango Depository
        depository.collateral_amount_deposited = depository
            .collateral_amount_deposited
            .checked_add(collateral_shorted_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.redeemable_amount_under_management = depository
            .redeemable_amount_under_management
            .checked_add(redeemable_minted_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.total_amount_paid_taker_fee = depository
            .total_amount_paid_taker_fee
            .checked_add(fee_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        // Controller
        controller.redeemable_circulating_supply = controller
            .redeemable_circulating_supply
            .checked_add(redeemable_minted_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        Ok(())
    }
}

// Verify that the order quantity matches the base position delta
fn check_perp_order_fully_filled(
    order_quantity: i64,
    pre_pa: &PerpAccount,
    post_pa: &PerpAccount,
) -> Result<()> {
    let pre_position = total_perp_base_lot_position(pre_pa)?;
    let post_position = total_perp_base_lot_position(post_pa)?;
    let filled_amount = (post_position
        .checked_sub(pre_position)
        .ok_or_else(|| error!(UxdError::MathError))?)
    .checked_abs()
    .ok_or_else(|| error!(UxdError::MathError))?;
    require!(
        order_quantity == filled_amount,
        UxdError::PerpOrderPartiallyFilled
    );
    Ok(())
}

// Validate input arguments
impl<'info> MintWithMangoDepository<'info> {
    pub(crate) fn validate(&self, collateral_amount: u64, limit_price: f32) -> Result<()> {
        require!(limit_price > 0f32, UxdError::InvalidLimitPrice);
        require!(collateral_amount != 0, UxdError::InvalidCollateralAmount);
        require!(
            self.user_collateral.amount >= collateral_amount,
            UxdError::InsufficientCollateralAmount
        );
        require!(
            !&self.depository.load()?.regular_minting_disabled,
            UxdError::MintingDisabled
        );

        validate_mango_group(self.mango_group.key())?;

        validate_perp_market_mints_matches_depository_mints(
            &self.mango_group,
            self.mango_program.key,
            self.mango_perp_market.key,
            &self.depository.load()?.collateral_mint,
            &self.depository.load()?.quote_mint,
        )?;
        Ok(())
    }
}
