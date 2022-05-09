use crate::error::UxdError;
use crate::events::DepositInsuranceToDepositoryEvent;
use crate::Controller;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::state::SafetyVault;
use anchor_comp::mango_markets_v3;
use anchor_comp::mango_markets_v3::MangoMarketV3;
use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

/// Takes x accounts
#[derive(Accounts)]
pub struct LiquidationKillSwitch<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
        constraint = controller.load()?.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #3 UXDProgram on chain account bound to a Controller instance
    /// The `MangoDepository` manages a MangoAccount for a single Collateral
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = mango_account @UxdError::InvalidMangoAccount,
    )]
    pub depository: AccountLoader<'info, MangoDepository>,

    #[account(
        mut,
        seeds = [SAFETY_VAULT_NAMESPACE, depository.key().as_ref()],
        bump = safety_vault.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority, 
        has_one = quote_vault,
    )]
    pub safety_vault: AccountLoader<'info, SafetyVault>,

    #[account(
        mut,
        seeds = [QUOTE_VAULT_NAMESPACE, safety_vault.key().as_ref()],
        bump = safety_vault.load()?.quote_vault_bump,
    )]
    pub quote_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.mango_account_bump,
    )]
    pub mango_account: AccountInfo<'info>,

    /// #6 [MangoMarkets CPI] Index grouping perp and spot markets
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_group: UncheckedAccount<'info>,

    /// #7 [MangoMarkets CPI] Cache
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_cache: UncheckedAccount<'info>,

    /// #8 [MangoMarkets CPI] Root Bank for the `depository`'s `quote_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_root_bank: UncheckedAccount<'info>,

    /// #9 [MangoMarkets CPI] Node Bank for the `depository`'s `quote_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_node_bank: UncheckedAccount<'info>,

    /// #10 [MangoMarkets CPI] Vault for the `depository`'s `quote_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_vault: UncheckedAccount<'info>,

    /// #11 Token Program
    pub token_program: Program<'info, Token>,

    /// #12 MangoMarketv3 Program
    pub mango_program: Program<'info, MangoMarketV3>,
}

pub fn handler(ctx: Context<LiquidationKillSwitch>, target_collateral: u128,) -> Result<()> {
    

    Ok(())
}
