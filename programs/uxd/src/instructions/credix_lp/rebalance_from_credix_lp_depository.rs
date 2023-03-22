use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;

use crate::error::UxdError;
use crate::state::controller::Controller;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::state::identity_depository::IdentityDepository;
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
use crate::IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_NAMESPACE;
use crate::SECONDS_IN_A_DAY;

#[derive(Accounts)]
pub struct RebalanceFromCredixLpDepository<'info> {
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
    #[account(
        mut,
        seeds = [IDENTITY_DEPOSITORY_NAMESPACE],
        bump = identity_depository.load()?.bump,
    )]
    pub identity_depository: AccountLoader<'info, IdentityDepository>,

    /// #18
    #[account(
        mut,
        seeds = [IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE],
        token::authority = identity_depository,
        token::mint = identity_depository.load()?.collateral_mint,
        bump = identity_depository.load()?.collateral_vault_bump,
    )]
    pub identity_depository_collateral: Box<Account<'info, TokenAccount>>,

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

pub(crate) fn handler(ctx: Context<RebalanceFromCredixLpDepository>) -> Result<()> {
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
    // -- Fetch all current onchain state
    // ---------------------------------------------------------------------

    let credix_lp_depository_collateral_amount_before: u64 =
        ctx.accounts.depository_collateral.amount;
    let identity_depository_collateral_amount_before: u64 =
        ctx.accounts.identity_depository_collateral.amount;
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
    let owned_shares_value_before: u64 = compute_value_for_shares_amount_floor(
        owned_shares_amount_before,
        total_shares_supply_before,
        total_shares_value_before,
    )?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Compute the desired amounts for profits collection and rebalancing
    // -- Both of those will be used to determine the actual withdrawn amount
    // ---------------------------------------------------------------------

    let profits_collateral_amount = ctx
        .accounts
        .calculate_profits_collateral_amount(owned_shares_value_before)?;

    let overflow_collateral_amount = ctx.accounts.calculate_overflow_collateral_amount()?;

    msg!(
        "[rebalance_from_credix_lp_depository:profits_collateral_amount:{}]",
        profits_collateral_amount
    );
    msg!(
        "[rebalance_from_credix_lp_depository:overflow_collateral_amount:{}]",
        overflow_collateral_amount
    );

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- If we are currently in the withdrawal request creation phase
    // -- Just create the withdrawal request and end the transaction
    // -- We will have to wait for continuing further later
    // ---------------------------------------------------------------------

    if ctx.accounts.check_if_request_phase_is_active()? {
        let requested_collateral_amount = overflow_collateral_amount
            .checked_add(profits_collateral_amount)
            .ok_or(UxdError::MathError)?;
        msg!(
            "[rebalance_from_credix_lp_depository:requested_collateral_amount:{}]",
            requested_collateral_amount
        );
        credix_client::cpi::create_withdraw_request(
            ctx.accounts
                .into_create_withdraw_request_from_credix_lp_context()
                .with_signer(depository_pda_signer),
            requested_collateral_amount,
        )?;
        return Ok(());
    }

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- If we got there it means we are in the redeem phase
    // -- Here we compute how much we would like to withdraw now,
    // -- We also need to compute how much credix will let us withdraw now,
    // -- And where the withdrawn collateral will be going
    // ---------------------------------------------------------------------

    // Read credix withdrawal accounts onchain state
    let locked_liquidity = ctx.accounts.credix_global_market_state.locked_liquidity;
    let investor_total_lp_amount = ctx
        .accounts
        .credix_withdraw_request
        .investor_total_lp_amount;
    let participating_investors_total_lp_amount = ctx
        .accounts
        .credix_withdraw_epoch
        .participating_investors_total_lp_amount;
    let base_amount_withdrawn = ctx.accounts.credix_withdraw_request.base_amount_withdrawn;

    // All investors gets an equivalent slice of the locked liquidity,
    // based on their relative position size in the lp pool
    let withdrawable_total_collateral_amount = locked_liquidity
        .checked_mul(investor_total_lp_amount)
        .ok_or(UxdError::MathError)?
        .checked_div(participating_investors_total_lp_amount)
        .ok_or(UxdError::MathError)?
        .checked_sub(base_amount_withdrawn)
        .ok_or(UxdError::MathError)?;

    // We prioritize withdrawing the overflow first based on what liquidity is available
    let withdrawable_overflow_collateral_amount = std::cmp::min(
        withdrawable_total_collateral_amount,
        overflow_collateral_amount,
    );
    // Then if liquidity is still available for profits, also withdraw it
    let withdrawable_profits_collateral_amount = std::cmp::min(
        withdrawable_total_collateral_amount
            .checked_sub(withdrawable_overflow_collateral_amount)
            .ok_or(UxdError::MathError)?,
        profits_collateral_amount,
    );
    // The sum of the two is what we will actually withdraw
    let withdraw_total_collateral_amount = withdrawable_overflow_collateral_amount
        .checked_add(withdrawable_profits_collateral_amount)
        .ok_or(UxdError::MathError)?;

    // ---------------------------------------------------------------------
    // -- Phase 5
    // -- Now we have to to handle precision loss,
    // -- we have to recompute the funds destinations amounts
    // -- based on the final rounded down amount
    // ---------------------------------------------------------------------

    let withdraw_total_shares_amount = compute_shares_amount_for_value_floor(
        withdraw_total_collateral_amount,
        total_shares_supply_before,
        total_shares_value_before,
    )?;
    let withdraw_total_collateral_amount_after_precision_loss =
        compute_value_for_shares_amount_floor(
            withdraw_total_shares_amount,
            total_shares_supply_before,
            total_shares_value_before,
        )?;
    let withdrawable_overflow_collateral_amount_after_precision_loss = std::cmp::min(
        withdraw_total_collateral_amount_after_precision_loss,
        withdrawable_overflow_collateral_amount,
    );
    let withdrawable_profits_collateral_amount_after_precision_loss = std::cmp::min(
        withdraw_total_collateral_amount_after_precision_loss
            .checked_sub(withdrawable_overflow_collateral_amount_after_precision_loss)
            .ok_or(UxdError::MathError)?,
        withdrawable_profits_collateral_amount,
    );

    // ---------------------------------------------------------------------
    // -- Phase 6
    // -- Actually run the withdrawal
    // ---------------------------------------------------------------------

    msg!(
        "[rebalance_from_credix_lp_depository:redeem_withdraw_request:{}]",
        withdraw_total_collateral_amount
    );
    credix_client::cpi::redeem_withdraw_request(
        ctx.accounts
            .into_redeem_withdraw_request_from_credix_lp_context()
            .with_signer(depository_pda_signer),
        withdraw_total_collateral_amount,
    )?;

    msg!(
        "[rebalance_from_credix_lp_depository:identity_depository_collateral_transfer:{}]",
        withdrawable_overflow_collateral_amount_after_precision_loss
    );
    token::transfer(
        ctx.accounts
            .into_transfer_depository_collateral_to_identity_depository_collateral_context()
            .with_signer(depository_pda_signer),
        withdrawable_overflow_collateral_amount_after_precision_loss,
    )?;

    msg!(
        "[rebalance_from_credix_lp_depository:profits_beneficiary_collateral_transfer:{}]",
        withdrawable_profits_collateral_amount_after_precision_loss
    );
    token::transfer(
        ctx.accounts
            .into_transfer_depository_collateral_to_profits_beneficiary_collateral_context()
            .with_signer(depository_pda_signer),
        withdrawable_profits_collateral_amount_after_precision_loss,
    )?;

    // Refresh account states after withdrawal
    ctx.accounts.depository_collateral.reload()?;
    ctx.accounts.depository_shares.reload()?;
    ctx.accounts.credix_global_market_state.reload()?;
    ctx.accounts.credix_liquidity_collateral.reload()?;
    ctx.accounts.credix_shares_mint.reload()?;
    ctx.accounts.profits_beneficiary_collateral.reload()?;

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
            == withdrawable_overflow_collateral_amount_after_precision_loss,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );
    require!(
        profits_beneficiary_collateral_amount_increase
            == withdrawable_profits_collateral_amount_after_precision_loss,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );

    // Check that we withdrew the correct amount of shares
    require!(
        total_shares_supply_decrease == withdraw_total_shares_amount,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );
    require!(
        owned_shares_amount_decrease == withdraw_total_shares_amount,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );

    // Check that the new state of the pool still makes sense
    require!(
        is_within_range_inclusive(
            total_shares_value_decrease,
            withdraw_total_collateral_amount_after_precision_loss,
            withdraw_total_collateral_amount,
        ),
        UxdError::CollateralDepositDoesntMatchTokenValue,
    );
    require!(
        is_within_range_inclusive(
            owned_shares_value_decrease,
            withdraw_total_collateral_amount_after_precision_loss,
            withdraw_total_collateral_amount,
        ),
        UxdError::CollateralDepositDoesntMatchTokenValue,
    );

    // Done
    Ok(())
}

// Utility functions
impl<'info> RebalanceFromCredixLpDepository<'info> {
    pub fn check_if_request_phase_is_active(&self) -> Result<bool> {
        let current_unix_timestamp = Clock::get()?.unix_timestamp;
        let go_live_unix_timestamp = self.credix_withdraw_epoch.go_live;

        let request_phase_days = self.credix_withdraw_epoch.request_days;
        let request_phase_seconds = i64::from(request_phase_days)
            .checked_mul(SECONDS_IN_A_DAY)
            .ok_or(UxdError::MathError)?;

        let end_of_request_phase_timestamp = go_live_unix_timestamp
            .checked_add(request_phase_seconds)
            .ok_or(UxdError::MathError)?;

        Ok(current_unix_timestamp < end_of_request_phase_timestamp)
    }

    pub fn calculate_profits_collateral_amount(&self, owned_shares_value: u64) -> Result<u64> {
        let redeemable_amount_under_management = checked_convert_u128_to_u64(
            self.depository.load()?.redeemable_amount_under_management,
        )?;
        // To find the profits we can safely withdraw
        // we find the current value of asset (the lp tokens)
        // minus the minimum liabilities (the outstanding redeemable tokens)
        Ok(owned_shares_value
            .checked_sub(redeemable_amount_under_management)
            .ok_or(UxdError::MathError)?)
    }

    pub fn calculate_overflow_collateral_amount(&self) -> Result<u64> {
        // TODO - use weights properly from step 1
        // We fetch the target amount of redeemable that we wish the depository have
        let redeemable_amount_under_management_target_amount =
            checked_convert_u128_to_u64(self.controller.load()?.redeemable_circulating_supply / 2)?;

        let redeemable_amount_under_management = checked_convert_u128_to_u64(
            self.depository.load()?.redeemable_amount_under_management,
        )?;

        // If the depository is currently underweight, there is no overflow
        if redeemable_amount_under_management < redeemable_amount_under_management_target_amount {
            return Ok(0);
        }
        // We substract the current redeemable amount from the target redeemable amount
        // to find how much needs to be withdrawn from the depository
        Ok(redeemable_amount_under_management_target_amount
            .checked_sub(redeemable_amount_under_management)
            .ok_or(UxdError::MathError)?)
    }
}

// Into functions
impl<'info> RebalanceFromCredixLpDepository<'info> {
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
impl<'info> RebalanceFromCredixLpDepository<'info> {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        Ok(())
    }
}
