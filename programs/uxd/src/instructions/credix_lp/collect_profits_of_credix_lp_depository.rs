use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;

use crate::error::UxdError;
use crate::events::CollectProfitsOfCredixLpDepositoryEvent;
use crate::state::controller::Controller;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::utils::compute_decrease;
use crate::utils::compute_increase;
use crate::utils::compute_shares_amount_for_value;
use crate::utils::compute_value_for_shares_amount;
use crate::utils::is_within_range_inclusive;
use crate::validate_is_program_frozen;
use crate::CONTROLLER_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_NAMESPACE;
use crate::CREDIX_LP_EXTERNAL_PASS_NAMESPACE;

#[derive(Accounts)]
pub struct CollectProfitsOfCredixLpDepository<'info> {
    /// #1
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #2
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.registered_credix_lp_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #3
    #[account(
        mut,
        seeds = [
            CREDIX_LP_DEPOSITORY_NAMESPACE,
            depository.load()?.credix_global_market_state.key().as_ref(),
            depository.load()?.collateral_mint.as_ref()
        ],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        has_one = depository_collateral @UxdError::InvalidDepositoryCollateral,
        has_one = depository_shares @UxdError::InvalidDepositoryShares,
        has_one = credix_program_state @UxdError::InvalidCredixProgramState,
        has_one = credix_global_market_state @UxdError::InvalidCredixGlobalMarketState,
        has_one = credix_signing_authority @UxdError::InvalidCredixSigningAuthority,
        has_one = credix_liquidity_collateral @UxdError::InvalidCredixLiquidityCollateral,
        has_one = credix_shares_mint @UxdError::InvalidCredixSharesMint,
        has_one = profits_beneficiary_collateral @UxdError::InvalidProfitsBeneficiaryCollateral,
    )]
    pub depository: AccountLoader<'info, CredixLpDepository>,

    /// #4
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #5
    #[account(mut)]
    pub depository_collateral: Box<Account<'info, TokenAccount>>,

    /// #6
    #[account(mut)]
    pub depository_shares: Box<Account<'info, TokenAccount>>,

    /// #7
    #[account(
        has_one = credix_multisig_key @UxdError::InvalidCredixMultisigKey,
    )]
    pub credix_program_state: Box<Account<'info, credix_client::ProgramState>>,

    /// #8
    #[account(
        mut,
        constraint = credix_global_market_state.treasury_pool_token_account == credix_treasury_collateral.key() @UxdError::InvalidCredixTreasuryCollateral,
    )]
    pub credix_global_market_state: Box<Account<'info, credix_client::GlobalMarketState>>,

    /// #9
    /// CHECK: unused by us, checked by credix
    pub credix_signing_authority: AccountInfo<'info>,

    /// #10
    #[account(mut)]
    pub credix_liquidity_collateral: Box<Account<'info, TokenAccount>>,

    /// #11
    #[account(mut)]
    pub credix_shares_mint: Box<Account<'info, Mint>>,

    /// #12
    #[account(
        mut,
        owner = credix_client::ID,
        seeds = [
            credix_global_market_state.key().as_ref(),
            depository.key().as_ref(),
            CREDIX_LP_EXTERNAL_PASS_NAMESPACE
        ],
        bump,
        seeds::program = credix_client::ID,
        constraint = credix_pass.user == depository.key() @UxdError::InvalidCredixPass,
        constraint = credix_pass.disable_withdrawal_fee @UxdError::InvalidCredixPassNoFees,
    )]
    pub credix_pass: Account<'info, credix_client::CredixPass>,

    /// #13
    #[account(
        mut,
        token::mint = collateral_mint,
    )]
    pub credix_treasury_collateral: Box<Account<'info, TokenAccount>>,

    /// #14
    /// CHECK: not used by us, checked by credix program
    pub credix_multisig_key: AccountInfo<'info>,

    /// #15
    #[account(
        mut,
        token::authority = credix_multisig_key,
        token::mint = collateral_mint,
    )]
    pub credix_multisig_collateral: Box<Account<'info, TokenAccount>>,

    /// #16
    #[account(
        mut,
        token::mint = collateral_mint,
    )]
    pub profits_beneficiary_collateral: Box<Account<'info, TokenAccount>>,

    /// #17
    pub system_program: Program<'info, System>,
    /// #18
    pub token_program: Program<'info, Token>,
    /// #19
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// #20
    pub credix_program: Program<'info, credix_client::program::Credix>,
    /// #21
    pub rent: Sysvar<'info, Rent>,
}

pub(crate) fn handler(ctx: Context<CollectProfitsOfCredixLpDepository>) -> Result<()> {
    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Fetch all current onchain state
    // -- and predict all future final state after mutation
    // ---------------------------------------------------------------------

    // Read all states before collect
    let depository_collateral_amount_before: u64 = ctx.accounts.depository_collateral.amount;
    let profits_beneficiary_collateral_amount_before: u64 =
        ctx.accounts.profits_beneficiary_collateral.amount;

    let liquidity_collateral_amount_before: u64 = ctx.accounts.credix_liquidity_collateral.amount;
    let outstanding_collateral_amount_before: u64 = ctx
        .accounts
        .credix_global_market_state
        .pool_outstanding_credit;

    let total_shares_supply_before: u64 = ctx.accounts.credix_shares_mint.supply;
    let total_shares_value_before: u64 = liquidity_collateral_amount_before
        .checked_add(outstanding_collateral_amount_before)
        .ok_or(UxdError::MathError)?;

    let owned_shares_amount_before: u64 = ctx.accounts.depository_shares.amount;
    let owned_shares_value_before: u64 = compute_value_for_shares_amount(
        owned_shares_amount_before,
        total_shares_supply_before,
        total_shares_value_before,
    )?;

    // How much collateral can we withdraw as profits
    let profits_value: u128 = {
        // Compute the set of liabilities owed to the users
        let liabilities_value: u128 = ctx
            .accounts
            .depository
            .load()?
            .redeemable_amount_under_management;
        msg!(
            "[collect_profits_of_credix_lp_depository:liabilities_value:{}]",
            liabilities_value
        );
        // Compute the set of assets owned in the LP
        let assets_value: u128 = owned_shares_value_before.into();
        msg!(
            "[collect_profits_of_credix_lp_depository:assets_value:{}]",
            assets_value
        );
        // Compute the amount of profits that we can safely withdraw
        assets_value
            .checked_sub(liabilities_value)
            .ok_or(UxdError::MathError)?
    };
    msg!(
        "[collect_profits_of_credix_lp_depository:profits_value:{}]",
        profits_value
    );

    // Assumes and enforce a collateral/redeemable 1:1 relationship on purpose
    let collateral_amount_before_precision_loss: u64 = checked_convert_u128_to_u64(profits_value)?;
    msg!(
        "[collect_profits_of_credix_lp_depository:collateral_amount_before_precision_loss:{}]",
        collateral_amount_before_precision_loss
    );

    // Compute the amount of shares that we need to withdraw based on the amount of wanted collateral
    let shares_amount: u64 = compute_shares_amount_for_value(
        collateral_amount_before_precision_loss,
        total_shares_supply_before,
        total_shares_value_before,
    )?;
    msg!(
        "[collect_profits_of_credix_lp_depository:shares_amount:{}]",
        shares_amount
    );

    // Compute the amount of collateral that the withdrawn shares are worth (after potential precision loss)
    let collateral_amount_after_precision_loss: u64 = compute_value_for_shares_amount(
        shares_amount,
        total_shares_supply_before,
        total_shares_value_before,
    )?;
    msg!(
        "[collect_profits_of_credix_lp_depository:collateral_amount_after_precision_loss:{}]",
        collateral_amount_after_precision_loss
    );

    // If nothing to withdraw, no need to continue, all profits have already been successfully collected
    if collateral_amount_after_precision_loss == 0 {
        msg!("[collect_profits_of_credix_lp_depository:no_profits_to_collect]",);
        return Ok(());
    }

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Actually runs the onchain mutation based on computed parameters
    // ---------------------------------------------------------------------

    // Make depository signer
    let credix_global_market_state = ctx.accounts.depository.load()?.credix_global_market_state;
    let collateral_mint = ctx.accounts.depository.load()?.collateral_mint;
    let depository_pda_signer: &[&[&[u8]]] = &[&[
        CREDIX_LP_DEPOSITORY_NAMESPACE,
        credix_global_market_state.as_ref(),
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.load()?.bump],
    ]];

    // Run a withdraw CPI from credix into the depository
    msg!("[collect_profits_of_credix_lp_depository:withdraw_funds]",);
    credix_client::cpi::withdraw_funds(
        ctx.accounts
            .into_withdraw_funds_from_credix_lp_context()
            .with_signer(depository_pda_signer),
        collateral_amount_before_precision_loss,
    )?;

    // Transfer the received collateral from the depository to the end user
    msg!("[collect_profits_of_credix_lp_depository:collateral_transfer]",);
    token::transfer(
        ctx.accounts
            .into_transfer_depository_collateral_to_profits_beneficiary_collateral_context()
            .with_signer(depository_pda_signer),
        collateral_amount_after_precision_loss,
    )?;

    // Refresh account states after withdrawal
    ctx.accounts.depository_collateral.reload()?;
    ctx.accounts.depository_shares.reload()?;
    ctx.accounts.credix_global_market_state.reload()?;
    ctx.accounts.credix_liquidity_collateral.reload()?;
    ctx.accounts.credix_shares_mint.reload()?;
    ctx.accounts.profits_beneficiary_collateral.reload()?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Strictly verify that the onchain state
    // -- after mutation exactly match previous predictions
    // ---------------------------------------------------------------------

    // Read all states after withdrawal
    let depository_collateral_amount_after: u64 = ctx.accounts.depository_collateral.amount;
    let profits_beneficiary_collateral_amount_after: u64 =
        ctx.accounts.profits_beneficiary_collateral.amount;

    let liquidity_collateral_amount_after: u64 = ctx.accounts.credix_liquidity_collateral.amount;
    let outstanding_collateral_amount_after: u64 = ctx
        .accounts
        .credix_global_market_state
        .pool_outstanding_credit;

    let total_shares_supply_after: u64 = ctx.accounts.credix_shares_mint.supply;
    let total_shares_value_after: u64 = liquidity_collateral_amount_after
        .checked_add(outstanding_collateral_amount_after)
        .ok_or(UxdError::MathError)?;

    let owned_shares_amount_after: u64 = ctx.accounts.depository_shares.amount;
    let owned_shares_value_after: u64 = compute_value_for_shares_amount(
        owned_shares_amount_after,
        total_shares_supply_after,
        total_shares_value_after,
    )?;

    // Compute changes in states
    let profits_beneficiary_collateral_amount_increase: u64 = compute_increase(
        profits_beneficiary_collateral_amount_before,
        profits_beneficiary_collateral_amount_after,
    )?;

    let total_shares_supply_decrease: u64 =
        compute_decrease(total_shares_supply_before, total_shares_supply_after)?;
    let total_shares_value_decrease: u64 =
        compute_decrease(total_shares_value_before, total_shares_value_after)?;

    let owned_shares_amount_decrease: u64 =
        compute_decrease(owned_shares_amount_before, owned_shares_amount_after)?;
    let owned_shares_value_decrease: u64 =
        compute_decrease(owned_shares_value_before, owned_shares_value_after)?;

    // Log deltas for debriefing the changes
    msg!(
        "[collect_profits_of_credix_lp_depository:profits_beneficiary_collateral_amount_increase:{}]",
        profits_beneficiary_collateral_amount_increase
    );
    msg!(
        "[collect_profits_of_credix_lp_depository:total_shares_supply_decrease:{}]",
        total_shares_supply_decrease
    );
    msg!(
        "[collect_profits_of_credix_lp_depository:total_shares_value_decrease:{}]",
        total_shares_value_decrease
    );
    msg!(
        "[collect_profits_of_credix_lp_depository:owned_shares_amount_decrease:{}]",
        owned_shares_amount_decrease
    );
    msg!(
        "[collect_profits_of_credix_lp_depository:owned_shares_value_decrease:{}]",
        owned_shares_value_decrease
    );

    // The depository collateral account should always be empty
    require!(
        depository_collateral_amount_before == depository_collateral_amount_after,
        UxdError::CollateralDepositHasRemainingDust
    );

    // Validate that the locked value moved exactly to the correct place
    require!(
        profits_beneficiary_collateral_amount_increase == collateral_amount_after_precision_loss,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );

    // Check that we withdrew the correct amount of shares
    require!(
        total_shares_supply_decrease == shares_amount,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );
    require!(
        owned_shares_amount_decrease == shares_amount,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );

    // Check that the new state of the pool still makes sense
    require!(
        is_within_range_inclusive(
            total_shares_value_decrease,
            collateral_amount_after_precision_loss,
            collateral_amount_before_precision_loss,
        ),
        UxdError::CollateralDepositDoesntMatchTokenValue,
    );
    require!(
        is_within_range_inclusive(
            owned_shares_value_decrease,
            collateral_amount_after_precision_loss,
            collateral_amount_before_precision_loss,
        ),
        UxdError::CollateralDepositDoesntMatchTokenValue,
    );

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- Emit resulting event, and update onchain accounting
    // ---------------------------------------------------------------------

    // Emit event
    emit!(CollectProfitsOfCredixLpDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: ctx.accounts.depository.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_amount: collateral_amount_after_precision_loss,
    });

    // Accounting for depository
    let mut depository = ctx.accounts.depository.load_mut()?;
    depository.update_onchain_accounting_following_profits_collection(
        collateral_amount_after_precision_loss,
    )?;

    // Accounting for controller
    let mut controller = ctx.accounts.controller.load_mut()?;
    controller.update_onchain_accounting_following_profits_collection(
        collateral_amount_after_precision_loss,
    )?;

    // Done
    Ok(())
}

// Into functions
impl<'info> CollectProfitsOfCredixLpDepository<'info> {
    pub fn into_withdraw_funds_from_credix_lp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, credix_client::cpi::accounts::WithdrawFunds<'info>> {
        let cpi_accounts = credix_client::cpi::accounts::WithdrawFunds {
            base_token_mint: self.collateral_mint.to_account_info(),
            investor: self.depository.to_account_info(),
            investor_token_account: self.depository_collateral.to_account_info(),
            investor_lp_token_account: self.depository_shares.to_account_info(),
            program_state: self.credix_program_state.to_account_info(),
            global_market_state: self.credix_global_market_state.to_account_info(),
            signing_authority: self.credix_signing_authority.to_account_info(),
            liquidity_pool_token_account: self.credix_liquidity_collateral.to_account_info(),
            lp_token_mint: self.credix_shares_mint.to_account_info(),
            credix_pass: self.credix_pass.to_account_info(),
            treasury_pool_token_account: self.credix_treasury_collateral.to_account_info(),
            credix_multisig_key: self.credix_multisig_key.to_account_info(),
            credix_multisig_token_account: self.credix_multisig_collateral.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            associated_token_program: self.associated_token_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.credix_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_depository_collateral_to_profits_beneficiary_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.depository_collateral.to_account_info(),
            to: self.profits_beneficiary_collateral.to_account_info(),
            authority: self.depository.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Validate
impl<'info> CollectProfitsOfCredixLpDepository<'info> {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        Ok(())
    }
}
