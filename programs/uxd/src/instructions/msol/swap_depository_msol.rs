use crate::error::UxdError;
use crate::mango_utils::get_native_deposit;
use crate::state::msol_config::MSolConfig;
use crate::state::msol_config::LIQUIDITY_RATIO_BASIS;
use crate::Controller;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::MSOL_CONFIG_NAMESPACE;
use anchor_comp::mango_markets_v3;
use anchor_comp::mango_markets_v3::MangoMarketV3;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;
use mango::state::MangoAccount;
use mango::state::MangoCache;
use mango::state::MangoGroup;
use marinade_onchain_helper::cpi_context_accounts::MarinadeLiquidUnstake;
use marinade_onchain_helper::{cpi_context_accounts::MarinadeDeposit, cpi_util};

#[derive(Accounts)]
pub struct SwapDepositoryMsol<'info> {
    /// #1 Public call accessible to any user
    #[account(mut)]
    pub user: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    // #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
    )]
    pub controller: AccountLoader<'info, Controller>,

    // #4 UXDProgram on chain account bound to a Controller instance
    // The `MangoDepository` manages a MangoAccount for a single Collateral
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = mango_account @UxdError::InvalidMangoAccount,
    )]
    pub depository: AccountLoader<'info, MangoDepository>,

    // #5 Msol config account for the `depository` instance
    #[account(
        seeds = [MSOL_CONFIG_NAMESPACE, depository.key().as_ref()],
        bump = msol_config.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = depository @UxdError::InvalidDepository,
    )]
    pub msol_config: AccountLoader<'info, MSolConfig>,

    // #6 The MangoMarkets Account (MangoAccount) managed by the `depository`
    /// CHECK : Seeds checked. Depository registered
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.mango_account_bump,
    )]
    pub mango_account: AccountInfo<'info>,

    /// #7 [MangoMarkets CPI] Index grouping perp and spot markets
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_group: UncheckedAccount<'info>,

    /// #8 [MangoMarkets CPI] Cache
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_cache: UncheckedAccount<'info>,

    /// #8 [MangoMarkets CPI] Signer PDA
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_signer: UncheckedAccount<'info>,

    /// #10 [MangoMarkets CPI] Root Bank for the `depository`'s `sol`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_sol_root_bank: UncheckedAccount<'info>,

    /// #11 [MangoMarkets CPI] Node Bank for the `depository`'s `sol`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_sol_node_bank: UncheckedAccount<'info>,

    /// #12 [MangoMarkets CPI] Vault for the `depository`'s `sol`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_sol_vault: UncheckedAccount<'info>,

    /// #10 [MangoMarkets CPI] Root Bank for the `depository`'s `msol`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_msol_root_bank: UncheckedAccount<'info>,

    /// #11 [MangoMarkets CPI] Node Bank for the `depository`'s `msol`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_msol_node_bank: UncheckedAccount<'info>,

    /// #12 [MangoMarkets CPI] Vault for the `depository`'s `msol`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_msol_vault: UncheckedAccount<'info>,

    /// #13 MangoMarketv3 Program
    pub mango_program: Program<'info, MangoMarketV3>,

    /// #14
    /// CHECK: Marinade CPI
    #[account(mut)]
    pub marinade_state: UncheckedAccount<'info>,

    /// #15
    /// CHECK: Marinade CPI
    #[account(mut)]
    pub msol_mint: Box<Account<'info, Mint>>,

    /// #16
    /// CHECK: Marinade CPI
    pub msol_mint_authority: UncheckedAccount<'info>,

    /// #17
    /// CHECK: Marinade CPI
    #[account(mut)]
    pub liq_pool_sol_leg_pda: UncheckedAccount<'info>,

    /// #18
    /// CHECK: Marinade CPI
    #[account(mut)]
    pub liq_pool_msol_leg: UncheckedAccount<'info>,

    /// #19
    /// CHECK: Marinade CPI
    pub liq_pool_msol_leg_authority: UncheckedAccount<'info>,

    /// #20
    /// CHECK: Marinade CPI
    #[account(mut)]
    pub treasury_msol_account: UncheckedAccount<'info>,

    /// #21
    /// CHECK: Marinade CPI
    #[account(mut)]
    pub reserve_pda: UncheckedAccount<'info>,

    /// #22 sol passthrough ata interact with marinade/mango cpi,
    /// either accept sol from mango, then pass it to marinade for swapping, or
    /// accept sol from swapped from marinade, then deposit to mango
    #[account(
        mut,
        constraint = sol_passthrough_ata.mint == spl_token::native_mint::id() @UxdError::InvalidNonNativeMintAtaUsed,
        constraint = &sol_passthrough_ata.owner == user.key @UxdError::InvalidOwner,
    )]
    pub sol_passthrough_ata: Box<Account<'info, TokenAccount>>,

    /// #23 msol passthrough ata interact with marinade/mango cpi
    /// either accept msol from mango, then pass it to marinade for swapping, or
    /// accept msol from swapped from marinade, then deposit to mango
    #[account(
        mut,
        constraint = msol_passthrough_ata.mint == msol_mint.key() @UxdError::InvalidNonMSolMintAtaUsed,
        constraint = &msol_passthrough_ata.owner == user.key @UxdError::InvalidOwner,
    )]
    pub msol_passthrough_ata: Box<Account<'info, TokenAccount>>,

    /// #24  Program
    /// CHECK: Marinade CPI
    #[account(address = marinade_finance::ID)]
    pub marinade_finance_program: AccountInfo<'info>,

    /// #25 System Program
    pub system_program: Program<'info, System>,

    /// #26 Token Program
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<SwapDepositoryMsol>) -> Result<()> {
    // 1. get depository's sol and msol amount
    msg!("msol_swap: 1 - get depository's sol and msol amount");
    let mango_group =
        MangoGroup::load_checked(&ctx.accounts.mango_group, ctx.accounts.mango_program.key)
            .map_err(|me| ProgramError::from(me))?;

    let mango_cache = MangoCache::load_checked(
        &ctx.accounts.mango_cache,
        ctx.accounts.mango_program.key,
        &mango_group,
    )
    .map_err(|me| ProgramError::from(me))?;

    let mango_account = MangoAccount::load_checked(
        &ctx.accounts.mango_account,
        ctx.accounts.mango_program.key,
        ctx.accounts.mango_group.key,
    )
    .map_err(|me| ProgramError::from(me))?;

    let depository_sol_amount: u64 = get_native_deposit(
        &spl_token::native_mint::id(),
        &mango_group,
        &mango_cache,
        &mango_account,
    )
    .map_err(|me| ProgramError::from(me))?
    .checked_to_num()
    .ok_or_else(|| error!(UxdError::MathError))?;

    let depository_msol_amount: u64 = get_native_deposit(
        &ctx.accounts.msol_mint.key(),
        &mango_group,
        &mango_cache,
        &mango_account,
    )
    .map_err(|me| ProgramError::from(me))?
    .checked_to_num()
    .ok_or_else(|| error!(UxdError::MathError))?;

    drop(mango_group);
    drop(mango_account);
    drop(mango_cache);

    // 2. liquidity ratio
    // liquidity_ratio[t] = liquid_SOL[t]/(liquid_SOL[t] + marinade_SOL[t]*MSOL_underlying_SOL[t])
    msg!("msol_swap: 2 - liquidity ratio");
    let marinade_state: Account<marinade_finance::state::State> =
        Account::try_from(&ctx.accounts.marinade_state)?;

    let depository_sol_amount_lamports = I80F48::checked_from_num(depository_sol_amount)
        .ok_or_else(|| error!(UxdError::MathError))?;

    let depository_msol_amount_lamports = I80F48::checked_from_num(
        // msol_amount * msol_price_in_sol
        marinade_state
            .calc_lamports_from_msol_amount(depository_msol_amount)
            .map_err(|me| ProgramError::from(me))?,
    )
    .ok_or_else(|| error!(UxdError::MathError))?;

    let total_depository_amount_lamports = depository_sol_amount_lamports
        .checked_add(depository_msol_amount_lamports)
        .ok_or_else(|| error!(UxdError::MathError))?;

    let liquidity_ratio = depository_sol_amount_lamports
        .checked_div(total_depository_amount_lamports)
        .ok_or_else(|| error!(UxdError::MathError))?;

    // 3. target_liquidity_ratio
    msg!("msol_swap: 3 - target_liquidity_ratio");

    let msol_config = ctx.accounts.msol_config.load()?;

    let target_liquidity_ratio = I80F48::checked_from_num(msol_config.target_liquidity_ratio)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_div(
            I80F48::checked_from_num(LIQUIDITY_RATIO_BASIS)
                .ok_or_else(|| error!(UxdError::MathError))?,
        )
        .ok_or_else(|| error!(UxdError::MathError))?;

    // 4. withdraw/deposit sol from depository account to sol/msol passthrough ata
    let diff_to_target_liquidity = liquidity_ratio
        .checked_sub(target_liquidity_ratio)
        .ok_or_else(|| error!(UxdError::MathError))?;

    if diff_to_target_liquidity.is_zero() {
        msg!("msol_swap: 4 - when equals target liquidity ratio");
        return Ok(());
    }

    let depository = ctx.accounts.depository.load()?;
    let collateral_mint = depository.collateral_mint;
    let depository_bump = depository.bump;
    drop(depository);

    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[depository_bump],
    ]];

    if diff_to_target_liquidity.is_positive() {
        msg!("msol_swap: 4 - when > target liquidity ratio");

        let deposit_lamports: u64 = diff_to_target_liquidity
            .checked_mul(total_depository_amount_lamports)
            .ok_or_else(|| error!(UxdError::MathError))?
            .checked_to_num()
            .ok_or_else(|| error!(UxdError::MathError))?;

        mango_markets_v3::withdraw(
            ctx.accounts
                .into_withdraw_from_mango_context(
                    &ctx.accounts.mango_sol_root_bank,
                    &ctx.accounts.mango_sol_node_bank,
                    &ctx.accounts.mango_sol_vault,
                    &ctx.accounts.sol_passthrough_ata,
                )
                .with_signer(depository_signer_seed),
            deposit_lamports,
            false,
        )?;
        msg!("msol_swap: withdraw sol from mango success");

        // 5. swap msol from sol passthrough ata to msol passthrough ata
        let cpi_ctx = ctx.accounts.into_marinade_deposit_cpi_ctx();
        let data = marinade_finance::instruction::Deposit {
            lamports: deposit_lamports,
        };
        cpi_util::invoke_signed(cpi_ctx, data)?;
        msg!("msol_swap: swap success");

        let msol_deposit_amount = marinade_state
            .calc_msol_from_lamports(deposit_lamports)
            .map_err(|me| ProgramError::from(me))?;

        // 6. deposit msol back to mango from msol passthrough ata
        mango_markets_v3::deposit(
            ctx.accounts
                .into_deposit_to_mango_context(
                    &ctx.accounts.mango_msol_root_bank,
                    &ctx.accounts.mango_msol_node_bank,
                    &ctx.accounts.mango_msol_vault,
                    &ctx.accounts.msol_passthrough_ata,
                )
                .with_signer(depository_signer_seed),
            msol_deposit_amount,
        )?;
        msg!("msol_swap: deposit success");

        return Ok(());
    } else if diff_to_target_liquidity.is_negative() {
        msg!("msol_swap: 4 - when < target liquidity ratio");

        let liquid_unstake_lamports: u64 = diff_to_target_liquidity
            .abs()
            .checked_mul(total_depository_amount_lamports)
            .ok_or_else(|| error!(UxdError::MathError))?
            .checked_to_num()
            .ok_or_else(|| error!(UxdError::MathError))?;

        let msol_liquid_unstake_amount = marinade_state
            .calc_msol_from_lamports(liquid_unstake_lamports)
            .map_err(|me| ProgramError::from(me))?;

        mango_markets_v3::withdraw(
            ctx.accounts
                .into_withdraw_from_mango_context(
                    &ctx.accounts.mango_msol_root_bank,
                    &ctx.accounts.mango_msol_node_bank,
                    &ctx.accounts.mango_msol_vault,
                    &ctx.accounts.msol_passthrough_ata,
                )
                .with_signer(depository_signer_seed),
            msol_liquid_unstake_amount,
            false,
        )?;
        msg!("msol_swap: withdraw msol from mango success");

        // 5. swap sol from msol passthrough ata to sol passthrough ata
        let cpi_ctx = ctx.accounts.into_liquid_unstake_cpi_ctx();
        let instruction_data = marinade_finance::instruction::LiquidUnstake {
            msol_amount: msol_liquid_unstake_amount,
        };
        cpi_util::invoke_signed(cpi_ctx, instruction_data)?;
        msg!("msol_swap: swap success");

        // 6. deposit sol back to mango from sol passthrough ata
        mango_markets_v3::deposit(
            ctx.accounts
                .into_deposit_to_mango_context(
                    &ctx.accounts.mango_sol_root_bank,
                    &ctx.accounts.mango_sol_node_bank,
                    &ctx.accounts.mango_sol_vault,
                    &ctx.accounts.sol_passthrough_ata,
                )
                .with_signer(depository_signer_seed),
            liquid_unstake_lamports,
        )?;
        msg!("msol_swap: deposit success");

        return Ok(());
    }
    Ok(())
}

impl<'info> SwapDepositoryMsol<'info> {
    pub fn into_marinade_deposit_cpi_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, MarinadeDeposit<'info>> {
        let cpi_accounts = MarinadeDeposit {
            state: self.marinade_state.to_account_info(),
            msol_mint: self.msol_mint.to_account_info(),
            liq_pool_sol_leg_pda: self.liq_pool_sol_leg_pda.to_account_info(),
            liq_pool_msol_leg: self.liq_pool_msol_leg.to_account_info(),
            liq_pool_msol_leg_authority: self.liq_pool_msol_leg_authority.to_account_info(),
            reserve_pda: self.reserve_pda.to_account_info(),
            transfer_from: self.user.to_account_info(),
            mint_to: self.msol_passthrough_ata.to_account_info(),
            msol_mint_authority: self.msol_mint_authority.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.marinade_finance_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_liquid_unstake_cpi_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, MarinadeLiquidUnstake<'info>> {
        let cpi_accounts = MarinadeLiquidUnstake {
            state: self.marinade_state.to_account_info(),
            msol_mint: self.msol_mint.to_account_info(),
            liq_pool_sol_leg_pda: self.liq_pool_sol_leg_pda.to_account_info(),
            liq_pool_msol_leg: self.liq_pool_msol_leg.to_account_info(),
            treasury_msol_account: self.treasury_msol_account.to_account_info(),
            get_msol_from: self.msol_passthrough_ata.to_account_info(),
            get_msol_from_authority: self.user.to_account_info(),
            transfer_sol_to: self.user.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.marinade_finance_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_deposit_to_mango_context(
        &self,
        root_bank: &UncheckedAccount<'info>,
        node_bank: &UncheckedAccount<'info>,
        vault: &UncheckedAccount<'info>,
        passthrough_ata: &Account<'info, TokenAccount>,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Deposit<'info>> {
        let cpi_accounts = mango_markets_v3::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.user.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: root_bank.to_account_info(),
            node_bank: node_bank.to_account_info(),
            vault: vault.to_account_info(),
            owner_token_account: passthrough_ata.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_withdraw_from_mango_context(
        &self,
        root_bank: &UncheckedAccount<'info>,
        node_bank: &UncheckedAccount<'info>,
        vault: &UncheckedAccount<'info>,
        passthrough_ata: &Account<'info, TokenAccount>,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Withdraw<'info>> {
        let cpi_accounts = mango_markets_v3::Withdraw {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: root_bank.to_account_info(),
            node_bank: node_bank.to_account_info(),
            vault: vault.to_account_info(),
            token_account: passthrough_ata.to_account_info(),
            signer: self.mango_signer.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> SwapDepositoryMsol<'info> {
    pub fn validate(&mut self) -> Result<()> {
        let msol_config = self.msol_config.load()?;
        require!(msol_config.enabled, UxdError::MSolSwappingDisabled);
        Ok(())
    }
}
