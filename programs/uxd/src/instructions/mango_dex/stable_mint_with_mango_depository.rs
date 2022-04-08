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
pub struct StableMintWithMangoDepository<'info> {
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

    /// #6 The `user`'s ATA for one of the `controller`s `registered_stable_mints`
    /// Will be debited during this instruction
    #[account(
        mut,
        constraint = controller.registered_stable_mints.contains(&user_stable.mint),
        constraint = user_stable.owner == *user.key,
    )]
    pub user_stable: Box<Account<'info, TokenAccount>>,

    /// #7 The `user`'s ATA for the `controller`'s `redeemable_mint`
    /// Will be credited during this instruction
    #[account(
        init_if_needed,
        associated_token::mint = redeemable_mint,
        associated_token::authority = user,
        payer = payer,
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

    /// #16 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook bids
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_bids: UncheckedAccount<'info>,

    /// #17 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook asks
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_asks: UncheckedAccount<'info>,

    /// #18 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market event queue
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_event_queue: UncheckedAccount<'info>,

    /// #19 System Program
    pub system_program: Program<'info, System>,

    /// #20 Token Program
    pub token_program: Program<'info, Token>,

    /// #21 Associated Token Program
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// #22 MangoMarketv3 Program
    pub mango_program: Program<'info, MangoMarketV3>,

    /// #23 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<StableMintWithMangoDepository>,
    stable_amount: u64,
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

    // Get how much redeemable has already been minted with stables
    let stable_minted = u64::try_from(depository.total_stable_minted)
        .map_err(|_e| error!(UxdError::MathError))?;

    let perp_unrealized_pnl_to_positive: u64;

    // Only allow stable minting if PnL is negative
    match perp_unrealized_pnl.is_negative() {
        true => {
            perp_unrealized_pnl_to_positive = perp_unrealized_pnl
            .checked_neg()
            .ok_or_else(|| error!(UxdError::MathError))?
            .checked_to_num()
            .ok_or_else(|| error!(UxdError::MathError))?;
        }
        false => {
            return Err(error!(UxdError::InvalidPnlPolarity));
        }
    }

    // Will become negative if more has been minted than the current negative PnL
    let stable_mintable = perp_unrealized_pnl_to_positive
        .checked_sub(stable_minted)
        .ok_or_else(|| error!(UxdError::MathError))?;

    // Check to ensure we are not minting more than we allocate to resolve negative PnL
    if stable_amount > stable_mintable {
        return Err(error!(UxdError::StableAmountTooHigh));
    }

    // - 4 [DEPOSIT STABLES INTO MANGO ACCOUNT] -------------------------------
    mango_markets_v3::deposit(
        ctx.accounts
        .into_deposit_stable_to_mango_context()
        .with_signer(depository_pda_signer),
        stable_amount,
    )?;

    // - 5 [MINT REDEEMABLE TO USER] ------------------------------------------
    let redeemable_mint_amount = stable_amount; // MAYBE CHECK?>
    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_pda_signer),
        redeemable_mint_amount,
    )?;

    // - 6 [UPDATE ACCOUNTING] ------------------------------------------------
    ctx.accounts.update_onchain_accounting(
        stable_amount.into(),
        redeemable_mint_amount.into(),
    )?;

    // - 7 [CHECK GLOBAL REDEEMABLE SUPPLY CAP OVERFLOW] ----------------------
    ctx.accounts.check_redeemable_global_supply_cap_overflow()?;

    Ok(())
}

impl<'info> StableMintWithMangoDepository<'info> {
    pub fn into_deposit_stable_to_mango_context(
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
            owner_token_account: self.user_stable.to_account_info(),
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
        stable_amount_deposited: u64,
        redeemable_minted_amount: u128,
    ) -> Result<()> {
        let depository = &mut self.depository;
        let controller = &mut self.controller;
        // Mango Depository
        depository.total_stable_minted = depository
            .total_stable_minted
            .checked_add(stable_amount_deposited)
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

// Additional convenience methods related to the inputted accounts
impl<'info> StableMintWithMangoDepository<'info> {
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
impl<'info> StableMintWithMangoDepository<'info> {
    pub fn validate(
        &self,
        stable_amount: u64,
    ) -> Result<()> {
        if stable_amount == 0 {
            return Err(error!(UxdError::InvalidStableAmount));
        }
        if self.user_stable.amount < stable_amount {
            return Err(error!(UxdError::InsufficientStableAmount));
        }

        Ok(())
    }
}
