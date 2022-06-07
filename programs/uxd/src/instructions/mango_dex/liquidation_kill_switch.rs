use crate::MANGO_PERP_MAX_FILL_EVENTS;
use crate::error::UxdError;
use crate::events::DepositInsuranceToDepositoryEvent;
use crate::Controller;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::SAFETY_VAULT_NAMESPACE;
use crate::COLLATERAL_VAULT_NAMESPACE;
use crate::QUOTE_VAULT_NAMESPACE;
use crate::mango_utils::PerpInfo;
use crate::mango_utils::derive_order_delta;
use crate::mango_utils::price_to_lot_price;
use crate::state::SafetyVault;
use crate::validate_perp_market_mint_matches_depository_collateral_mint;
use amm_anchor::Amm;
use amm_anchor::swap_base_in;
use anchor_comp::mango_markets_v3;
use anchor_comp::mango_markets_v3::MangoMarketV3;
use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;
use mango::matching::OrderType;
use mango::matching::Side;
use mango::state::MangoAccount;
use mango::state::PerpAccount;
use num_traits::Zero;

/// Takes x accounts
#[derive(Accounts)]
pub struct LiquidationKillSwitch<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
        constraint = controller.load()?.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #3 UXDProgram on chain account bound to a Controller instance
    /// The `MangoDepository` manages a MangoAccount for a single Collateral
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = mango_account @UxdError::InvalidMangoAccount,
    )]
    pub depository: AccountLoader<'info, MangoDepository>,

    #[account(
        mut,
        seeds = [SAFETY_VAULT_NAMESPACE, depository.key().as_ref()],
        bump = safety_vault.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority, 
        has_one = quote_vault,
    )]
    pub safety_vault: AccountLoader<'info, SafetyVault>,

    #[account(
        mut,
        seeds = [QUOTE_VAULT_NAMESPACE, safety_vault.key().as_ref()],
        bump = safety_vault.load()?.quote_vault_bump,
    )]
    pub quote_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [COLLATERAL_VAULT_NAMESPACE, safety_vault.key().as_ref()],
        bump = safety_vault.load()?.collateral_vault_bump,
    )]
    pub collateral_vault: Box<Account<'info, TokenAccount>>,

    // - MANGO ACCOUNTS -------------------------------------------------------

    /// CHECK: Mango account associated with the depository and collateral
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

    /// CHECK: Mango CPI - checked MangoMarketsV3 side
    pub mango_signer: UncheckedAccount<'info>,
    
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

    // // - RAYDIUM ACCOUNTS -----------------------------------------------------

    // /// CHECK: Safe. amm Account
    // #[account(mut)]
    // pub amm: AccountInfo<'info>,

    // /// CHECK: Safe. Amm authority Account
    // pub amm_authority: AccountInfo<'info>,

    // /// CHECK: Safe. amm open_orders Account
    // #[account(mut)]
    // pub amm_open_orders: AccountInfo<'info>,

    // /// CHECK: Safe. amm target_orders Account
    // #[account(mut)]
    // pub amm_target_orders: AccountInfo<'info>,

    // /// CHECK: Safe. pool_token_coin Amm Account to swap FROM or To,
    // #[account(mut)]
    // pub pool_coin_token_account: AccountInfo<'info>,

    // /// CHECK: Safe. pool_token_pc Amm Account to swap FROM or To,
    // #[account(mut)]
    // pub pool_pc_token_account: AccountInfo<'info>,

    // /// CHECK: Safe. serum dex program id
    // pub serum_program: AccountInfo<'info>,

    // /// CHECK: Safe. serum market Account. serum_dex program is the owner.
    // #[account(mut)]
    // pub serum_market: AccountInfo<'info>,

    // /// CHECK: Safe. bids Account
    // #[account(mut)]
    // pub serum_bids: AccountInfo<'info>,

    // /// CHECK: Safe. asks Account
    // #[account(mut)]
    // pub serum_asks: AccountInfo<'info>,

    // /// CHECK: Safe. event_q Account
    // #[account(mut)]
    // pub serum_event_queue: AccountInfo<'info>,

    // /// CHECK: Safe. coin_vault Account
    // #[account(mut)]
    // pub serum_coin_vault_account: AccountInfo<'info>,

    // /// CHECK: Safe. pc_vault Account
    // #[account(mut)]
    // pub serum_pc_vault_account: AccountInfo<'info>,

    // /// CHECK: Safe. vault_signer Account
    // #[account(mut)]
    // pub serum_vault_signer: AccountInfo<'info>,

    // - MISC ACCOUNTS --------------------------------------------------------

    /// #18 System Program
    pub system_program: Program<'info, System>,

    /// #19 Token Program
    pub token_program: Program<'info, Token>,

    /// #20 MangoMarketv3 Program
    pub mango_program: Program<'info, MangoMarketV3>,

    // pub raydium_program: Program<'info, Amm>,
}

pub fn handler(
    ctx: Context<LiquidationKillSwitch>, 
    amount_to_liquidate: u64, 
    limit_price: f32,
) -> Result<()> {
    let depository = ctx.accounts.depository.load()?;
    let safety_vault = ctx.accounts.safety_vault.load()?;
    let collateral_mint = depository.collateral_mint;
    let depository_bump = depository.bump;
    msg!("amount_to_liquidate: {}", amount_to_liquidate);

    let depository_pda_signer: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[depository_bump]
    ]];

    // let safety_vault_pda_signer: &[&[&[u8]]] = &[&[
    //     SAFETY_VAULT_NAMESPACE,
    //     ctx.accounts.depository.key().as_ref(), // Is this the right way to get this?
    //     &[safety_vault.bump]
    // ]];

    drop(depository);
    drop(safety_vault);
    
    // - 1 [CLOSE THE EQUIVALENT PERP SHORT ON MANGO] -------------------------

    // - [Get perp information]
    let perp_info = ctx.accounts.perpetual_info()?;
    
    // - [Calculates the quantity of short to close]
    let quote_exposure_delta = I80F48::from_num(amount_to_liquidate);

    
    // - [Find the max taker fees mango will take on the perp order and remove it from the exposure delta to be sure the amount order + fees don't overflow the redeemed amount]
    let max_fee_amount = quote_exposure_delta
        .checked_mul(perp_info.effective_fee)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_ceil()
        .ok_or_else(|| error!(UxdError::MathError))?;
    msg!("max_fee_amount: {}", max_fee_amount);
    let quote_exposure_delta_minus_fees = quote_exposure_delta
        .checked_sub(max_fee_amount)
        .ok_or_else(|| error!(UxdError::MathError))?;
    msg!("quote_exposure_delta_minus_fees: {}", quote_exposure_delta_minus_fees);

    // - [Perp account state PRE perp order]
    let pre_pa = ctx.accounts.perp_account(&perp_info)?;
    msg!("pre_pa_base: {:?}\n pre_pa_quote: {:?}\n pre_pa_taker_base: {:?}\n pre_pa_taker_quote: {:?}\n", pre_pa.base_position, pre_pa.quote_position, pre_pa.taker_base, pre_pa.taker_quote);

    // Will fail if max_quote_quantity was computed to a negative number
    let max_quote_quantity: u64 = quote_exposure_delta_minus_fees
        .checked_div(perp_info.quote_lot_size)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_to_num::<u64>()
        .ok_or_else(|| error!(UxdError::MathError))?;
    msg!("max_quote_quantity: {}", max_quote_quantity);
    msg!("perp_info.quote_lot_size: {}", perp_info.quote_lot_size);

    require!(
        !max_quote_quantity.is_zero(),
        UxdError::QuantityBelowContractSize
    );

    // Note : Reduce the delta neutral position, increasing long exposure, by buying perp.
    //        [BID: taker (us, the caller) | ASK: maker]
    let taker_side = Side::Bid;
    let limit_price =
        I80F48::checked_from_num(limit_price).ok_or_else(|| error!(UxdError::MathError))?;
    let limit_price_lot = price_to_lot_price(limit_price, &perp_info)?;
    msg!("limit_price_lot: {:?}", limit_price_lot);

    // - [CPI MangoMarkets - Place perp order]
    mango_markets_v3::place_perp_order2(
        ctx.accounts
            .into_close_mango_short_perp_context()
            .with_signer(depository_pda_signer),
        taker_side,
        limit_price_lot.to_num(),
        i64::MAX,
        max_quote_quantity
            .try_into()
            .unwrap(),
        0,
        OrderType::ImmediateOrCancel,
        true,
        None,
        MANGO_PERP_MAX_FILL_EVENTS,
    )?;

    // - [Perp account state POST perp order]
    let post_pa = ctx.accounts.perp_account(&perp_info)?;
    msg!("post_pa_base: {:?}\n post_pa_quote: {:?}\n post_pa_taker_base: {:?}\n post_pa_taker_quote: {:?}\n", post_pa.base_position, post_pa.quote_position, post_pa.taker_base, post_pa.taker_quote);

    // - 2 [WITHDRAW COLLATERAL FROM MANGO] -----------------------------------
    require!(
        pre_pa.taker_quote >= post_pa.taker_quote,
        UxdError::InvalidOrderDirection
    );
    let order_delta = derive_order_delta(&pre_pa, &post_pa, &perp_info)?;
    let collateral_withdraw_amount = order_delta
        .base
        .checked_abs()
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_to_num()
        .ok_or_else(|| error!(UxdError::MathError))?;

    // - [CPI MangoMarkets - Withdraw]
    mango_markets_v3::withdraw(
        ctx.accounts
            .into_withdraw_collateral_from_mango_context()
            .with_signer(depository_pda_signer),
        collateral_withdraw_amount,
        false,
    )?;

    // - 3 [UPDATE ONCHAIN ACCOUNTING] ----------------------------------------
    ctx.accounts.update_onchain_accounting(
        collateral_withdraw_amount.into()
    )?;
        
    // DO SWAP CPI IN THIS IX OR IN ANOTHER ONE?

    // - 3 [CPI SWAP FROM COLLATERAL TO QUOTE] --------------------------------
    // TODO
    // amm_anchor::swap_base_in(
    //     ctx.accounts
    //         .into_swap_collateral_raydium_context()
    //         .with_signer(safety_vault_pda_signer), 
    //     safety_vault, 
    //     minimum_amount_out,
    // );

    Ok(())
}

// Contexts
impl<'info> LiquidationKillSwitch<'info> {
    pub fn into_close_mango_short_perp_context(
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

    pub fn into_withdraw_collateral_from_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Withdraw<'info>> {
        let cpi_accounts = mango_markets_v3::Withdraw {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_root_bank.to_account_info(),
            node_bank: self.mango_node_bank.to_account_info(),
            vault: self.mango_vault.to_account_info(),
            token_account: self.collateral_vault.to_account_info(),
            signer: self.mango_signer.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    // TODO
    // pub fn into_swap_collateral_raydium_context(
    //     &self,
    // ) -> CpiContext<'_, '_, '_, 'info, amm_anchor::SwapBaseIn<'info>> {
    //     let cpi_accounts = amm_anchor::SwapBaseIn {
    //         amm: self.amm.to_account_info(),
    //         amm_authority: self.amm_authority.to_account_info(),
    //         amm_open_orders: self.amm_open_orders.to_account_info(),
    //         amm_target_orders: self.amm_target_orders.to_account_info(),
    //         pool_coin_token_account: self.pool_coin_token_account.to_account_info(),
    //         pool_pc_token_account: self.pool_pc_token_account.to_account_info(),
    //         serum_program: self.serum_program.to_account_info(),
    //         serum_market: self.serum_market.to_account_info(),
    //         serum_bids: self.serum_bids.to_account_info(),
    //         serum_asks: self.serum_asks.to_account_info(),
    //         serum_event_queue: self.serum_event_queue.to_account_info(),
    //         serum_coin_vault_account: self.serum_coin_vault_account.to_account_info(),
    //         serum_pc_vault_account: self.serum_pc_vault_account.to_account_info(),
    //         serum_vault_signer: self.serum_vault_signer.to_account_info(),
    //         user_source_token_account: self.collateral_vault.to_account_info(),
    //         user_destination_token_account: self.quote_vault.to_account_info(),
    //         user_source_owner: self.safety_vault.to_account_info(),
    //         spl_token_program: self.token_program.to_account_info(),
    //     };
    //     let cpi_program = self.raydium_program.to_account_info();
    //     CpiContext::new(cpi_program, cpi_accounts)
    // }
}

// Additional convenience methods related to the inputted accounts
impl<'info> LiquidationKillSwitch<'info> {
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
        // msg!("perp_info {:?}", perp_info);
        Ok(perp_info)
    }

    // Return the uncommitted PerpAccount that represent the account balances
    fn perp_account(&self, perp_info: &PerpInfo) -> Result<PerpAccount> {
        // - loads Mango's accounts
        let mango_account = MangoAccount::load_checked(
            &self.mango_account,
            self.mango_program.key,
            self.mango_group.key,
        )
        .map_err(|me| ProgramError::from(me))?;
        Ok(mango_account.perp_accounts[perp_info.market_index])
    }

    fn update_onchain_accounting(
        &mut self,
        collateral_withdraw_amount: u128,
    ) -> Result<()> {
        let depository = &mut self.depository.load_mut()?;
        let safety_vault = &mut self.safety_vault.load_mut()?;

        depository.collateral_amount_deposited = depository.collateral_amount_deposited
            .checked_sub(collateral_withdraw_amount.into())
            .ok_or_else(|| error!(UxdError::MathError))?;
        safety_vault.collateral_liquidated = safety_vault.collateral_liquidated
            .checked_add(collateral_withdraw_amount.into())
            .ok_or_else(|| error!(UxdError::MathError))?;

        Ok(())
    }
}

// Validate input arguments
impl<'info> LiquidationKillSwitch<'info> {
    pub fn validate(&self, amount_to_liquidate: u64) -> Result<()> {
        // require!(
        //     u128::from(amount_to_liquidate) < self.depository.load()?.collateral_amount_deposited, 
        //     UxdError::LiquidateCollateral
        // );

        validate_perp_market_mint_matches_depository_collateral_mint(
            &self.mango_group,
            self.mango_program.key,
            self.mango_perp_market.key,
            &self.depository.load()?.collateral_mint,
        )?;
        Ok(())
    }
}