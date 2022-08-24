use crate::error::UxdError;
use crate::Controller;
use crate::MercurialVaultDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use solana_program::instruction::{AccountMeta, Instruction};

#[derive(Accounts)]
pub struct MintWithMercurialVaultDepository<'info> {
    /// #1
    pub user: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.registered_mercurial_vault_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
        has_one = redeemable_mint @UxdError::InvalidRedeemableMint
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4
    #[account(
        mut,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
    )]
    pub depository: AccountLoader<'info, MercurialVaultDepository>,

    /// #5
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.load()?.redeemable_mint_bump,
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// #6
    #[account(
        mut,
        constraint = user_collateral.mint == depository.load()?.collateral_mint @UxdError::InvalidCollateralMint,
        constraint = &user_collateral.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #7
    #[account(
        mut,
        constraint = user_redeemable.mint == controller.load()?.redeemable_mint @UxdError::InvalidRedeemableMint,
        constraint = &user_redeemable.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #8
    #[account(mut)]
    pub mercurial_vault: Box<Account<'info, mercurial_vault::state::Vault>>,

    /// #9
    #[account(mut)]
    pub mercurial_vault_lp_mint: Box<Account<'info, Mint>>,

    /// #10
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #11
    #[account(
        mut,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref(), mercurial_vault_lp_mint.key().as_ref()],
        token::authority = depository,
        token::mint = mercurial_vault_lp_mint,
        bump = depository.load()?.lp_tokens_vault_bump,
    )]
    pub depository_lp_token_vault: Box<Account<'info, TokenAccount>>,

    /// #12
    #[account(
        mut,
        token::mint = collateral_mint,
    )]
    pub mercurial_vault_program_collateral_token_vault: Box<Account<'info, TokenAccount>>,

    /// #13
    pub system_program: Program<'info, System>,

    /// #14
    pub token_program: Program<'info, Token>,

    /// #15
    pub mercurial_vault_program: Program<'info, mercurial_vault::program::Vault>,
}

pub fn handler(
    ctx: Context<MintWithMercurialVaultDepository>,
    collateral_amount: u64,       // native units
    minimum_lp_token_amount: u64, // native units
) -> Result<()> {
    let controller_bump = ctx.accounts.controller.load()?.bump;
    let controller_pda_signer: &[&[&[u8]]] = &[&[CONTROLLER_NAMESPACE, &[controller_bump]]];

    let depository = ctx.accounts.depository.load()?;
    let collateral_mint = depository.collateral_mint;
    let depository_bump = depository.bump;
    drop(depository);

    let depository_pda_signer: &[&[&[u8]]] = &[&[
        MERCURIAL_VAULT_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[depository_bump],
    ]];

    // Deposit the collateral tokens on mercurial vault and mint uxd to user redeemable account
    let before_lp_token_vault_balance = ctx.accounts.depository_lp_token_vault.amount;

    // 1 - Deposit collateral to mercurial vault and get lp tokens
    MintWithMercurialVaultDepository::mercurial_vault_deposit(
        ctx.accounts
            .into_deposit_collateral_to_mercurial_vault_context()
            .with_signer(depository_pda_signer),
        collateral_amount,
        minimum_lp_token_amount,
    )?;

    let after_lp_token_vault_balance = ctx.accounts.depository_lp_token_vault.amount;

    // 2 - Calculate the redeemable amount to mint, based on the minted LP tokens amount and the price of the pool
    let lp_token_change = after_lp_token_vault_balance
        .checked_sub(before_lp_token_vault_balance)
        .ok_or_else(|| error!(UxdError::MathError))?;

    let redeemable_mint_amount = lp_token_change;

    // 3 - Mint redeemable
    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_pda_signer),
        redeemable_mint_amount,
    )?;

    // @TODO => Maybe remove? Imagine if we want to deposit funds with DAO, then we lose ours WSOL and kaboom
    // It means also that we can't mint or redeem with SOL with actual UXD Program using DAO
    // if ATA mint is WSOL => unwrap
    //if ctx.accounts.collateral_mint.key() == spl_token::native_mint::id() {
    //    token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;
    //}

    // 4 - Update accounting
    // @TODO

    // 6 - Check that we don't mint more UXD than the fixed limit
    // @TODO
    // ctx.accounts.check_redeemable_global_supply_cap_overflow()?;
    Ok(())
}

// Into functions
impl<'info> MintWithMercurialVaultDepository<'info> {
    pub fn into_deposit_collateral_to_mercurial_vault_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mercurial_vault::context::DepositWithdrawLiquidity<'info>>
    {
        let cpi_accounts = mercurial_vault::context::DepositWithdrawLiquidity {
            vault: self.mercurial_vault.clone(),
            lp_mint: *self.mercurial_vault_lp_mint.clone(),
            token_program: self.token_program.clone(),
            user_lp: *self.depository_lp_token_vault.clone(),
            token_vault: *self.mercurial_vault_program_collateral_token_vault.clone(),
            user: self.user.clone(),
            user_token: *self.user_collateral.clone(),
        };
        let cpi_program = self.mercurial_vault_program.to_account_info();
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

    pub fn into_unwrap_wsol_by_closing_ata_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::CloseAccount<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::CloseAccount {
            account: self.user_collateral.to_account_info(),
            destination: self.user.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Wrap mercurial vault instruction
impl<'info> MintWithMercurialVaultDepository<'info> {
    pub fn mercurial_vault_deposit(
        ctx: CpiContext<
            '_,
            '_,
            '_,
            'info,
            mercurial_vault::context::DepositWithdrawLiquidity<'info>,
        >,
        collateral_amount: u64,
        minimum_lp_token_amount: u64,
    ) -> Result<()> {
        let accounts = vec![
            AccountMeta::new(ctx.accounts.vault.key(), false),
            AccountMeta::new(ctx.accounts.token_vault.key(), false),
            AccountMeta::new(ctx.accounts.lp_mint.key(), false),
            AccountMeta::new(ctx.accounts.user_token.key(), false),
            AccountMeta::new(ctx.accounts.user_lp.key(), false),
            AccountMeta::new_readonly(ctx.accounts.user.key(), true),
            AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
        ];

        let instr = mercurial_vault::instruction::Deposit {
            token_amount: collateral_amount,
            minimum_lp_token_amount,
        };

        let mut data: Vec<u8> = Vec::new();

        // TODO
        // Find a way to not have to specify the anchor discriminator here
        data.push(0xf2);
        data.push(0x23);
        data.push(0xc6);
        data.push(0x89);
        data.push(0x52);
        data.push(0xe1);
        data.push(0xf2);
        data.push(0xb6);

        instr.serialize(&mut data)?;

        msg!("DATA {:?}", data);

        let ix = Instruction {
            program_id: mercurial_vault::ID,
            accounts,
            data,
        };

        solana_program::program::invoke_signed(
            &ix,
            &ToAccountInfos::to_account_infos(&ctx),
            ctx.signer_seeds,
        )
        .map_err(Into::into)
    }
}

// Validate
impl<'info> MintWithMercurialVaultDepository<'info> {
    pub fn validate(&self, collateral_amount: u64, minimum_lp_token_amount: u64) -> Result<()> {
        require!(collateral_amount != 0, UxdError::InvalidCollateralAmount);

        Ok(())
    }
}
