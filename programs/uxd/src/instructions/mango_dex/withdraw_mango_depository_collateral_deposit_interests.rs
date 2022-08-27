use crate::error::UxdError;
use crate::events::WithdrawMangoDepositoryCollateralDepositInterestsEvent;
use crate::Controller;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use anchor_comp::mango_markets_v3;
use anchor_comp::mango_markets_v3::MangoMarketV3;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;
use mango::state::MangoAccount;
use mango::state::MangoCache;
use mango::state::MangoGroup;

/// Takes 13 accounts - 5 used locally - 7 for MangoMarkets CPI - 1 Programs
#[derive(Accounts)]
pub struct WithdrawMangoDepositoryCollateralDepositInterests<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
        constraint = controller.load()?.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository
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

    /// #4 The `authority`'s ATA for the `depository`'s `collateral_mint`
    /// Will be credited during this instruction
    #[account(
        mut,
        constraint = authority_collateral.mint == depository.load()?.collateral_mint @UxdError::InvalidCollateralMint,
        constraint = &authority_collateral.owner == authority.key @UxdError::InvalidOwner,
    )]
    pub authority_collateral: Account<'info, TokenAccount>,

    /// #5 The MangoMarkets Account (MangoAccount) managed by the `depository`
    /// CHECK : Seeds checked. Depository registered
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

    /// #8 [MangoMarkets CPI] Signer PDA
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_signer: UncheckedAccount<'info>,

    /// #9 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_perp_market: UncheckedAccount<'info>,

    /// #10 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub collateral_root_bank: UncheckedAccount<'info>,

    /// #11 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub collateral_node_bank: UncheckedAccount<'info>,

    /// #12 [MangoMarkets CPI] Vault for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub collateral_vault: UncheckedAccount<'info>,

    /// #13 MangoMarketv3 Program
    pub mango_program: Program<'info, MangoMarketV3>,
}

pub(crate) fn handler(
    ctx: Context<WithdrawMangoDepositoryCollateralDepositInterests>,
) -> Result<()> {
    let depository = ctx.accounts.depository.load()?;
    let collateral_mint = depository.collateral_mint;
    let depository_bump = depository.bump;
    drop(depository);

    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[depository_bump],
    ]];

    // - 0 [DETERMINES THE AMOUNT OF COLLATERAL TO WITHDRAW] -------------------
    let collateral_deposited = ctx.accounts.get_collateral_deposited()?;
    let collateral_balance = ctx.accounts.get_collateral_balance()?;

    let collateral_interests: u64 = collateral_balance
        .checked_sub(collateral_deposited)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_to_num()
        .ok_or_else(|| error!(UxdError::MathError))?;

    require!(
        collateral_interests > 0,
        UxdError::NoCollateralDepositInterests
    );

    // - 1 [WITHDRAW COLLATERAL INTEREST FROM MANGO THEN RETURN TO AUTHORITY] -----------

    // - mango withdraw interest_amount
    mango_markets_v3::withdraw(
        ctx.accounts
            .to_withdraw_collateral_interest_from_mango_context()
            .with_signer(depository_signer_seed),
        collateral_interests,
        false,
    )?;

    emit!(WithdrawMangoDepositoryCollateralDepositInterestsEvent {
        version: ctx.accounts.controller.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.depository.load()?.quote_mint,
        collateral_mint_decimals: ctx.accounts.depository.load()?.quote_mint_decimals,
        withdrawn_amount: collateral_interests,
    });

    Ok(())
}

impl<'info> WithdrawMangoDepositoryCollateralDepositInterests<'info> {
    fn to_withdraw_collateral_interest_from_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Withdraw<'info>> {
        let cpi_accounts = mango_markets_v3::Withdraw {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.collateral_root_bank.to_account_info(),
            node_bank: self.collateral_node_bank.to_account_info(),
            vault: self.collateral_vault.to_account_info(),
            token_account: self.authority_collateral.to_account_info(),
            signer: self.mango_signer.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> WithdrawMangoDepositoryCollateralDepositInterests<'info> {
    // - [Return the collateral mint total deposits from depository mango account]
    fn get_collateral_balance(&self) -> Result<I80F48> {
        // - [Loads Mango's account, cache, group]
        let mango_group = MangoGroup::load_checked(&self.mango_group, self.mango_program.key)
            .map_err(ProgramError::from)?;
        let mango_cache =
            MangoCache::load_checked(&self.mango_cache, self.mango_program.key, &mango_group)
                .map_err(ProgramError::from)?;
        let mango_account = MangoAccount::load_checked(
            &self.mango_account,
            self.mango_program.key,
            self.mango_group.key,
        )
        .map_err(ProgramError::from)?;

        // - [Get the collateral mint native deposits via matching the token index]
        let token_index = match mango_group.find_token_index(&self.authority_collateral.key()) {
            None => return Err(error!(UxdError::RootBankIndexNotFound)),
            Some(i) => i,
        };
        let root_bank_cache = mango_cache.root_bank_cache[token_index];
        let native_deposit = mango_account
            .get_native_deposit(&root_bank_cache, token_index)
            .map_err(ProgramError::from)?;

        Ok(native_deposit)
    }
}

impl<'info> WithdrawMangoDepositoryCollateralDepositInterests<'info> {
    // - [Return the total collateral deposited from depository accounting]
    fn get_collateral_deposited(&self) -> Result<I80F48> {
        let depository = self.depository.load()?;
        let collateral_deposited = depository.collateral_amount_deposited;
        drop(depository);
        return I80F48::checked_from_num(collateral_deposited)
            .ok_or_else(|| error!(UxdError::MathError));
    }
}
