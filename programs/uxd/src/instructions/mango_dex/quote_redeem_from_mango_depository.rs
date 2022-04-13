use crate::mango_utils::total_perp_base_lot_position;
use crate::mango_utils::PerpInfo;
use crate::Controller;
use crate::MangoDepository;
use crate::UxdError;
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
        bump = controller.bump,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #4 UXDProgram on chain account bound to a Controller instance.
    /// The `MangoDepository` manages a MangoAccount for a single Collateral.
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = mango_account @UxdError::InvalidMangoAccount,
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

    /// #x The quote mint of the depository
    #[account(
      constraint = quote_mint.key() == depository.quote_mint,
    )]
    pub quote_mint: Box<Account<'info, Mint>>,

    /// #6 The `user`'s ATA for one the `mango depository`s `quote_mint`
    /// Will be credited during this instruction
    #[account(
        init_if_needed,
        associated_token::mint = quote_mint,
        associated_token::authority = user,
        payer = payer,
    )]
    pub user_quote: Box<Account<'info, TokenAccount>>,

    /// #7 The `user`'s ATA for the `controller`'s `redeemable_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        constraint = user_redeemable.mint == controller.redeemable_mint,
        constraint = user_redeemable.owner == *user.key,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #9 The MangoMarkets Account (MangoAccount) managed by the `depository` ******************
    /// CHECK : Seeds checked. Depository registered
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.mango_account_bump,
    )]
    pub mango_account: AccountInfo<'info>,

    /// #10 [MangoMarkets CPI] Index grouping perp and spot markets
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_group: UncheckedAccount<'info>,

    /// #11 [MangoMarkets CPI] Cache
    /// CHECK: Mango CPI - checked MangoMarketV3 side
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

pub fn handler(
    ctx: Context<QuoteRedeemFromMangoDepository>,
    redeemable_amount: u64,
) -> Result<()> {
    let depository = &ctx.accounts.depository;
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

    // Will not overflow as `perp_position_notional_size` and `redeemable_under_management`
    // will vary together.
    let perp_unrealized_pnl = I80F48::checked_from_num(
        redeemable_under_management
            .checked_sub(perp_position_notional_size)
            .ok_or_else(|| error!(UxdError::MathError))?,
    )
    .ok_or_else(|| error!(UxdError::MathError))?;

    // - 2 [FIND HOW MUCH REDEEMABLE CAN BE MINTED] ---------------------------

    // Get how much redeemable has already been minted with the quote mint
    let quote_minted = depository.total_quote_minted;

    // Only allow quote redeeming if PnL is positive
    require!(perp_unrealized_pnl.is_positive(), UxdError::InvalidPnlPolarity);

    let quote_redeemable: u64 = perp_unrealized_pnl
    .checked_abs()
    .ok_or_else(|| error!(UxdError::MathError))?
    .checked_to_num::<i128>()
    .ok_or_else(|| error!(UxdError::MathError))?
    .checked_add(quote_minted.try_into().unwrap())
    .ok_or_else(|| error!(UxdError::MathError))?
    .try_into()
    .unwrap();

    // Check to ensure we are not redeeming more than we allocate to resolve positive PnL
    require!(redeemable_amount <= quote_redeemable, UxdError::RedeemableAmountTooHigh);

    // - x [BURN USER'S REDEEMABLE] -------------------------------------------
    token::burn(
        ctx.accounts.into_burn_redeemable_context(),
        redeemable_amount,
    )?;

    // - x [WITHDRAW QUOTE MINT FROM MANGO ACCOUNT] ---------------------------
    let quote_redeem_fee = depository.quote_mint_and_redeem_fees;
    let percentage_less_fees: f64 = (1 as f64) - (
        (quote_redeem_fee as f64)/((10 as f64).powi(4.into()))
    );
    let quote_withdraw_amount: u64 = I80F48::checked_from_num::<f64>(percentage_less_fees)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_mul_int(redeemable_amount.into())
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_floor()
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_to_num::<u64>()
        .ok_or_else(|| error!(UxdError::MathError))?;

    mango_markets_v3::withdraw(
        ctx.accounts
            .into_withdraw_quote_mint_from_mango_context()
            .with_signer(depository_signer_seed),
        quote_withdraw_amount,
        false,
    )?;

    // - x [UPDATE ACCOUNTING] ------------------------------------------------
    ctx.accounts.update_onchain_accounting(
        redeemable_amount,
        quote_withdraw_amount,
    )?;

    Ok(())
}

impl<'info> QuoteRedeemFromMangoDepository<'info> {
    pub fn into_burn_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.redeemable_mint.to_account_info(),
            to: self.user_redeemable.to_account_info(),
            authority: self.user.to_account_info(),
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
        let depository = &mut self.depository;
        let controller = &mut self.controller;
        // Mango Depository
        depository.total_quote_minted = depository
            .total_quote_minted
            .checked_sub(quote_withdraw_amount.into())
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.redeemable_amount_under_management = depository
            .redeemable_amount_under_management
            .checked_sub(redeemable_amount.into())
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
        .map_err(|me| ProgramError::from(me))?;
        Ok(mango_account.perp_accounts[perp_info.market_index])
    }
}

// Validate input arguments
impl<'info> QuoteRedeemFromMangoDepository<'info> {
    pub fn validate(
        &self,
        redeemable_amount: u64,
    ) -> Result<()> {
        if redeemable_amount == 0 {
            return Err(error!(UxdError::InvalidRedeemableAmount));
        }
        if self.user_redeemable.amount < redeemable_amount {
            return Err(error!(UxdError::InsufficientRedeemableAmountMint));
        }

        Ok(())
    }
}

