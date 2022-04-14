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
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;
use mango::state::MangoAccount;
use mango::state::PerpAccount;

#[derive(Accounts)]
pub struct QuoteMintWithMangoDepository<'info> {
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

    /// #6 The `user`'s ATA for one the `mango depository`s `quote_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        constraint = user_quote.mint == depository.quote_mint,
        constraint = user_quote.owner == *user.key,
    )]
    pub user_quote: Box<Account<'info, TokenAccount>>,

    /// #7 The `user`'s ATA for the `controller`'s `redeemable_mint`
    /// Will be credited during this instruction
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
    ctx: Context<QuoteMintWithMangoDepository>,
    quote_amount: u64,
) -> Result<()> {
    let depository = &ctx.accounts.depository;
    let controller = &ctx.accounts.controller;
    let depository_pda_signer: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        depository.collateral_mint.as_ref(),
        &[depository.bump],
    ]];
    let controller_pda_signer: &[&[&[u8]]] = &[&[CONTROLLER_NAMESPACE, &[controller.bump]]];

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

    // Only allow quote minting if PnL is negative
    require!(perp_unrealized_pnl.is_negative(), UxdError::InvalidPnlPolarity);

    // Will become negative if more has been minted than the current negative PnL
    let quote_mintable: u64 = perp_unrealized_pnl
        .checked_abs()
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_to_num::<u64>()
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_sub(quote_minted.try_into().unwrap())
        .ok_or_else(|| error!(UxdError::MathError))?;

    // Check to ensure we are not minting more than we allocate to resolve negative PnL
    require!(quote_amount <= quote_mintable, UxdError::QuoteAmountTooHigh);

    // - 4 [DEPOSIT QUOTE MINT INTO MANGO ACCOUNT] -------------------------------
    mango_markets_v3::deposit(
        ctx.accounts
        .into_deposit_quote_to_mango_context()
        .with_signer(depository_pda_signer),
        quote_amount,
    )?;

    // - 5 [MINT REDEEMABLE TO USER] ------------------------------------------
    let quote_mint_fee = depository.quote_mint_and_redeem_fees;
    let percentage_less_fees: f64 = (1 as f64) - (
        (quote_mint_fee as f64)/((10 as f64).powi(4.into()))
    );
    let redeemable_mint_less_fees: u64 = I80F48::checked_from_num::<f64>(percentage_less_fees)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_mul_int(quote_amount.into())
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_floor()
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_to_num::<u64>()
        .ok_or_else(|| error!(UxdError::MathError))?;

    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_pda_signer),
        redeemable_mint_less_fees,
    )?;

    // - 6 [UPDATE ACCOUNTING] ------------------------------------------------
    ctx.accounts.update_onchain_accounting(
        quote_amount,
        redeemable_mint_less_fees,
    )?;

    // - 7 [CHECK GLOBAL REDEEMABLE SUPPLY CAP OVERFLOW] ----------------------
    ctx.accounts.check_redeemable_global_supply_cap_overflow()?;

    Ok(())
}

impl<'info> QuoteMintWithMangoDepository<'info> {
    pub fn into_deposit_quote_to_mango_context(
        &self
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Deposit<'info>> {
        let cpi_accounts = mango_markets_v3::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.user.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_root_bank.to_account_info(),
            node_bank: self.mango_node_bank.to_account_info(),
            vault: self.mango_vault.to_account_info(),
            owner_token_account: self.user_quote.to_account_info(),
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

    // Ensure that the minted amount does not raise the Redeemable supply beyond the Global Redeemable Supply Cap
    fn check_redeemable_global_supply_cap_overflow(&self) -> Result<()> {
        require!(
            self.controller.redeemable_circulating_supply 
            <= self.controller.redeemable_global_supply_cap, 
            UxdError::RedeemableGlobalSupplyCapReached
        );
       
        Ok(())
    }

    // Update the accounting in the Depository and Controller Accounts to reflect changes
    fn update_onchain_accounting(
        &mut self,
        quote_amount_deposited: u64,
        redeemable_minted_amount: u64,
    ) -> Result<()> {
        let depository = &mut self.depository;
        let controller = &mut self.controller;
        // Mango Depository
        depository.total_quote_minted = depository
            .total_quote_minted
            .checked_add(quote_amount_deposited.into())
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.redeemable_amount_under_management = depository
            .redeemable_amount_under_management
            .checked_add(redeemable_minted_amount.into())
            .ok_or_else(|| error!(UxdError::MathError))?;
        // Controller
        controller.redeemable_circulating_supply = controller
            .redeemable_circulating_supply
            .checked_add(redeemable_minted_amount.into())
            .ok_or_else(|| error!(UxdError::MathError))?;
        Ok(())
    }
}

// Additional convenience methods related to the inputted accounts
impl<'info> QuoteMintWithMangoDepository<'info> {
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
impl<'info> QuoteMintWithMangoDepository<'info> {
    pub fn validate(
        &self,
        quote_amount: u64,
    ) -> Result<()> {
        require!(quote_amount != 0, UxdError::InvalidQuoteAmount);
        require!(self.user_quote.amount >= quote_amount, UxdError::InsufficientQuoteAmountMint);
        validate_perp_market_mint_matches_depository_collateral_mint(
            &self.mango_group,
            self.mango_program.key,
            self.mango_perp_market.key,
            &self.depository.collateral_mint,
        )?;

        Ok(())
    }
}