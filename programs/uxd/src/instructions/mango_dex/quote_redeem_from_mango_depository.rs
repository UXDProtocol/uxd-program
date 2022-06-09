use crate::mango_utils::total_perp_base_lot_position;
use crate::mango_utils::PerpInfo;
use crate::validate_perp_market_mint_matches_depository_collateral_mint;
use crate::Controller;
use crate::MangoDepository;
use crate::UxdError;
use crate::BPS_UNIT_CONVERSION;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use anchor_comp::mango_markets_v3;
use anchor_comp::mango_markets_v3::MangoMarketV3;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Burn;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;
use mango::state::MangoAccount;
use mango::state::PerpAccount;

#[derive(Accounts)]
pub struct QuoteRedeemFromMangoDepository<'info> {
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
        has_one = redeemable_mint,
        constraint = controller.load()?.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository
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

    /// #6 The quote mint of the depository
    #[account(
      constraint = quote_mint.key() == depository.load()?.quote_mint,
    )]
    pub quote_mint: Box<Account<'info, Mint>>,

    /// #7 The `user`'s ATA for one the `mango depository`s `quote_mint`
    /// Will be credited during this instruction
    #[account(
        mut,
        constraint = user_quote.mint == depository.load()?.quote_mint,
        constraint = user_quote.owner == *user.key,
    )]
    pub user_quote: Box<Account<'info, TokenAccount>>,

    /// #8 The `user`'s ATA for the `controller`'s `redeemable_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        constraint = user_redeemable.mint == controller.load()?.redeemable_mint,
        constraint = user_redeemable.owner == *user.key,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #9 The MangoMarkets Account (MangoAccount) managed by the `depository`
    /// CHECK : Seeds checked. Depository registered
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.mango_account_bump,
    )]
    pub mango_account: AccountInfo<'info>,

    /// #10 [MangoMarkets CPI] Index grouping perp and spot markets
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_group: UncheckedAccount<'info>,

    /// #11 [MangoMarkets CPI] Cache
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_cache: UncheckedAccount<'info>,

    /// #12 [MangoMarkets CPI] Signer PDA
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_signer: UncheckedAccount<'info>,

    /// #12 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_root_bank: UncheckedAccount<'info>,

    /// #13 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_node_bank: UncheckedAccount<'info>,

    /// #14 [MangoMarkets CPI] Vault for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_vault: UncheckedAccount<'info>,

    /// #15 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_perp_market: UncheckedAccount<'info>,

    /// #16 System Program
    pub system_program: Program<'info, System>,

    /// #17 Token Program
    pub token_program: Program<'info, Token>,

    /// #18 Associated Token Program
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// #19 MangoMarketv3 Program
    pub mango_program: Program<'info, MangoMarketV3>,

    /// #20 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<QuoteRedeemFromMangoDepository>, redeemable_amount: u64) -> Result<()> {
    let depository = ctx.accounts.depository.load()?;
    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        depository.collateral_mint.as_ref(),
        &[depository.bump],
    ]];

    // - [Get perp information]
    let perp_info = ctx.accounts.perpetual_info()?;

    // - [Perp account state PRE perp order]
    let pre_pa = ctx.accounts.perp_account(&perp_info)?;

    // - 1 [FIND CURRENT UNREALIZED PNL AMOUNT] -------------------------------

    // - [find out current perp Unrealized PnL]
    let contract_size = perp_info.base_lot_size;
    // Note : Loose precision but an average value is fine here, we just want a value close to the current PnL
    let perp_position_notional_size: i128 =
        I80F48::from_num(total_perp_base_lot_position(&pre_pa)?)
            .checked_mul(contract_size)
            .ok_or_else(|| error!(UxdError::MathError))?
            .checked_mul(perp_info.price)
            .ok_or_else(|| error!(UxdError::MathError))?
            .abs()
            .checked_to_num()
            .ok_or_else(|| error!(UxdError::MathError))?;

    // The perp position unrealized PnL is equal to the outstanding amount of redeemable
    // minus the perp position notional size in quote.
    // Ideally they stay 1:1, to have the redeemable fully backed by the delta neutral
    // position and no paper profits.
    let redeemable_under_management = i128::try_from(depository.redeemable_amount_under_management)
        .map_err(|_e| error!(UxdError::MathError))?;

    msg!(
        "redeemable_under_management {}",
        redeemable_under_management
    );

    // Will not overflow as `perp_position_notional_size` and `redeemable_under_management`
    // will vary together.
    let perp_unrealized_pnl = redeemable_under_management
        .checked_sub(perp_position_notional_size)
        .ok_or_else(|| error!(UxdError::MathError))?;
    msg!("perp_unrealized_pnl {}", perp_unrealized_pnl);

    // - 2 [FIND HOW MUCH REDEEMABLE CAN BE REDEEMED] -------------------------

    // In order to redeem, the adjusted PnL must be positive so that we can convert it into more delta neutral position
    require!(
        perp_unrealized_pnl.is_positive(),
        UxdError::InvalidPnlPolarity
    );

    msg!("redeemable_amount {}", redeemable_amount);
    msg!("perp_unrealized_pnl {}", perp_unrealized_pnl);

    // Checks that the requested redeem amount is lesser than or equal to the available amount
    let redeemable_amount_i128 =
        i128::try_from(redeemable_amount).map_err(|_| error!(UxdError::MathError))?;
    require!(
        redeemable_amount_i128 <= perp_unrealized_pnl,
        UxdError::RedeemableAmountTooHigh
    );

    // - 3 [BURN USER'S REDEEMABLE] -------------------------------------------
    // Burn will fail if user does not have enough redeemable
    token::burn(
        ctx.accounts.into_burn_redeemable_context(),
        redeemable_amount,
    )?;

    // - 4 [WITHDRAW QUOTE MINT FROM MANGO ACCOUNT] ---------------------------
    let quote_redeem_fee = depository.quote_mint_and_redeem_fee;

    // Math: 5 bps fee would equate to bps_redeemed_to_user
    // being 9995 since 10000 - 5 = 9995
    let bps_redeemed_to_user: I80F48 = I80F48::checked_from_num(BPS_UNIT_CONVERSION)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_sub(quote_redeem_fee.into())
        .ok_or_else(|| error!(UxdError::MathError))?;

    // Math: Multiplies the redeemable_amount by how many BPS the user will get
    // but the units are still in units of BPS, not as a decimal, so then
    // divide by the BPS_POW to get to the right order of magnitude.
    let quote_withdraw_amount_less_fees: u64 = bps_redeemed_to_user
        .checked_mul_int(redeemable_amount.into())
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_div_int(BPS_UNIT_CONVERSION.into())
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_floor()
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_to_num::<u64>()
        .ok_or_else(|| error!(UxdError::MathError))?;

    mango_markets_v3::withdraw(
        ctx.accounts
            .into_withdraw_quote_mint_from_mango_context()
            .with_signer(depository_signer_seed),
        quote_withdraw_amount_less_fees,
        false,
    )?;

    // - 5 [UPDATE ACCOUNTING] ------------------------------------------------
    drop(depository);
    ctx.accounts
        .update_onchain_accounting(redeemable_amount, quote_withdraw_amount_less_fees)?;

    Ok(())
}

impl<'info> QuoteRedeemFromMangoDepository<'info> {
    pub fn into_burn_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.redeemable_mint.to_account_info(),
            authority: self.user.to_account_info(),
            from: self.user_redeemable.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_withdraw_quote_mint_from_mango_context(
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
            token_account: self.user_quote.to_account_info(),
            signer: self.mango_signer.to_account_info(),
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

    // Update the accounting in the Depository and Controller Accounts to reflect changes
    fn update_onchain_accounting(
        &mut self,
        redeemable_amount: u64,
        quote_withdraw_amount: u64,
    ) -> Result<()> {
        let depository = &mut self.depository.load_mut()?;
        let controller = &mut self.controller.load_mut()?;
        let fees_accrued: u64 = redeemable_amount
            .checked_sub(quote_withdraw_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        // Mango Depository
        depository.net_quote_minted = depository
            .net_quote_minted
            .checked_sub(quote_withdraw_amount.into())
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.redeemable_amount_under_management = depository
            .redeemable_amount_under_management
            .checked_sub(redeemable_amount.into())
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.total_quote_mint_and_redeem_fees = depository
            .total_quote_mint_and_redeem_fees
            .checked_add(fees_accrued.into())
            .ok_or_else(|| error!(UxdError::MathError))?;
        // Controller
        controller.redeemable_circulating_supply = controller
            .redeemable_circulating_supply
            .checked_sub(redeemable_amount.into())
            .ok_or_else(|| error!(UxdError::MathError))?;
        Ok(())
    }
}

// Additional convenience methods related to the inputted accounts
impl<'info> QuoteRedeemFromMangoDepository<'info> {
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
}

// Validate input arguments
impl<'info> QuoteRedeemFromMangoDepository<'info> {
    pub fn validate(&self, redeemable_amount: u64) -> Result<()> {
        require!(redeemable_amount != 0, UxdError::InvalidRedeemableAmount);
        validate_perp_market_mint_matches_depository_collateral_mint(
            &self.mango_group,
            self.mango_program.key,
            self.mango_perp_market.key,
            &self.depository.load()?.collateral_mint,
        )?;

        Ok(())
    }
}
