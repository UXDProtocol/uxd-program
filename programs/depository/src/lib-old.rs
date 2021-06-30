// This program is intended



// XXX OK COOL hana notes time
// lessee our functions are: init, deposit, withdraw, xfer, proxy xfer
// init is kinda inscrutable but its purpose is to set up whatever we need
// so think abt it last, it only makes sense knowing what everything else is
// other functions purposes are pretty obvi. assume proxy xfer is transferFrom
// i dont rly understand what "authority" is supposed to be
// i guess the eoa that everything is taking place on behalf of?

// ok first thing i did was refamiliarize w token program
// a mint is an address with an owner allowed to arbitrarily create tokens
// i guess solana people use "authority" to disambiguate from literal owner (the program)
// tokens can only be in accounts, the mint holds no tokens
// minting is simply assigning to some user. minting can be permanently disabled
// explicit burn instruction. approve/revoke to delegate authority for transfers
// accounts can be frozen by a freeze authority lol. otherwise very simple

// yea so in deposit. they have depository token account (dest?)
// user authority (eoa), user token account (assert owned by authority)
// token program (the spl program), user redeemable account (deposit vouchers?)
// and redeemable mint. so i guess the idea is just like
// ok yea he partly impled this one. xfers token from user to depository
// and then presumably gives users tokens representing the deposit

// ok first of all this is all really hacky and mistake-riddled
// even the little bits of code that exist are full of bugs...
// i think his intent is for each coin type to have its own depository
// im not really sure the point of this two legged process tho
// you deposit btc and get redeemables
// and you deliver redeemables to get back btc
// but you can trade those redeemables for btc? do we hold a fraction liquid?
// and what, do you trade redeemables for usds? why not just give usds?
// it also raises the question of... how exactly is the position opened lol
// do we just crank it? uhh... also, we cant keep liquid holdings anyway...
// because that gives us delta exposure. creates risk of insolvency



use anchor_lang::prelude::*;
use anchor_spl::token::{self, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;
use anchor_spl::token::{self, Burn, Mint, MintTo, TokenAccount, Transfer};

#[program]
pub mod depository {
    use super::*;

    // #[access_control(InitializeDepository::accounts(&ctx, nonce)]
    pub fn initialize_depository(
        ctx: Context<InitializeDepository>,
    ) -> ProgramResult {
        let dep_acct = &mut ctx.accounts.depository_account;
        dep_acct.redeemable_mint = *ctx.accounts.redeemable_mint.to_account_info();
        dep_acct.asset_mint = *ctx.accounts.asset_mint

        ctx.accounts.authority.
        Ok()
    }


    pub fn deposit(
        ctx: Context<Deposit>,
        deposit_amount: u64,
    ) -> ProgramResult {

        //transfer from user to depository
        let cpi_accounts = Transfer{
            from: ctx.accounts.depository_token_account();
            to: ctx.accounts.depository_token_account();
            authority: accounts.user_authority.clone()
        }
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::Transfer(cpi_ctx, amount)?;
        // Mint Redeemable to user account

    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        withdraw_amount: u64,
    ) -> ProgramResult {


    }

    pub fn transfer(
        ctx: Context<Transfer>,
        transfer_amount: u64,
    ) -> ProgramResult {

    }

    pub fn proxy_transfer(
        ctx: Context<Transfer>
    )

    #[state]
    pub struct Depository {
        pub authority: Pubkey,
        pub redeemable_mint: Pubkey,
        pub
    }


}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AuthorityType {
    /// Authority to mint new tokens
    MintTokens,
    /// Authority to freeze any account associated with the Mint
    FreezeAccount,
    /// Owner of a given token account
    AccountOwner,
    /// Authority to close a token account
    CloseAccount,
}

#[derive(Accounts)]
pub struct InitializeDepository<'info> {
    #[account(init)]
    pub depository_account: ProgramAccount<'info, DepositoryAccount>,
    pub depository_signer: AccountInfo<'info>,
    #[account(
    "redeemable_mint.mint_authority == COption::Some(*pool_signer.key)", //figure out later
    "redeemable_mint.supply == 0"
    )]
    pub redeemable_mint: CpiAccount<'info, Mint>,  // internal token
    #[account()]
    pub deposit_token_mint: CpiAccount<'info, Mint>, // external token ie. sol
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut, "depository_token_account.owner == *depository_signer.key")]
    pub depository_token_account: CpiAccount<'info, TokenAccount>,
    //
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitializeDepository<'info> {
    // haven't even started this
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account()]
    pub depository_token_account: ProgramAccount<'info, TokenAccount>,
    #[account(signer)]
    pub user_authority: AccountInfo<'info>,
    #[account(mut, "user_token_account.owner == user_authority.key")]
    pub user_token_account: CpiAccount<'info, TokenAccount>,
    #[account("token_program.key == &token::ID")]
    pub token_program: AccountInfo<'info>,
    #[account("user_redeemable_account.owner == user_authority.key")]
    pub user_redeemable_account: CpiAccount<'info, TokenAccount>,
    #[account(mut, "redeemable_mint.mint_authority == COption::Some(*pool_signer.key)")] //investigate this line further
    pub redeemable_mint: CpiAccount<'info, Mint>,
}



impl<'a, 'b, 'c, 'info> From<&mut ProxyTransfer<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(accounts: &mut ProxyTransfer<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.from.clone(),
            to: accounts.to.clone(),
            authority: accounts.authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
