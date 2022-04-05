use anchor_lang::prelude::*;
use crate::ZoDepository;
use crate::Controller;
use crate::error::UxdError;
use crate::CONTROLLER_NAMESPACE;
use crate::ZO_DEPOSITORY_NAMESPACE;
use crate::ZO_MARGIN_ACCOUNT_NAMESPACE;
use crate::events::InitializeZoDepositoryEvent;
use zo_abi as zo;

/// Takes 14 accounts - 5 used locally - 6 for CPI - 2 Programs - 1 Sysvar
#[derive(Accounts)]
pub struct InitializeZoDepository<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE], 
        bump = controller.bump,
        has_one = authority @UxdError::InvalidAuthority,
        constraint = controller.registered_zo_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #4 UXDProgram on chain account bound to a Controller instance (ZO)
    /// The `ZoDepository` manages a ZeroOne account for a single Collateral
    #[account(
        mut,
        seeds = [ZO_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
    )]
    pub depository: AccountLoader<'info, ZoDepository>,

    /// #5 The ZeroOne Account managed by the `depository`
    /// CHECK : Seeds checked. Initialized during ZO cpi (a.k.a. Margin)
    #[account(
        mut,
        seeds = [depository.key().as_ref(), zo_state.key().as_ref(), ZO_MARGIN_ACCOUNT_NAMESPACE],
        seeds::program = zo_program.key(),
        bump
    )]
    pub zo_account: AccountInfo<'info>,

    /// #6 [ZeroOne CPI] This account must be created for the Collateral 
    /// https://github.com/01protocol/zo-client/blob/ea38a2ba5cfdd53d62ce5e993f22113e83484f2f/src/accounts/margin/MarginWeb3.ts#L645
    /// CHECK: Done ZeroOne side
    #[account(mut)]
    pub zo_open_orders: AccountInfo<'info>,

    /// #7 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    pub zo_state: AccountInfo<'info>,

    /// #8 [ZeroOne CPI]
    /// CHECK: Done ZeroOne side
    #[account(mut)]
    pub zo_state_signer: AccountInfo<'info>,

    /// #9 [ZeroOne CPI]
    /// CHECK: Done ZeroOneSide - Tied to the Depository during this call
    #[account(mut)]
    pub zo_dex_market: AccountInfo<'info>,

    /// #10 [ZeroOne CPI] Control
    /// CHECK: ZeroOne CPI - checked ZeroOne side
    #[account(mut)]
    pub zo_control: AccountInfo<'info>,

    /// #11 [ZeroOne CPI] Zo Dex program
    /// CHECK: ZeroOne CPI
    pub zo_dex_program: AccountInfo<'info>,

    /// #12 System Program
    pub system_program: Program<'info, System>,

    /// #13 ZeroOne Program
    pub zo_program: Program<'info, zo::program::ZoAbi>,

    /// #14 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

// This should be refactors into the register IX later on
// Stack overflow for now (solana pre computing IX increase as of now 24/03/22)
pub fn handler(
    ctx: Context<InitializeZoDepository>
) -> Result<()>  {
    let depository = ctx.accounts.depository.load()?;
    let collateral_mint = depository.collateral_mint.clone();
    let depository_pda_signer: &[&[&[u8]]] = &[&[
        ZO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[depository.bump],
    ]];

    // - Initialize ZO Margin Account
    let zo_account_bump = *ctx
        .bumps
        .get("zo_account")
        .ok_or_else(|| error!(UxdError::BumpError))?;

    drop(depository);

    zo::cpi::create_margin(
        ctx.accounts
        .into_zo_create_margin_context()
        .with_signer(depository_pda_signer),
        zo_account_bump)?;

    // Create the perp open order for the collateral 
    zo::cpi::create_perp_open_orders(
        ctx.accounts
        .into_zo_create_perp_open_orders_context()
        .with_signer(depository_pda_signer))?;

    // - Update Depository state
    let depository = &mut ctx.accounts.depository.load_mut()?;

    depository.is_initialized = true;
    depository.zo_account = ctx.accounts.zo_account.key();
    depository.zo_account_bump = zo_account_bump;
    // This prevent injecting the wrong perp market later on
    depository.zo_dex_market = ctx.accounts.zo_dex_market.key();

    emit!(InitializeZoDepositoryEvent {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        zo_account: ctx.accounts.zo_account.key(),
    });

    Ok(())
}

impl<'info> InitializeZoDepository<'info> {

    pub fn into_zo_create_margin_context(&self) -> CpiContext<'_, '_, '_, 'info, zo::cpi::accounts::CreateMargin<'info>> {
        let cpi_program = self.zo_program.to_account_info();
        let cpi_accounts = zo::cpi::accounts::CreateMargin {
            authority: self.depository.to_account_info(),
            control: self.zo_control.to_account_info(),
            margin: self.zo_account.to_account_info(),
            state: self.zo_state.to_account_info(),
            payer: self.payer.to_account_info(),
            rent: self.rent.to_account_info(),
            system_program: self.system_program.to_account_info(),   
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
    pub fn into_zo_create_perp_open_orders_context(&self) -> CpiContext<'_, '_, '_, 'info, zo::cpi::accounts::CreatePerpOpenOrders<'info>> {
        let cpi_program = self.zo_program.to_account_info();
        let cpi_accounts = zo::cpi::accounts::CreatePerpOpenOrders {
            authority: self.depository.to_account_info(),
            control: self.zo_control.to_account_info(),
            margin: self.zo_account.to_account_info(),
            state: self.zo_state.to_account_info(),
            state_signer: self.zo_state_signer.to_account_info(),
            open_orders: self.zo_open_orders.to_account_info(),
            dex_market: self.zo_dex_market.to_account_info(),
            dex_program: self.zo_dex_program.to_account_info(),  
            payer: self.payer.to_account_info(),
            rent: self.rent.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Validate input arguments
impl<'info> InitializeZoDepository<'info> {
    pub fn validate(&self) -> Result<()> {
        let depository = self.depository.load()?;
        if depository.is_initialized {
            return Err(error!(UxdError::ZoDepositoryAlreadyInitialized));
        }
        Ok(())
    }
}
