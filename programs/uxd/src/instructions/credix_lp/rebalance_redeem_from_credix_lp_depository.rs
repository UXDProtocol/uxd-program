use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;

use crate::error::UxdError;
use crate::events::RebalanceRedeemFromCredixLpDepositoryEvent;
use crate::state::controller::Controller;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::utils::checked_convert_u128_to_u64;
use crate::utils::compute_decrease;
use crate::utils::compute_increase;
use crate::utils::compute_shares_amount_for_value_floor;
use crate::utils::compute_value_for_shares_amount_floor;
use crate::utils::is_within_range_inclusive;
use crate::validate_is_program_frozen;
use crate::CONTROLLER_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_NAMESPACE;
use crate::CREDIX_LP_EXTERNAL_PASS_NAMESPACE;
use crate::CREDIX_LP_EXTERNAL_WITHDRAW_EPOCH_NAMESPACE;
use crate::CREDIX_LP_EXTERNAL_WITHDRAW_REQUEST_NAMESPACE;

#[derive(Accounts)]
pub struct RebalanceRedeemFromCredixLpDepository<'info> {
    /// #1 // Permissionless IX that can be called by anyone at any time
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

    /// #15
    #[account(
        mut,
        owner = credix_client::ID,
        seeds = [
            credix_global_market_state.key().as_ref(),
            &credix_global_market_state.latest_withdraw_epoch_idx.to_be_bytes(),
            CREDIX_LP_EXTERNAL_WITHDRAW_EPOCH_NAMESPACE
        ],
        bump,
        seeds::program = credix_client::ID,
    )]
    pub credix_withdraw_epoch: Account<'info, credix_client::WithdrawEpoch>,

    #[account(
        mut,
        owner = credix_client::ID,
        seeds = [
            credix_global_market_state.key().as_ref(),
            depository.key().as_ref(),
            &credix_global_market_state.latest_withdraw_epoch_idx.to_be_bytes(),
            CREDIX_LP_EXTERNAL_WITHDRAW_REQUEST_NAMESPACE
        ],
        bump,
        seeds::program = credix_client::ID,
    )]
    pub credix_withdraw_request: Account<'info, credix_client::WithdrawRequest>,

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

pub(crate) fn handler(ctx: Context<RebalanceRedeemFromCredixLpDepository>) -> Result<()> {
    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Fetch all current onchain state
    // -- and predict all future final state after mutation
    // ---------------------------------------------------------------------

    // Read all states before withdrawal
    let depository_collateral_amount_before: u64 = ctx.accounts.depository_collateral.amount;
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
    let owned_shares_value_before: u64 = compute_value_for_shares_amount_floor(
        owned_shares_amount_before,
        total_shares_supply_before,
        total_shares_value_before,
    )?;

    // How much collateral can we withdraw as profits
    let ideal_profits_collateral_amount = {
        // Compute the set of liabilities owed to the users
        let liabilities_redeemable_amount = checked_convert_u128_to_u64(
            ctx.accounts
                .depository
                .load()?
                .redeemable_amount_under_management,
        )?;
        msg!(
            "[rebalance_request_from_credix_lp_depository:liabilities_redeemable_amount:{}]",
            liabilities_redeemable_amount
        );
        // We assume that liabilities in redeemable are equivalent 1:1 to the same amount in collateral
        let liabilities_collateral_amount = liabilities_redeemable_amount;
        // Compute the set of assets owned in the LP in collateral amount
        let assets_collateral_amount = owned_shares_value_before;
        msg!(
            "[rebalance_request_from_credix_lp_depository:assets_collateral_amount:{}]",
            assets_collateral_amount
        );
        // Compute the amount of profits that we can safely withdraw
        assets_collateral_amount
            .checked_sub(liabilities_collateral_amount)
            .ok_or(UxdError::MathError)?
    };
    msg!(
        "[rebalance_request_from_credix_lp_depository:ideal_profits_collateral_amount:{}]",
        ideal_profits_collateral_amount
    );

    let ideal_rebalanced_redeemable_amount = {
        // TODO - use weights
        let redeemable_amount_under_management_target_amount = checked_convert_u128_to_u64(
            ctx.accounts
                .controller
                .load()?
                .redeemable_circulating_supply
                / 2,
        )?;
        msg!(
            "[rebalance_request_from_credix_lp_depository:redeemable_amount_under_management_target_amount:{}]",
            redeemable_amount_under_management_target_amount
        );
        let redeemable_amount_under_management = checked_convert_u128_to_u64(
            ctx.accounts
                .depository
                .load()?
                .redeemable_amount_under_management,
        )?;
        msg!(
            "[rebalance_request_from_credix_lp_depository:redeemable_amount_under_management:{}]",
            redeemable_amount_under_management
        );
        redeemable_amount_under_management_target_amount
            .checked_sub(redeemable_amount_under_management)
            .ok_or(UxdError::MathError)?
    };
    msg!(
        "[rebalance_request_from_credix_lp_depository:ideal_rebalanced_redeemable_amount:{}]",
        ideal_rebalanced_redeemable_amount
    );

    // We assume that the redeemable amount we need to rebalance is the same amount in collateral token
    let ideal_rebalanced_collateral_amount = ideal_rebalanced_redeemable_amount;

    let ideal_withdraw_collateral_amount = ideal_rebalanced_collateral_amount
        .checked_add(ideal_profits_collateral_amount)
        .ok_or(UxdError::MathError)?;
    msg!(
        "[rebalance_request_from_credix_lp_depository:ideal_withdraw_collateral_amount:{}]",
        ideal_withdraw_collateral_amount
    );

    // Assumes and enforce a collateral/redeemable 1:1 relationship on purpose
    let collateral_amount_before_precision_loss: u64 = ideal_withdraw_collateral_amount
    msg!(
        "[rebalance_request_from_credix_lp_depository:collateral_amount_before_precision_loss:{}]",
        collateral_amount_before_precision_loss
    );

    // Compute the amount of shares that we need to withdraw based on the amount of wanted collateral
    let shares_amount: u64 = compute_shares_amount_for_value_floor(
        collateral_amount_before_precision_loss,
        total_shares_supply_before,
        total_shares_value_before,
    )?;
    msg!(
        "[rebalance_request_from_credix_lp_depository:shares_amount:{}]",
        shares_amount
    );

    // Compute the amount of collateral that the withdrawn shares are worth (after potential precision loss)
    let collateral_amount_after_precision_loss: u64 = compute_value_for_shares_amount_floor(
        shares_amount,
        total_shares_supply_before,
        total_shares_value_before,
    )?;
    msg!(
        "[rebalance_request_from_credix_lp_depository:collateral_amount_after_precision_loss:{}]",
        collateral_amount_after_precision_loss
    );

    // If nothing to withdraw, no need to continue, all profits have already been successfully collected
    if collateral_amount_after_precision_loss == 0 {
        msg!("[rebalance_request_from_credix_lp_depository:no_profits_to_collect]",);
        return Ok(());
    }

    let test0 = ctx.accounts.credix_global_market_state.locked_liquidity;

    let test = ctx.accounts.credix_withdraw_epoch.participating_investors_total_lp_amount;
    let test2 = ctx.accounts.credix_withdraw_epoch.total_requested_base_amount;

    let test3 = ctx.accounts.credix_withdraw_request.base_amount;
    let test4 = ctx.accounts.credix_withdraw_request.investor_total_lp_amount;

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

    msg!("[rebalance_request_from_credix_lp_depository:create_withdraw_request]",);
    credix_client::cpi::create_withdraw_request(
        ctx.accounts
            .into_create_withdraw_request_from_credix_lp_context()
            .with_signer(depository_pda_signer),
        collateral_amount_before_precision_loss,
    )?;
    

    msg!("[rebalance_request_from_credix_lp_depository:redeem_withdraw_request]",);
    credix_client::cpi::redeem_withdraw_request(
        ctx.accounts
            .into_redeem_withdraw_request_from_credix_lp_context()
            .with_signer(depository_pda_signer),
        collateral_amount_before_precision_loss,
    )?;

    // Transfer the received collateral from the depository to the end user
    msg!("[rebalance_request_from_credix_lp_depository:collateral_transfer]",);
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
    // -- Phase 4
    // -- Emit resulting event, and update onchain accounting
    // ---------------------------------------------------------------------

    // Emit event
    emit!(RebalanceRedeemFromCredixLpDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: ctx.accounts.depository.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_amount: collateral_amount_after_precision_loss,
    });

    // Accounting for depository
    let mut depository = ctx.accounts.depository.load_mut()?;
    // TODO
    depository.update_onchain_accounting_following_profits_collection(
        collateral_amount_after_precision_loss,
    )?;

    // Accounting for controller
    let mut controller = ctx.accounts.controller.load_mut()?;
    // TODO
    controller.update_onchain_accounting_following_profits_collection(
        collateral_amount_after_precision_loss,
    )?;

    // Done
    Ok(())
}

// Into functions
impl<'info> RebalanceRedeemFromCredixLpDepository<'info> {
    pub fn into_create_withdraw_request_from_credix_lp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, credix_client::cpi::accounts::CreateWithdrawRequest<'info>>
    {
        let cpi_accounts = credix_client::cpi::accounts::CreateWithdrawRequest {
            investor: self.depository.to_account_info(),
            investor_lp_token_account: self.depository_shares.to_account_info(),
            global_market_state: self.credix_global_market_state.to_account_info(),
            signing_authority: self.credix_signing_authority.to_account_info(),
            liquidity_pool_token_account: self.credix_liquidity_collateral.to_account_info(),
            lp_token_mint: self.credix_shares_mint.to_account_info(),
            credix_pass: self.credix_pass.to_account_info(),
            withdraw_epoch: self.credix_withdraw_epoch.to_account_info(),
            withdraw_request: self.credix_withdraw_request.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        let cpi_program = self.credix_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_redeem_withdraw_request_from_credix_lp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, credix_client::cpi::accounts::RedeemWithdrawRequest<'info>>
    {
        let cpi_accounts = credix_client::cpi::accounts::RedeemWithdrawRequest {
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
            withdraw_epoch: self.credix_withdraw_epoch.to_account_info(),
            withdraw_request: self.credix_withdraw_request.to_account_info(),
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
impl<'info> RebalanceRedeemFromCredixLpDepository<'info> {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        Ok(())
    }
}
