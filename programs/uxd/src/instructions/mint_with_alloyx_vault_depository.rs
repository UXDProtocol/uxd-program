use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;

use crate::error::UxdError;
use crate::events::MintWithAlloyxVaultDepositoryEvent;
use crate::state::alloyx_vault_depository::AlloyxVaultDepository;
use crate::state::controller::Controller;
use crate::utils::calculate_amount_less_fees;
use crate::utils::checked_add;
use crate::utils::compute_decrease;
use crate::utils::compute_increase;
use crate::utils::compute_shares_amount_for_value_floor;
use crate::utils::compute_value_for_shares_amount_floor;
use crate::utils::is_within_range_inclusive;
use crate::utils::validate_collateral_amount;
use crate::validate_is_program_frozen;
use crate::ALLOYX_VAULT_DEPOSITORY_NAMESPACE;
use crate::CONTROLLER_NAMESPACE;

#[derive(Accounts)]
#[instruction(collateral_amount: u64)]
pub struct MintWithAlloyxVaultDepository<'info> {
    /// #1 This IX should only be accessible by the router or the DAO
    #[account(
        constraint = (
            authority.key() == controller.key()
            || authority.key() == controller.load()?.authority
        )  @UxdError::InvalidAuthority,
    )]
    pub authority: Signer<'info>,

    /// #2
    pub user: Signer<'info>,

    /// #3
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #4
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.alloyx_vault_depository == depository.key() @UxdError::InvalidDepository,
        has_one = redeemable_mint @UxdError::InvalidRedeemableMint
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #5
    #[account(
        mut,
        seeds = [
            ALLOYX_VAULT_DEPOSITORY_NAMESPACE,
            depository.load()?.alloyx_vault_info.key().as_ref(),
            depository.load()?.collateral_mint.as_ref()
        ],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        has_one = depository_collateral @UxdError::InvalidDepositoryCollateral,
        has_one = depository_shares @UxdError::InvalidDepositoryShares,
        has_one = alloyx_vault_info @UxdError::InvalidAlloyxVaultInfo,
        has_one = alloyx_vault_collateral @UxdError::InvalidAlloyxVaultCollateral,
        has_one = alloyx_vault_shares @UxdError::InvalidAlloyxVaultShares,
        has_one = alloyx_vault_mint @UxdError::InvalidAlloyxVaultMint,
    )]
    pub depository: AccountLoader<'info, AlloyxVaultDepository>,

    /// #6
    #[account(mut)]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// #7
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #8
    #[account(
        mut,
        constraint = user_redeemable.owner == user.key() @UxdError::InvalidOwner,
        constraint = user_redeemable.mint == redeemable_mint.key() @UxdError::InvalidRedeemableMint,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #9
    #[account(
        mut,
        constraint = user_collateral.owner == user.key() @UxdError::InvalidOwner,
        constraint = user_collateral.mint == collateral_mint.key() @UxdError::InvalidCollateralMint,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #10
    #[account(mut)]
    pub depository_collateral: Box<Account<'info, TokenAccount>>,

    /// #11
    #[account(mut)]
    pub depository_shares: Box<Account<'info, TokenAccount>>,

    /// #12
    pub alloyx_vault_info: Box<Account<'info, alloyx_cpi::VaultInfo>>,

    /// #13
    #[account(mut)]
    pub alloyx_vault_collateral: Box<Account<'info, TokenAccount>>,

    /// #14
    #[account(mut)]
    pub alloyx_vault_shares: Box<Account<'info, TokenAccount>>,

    /// #15
    #[account(mut)]
    pub alloyx_vault_mint: Box<Account<'info, Mint>>,

    /// #16
    #[account(
        constraint = alloyx_vault_pass.investor == depository.key() @UxdError::InvalidAlloyxVaultPass,
    )]
    pub alloyx_vault_pass: Account<'info, alloyx_cpi::PassInfo>,

    /// #17
    pub system_program: Program<'info, System>,
    /// #18
    pub token_program: Program<'info, Token>,
    /// #19
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// #20
    pub alloyx_program: Program<'info, alloyx_cpi::program::AlloyxSolana>,
    /// #21
    pub rent: Sysvar<'info, Rent>,
}

pub(crate) fn handler(
    ctx: Context<MintWithAlloyxVaultDepository>,
    collateral_amount: u64,
) -> Result<()> {
    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Fetch all current onchain state
    // -- and predict all future final state after mutation
    // ---------------------------------------------------------------------

    // Read all state before deposit
    let depository_collateral_amount_before: u64 = ctx.accounts.depository_collateral.amount;
    let user_collateral_amount_before: u64 = ctx.accounts.user_collateral.amount;
    let user_redeemable_amount_before: u64 = ctx.accounts.user_redeemable.amount;

    let alloyx_vault_base_collateral_amount_before: u64 =
        ctx.accounts.alloyx_vault_collateral.amount;
    let alloyx_vault_desk_collateral_amount_before: u64 =
        ctx.accounts.alloyx_vault_info.wallet_desk_usdc_value;

    let total_shares_supply_before: u64 = ctx.accounts.alloyx_vault_mint.supply;
    let total_shares_value_before: u64 = checked_add(
        alloyx_vault_base_collateral_amount_before,
        alloyx_vault_desk_collateral_amount_before,
    )?;

    let owned_shares_amount_before: u64 = ctx.accounts.depository_shares.amount;
    let owned_shares_value_before: u64 = compute_value_for_shares_amount_floor(
        owned_shares_amount_before,
        total_shares_supply_before,
        total_shares_value_before,
    )?;

    // Initial amount
    msg!(
        "[mint_with_alloyx_vault_depository:collateral_amount:{}]",
        collateral_amount
    );

    // Compute the amount of shares that we will get for our collateral
    let shares_amount: u64 = compute_shares_amount_for_value_floor(
        collateral_amount,
        total_shares_supply_before,
        total_shares_value_before,
    )?;
    msg!(
        "[mint_with_alloyx_vault_depository:shares_amount:{}]",
        shares_amount
    );
    require!(
        shares_amount > 0,
        UxdError::MinimumMintedRedeemableAmountError
    );

    // Compute the amount of collateral that the received shares are worth (after potential precision loss)
    let collateral_amount_after_precision_loss: u64 = compute_value_for_shares_amount_floor(
        shares_amount,
        total_shares_supply_before,
        total_shares_value_before,
    )?;
    msg!(
        "[mint_with_alloyx_vault_depository:collateral_amount_after_precision_loss:{}]",
        collateral_amount_after_precision_loss
    );
    require!(
        collateral_amount_after_precision_loss > 0,
        UxdError::MinimumMintedRedeemableAmountError
    );

    // Assumes and enforce a collateral/redeemable 1:1 relationship on purpose
    let redeemable_amount_before_fees: u64 = collateral_amount_after_precision_loss;

    // Apply the redeeming fees
    let redeemable_amount_after_fees: u64 = calculate_amount_less_fees(
        redeemable_amount_before_fees,
        ctx.accounts.depository.load()?.minting_fee_in_bps,
    )?;
    msg!(
        "[mint_with_alloyx_vault_depository:redeemable_amount_after_fees:{}]",
        redeemable_amount_after_fees
    );
    require!(
        redeemable_amount_after_fees > 0,
        UxdError::MinimumMintedRedeemableAmountError
    );
    require!(
        redeemable_amount_after_fees <= collateral_amount,
        UxdError::MaximumMintedRedeemableAmountError
    );

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Actually runs the onchain mutation based on computed parameters
    // ---------------------------------------------------------------------

    // Make controller signer
    let controller_pda_signer: &[&[&[u8]]] = &[&[
        CONTROLLER_NAMESPACE,
        &[ctx.accounts.controller.load()?.bump],
    ]];

    // Make depository signer
    let alloyx_vault_info = ctx.accounts.depository.load()?.alloyx_vault_info;
    let collateral_mint = ctx.accounts.depository.load()?.collateral_mint;
    let depository_pda_signer: &[&[&[u8]]] = &[&[
        ALLOYX_VAULT_DEPOSITORY_NAMESPACE,
        alloyx_vault_info.as_ref(),
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.load()?.bump],
    ]];

    // Transfer the collateral to an account owned by the depository
    msg!("[mint_with_alloyx_vault_depository:collateral_transfer]",);
    token::transfer(
        ctx.accounts
            .into_transfer_user_collateral_to_depository_collateral_context(),
        collateral_amount,
    )?;

    // Do the deposit by placing collateral owned by the depository into the pool
    msg!("[mint_with_alloyx_vault_depository:deposit_controller]",);
    alloyx_cpi::cpi::deposit(
        ctx.accounts
            .into_deposit_collateral_to_alloyx_vault_context()
            .with_signer(depository_pda_signer),
        "hello".to_string(),
        collateral_amount,
    )?;

    // Mint redeemable to the user
    msg!("[mint_with_alloyx_vault_depository:mint_redeemable]",);
    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_pda_signer),
        redeemable_amount_after_fees,
    )?;

    // Refresh account states after deposit
    ctx.accounts.depository_collateral.reload()?;
    ctx.accounts.depository_shares.reload()?;
    ctx.accounts.user_collateral.reload()?;
    ctx.accounts.user_redeemable.reload()?;
    ctx.accounts.alloyx_vault_info.reload()?;
    ctx.accounts.alloyx_vault_collateral.reload()?;
    ctx.accounts.alloyx_vault_mint.reload()?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Strictly verify that the onchain state
    // -- after mutation exactly match previous predictions
    // ---------------------------------------------------------------------

    // Read all state after deposit
    let depository_collateral_amount_after: u64 = ctx.accounts.depository_collateral.amount;
    let user_collateral_amount_after: u64 = ctx.accounts.user_collateral.amount;
    let user_redeemable_amount_after: u64 = ctx.accounts.user_redeemable.amount;

    let alloyx_vault_base_collateral_amount_after: u64 =
        ctx.accounts.alloyx_vault_collateral.amount;
    let alloyx_vault_desk_collateral_amount_after: u64 =
        ctx.accounts.alloyx_vault_info.wallet_desk_usdc_value;

    let total_shares_supply_after: u64 = ctx.accounts.alloyx_vault_mint.supply;
    let total_shares_value_after: u64 = checked_add(
        alloyx_vault_base_collateral_amount_after,
        alloyx_vault_desk_collateral_amount_after,
    )?;

    let owned_shares_amount_after: u64 = ctx.accounts.depository_shares.amount;
    let owned_shares_value_after: u64 = compute_value_for_shares_amount_floor(
        owned_shares_amount_after,
        total_shares_supply_after,
        total_shares_value_after,
    )?;

    // Compute changes in states
    let user_collateral_amount_decrease: u64 =
        compute_decrease(user_collateral_amount_before, user_collateral_amount_after)?;
    let user_redeemable_amount_increase: u64 =
        compute_increase(user_redeemable_amount_before, user_redeemable_amount_after)?;

    let total_shares_supply_increase: u64 =
        compute_increase(total_shares_supply_before, total_shares_supply_after)?;
    let total_shares_value_increase: u64 =
        compute_increase(total_shares_value_before, total_shares_value_after)?;

    let owned_shares_amount_increase: u64 =
        compute_increase(owned_shares_amount_before, owned_shares_amount_after)?;
    let owned_shares_value_increase: u64 =
        compute_increase(owned_shares_value_before, owned_shares_value_after)?;

    // Log deltas for debriefing the changes
    msg!(
        "[mint_with_alloyx_vault_depository:user_collateral_amount_decrease:{}]",
        user_collateral_amount_decrease
    );
    msg!(
        "[mint_with_alloyx_vault_depository:user_redeemable_amount_increase:{}]",
        user_redeemable_amount_increase
    );
    msg!(
        "[mint_with_alloyx_vault_depository:total_shares_supply_increase:{}]",
        total_shares_supply_increase
    );
    msg!(
        "[mint_with_alloyx_vault_depository:total_shares_value_increase:{}]",
        total_shares_value_increase
    );
    msg!(
        "[mint_with_alloyx_vault_depository:owned_shares_amount_increase:{}]",
        owned_shares_amount_increase
    );
    msg!(
        "[mint_with_alloyx_vault_depository:owned_shares_value_increase:{}]",
        owned_shares_value_increase
    );

    // The depository collateral account should always be empty
    require!(
        depository_collateral_amount_before == depository_collateral_amount_after,
        UxdError::CollateralDepositHasRemainingDust
    );

    // Validate that the locked value moved exactly to the correct place
    require!(
        user_collateral_amount_decrease == collateral_amount,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );
    require!(
        user_redeemable_amount_increase == redeemable_amount_after_fees,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );

    // Check that we received the correct amount of shares
    require!(
        total_shares_supply_increase == shares_amount,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );
    require!(
        owned_shares_amount_increase == shares_amount,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );

    // Check that the new state of the pool still makes sense
    require!(
        is_within_range_inclusive(
            total_shares_value_increase,
            collateral_amount_after_precision_loss,
            collateral_amount
        ),
        UxdError::CollateralDepositDoesntMatchTokenValue,
    );
    require!(
        is_within_range_inclusive(
            owned_shares_value_increase,
            collateral_amount_after_precision_loss,
            collateral_amount
        ),
        UxdError::CollateralDepositDoesntMatchTokenValue,
    );

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- Emit resulting event, and update onchain accounting
    // ---------------------------------------------------------------------

    // Compute how much fees was paid
    let minting_fee_paid: u64 =
        compute_decrease(redeemable_amount_before_fees, redeemable_amount_after_fees)?;

    // Emit event
    emit!(MintWithAlloyxVaultDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: ctx.accounts.depository.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        user: ctx.accounts.user.key(),
        collateral_amount,
        redeemable_amount: redeemable_amount_after_fees,
        minting_fee_paid,
    });

    // Accouting for depository
    let mut depository = ctx.accounts.depository.load_mut()?;
    depository.minting_fee_accrued(minting_fee_paid)?;
    depository.collateral_deposited_and_redeemable_minted(
        collateral_amount,
        redeemable_amount_after_fees,
    )?;
    require!(
        depository.redeemable_amount_under_management
            <= depository.redeemable_amount_under_management_cap,
        UxdError::RedeemableUnderManagementCapReached
    );

    // Accouting for controller
    let redeemable_amount_change: i128 = redeemable_amount_after_fees.into();
    let mut controller = ctx.accounts.controller.load_mut()?;
    controller.update_onchain_accounting_following_mint_or_redeem(redeemable_amount_change)?;
    require!(
        controller.redeemable_circulating_supply <= controller.redeemable_global_supply_cap,
        UxdError::RedeemableGlobalSupplyCapReached
    );

    // Done
    Ok(())
}

// Into functions
impl<'info> MintWithAlloyxVaultDepository<'info> {
    pub fn into_deposit_collateral_to_alloyx_vault_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, alloyx_cpi::cpi::accounts::Deposit<'info>> {
        let cpi_accounts = alloyx_cpi::cpi::accounts::Deposit {
            signer: self.depository.to_account_info(),
            investor_pass: self.alloyx_vault_pass.to_account_info(),
            vault_info_account: self.alloyx_vault_info.to_account_info(),
            usdc_vault_account: self.alloyx_vault_collateral.to_account_info(),
            usdc_mint: self.collateral_mint.to_account_info(),
            alloyx_vault_account: self.alloyx_vault_shares.to_account_info(),
            alloyx_mint: self.alloyx_vault_mint.to_account_info(),
            user_usdc_account: self.depository_collateral.to_account_info(),
            user_alloyx_account: self.depository_shares.to_account_info(),
            token_program: self.token_program.to_account_info(),
            associated_token_program: self.associated_token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        let cpi_program = self.alloyx_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_user_collateral_to_depository_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_collateral.to_account_info(),
            to: self.depository_collateral.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
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
}

// Validate
impl<'info> MintWithAlloyxVaultDepository<'info> {
    pub(crate) fn validate(&self, collateral_amount: u64) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        validate_collateral_amount(&self.user_collateral, collateral_amount)?;
        require!(
            !&self.depository.load()?.minting_disabled,
            UxdError::MintingDisabled
        );
        Ok(())
    }
}
