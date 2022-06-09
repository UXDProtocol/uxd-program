use crate::error::UxdError;
use crate::state::msol_config::MSolConfig;
use crate::Controller;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::MSOL_CONFIG_NAMESPACE;
use anchor_comp::mango_markets_v3;
use anchor_comp::mango_markets_v3::MangoMarketV3;
use anchor_comp::marinade;
use anchor_comp::spl_token::SyncNative;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_lang::system_program::Transfer;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

use fixed::types::I80F48;
use marinade_onchain_helper::cpi_context_accounts::MarinadeLiquidUnstake;
use marinade_onchain_helper::{cpi_context_accounts::MarinadeDeposit};

use super::MsolInfo;

#[derive(Accounts)]
pub struct RebalanceMangoDepositoryMsolRatio<'info> {
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

    /// #9 [MangoMarkets CPI] Signer PDA
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

    /// #13 [MangoMarkets CPI] Root Bank for the `depository`'s `msol`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_msol_root_bank: UncheckedAccount<'info>,

    /// #14 [MangoMarkets CPI] Node Bank for the `depository`'s `msol`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_msol_node_bank: UncheckedAccount<'info>,

    /// #15 [MangoMarkets CPI] Vault for the `depository`'s `msol`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_msol_vault: UncheckedAccount<'info>,

    /// #16 MangoMarketv3 Program
    pub mango_program: Program<'info, MangoMarketV3>,

    /// #17
    /// CHECK: Marinade CPI
    #[account(mut)]
    pub marinade_state: UncheckedAccount<'info>,

    /// #18
    /// CHECK: Marinade CPI
    #[account(mut)]
    pub msol_mint: Box<Account<'info, Mint>>,

    /// #19
    /// CHECK: Marinade CPI
    pub msol_mint_authority: UncheckedAccount<'info>,

    /// #20
    /// CHECK: Marinade CPI
    #[account(mut)]
    pub liq_pool_sol_leg_pda: UncheckedAccount<'info>,

    /// #21
    /// CHECK: Marinade CPI
    #[account(mut)]
    pub liq_pool_msol_leg: UncheckedAccount<'info>,

    /// #22
    /// CHECK: Marinade CPI
    pub liq_pool_msol_leg_authority: UncheckedAccount<'info>,

    /// #23
    /// CHECK: Marinade CPI
    #[account(mut)]
    pub treasury_msol_account: UncheckedAccount<'info>,

    /// #24
    /// CHECK: Marinade CPI
    #[account(mut)]
    pub reserve_pda: UncheckedAccount<'info>,

    /// #25 sol passthrough ata interact with marinade/mango cpi,
    /// either accept sol from mango, then pass it to marinade for swapping, or
    /// accept sol from swapped from marinade, then deposit to mango
    #[account(
        mut,
        constraint = sol_passthrough_ata.mint == spl_token::native_mint::id() @UxdError::InvalidNonNativeMintAtaUsed,
        constraint = &sol_passthrough_ata.owner == user.key @UxdError::InvalidOwner,
    )]
    pub sol_passthrough_ata: Box<Account<'info, TokenAccount>>,

    /// #26 msol passthrough ata interact with marinade/mango cpi
    /// either accept msol from mango, then pass it to marinade for swapping, or
    /// accept msol from swapped from marinade, then deposit to mango
    #[account(
        mut,
        constraint = msol_passthrough_ata.mint == msol_mint.key() @UxdError::InvalidNonMSolMintAtaUsed,
        constraint = &msol_passthrough_ata.owner == user.key @UxdError::InvalidOwner,
    )]
    pub msol_passthrough_ata: Box<Account<'info, TokenAccount>>,

    /// #27  Program
    /// CHECK: Marinade CPI
    #[account(address = marinade_finance::ID)]
    pub marinade_finance_program: AccountInfo<'info>,

    /// #28 System Program
    pub system_program: Program<'info, System>,

    /// #29 Token Program
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<RebalanceMangoDepositoryMsolRatio>) -> Result<()> {
    // 1. load marinade state
    let marinade_state: Account<marinade_finance::state::State> =
        Account::try_from(&ctx.accounts.marinade_state)?;

    // 2. create msol info instance, providing info for liquidity_ratio and sol/msol amounts
    let msol_info = MsolInfo::new(
        &ctx.accounts.mango_group,
        &ctx.accounts.mango_cache,
        &ctx.accounts.mango_account,
        ctx.accounts.mango_group.key,
        ctx.accounts.mango_program.key,
        &marinade_state,
        &ctx.accounts.msol_mint.key(),
    )?;
    msg!("msol_info {:?}", msol_info);

    // 3. load msol config state and get target
    let msol_config = ctx.accounts.msol_config.load()?;

    // 4. difference of the current liquidity ratio to the target
    let diff_to_target_liquidity = msol_config
        .diff_to_target_liquidity(msol_info.liquidity_ratio()?)
        .map_err(ProgramError::from)?;
    msg!("diff_to_target_liquidity {:?}", diff_to_target_liquidity);

    // 5. depository seed
    let depository = ctx.accounts.depository.load()?;
    let collateral_mint = depository.collateral_mint;
    let depository_bump = depository.bump;
    drop(depository);

    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[depository_bump],
    ]];

    match RebalanceRoute::from_diff_to_target_liquidity(diff_to_target_liquidity) {
        RebalanceRoute::Deposit => {
            rebalance_by_deposit(
                &ctx,
                diff_to_target_liquidity,
                &msol_info,
                &marinade_state,
                depository_signer_seed,
            )?;
        }
        RebalanceRoute::LiquidUnstake => {
            rebalance_by_liquid_unstake(
                &ctx,
                diff_to_target_liquidity,
                &msol_info,
                &marinade_state,
                depository_signer_seed,
            )?;
        }
        RebalanceRoute::NoSwapRequired => {
            // no action
        }
    }

    Ok(())
}

impl<'info> RebalanceMangoDepositoryMsolRatio<'info> {
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

    pub fn into_deposit_to_mango_msol_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Deposit<'info>> {
        let cpi_accounts = mango_markets_v3::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.user.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_msol_root_bank.to_account_info(),
            node_bank: self.mango_msol_node_bank.to_account_info(),
            vault: self.mango_msol_vault.to_account_info(),
            owner_token_account: self.msol_passthrough_ata.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_deposit_to_mango_sol_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Deposit<'info>> {
        let cpi_accounts = mango_markets_v3::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.user.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_sol_root_bank.to_account_info(),
            node_bank: self.mango_sol_node_bank.to_account_info(),
            vault: self.mango_sol_vault.to_account_info(),
            owner_token_account: self.sol_passthrough_ata.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_withdraw_from_mango_msol_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Withdraw<'info>> {
        let cpi_accounts = mango_markets_v3::Withdraw {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_msol_root_bank.to_account_info(),
            node_bank: self.mango_msol_node_bank.to_account_info(),
            vault: self.mango_msol_vault.to_account_info(),
            token_account: self.msol_passthrough_ata.to_account_info(),
            signer: self.mango_signer.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_withdraw_from_mango_sol_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Withdraw<'info>> {
        let cpi_accounts = mango_markets_v3::Withdraw {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_sol_root_bank.to_account_info(),
            node_bank: self.mango_sol_node_bank.to_account_info(),
            vault: self.mango_sol_vault.to_account_info(),
            token_account: self.sol_passthrough_ata.to_account_info(),
            signer: self.mango_signer.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_unwrap_wsol_by_closing_ata_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::CloseAccount<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::CloseAccount {
            account: self.sol_passthrough_ata.to_account_info(),
            destination: self.user.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_wrap_sol_to_ata_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.sol_passthrough_ata.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_sync_native_wsol_ata(&self) -> CpiContext<'_, '_, '_, 'info, SyncNative<'info>> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = SyncNative {
            account: self.sol_passthrough_ata.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> RebalanceMangoDepositoryMsolRatio<'info> {
    pub fn validate(&mut self) -> Result<()> {
        let msol_config = self.msol_config.load()?;
        require!(msol_config.enabled, UxdError::MSolSwappingDisabled);
        Ok(())
    }
}

enum RebalanceRoute {
    Deposit,
    LiquidUnstake,
    // depends on the current difference to the target ratio,
    // shd do delay unstake most of the time since liquid unstake takes extra fee
    NoSwapRequired,
}

impl RebalanceRoute {
    pub fn from_diff_to_target_liquidity(diff_to_target_liquidity: I80F48) -> Self {
        return if diff_to_target_liquidity.is_positive() {
            msg!("when > target liquidity ratio");
            RebalanceRoute::Deposit
        } else if diff_to_target_liquidity.is_negative() {
            msg!("when < target liquidity ratio");
            RebalanceRoute::LiquidUnstake
        } else {
            msg!("when = target liquidity ratio");
            RebalanceRoute::NoSwapRequired
        };
    }
}

fn rebalance_by_deposit(
    ctx: &Context<RebalanceMangoDepositoryMsolRatio>,
    diff_to_target_liquidity: I80F48,
    msol_info: &MsolInfo,
    marinade_state: &Account<marinade_finance::state::State>,
    depository_signer_seed: &[&[&[u8]]],
) -> Result<()> {
    // 6. lamports need to withdraw from mango
    let deposit_lamports: u64 = diff_to_target_liquidity
        .checked_mul(msol_info.total_depository_amount_lamports()?)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_to_num()
        .ok_or_else(|| error!(UxdError::MathError))?;

    // marinade deposit gives program error when lamports input is 0
    if deposit_lamports == 0 {
        msg!("deposit lamports is zero, no swapping is required");
        return Ok(());
    }

    // 7. withdraw sol from depository account to passthrough ata
    mango_markets_v3::withdraw(
        ctx.accounts
            .into_withdraw_from_mango_sol_context()
            .with_signer(depository_signer_seed),
        deposit_lamports,
        false,
    )?;

    // 8. unwrap wsol
    token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;

    // 9. convert sol from wsol passthrough to msol and transfer to msol passthrough
    marinade::deposit(
        ctx.accounts.into_marinade_deposit_cpi_ctx(),
        deposit_lamports
    )?;

    // 10. msol amount converted
    let msol_deposit_amount = marinade_state
        .calc_msol_from_lamports(deposit_lamports)
        .map_err(ProgramError::from)?;

    // 11. deposit msol back to mango from msol passthrough
    mango_markets_v3::deposit(
        ctx.accounts
            .into_deposit_to_mango_msol_context()
            .with_signer(depository_signer_seed),
        msol_deposit_amount,
    )?;

    Ok(())
}

fn rebalance_by_liquid_unstake(
    ctx: &Context<RebalanceMangoDepositoryMsolRatio>,
    diff_to_target_liquidity: I80F48,
    msol_info: &MsolInfo,
    marinade_state: &Account<marinade_finance::state::State>,
    depository_signer_seed: &[&[&[u8]]],
) -> Result<()> {
    // 6. msol equivalent lamports need to withdraw from mango
    let liquid_unstake_lamports: u64 = diff_to_target_liquidity
        .abs()
        .checked_mul(msol_info.total_depository_amount_lamports()?)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_to_num()
        .ok_or_else(|| error!(UxdError::MathError))?;

    // 7. msol amount of (6)
    let msol_liquid_unstake_amount = marinade_state
        .calc_msol_from_lamports(liquid_unstake_lamports)
        .map_err(ProgramError::from)?;

    if msol_liquid_unstake_amount == 0 {
        msg!("msol liquid unstake amount is zero, no swapping is required");
        return Ok(());
    }

    // 8. withdraw msol from depository account to passthrough ata
    mango_markets_v3::withdraw(
        ctx.accounts
            .into_withdraw_from_mango_msol_context()
            .with_signer(depository_signer_seed),
        msol_liquid_unstake_amount,
        false,
    )?;

    // 9. convert msol from msol passthrough to sol and transfer to the user
    marinade::liquid_unstake(
        ctx.accounts.into_liquid_unstake_cpi_ctx(),
        msol_liquid_unstake_amount
    )?;

    // 10. wrap sol
    system_program::transfer(
        ctx.accounts.into_wrap_sol_to_ata_context(),
        liquid_unstake_lamports,
    )?;

    // essential call to make after wrapping
    // shd be replaced by anchor sync native wrapper once it's released
    // https://github.com/project-serum/anchor/pull/1833
    anchor_comp::spl_token::sync_native(
        ctx.accounts.into_sync_native_wsol_ata()
    )?;

    // 7. deposit wsol back to mango from wsol passthrough
    mango_markets_v3::deposit(
        ctx.accounts
            .into_deposit_to_mango_sol_context()
            .with_signer(depository_signer_seed),
        liquid_unstake_lamports,
    )?;

    Ok(())
}