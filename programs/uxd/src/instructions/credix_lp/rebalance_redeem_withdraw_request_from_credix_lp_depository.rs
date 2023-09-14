use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;

use crate::error::UxdError;
use crate::events::RebalanceRedeemWithdrawRequestFromCredixLpDepositoryEvent;
use crate::state::controller::Controller;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::state::identity_depository::IdentityDepository;
use crate::state::mercurial_vault_depository::MercurialVaultDepository;
use crate::utils::calculate_credix_lp_depository_target_amount;
use crate::utils::checked_add;
use crate::utils::checked_as_u64;
use crate::utils::checked_sub;
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
use crate::IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_NAMESPACE;

#[derive(Accounts)]
pub struct RebalanceRedeemWithdrawRequestFromCredixLpDepository<'info> {
    /// #1 // Permissionless IX that can be called by anyone at any time
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #2
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.identity_depository == identity_depository.key() @UxdError::InvalidDepository,
        constraint = controller.load()?.mercurial_vault_depository == mercurial_vault_depository.key() @UxdError::InvalidDepository,
        constraint = controller.load()?.credix_lp_depository == depository.key() @UxdError::InvalidDepository,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #3
    #[account(
        mut,
        seeds = [IDENTITY_DEPOSITORY_NAMESPACE],
        bump = identity_depository.load()?.bump,
    )]
    pub identity_depository: AccountLoader<'info, IdentityDepository>,

    /// #4
    #[account(
        mut,
        seeds = [IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE],
        token::authority = identity_depository,
        token::mint = identity_depository.load()?.collateral_mint,
        bump = identity_depository.load()?.collateral_vault_bump,
    )]
    pub identity_depository_collateral: Box<Account<'info, TokenAccount>>,

    /// #5
    #[account(
        mut,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_NAMESPACE, mercurial_vault_depository.load()?.mercurial_vault.key().as_ref(), mercurial_vault_depository.load()?.collateral_mint.as_ref()],
        bump = mercurial_vault_depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
    )]
    pub mercurial_vault_depository: AccountLoader<'info, MercurialVaultDepository>,

    /// #6
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

    /// #7
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #8
    #[account(mut)]
    pub depository_collateral: Box<Account<'info, TokenAccount>>,

    /// #9
    #[account(mut)]
    pub depository_shares: Box<Account<'info, TokenAccount>>,

    /// #10
    #[account(
        has_one = credix_treasury @UxdError::InvalidCredixMultisigKey,
    )]
    pub credix_program_state: Box<Account<'info, credix_client::ProgramState>>,

    /// #11
    #[account(
        mut,
        constraint = credix_global_market_state.treasury_pool_token_account == credix_treasury_pool_collateral.key() @UxdError::InvalidCredixTreasuryCollateral,
    )]
    pub credix_global_market_state: Box<Account<'info, credix_client::GlobalMarketState>>,

    /// #12
    /// CHECK: unused by us, checked by credix
    pub credix_signing_authority: AccountInfo<'info>,

    /// #13
    #[account(mut)]
    pub credix_liquidity_collateral: Box<Account<'info, TokenAccount>>,

    /// #14
    #[account(mut)]
    pub credix_shares_mint: Box<Account<'info, Mint>>,

    /// #15
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
        constraint = credix_pass.disable_withdrawal_fee() @UxdError::InvalidCredixPassNoFees,
    )]
    pub credix_pass: Account<'info, credix_client::CredixPass>,

    /// #16
    #[account(
        mut,
        token::mint = collateral_mint,
    )]
    pub credix_treasury_pool_collateral: Box<Account<'info, TokenAccount>>,

    /// #17
    /// CHECK: not used by us, checked by credix program
    pub credix_treasury: AccountInfo<'info>,

    /// #18
    #[account(
        mut,
        token::authority = credix_treasury,
        token::mint = collateral_mint,
    )]
    pub credix_treasury_collateral: Box<Account<'info, TokenAccount>>,

    /// #19
    #[account(
        mut,
        owner = credix_client::ID,
        seeds = [
            credix_global_market_state.key().as_ref(),
            &credix_global_market_state.latest_withdraw_epoch_idx.to_le_bytes(),
            CREDIX_LP_EXTERNAL_WITHDRAW_EPOCH_NAMESPACE
        ],
        bump,
        seeds::program = credix_client::ID,
    )]
    pub credix_withdraw_epoch: Account<'info, credix_client::WithdrawEpoch>,

    /// #20
    #[account(
        mut,
        token::mint = collateral_mint,
    )]
    pub profits_beneficiary_collateral: Box<Account<'info, TokenAccount>>,

    /// #21
    pub system_program: Program<'info, System>,
    /// #22
    pub token_program: Program<'info, Token>,
    /// #23
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// #24
    pub credix_program: Program<'info, credix_client::program::Credix>,
    /// #25
    pub rent: Sysvar<'info, Rent>,
}

pub(crate) fn handler(
    ctx: Context<RebalanceRedeemWithdrawRequestFromCredixLpDepository>,
) -> Result<()> {
    let credix_global_market_state = ctx.accounts.depository.load()?.credix_global_market_state;
    let collateral_mint = ctx.accounts.depository.load()?.collateral_mint;
    let depository_pda_signer: &[&[&[u8]]] = &[&[
        CREDIX_LP_DEPOSITORY_NAMESPACE,
        credix_global_market_state.as_ref(),
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.load()?.bump],
    ]];

    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Check if the withdraw epoch's redeem phase is active at the moment
    // ---------------------------------------------------------------------

    require!(
        ctx.accounts.credix_withdraw_epoch.status()?
            == credix_client::WithdrawEpochStatus::RedeemPhase,
        UxdError::InvalidCredixWithdrawEpochRedeemPhase,
    );

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Fetch all current onchain state
    // ---------------------------------------------------------------------

    let credix_lp_depository_collateral_amount_before: u64 =
        ctx.accounts.depository_collateral.amount;
    let identity_depository_collateral_amount_before: u64 =
        ctx.accounts.identity_depository_collateral.amount;
    let profits_beneficiary_collateral_amount_before: u64 =
        ctx.accounts.profits_beneficiary_collateral.amount;

    let total_shares_supply_before: u64 = ctx.accounts.credix_shares_mint.supply;
    let total_shares_value_before: u64 = {
        let liquidity_collateral_amount_before: u64 =
            ctx.accounts.credix_liquidity_collateral.amount;
        let outstanding_collateral_amount_before: u64 = ctx
            .accounts
            .credix_global_market_state
            .pool_outstanding_credit;
        checked_add(
            liquidity_collateral_amount_before,
            outstanding_collateral_amount_before,
        )?
    };

    let owned_shares_amount_before: u64 = ctx.accounts.depository_shares.amount;
    let owned_shares_value_before: u64 = compute_value_for_shares_amount_floor(
        owned_shares_amount_before,
        total_shares_supply_before,
        total_shares_value_before,
    )?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Compute the profits and the overflow of the depository we will want to withdraw
    // ---------------------------------------------------------------------

    let redeemable_amount_under_management = checked_as_u64(
        ctx.accounts
            .depository
            .load()?
            .redeemable_amount_under_management,
    )?;

    let profits_collateral_amount = checked_sub(
        owned_shares_value_before,
        redeemable_amount_under_management,
    )?;
    msg!(
        "[rebalance_redeem_withdraw_request_from_credix_lp_depository:profits_collateral_amount:{}]",
        profits_collateral_amount
    );

    let overflow_value = {
        let redeemable_amount_under_management_target_amount =
            calculate_credix_lp_depository_target_amount(
                &ctx.accounts.controller,
                &ctx.accounts.identity_depository,
                &ctx.accounts.mercurial_vault_depository,
                &ctx.accounts.depository,
            )?;
        if redeemable_amount_under_management < redeemable_amount_under_management_target_amount {
            0
        } else {
            checked_sub(
                redeemable_amount_under_management,
                redeemable_amount_under_management_target_amount,
            )?
        }
    };
    msg!(
        "[rebalance_redeem_withdraw_request_from_credix_lp_depository:overflow_value:{}]",
        overflow_value
    );

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- Here we compute how much we would like to withdraw now,
    // -- We also need to compute how much credix will let us withdraw now,
    // -- And where the withdrawn collateral will be going (profits or rebalanced)
    // ---------------------------------------------------------------------

    let withdrawable_total_collateral_amount =
        ctx.accounts.credix_withdraw_epoch.max_withdrawable_amount(
            &ctx.accounts.credix_global_market_state,
            ctx.accounts.depository.key(),
        )?;

    // How much we CAN withdraw now (may be lower than how much we want)
    msg!(
        "[rebalance_redeem_withdraw_request_from_credix_lp_depository:withdrawable_total_collateral_amount:{}]",
        withdrawable_total_collateral_amount
    );

    // We prioritize withdrawing the profits
    let withdrawal_profits_collateral_amount = std::cmp::min(
        withdrawable_total_collateral_amount,
        profits_collateral_amount,
    );

    // then we withdraw depository overflow first based on what liquidity is remaining from there
    let withdrawal_overflow_value = std::cmp::min(
        checked_sub(
            withdrawable_total_collateral_amount,
            withdrawal_profits_collateral_amount,
        )?,
        overflow_value,
    );

    // The sum of the two is what we will actually withdraw
    let withdrawal_total_collateral_amount = checked_add(
        withdrawal_profits_collateral_amount,
        withdrawal_overflow_value,
    )?;

    // ---------------------------------------------------------------------
    // -- Phase 5
    // -- Now we have to to predict the precision loss,
    // -- we have to recompute the funds destinations amounts
    // -- based on the final rounded down amount
    // ---------------------------------------------------------------------

    let withdrawal_total_shares_amount = compute_shares_amount_for_value_floor(
        withdrawal_total_collateral_amount,
        total_shares_supply_before,
        total_shares_value_before,
    )?;

    if withdrawal_total_shares_amount == 0 {
        msg!("[rebalance_redeem_withdraw_request_from_credix_lp_depository:no_withdrawable_liquidity]",);
        return Ok(());
    }

    let withdrawal_total_collateral_amount_after_precision_loss =
        compute_value_for_shares_amount_floor(
            withdrawal_total_shares_amount,
            total_shares_supply_before,
            total_shares_value_before,
        )?;

    // Precision loss should be taken from the profits, not the overflow
    // Otherwise this means that the precision loss would take out of the backing value
    let withdrawal_overflow_value_after_precision_loss = withdrawal_overflow_value;
    let withdrawal_profits_collateral_amount_after_precision_loss = checked_sub(
        withdrawal_total_collateral_amount_after_precision_loss,
        withdrawal_overflow_value_after_precision_loss,
    )?;

    // ---------------------------------------------------------------------
    // -- Phase 6
    // -- Actually run the withdrawal and transfer the collateral to the correct accounts
    // ---------------------------------------------------------------------

    msg!(
        "[rebalance_redeem_withdraw_request_from_credix_lp_depository:redeem_withdraw_request:{}]",
        withdrawal_total_collateral_amount
    );
    credix_client::cpi::redeem_withdraw_request(
        ctx.accounts
            .into_redeem_withdraw_request_from_credix_lp_context()
            .with_signer(depository_pda_signer),
        withdrawal_total_collateral_amount,
    )?;

    msg!(
        "[rebalance_redeem_withdraw_request_from_credix_lp_depository:profits_beneficiary_collateral_transfer:{}]",
        withdrawal_profits_collateral_amount_after_precision_loss
    );
    token::transfer(
        ctx.accounts
            .into_transfer_depository_collateral_to_profits_beneficiary_collateral_context()
            .with_signer(depository_pda_signer),
        withdrawal_profits_collateral_amount_after_precision_loss,
    )?;

    msg!(
        "[rebalance_redeem_withdraw_request_from_credix_lp_depository:identity_depository_collateral_transfer:{}]",
        withdrawal_overflow_value_after_precision_loss
    );
    token::transfer(
        ctx.accounts
            .into_transfer_depository_collateral_to_identity_depository_collateral_context()
            .with_signer(depository_pda_signer),
        withdrawal_overflow_value_after_precision_loss,
    )?;

    // Refresh account states after withdrawal
    {
        ctx.accounts.depository_collateral.reload()?;
        ctx.accounts.depository_shares.reload()?;
        ctx.accounts.credix_global_market_state.reload()?;
        ctx.accounts.credix_liquidity_collateral.reload()?;
        ctx.accounts.credix_shares_mint.reload()?;
        ctx.accounts.identity_depository_collateral.reload()?;
        ctx.accounts.profits_beneficiary_collateral.reload()?;
    }

    // ---------------------------------------------------------------------
    // -- Phase 7
    // -- Check the new ui state after the mutations
    // ---------------------------------------------------------------------

    let credix_lp_depository_collateral_amount_after: u64 =
        ctx.accounts.depository_collateral.amount;
    let identity_depository_collateral_amount_after: u64 =
        ctx.accounts.identity_depository_collateral.amount;
    let profits_beneficiary_collateral_amount_after: u64 =
        ctx.accounts.profits_beneficiary_collateral.amount;

    let total_shares_supply_after: u64 = ctx.accounts.credix_shares_mint.supply;
    let total_shares_value_after: u64 = {
        let liquidity_collateral_amount_after: u64 =
            ctx.accounts.credix_liquidity_collateral.amount;
        let outstanding_collateral_amount_after: u64 = ctx
            .accounts
            .credix_global_market_state
            .pool_outstanding_credit;
        checked_add(
            liquidity_collateral_amount_after,
            outstanding_collateral_amount_after,
        )?
    };

    let owned_shares_amount_after: u64 = ctx.accounts.depository_shares.amount;
    let owned_shares_value_after: u64 = compute_value_for_shares_amount_floor(
        owned_shares_amount_after,
        total_shares_supply_after,
        total_shares_value_after,
    )?;

    // ---------------------------------------------------------------------
    // -- Phase 8
    // -- Check that everything went exactly as planned
    // ---------------------------------------------------------------------

    let identity_depository_collateral_amount_increase: u64 = compute_increase(
        identity_depository_collateral_amount_before,
        identity_depository_collateral_amount_after,
    )?;
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

    // The credix lp depository collateral account should always be empty
    require!(
        credix_lp_depository_collateral_amount_before
            == credix_lp_depository_collateral_amount_after,
        UxdError::CollateralDepositHasRemainingDust
    );

    // Validate that the locked value moved exactly to the correct place
    require!(
        identity_depository_collateral_amount_increase
            == withdrawal_overflow_value_after_precision_loss,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );
    require!(
        profits_beneficiary_collateral_amount_increase
            == withdrawal_profits_collateral_amount_after_precision_loss,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );

    // Check that we withdrew the correct amount of shares
    require!(
        total_shares_supply_decrease == withdrawal_total_shares_amount,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );
    require!(
        owned_shares_amount_decrease == withdrawal_total_shares_amount,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );

    // Check that the new state of the pool still makes sense
    require!(
        is_within_range_inclusive(
            total_shares_value_decrease,
            withdrawal_total_collateral_amount_after_precision_loss,
            withdrawal_total_collateral_amount,
        ),
        UxdError::CollateralDepositDoesntMatchTokenValue,
    );
    require!(
        is_within_range_inclusive(
            owned_shares_value_decrease,
            withdrawal_total_collateral_amount_after_precision_loss,
            withdrawal_total_collateral_amount,
        ),
        UxdError::CollateralDepositDoesntMatchTokenValue,
    );

    // ---------------------------------------------------------------------
    // -- Phase 9
    // -- Emit resulting event, and update onchain accounting
    // ---------------------------------------------------------------------

    // Emit event
    emit!(RebalanceRedeemWithdrawRequestFromCredixLpDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: ctx.accounts.depository.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        overflow_value: withdrawal_overflow_value_after_precision_loss,
        profits_collateral_amount: withdrawal_profits_collateral_amount_after_precision_loss,
        requested_collateral_amount: withdrawal_total_collateral_amount_after_precision_loss,
    });

    // Edit onchain accounts
    let mut controller = ctx.accounts.controller.load_mut()?;
    let mut depository = ctx.accounts.depository.load_mut()?;
    let mut identity_depository = ctx.accounts.identity_depository.load_mut()?;

    // Profit collection accounting updates
    controller.update_onchain_accounting_following_profits_collection(
        withdrawal_profits_collateral_amount_after_precision_loss,
    )?;
    depository.update_onchain_accounting_following_profits_collection(
        withdrawal_profits_collateral_amount_after_precision_loss,
    )?;

    // Collateral amount deposited accounting updates
    depository.collateral_amount_deposited = checked_sub(
        depository.collateral_amount_deposited,
        withdrawal_overflow_value_after_precision_loss.into(),
    )?;
    identity_depository.collateral_amount_deposited = checked_add(
        identity_depository.collateral_amount_deposited,
        withdrawal_overflow_value_after_precision_loss.into(),
    )?;

    // Redeemable under management accounting updates
    depository.redeemable_amount_under_management = checked_sub(
        depository.redeemable_amount_under_management,
        withdrawal_overflow_value_after_precision_loss.into(),
    )?;
    identity_depository.redeemable_amount_under_management = checked_add(
        identity_depository.redeemable_amount_under_management,
        withdrawal_overflow_value_after_precision_loss.into(),
    )?;

    // Done
    Ok(())
}

// Into functions
impl<'info> RebalanceRedeemWithdrawRequestFromCredixLpDepository<'info> {
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
            treasury_pool_token_account: self.credix_treasury_pool_collateral.to_account_info(),
            credix_treasury: self.credix_treasury.to_account_info(),
            credix_treasury_token_account: self.credix_treasury_collateral.to_account_info(),
            withdraw_epoch: self.credix_withdraw_epoch.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            associated_token_program: self.associated_token_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.credix_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_depository_collateral_to_identity_depository_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.depository_collateral.to_account_info(),
            to: self.identity_depository_collateral.to_account_info(),
            authority: self.depository.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
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
impl<'info> RebalanceRedeemWithdrawRequestFromCredixLpDepository<'info> {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        Ok(())
    }
}
