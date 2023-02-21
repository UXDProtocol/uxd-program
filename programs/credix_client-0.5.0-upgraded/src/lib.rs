/*!
# Credix Rust client

A crate to interact with the Credix program via CPI

## Features

To target our pre-mainnet environment you can enable the `pre-mainnet` feature

```toml
credix_client = { version="0.4.0", features = ["pre-mainnet"] }
```

## Instructions

### Deposit Funds

This instruction will deposit funds into the liquidity pool. A Credix pass is needed to be allowed to do this.

#### Accounts

```text
investor:                     [writable, signer]
global_market_state:          []
signing_authority:            []
investor_token_account:       [writable]
liquidity_pool_token_account: [writable]
lp_token_mint:                [writable]
investor_lp_token_account:    [writable]
credix_pass:                  []
base_token_mint:              []
associated_token_program:     []
rent:                         []
token_program:                []
system_program:               []
```

### Withdraw Funds

This instruction will withdraw funds from the liquidity pool. A Credix pass is needed to be allowed to do this.

#### Accounts

```text
investor:                       [writable, signer]
global_market_state:            [writable]
program_state:                  []
signing_authority:              []
investor_lp_token_account:      [writable]
investor_token_account:         [writable]
liquidity_pool_token_account:   [writable]
credix_multisig_key:            []
credix_multisig_token_account:  [writable]
treasury_pool_token_account:    [writable]
lp_token_mint:                  [writable]
credix_pass:                    []
base_token_mint:                []
associated_token_program:       []
token_program:                  []
rent:                           []
system_program:                 []
```

## Examples

### On-chain

```Rust
use anchor_lang::prelude::*;
use credix_client::cpi::accounts::DepositFunds;
use credix_client::cpi::deposit_funds;
use credix_client::program::Credix;

use credix_client::cpi::accounts::WithdrawFunds;
use credix_client::cpi::withdraw_funds;

# [program]
pub mod example {
    use super::*;

    pub fn deposit_cpi(ctx: <DepositFundsCpi>, amount: u64) -> Result<()> {
        let cpi_ = Cpi::new(
            ctx.accounts.credix_program.to_account_info(),
            DepositFunds {
                investor: ctx.accounts.investor.to_account_info(),
                global_market_state: ctx.accounts.global_market_state.to_account_info(),
                signing_authority: ctx.accounts.signing_authority.to_account_info(),
                investor_token_account: ctx.accounts.investor_token_account.to_account_info(),
                credix_pass: ctx.accounts.credix_pass.to_account_info(),
                investor_lp_token_account: ctx.accounts.investor_lp_token_account.to_account_info(),
                liquidity_pool_token_account: ctx
                    .accounts
                    .liquidity_pool_token_account
                    .to_account_info(),
                lp_token_mint: ctx.accounts.lp_token_mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
                base_token_mint: ctx.accounts.base_token_mint.to_account_info(),
            },
        );

        deposit_funds(cpi_, amount)?;

        Ok(())
    }

    pub fn withdraw_cpi(ctx: <WithdrawFundsCpi>, amount: u64) -> Result<()> {
        let cpi_ = Cpi::new(
            ctx.accounts.credix_program.to_account_info(),
            WithdrawFunds {
                investor: ctx.accounts.investor.to_account_info(),
                global_market_state: ctx.accounts.global_market_state.to_account_info(),
                signing_authority: ctx.accounts.signing_authority.to_account_info(),
                investor_lp_token_account: ctx.accounts.investor_lp_token_account.to_account_info(),
                investor_token_account: ctx.accounts.investor_token_account.to_account_info(),
                liquidity_pool_token_account: ctx
                    .accounts
                    .liquidity_pool_token_account
                    .to_account_info(),
                lp_token_mint: ctx.accounts.lp_token_mint.to_account_info(),
                program_state: ctx.accounts.program_state.to_account_info(),
                credix_multisig_key: ctx.accounts.credix_multisig_key.to_account_info(),
                credix_multisig_token_account: ctx
                    .accounts
                    .credix_multisig_token_account
                    .to_account_info(),
                treasury_pool_token_account: ctx
                    .accounts
                    .treasury_pool_token_account
                    .to_account_info(),
                credix_pass: ctx.accounts.credix_pass.to_account_info(),
                base_token_mint: ctx.accounts.base_token_mint.to_account_info(),
                associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
        );

        withdraw_funds(cpi_, amount)?;

        Ok(())
    }
}
```

### Off-chain

#### Typescript

For off-chain development we provide a [Typescript client](https://www.npmjs.com/package/@credix/credix-client) to help with gathering different accounts. See the README of that package to get started with it.

```typescript
...
const market = await client.fetchMarket("credix-marketplace");

// Our client can help you with finding the keys of following accounts
const globalMarketState = market.address;
const programState = (await client.fetchProgramState()).address;
const signingAuthority = (await market.generateSigningAuthorityPDA())[0];
const investorLpTokenAccount = await market.findLPTokenAccount(investor);
const investorTokenAccount = await market.findBaseTokenAccount(investor);
const liquidityPoolTokenAccount = await market.findLiquidityPoolTokenAccount();
const credixMultisigKey = (await client.fetchProgramState()).credixMultisigKey;
const credixMultisigTokenAccount = await market.findBaseTokenAccount(credixMultisigKey);
const treasuryPoolTokenAccount = market.treasury;
const lpTokenMint = market.lpMintPK;
const credixPass = (await market.fetchCredixPass(investor)).address
const baseTokenMint = await market.baseMintPK;
...
```

#### Rust

This crate also provides functions to help generate PDA's. See the Rust [docs](https://docs.rs/credix_client/latest/credix_client/state/)

### Disclaimer

These examples are provided as is. Do not blindly copy paste for use in production.

*/

#[cfg(not(feature = "pre-mainnet"))]
declare_id!("CRDx2YkdtYtGZXGHZ59wNv1EwKHQndnRc1gT4p8i2vPX");
#[cfg(feature = "pre-mainnet")]
declare_id!("CRdXwuY984Au227VnMJ2qvT7gPd83HwARYXcbHfseFKC");

const BORROWER_INFO_SEED: &str = "borrower-info";
const DEAL_INFO_SEED: &str = "deal-info";
const CREDIX_PASS_SEED: &str = "credix-pass";
const LP_TOKEN_MINT_SEED: &str = "lp-token-mint";
const TRANCHE_PASS_SEED: &str = "tranche-pass";
const TRANCHE_MINT_SEED: &str = "tranche-mint";
const TRANCHES_SEED: &str = "tranches";
const REPAYMENT_SCHEDULE: &str = "repayment-schedule";
const DEAL_TOKEN_ACCOUNT: &str = "deal-token-account";
const INVESTOR_TRANCHE_SEED: &str = "tranche";
const PROGRAM_STATE_SEED: &str = "program-state";
const MARKET_ADMINS_SEED: &str = "admins";

anchor_gen::generate_cpi_crate!("./src/credix.json");

impl ProgramState {
    pub fn generate_pda() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[&PROGRAM_STATE_SEED.as_bytes()], &ID)
    }
}

impl GlobalMarketState {
    pub fn generate_pda(market_seeds: &String) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[&market_seeds.as_bytes()], &ID)
    }

    pub fn generate_signing_authority_pda(market_seeds: &String) -> (Pubkey, u8) {
        let global_market_state = Self::generate_pda(market_seeds).0;

        Pubkey::find_program_address(&[&global_market_state.to_bytes()], &ID)
    }

    pub fn generate_lp_token_mint_pda(market_seeds: &String) -> (Pubkey, u8) {
        let global_market_state = Self::generate_pda(market_seeds).0;

        Pubkey::find_program_address(
            &[
                &global_market_state.to_bytes(),
                LP_TOKEN_MINT_SEED.as_bytes(),
            ],
            &ID,
        )
    }
}

impl BorrowerInfo {
    pub fn generate_pda(market_seeds: &String, borrower: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                &market_seeds.as_bytes(),
                borrower.as_ref(),
                BORROWER_INFO_SEED.as_bytes(),
            ],
            &ID,
        )
    }
}

impl CredixPass {
    pub fn generate_pda(global_market_state: Pubkey, owner: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                &global_market_state.to_bytes(),
                &owner.to_bytes(),
                CREDIX_PASS_SEED.as_bytes(),
            ],
            &ID,
        )
    }
}

impl DealTranches {
    pub fn generate_pda(global_market_state: Pubkey, deal: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                &global_market_state.to_bytes(),
                &deal.to_bytes(),
                TRANCHES_SEED.as_bytes(),
            ],
            &ID,
        )
    }

    pub fn generate_tranche_mint(
        global_market_state: Pubkey,
        deal: Pubkey,
        num: u8,
    ) -> (Pubkey, u8) {
        let deal_tranches = Self::generate_pda(global_market_state, deal).0;
        let num = num.to_le_bytes();
        let seeds = &[deal_tranches.as_ref(), &num, TRANCHE_MINT_SEED.as_bytes()];
        Pubkey::find_program_address(seeds, &crate::ID)
    }
}

impl Deal {
    pub fn generate_pda(
        global_market_state: Pubkey,
        borrower: Pubkey,
        deal_number: u16,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                &global_market_state.to_bytes(),
                &borrower.to_bytes(),
                &deal_number.to_le_bytes(),
                DEAL_INFO_SEED.as_bytes(),
            ],
            &ID,
        )
    }

    pub fn generate_deal_token_account_pda(
        global_market_state: Pubkey,
        deal: Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                &global_market_state.to_bytes(),
                &deal.to_bytes(),
                DEAL_TOKEN_ACCOUNT.as_bytes(),
            ],
            &ID,
        )
    }
}

impl InvestorTranche {
    pub fn generate_pda(
        global_market_state: Pubkey,
        investor: Pubkey,
        deal: Pubkey,
        tranche_index: u8,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                &global_market_state.to_bytes(),
                &investor.to_bytes(),
                &deal.to_bytes(),
                &tranche_index.to_le_bytes(),
                INVESTOR_TRANCHE_SEED.as_bytes(),
            ],
            &ID,
        )
    }
}

impl MarketAdmins {
    pub fn generate_pda(global_market_state: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[global_market_state.as_ref(), MARKET_ADMINS_SEED.as_bytes()],
            &ID,
        )
    }
}

impl RepaymentSchedule {
    pub fn generate_pda(global_market_state: Pubkey, deal: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                &global_market_state.to_bytes(),
                &deal.to_bytes(),
                REPAYMENT_SCHEDULE.as_bytes(),
            ],
            &ID,
        )
    }
}

impl TranchePass {
    pub fn generate_pda(
        global_market_state: Pubkey,
        owner: Pubkey,
        deal: Pubkey,
        tranche_index: u8,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                &global_market_state.to_bytes(),
                &owner.to_bytes(),
                &deal.to_bytes(),
                &tranche_index.to_le_bytes(),
                TRANCHE_PASS_SEED.as_bytes(),
            ],
            &ID,
        )
    }
}
