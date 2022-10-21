use crate::error::UxdError;
use crate::Controller;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use ::mango_v3_reimbursement::program::MangoV3Reimbursement;
use ::mango_v3_reimbursement::state::Group;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use mango_v3_reimbursement::state::ReimbursementAccount;

#[derive(Accounts)]
#[instruction(token_index: usize, index_into_table: usize)]
pub struct MangoReimburse<'info> {
    /// #1
    pub authority: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = mango_account @UxdError::InvalidMangoAccount,
    )]
    pub depository: AccountLoader<'info, MangoDepository>,

    /// #5 The mint of the token received from mango reimbursement program
    pub token_mint: Box<Account<'info, Mint>>,

    /// #6 Authority's token account which will receives the funds
    #[account(
        mut,
        constraint = authority_token_account.mint == token_mint.key(),
        constraint = &authority_token_account.owner == authority.key @UxdError::InvalidOwner,
    )]
    pub authority_token_account: Box<Account<'info, TokenAccount>>,

    /// #7 Depository's token account which will temporarily receives the funds from mango
    /// before transferring it to authority_token_account
    #[account(
        mut,
        constraint = authority_token_account.mint == token_mint.key(),
        constraint = &authority_token_account.owner == authority.key @UxdError::InvalidOwner,
    )]
    pub depository_token_account: Box<Account<'info, TokenAccount>>,

    /// #8
    /// CHECK : Seeds checked. Depository registered
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.mango_account_bump,
    )]
    pub mango_account: AccountInfo<'info>,

    /// #9
    #[account(mut)]
    pub mango_reimbursement_group: AccountLoader<'info, Group>,

    /// #10
    #[account(
        mut,
        address = mango_reimbursement_group.load()?.vaults[token_index]
    )]
    pub mango_reimbursement_vault: Account<'info, TokenAccount>,

    /// #11
    #[account(
        mut,
        seeds = [b"ReimbursementAccount".as_ref(), mango_reimbursement_group.key().as_ref(), depository.key().as_ref()],
        bump,
    )]
    pub mango_reimbursement_account: AccountLoader<'info, ReimbursementAccount>,

    /// #12
    #[account(
        mut,
        associated_token::mint = mango_reimbursement_claim_mint,
        associated_token::authority = mango_reimbursement_group.load()?.claim_transfer_destination,
    )]
    pub mango_reimbursement_claim_mint_token_account: Box<Account<'info, TokenAccount>>,

    /// #13
    #[account(
        mut,
        address = mango_reimbursement_group.load()?.claim_mints[token_index]
    )]
    pub mango_reimbursement_claim_mint: Box<Account<'info, Mint>>,

    /// #14
    /// CHECK: Checked by the mango reimbursement program
    pub mango_reimbursement_table: UncheckedAccount<'info>,

    /// #15
    pub system_program: Program<'info, System>,

    /// #16
    pub token_program: Program<'info, Token>,

    /// #17
    pub mango_reimbursement_program: Program<'info, MangoV3Reimbursement>,

    pub rent: Sysvar<'info, Rent>,
}

pub(crate) fn handler(
    ctx: Context<MangoReimburse>,
    token_index: usize,
    index_into_table: usize,
) -> Result<()> {
    let depository = ctx.accounts.depository.load()?;
    let collateral_mint = depository.collateral_mint;
    let depository_bump = depository.bump;
    drop(depository);

    let depository_signer_seeds: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[depository_bump],
    ]];

    // Means the mango reimbursement program mint 1 claim tokens for each 1 token received
    let transfer_claim: bool = true;

    // 1 - snapshot the balance of the ATAs
    let depository_token_account_balance_at_start = ctx.accounts.depository_token_account.amount;
    let authority_token_account_balance_at_start = ctx.accounts.authority_token_account.amount;

    // 2 - Receive tokens from mango reimbursement program in the depository's token ATA
    mango_v3_reimbursement::cpi::reimburse(
        ctx.accounts
            .to_mango_reimburse_context()
            .with_signer(depository_signer_seeds),
        token_index,
        index_into_table,
        transfer_claim,
    )?;

    // 3 - Calculate the amount of tokens that have been received
    ctx.accounts.depository_token_account.reload()?;

    let depository_token_account_balance_after_reimburse_call =
        ctx.accounts.depository_token_account.amount;

    let received_token_amount = depository_token_account_balance_after_reimburse_call
        .checked_sub(depository_token_account_balance_at_start)
        .ok_or_else(|| error!(UxdError::MathError))?;

    require!(
        received_token_amount > 0,
        UxdError::MangoReimburseSentBackZeroToken,
    );

    // 4 - Transfer the tokens from the depository's token ATA to the authority's token ATA
    token::transfer(
        ctx.accounts
            .to_transfer_tokens_from_depository_ata_to_authority_ata_context()
            .with_signer(depository_signer_seeds),
        received_token_amount,
    )?;

    // 4 - Check the token accounts balances
    ctx.accounts.depository_token_account.reload()?;
    ctx.accounts.authority_token_account.reload()?;

    let depository_token_account_balance_after_final_transfer =
        ctx.accounts.depository_token_account.amount;

    let authority_token_account_balance_after_final_transfer =
        ctx.accounts.authority_token_account.amount;

    // Depository's token account balance should be the same
    require!(
        depository_token_account_balance_after_final_transfer
            == depository_token_account_balance_at_start,
        UxdError::TokenAccountsBalanceUnexpectedDifferences,
    );

    // Authority's token account balance should be increased of received_token_amount
    let authority_token_account_change = authority_token_account_balance_after_final_transfer
        .checked_sub(authority_token_account_balance_at_start)
        .ok_or_else(|| error!(UxdError::MathError))?;

    require!(
        authority_token_account_change == received_token_amount,
        UxdError::TokenAccountsBalanceUnexpectedDifferences,
    );

    // 5 - done
    Ok(())
}

// Into methods
impl<'info> MangoReimburse<'info> {
    fn to_transfer_tokens_from_depository_ata_to_authority_ata_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = token::Transfer {
            from: self.depository_token_account.to_account_info(),
            to: self.authority_token_account.to_account_info(),
            authority: self.depository.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn to_mango_reimburse_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_v3_reimbursement::cpi::accounts::Reimburse<'info>>
    {
        let cpi_accounts = mango_v3_reimbursement::cpi::accounts::Reimburse {
            group: self.mango_reimbursement_group.to_account_info(),
            token_account: self.depository_token_account.to_account_info(),
            reimbursement_account: self.mango_reimbursement_account.to_account_info(),
            mango_account_owner: self.depository.to_account_info(),
            signer: self.depository.to_account_info(),
            claim_mint_token_account: self
                .mango_reimbursement_claim_mint_token_account
                .to_account_info(),
            claim_mint: self.mango_reimbursement_claim_mint.to_account_info(),
            table: self.mango_reimbursement_table.to_account_info(),
            vault: self.mango_reimbursement_vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.mango_reimbursement_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
