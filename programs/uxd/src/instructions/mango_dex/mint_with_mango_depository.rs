use crate::error::UxdError;
// use crate::events::MintWithMangoDepositoryEvent;
use crate::mango_utils::derive_order_delta;
use crate::mango_utils::limit_price;
use crate::mango_utils::price_to_lot_price;
use crate::mango_utils::total_perp_base_lot_position;
use crate::mango_utils::PerpInfo;
use crate::Controller;
use crate::MangoDepository;
use crate::COLLATERAL_PASSTHROUGH_NAMESPACE;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use crate::SLIPPAGE_BASIS;
use anchor_comp::mango_markets_v3;
use anchor_comp::mango_markets_v3::MangoMarketV3;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use fixed::types::I80F48;
use mango::matching::OrderType;
use mango::matching::Side;
use mango::state::MangoAccount;
use mango::state::PerpAccount;

/// Takes 24 accounts - 10 used locally - 9 for MangoMarkets CPI - 4 Programs - 1 Sysvar
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
        bump = controller.bump
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #4 UXDProgram on chain account bound to a Controller instance.
    /// The `MangoDepository` manages a MangoAccount for a single Collateral.
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.bump,
        has_one = controller @UxdError::InvalidController,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub depository: Box<Account<'info, MangoDepository>>,

    /// #5 The redeemable mint managed by the `controller` instance
    /// Tokens will be minted during this instruction
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.redeemable_mint_bump,
        constraint = redeemable_mint.key() == controller.redeemable_mint @UxdError::InvalidRedeemableMint
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// #6 The collateral mint and used by the `depository` instance
    #[account(
        constraint = collateral_mint.key() == depository.collateral_mint @UxdError::InvalidCollateralMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #7 The `user`'s ATA for the `depository` `collateral_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        associated_token::mint = collateral_mint,
        associated_token::authority = user,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #8 The `user`'s ATA for the `controller`'s `redeemable_mint`
    /// Will be credited during this instruction
    #[account(
        init_if_needed,
        associated_token::mint = redeemable_mint,
        associated_token::authority = user,
        payer = payer,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #9 The `depository`'s TA for its `insurance_mint`
    /// MangoAccounts can only transact with the TAs owned by their authority
    /// and this only serves as a passthrough
    #[account(
        mut,
        seeds = [COLLATERAL_PASSTHROUGH_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.collateral_passthrough_bump,
        constraint = depository.collateral_passthrough == depository_collateral_passthrough_account.key() @UxdError::InvalidCollateralPassthroughAccount,
        constraint = depository_collateral_passthrough_account.mint == depository.collateral_mint @UxdError::InvalidCollateralPassthroughATAMint
    )]
    pub depository_collateral_passthrough_account: Box<Account<'info, TokenAccount>>,

    /// #10 The MangoMarkets Account (MangoAccount) managed by the `depository`
    /// CHECK : Seeds checked. Depository registered
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.mango_account_bump,
        constraint = depository.mango_account == depository_mango_account.key() @UxdError::InvalidMangoAccount,
    )]
    pub depository_mango_account: AccountInfo<'info>,

    /// #11 [MangoMarkets CPI] Index grouping perp and spot markets
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_group: UncheckedAccount<'info>,

    /// #12 [MangoMarkets CPI] Cache
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_cache: UncheckedAccount<'info>,

    /// #13 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_root_bank: UncheckedAccount<'info>,

    /// #14 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_node_bank: UncheckedAccount<'info>,

    /// #15 [MangoMarkets CPI] Vault for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_vault: UncheckedAccount<'info>,

    /// #16 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_perp_market: UncheckedAccount<'info>,

    /// #17 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook bids
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_bids: UncheckedAccount<'info>,

    /// #18 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook asks
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_asks: UncheckedAccount<'info>,

    /// #19 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market event queue
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_event_queue: UncheckedAccount<'info>,

    /// #20 System Program
    pub system_program: Program<'info, System>,

    /// #21 Token Program
    pub token_program: Program<'info, Token>,

    /// #22 Associated Token Program
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// #23 MangoMarketv3 Program
    pub mango_program: Program<'info, MangoMarketV3>,

    /// #24 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<MintWithMangoDepository>,
    collateral_amount: u64,
    slippage: u32,
) -> Result<()> {
    let depository_pda_signer: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        ctx.accounts.depository.collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];
    let controller_pda_signer: &[&[&[u8]]] =
        &[&[CONTROLLER_NAMESPACE, &[ctx.accounts.controller.bump]]];

    // - 1 [FIND BEST ORDER FOR SHORT PERP POSITION] --------------------------

    // - [Get MangoMarkets  Collateral-Perp information]
    let perp_info = ctx.accounts.perpetual_info()?;

    // - [Get the amount of Base Lots for the perp order (odd lots won't be processed)]
    let max_base_quantity = I80F48::from_num(collateral_amount)
        .checked_div(perp_info.base_lot_size)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_floor()
        .ok_or_else(|| error!(UxdError::MathError))?;

    // - 2 [TRANSFER COLLATERAL TO MANGO (LONG)] ------------------------------
    // It's the amount we are depositing on the MangoAccount and that will be used as collateral
    // to open the short perp
    let deposited_collateral = max_base_quantity
        .checked_mul(perp_info.base_lot_size)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_to_num()
        .ok_or_else(|| error!(UxdError::MathError))?;

    // - [Transferring user collateral to the passthrough account]
    token::transfer(
        ctx.accounts
            .into_transfer_user_collateral_to_passthrough_context(),
        deposited_collateral,
    )?;

    // - [MangoMarkets CPI - Deposit collateral to Depository MangoAccount]
    mango_markets_v3::deposit(
        ctx.accounts
            .into_deposit_to_mango_context()
            .with_signer(depository_pda_signer),
        deposited_collateral,
    )?;

    // - 3 [OPEN SHORT PERP] --------------------------------------------------

    // - [Perp account state PRE perp order]
    let pre_pa = ctx.accounts.perp_account(&perp_info)?;

    // Note : Augment the delta neutral position, increasing short exposure, by selling perp.
    //        [BID: maker | ASK: taker (us, the caller)]
    let taker_side = Side::Ask;
    let limit_price = limit_price(perp_info.price, slippage, taker_side)?;
    let limit_price_lot = price_to_lot_price(limit_price, &perp_info)?;
    // - [MangoMarkets CPI - Place perp order]
    mango_markets_v3::place_perp_order2(
        ctx.accounts
            .into_open_mango_short_perp_context()
            .with_signer(depository_pda_signer),
        taker_side,
        limit_price_lot.to_num(),
        max_base_quantity.to_num(),
        i64::MAX,
        0,
        OrderType::ImmediateOrCancel,
        false,
        None,
        10,
    )?;

    // - [Perp account state POST perp order]
    let post_pa = ctx.accounts.perp_account(&perp_info)?;

    // - [Checks that the order was fully filled (FoK)]
    let post_perp_order_base_position = total_perp_base_lot_position(&post_pa)?;
    let pre_perp_order_base_position = total_perp_base_lot_position(&pre_pa)?;
    check_perp_order_fully_filled(
        max_base_quantity,
        pre_perp_order_base_position,
        post_perp_order_base_position,
    )?;

    // - 3 [CHECK REDEEMABLE SOFT CAP OVERFLOW] -------------------------------

    // ensure current context is valid as the derive_order_delta is generic
    if pre_pa.taker_quote < post_pa.taker_quote {
        error!(UxdError::InvalidOrderDirection);
    }
    let order_delta = derive_order_delta(&pre_pa, &post_pa, &perp_info)?;
    msg!("order_delta {:?}", order_delta);

    // The resulting UXD amount is equal to the quote delta minus the fees.
    // By sending the amount less the fees, the user is paying them.
    let redeemable_delta = order_delta
        .quote
        .checked_sub(order_delta.fee)
        .ok_or_else(|| error!(UxdError::MathError))?;
    let mint_amount = redeemable_delta
        .checked_to_num()
        .ok_or_else(|| error!(UxdError::MathError))?;

    ctx.accounts
        .check_mango_depositories_redeemable_soft_cap_overflow(mint_amount)?;

    // validate that the deposited_collateral matches the amount shorted
    if deposited_collateral != order_delta.base.unsigned_abs() {
        return Err(error!(UxdError::InvalidCollateralDelta));
    }

    // - 4 [MINTS THE HEDGED AMOUNT OF REDEEMABLE (minus fees)] ----------------
    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_pda_signer),
        mint_amount,
    )?;

    // - [if ATA mint is WSOL, unwrap]
    if ctx.accounts.depository.collateral_mint == spl_token::native_mint::id() {
        token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;
    }

    // - 5 [UPDATE ACCOUNTING] ------------------------------------------------
    ctx.accounts
        .update_onchain_accounting(order_delta.base, redeemable_delta, order_delta.fee);

    // - 6 [CHECK GLOBAL REDEEMABLE SUPPLY CAP OVERFLOW] ----------------------
    ctx.accounts.check_redeemable_global_supply_cap_overflow()?;

    // emit!(MintWithMangoDepositoryEvent {
    //     version: ctx.accounts.controller.version,
    //     controller: ctx.accounts.controller.key(),
    //     depository: ctx.accounts.depository.key(),
    //     user: ctx.accounts.user.key(),
    //     collateral_amount,
    //     slippage,
    //     base_delta: order_delta.base.to_num(),
    //     quote_delta: order_delta.quote.to_num(),
    //     fee_delta: order_delta.fee.to_num(),
    // });

    Ok(())
}

impl<'info> MintWithMangoDepository<'info> {
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

    pub fn into_deposit_to_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Deposit<'info>> {
        let cpi_accounts = mango_markets_v3::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_root_bank.to_account_info(),
            node_bank: self.mango_node_bank.to_account_info(),
            vault: self.mango_vault.to_account_info(),
            owner_token_account: self
                .depository_collateral_passthrough_account
                .to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_open_mango_short_perp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::PlacePerpOrder2<'info>> {
        let cpi_accounts = mango_markets_v3::PlacePerpOrder2 {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
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
            &self.depository_mango_account,
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
            &self.depository_mango_account,
            self.mango_program.key,
            self.mango_group.key,
        )
        .map_err(|me| ProgramError::from(me))?;
        Ok(mango_account.perp_accounts[perp_info.market_index])
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

    fn check_mango_depositories_redeemable_soft_cap_overflow(
        &self,
        redeemable_delta: u64,
    ) -> Result<()> {
        if redeemable_delta > self.controller.mango_depositories_redeemable_soft_cap {
            return Err(error!(UxdError::MangoDepositoriesSoftCapOverflow));
        }
        Ok(())
    }

    // Update the accounting in the Depository and Controller Accounts to reflect changes
    fn update_onchain_accounting(
        &mut self,
        collateral_delta: I80F48,
        redeemable_delta: I80F48,
        fee_delta: I80F48,
    ) {
        // Mango Depository
        self.depository.collateral_amount_deposited = self
            .depository
            .collateral_amount_deposited
            .wrapping_add(collateral_delta.to_num());
        self.depository.redeemable_amount_under_management = self
            .depository
            .redeemable_amount_under_management
            .wrapping_add(redeemable_delta.to_num());
        self.depository.total_amount_paid_taker_fee = self
            .depository
            .total_amount_paid_taker_fee
            .wrapping_add(fee_delta.abs().to_num());
        // Controller
        self.controller.redeemable_circulating_supply = self
            .controller
            .redeemable_circulating_supply
            .wrapping_add(redeemable_delta.to_num());
    }
}

// Verify that the order quantity matches the base position delta
fn check_perp_order_fully_filled(
    order_quantity: I80F48,
    pre_position: I80F48,
    post_position: I80F48,
) -> Result<()> {
    let filled_amount = (post_position
        .checked_sub(pre_position)
        .ok_or_else(|| error!(UxdError::MathError))?)
    .checked_abs()
    .ok_or_else(|| error!(UxdError::MathError))?;
    if order_quantity != filled_amount {
        return Err(error!(UxdError::PerpOrderPartiallyFilled));
    }
    Ok(())
}

// Validate input arguments
impl<'info> MintWithMangoDepository<'info> {
    pub fn validate(&self, collateral_amount: u64, slippage: u32) -> Result<()> {
        if slippage == 0 || slippage >= SLIPPAGE_BASIS {
            return Err(error!(UxdError::InvalidSlippage));
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
