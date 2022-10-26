
# Deposit

### Deposit Funds

##### Raw param

- investor
- globalMarketState
- signingAuthority (pda seed: globalMarketState)
- investorTokenAccount
- liquidityPoolTokenAccount
- lpTokenMint
- investorLpTokenAccount
- credixPass (pda seed: globalMarketState + investor + "credix-pass")
- baseTokenMint
- associatedTokenProgram
- rent
- tokenProgram
- systemProgram

- amount: u64

##### JS Encoder

```ts
depositBuilder(amount, investor) {
    return __awaiter(this, void 0, void 0, function* () {
        const [signingAuthority] = yield this.generateSigningAuthorityPDA();
        const investorTokenAccount = yield this.findBaseTokenAccount(investor);
        const liquidityPoolTokenAccount = yield this.findLiquidityPoolTokenAccount();
        const investorLPTokenAccount = yield this.findLPTokenAccount(investor);
        const [credixPass] = yield this.generateCredixPassPDA(investor);
        return this.anchorProgram.methods.depositFunds(new anchor_1.BN(amount)).accounts({
            investor,
            globalMarketState: this.address,
            signingAuthority: signingAuthority,
            investorTokenAccount: investorTokenAccount,
            liquidityPoolTokenAccount: liquidityPoolTokenAccount,
            lpTokenMint: this.lpMintPK,
            investorLpTokenAccount: investorLPTokenAccount,
            baseTokenMint: this.baseMintPK,
            tokenProgram: spl_token_1.TOKEN_PROGRAM_ID,
            credixPass,
            systemProgram: web3_js_1.SystemProgram.programId,
            rent: anchor_1.web3.SYSVAR_RENT_PUBKEY,
            associatedTokenProgram: spl_token_1.ASSOCIATED_TOKEN_PROGRAM_ID,
        });
    });
}
```

### Deposit Tranche

- investor
- globalMarketState
- signingAuthority (pda seed: globalMarketState)
- investorBaseAccount
- deal (pda seed: globalMarketState + deal.borrower + deal.deal_number + "deal-info")
- dealTranches (pda seed: globalMarketState + deal + "tranches")
- repaymentSchedule (pda seed: globalMarketState + deal + "repayment-schedule")
- dealTokenAccount (pda seed: globalMarketState + deal + "deal-token-account")
- trancheTokenMint (pda seed: deal_tranches + tranche_index + "tranche-mint")
- investorTrancheTokenAccount
- tranchePass (pda seed: globalMarketState + investor + deal + tranche_index + "tranche-pass")
- baseTokenMint
- associatedTokenProgram
- rent
- tokenProgram
- systemProgram

- amount :u64
- trancheIndex :u8

### Withdraw Funds

- investor
- globalMarketState
- signingAuthority (pda seed: globalMarketState)
- investorTokenAccount
- investorTokenAccount
- liquidityPoolTokenAccount
- treasuryPoolTokenAccount
- lpTokenMint
- credixPass
- baseTokenMint
- associatedTokenProgram
- tokenProgram

# Raw data

## Instructions

- initializeMarket
- depositFunds
- createDeal
- setTranches
- setRepaymentSchedule
- openDeal
- activateDeal
- repayDeal
- withdrawFunds
- createCredixPass
- updateCredixPass
- freezeGlobalMarketState
- thawGlobalMarketState
- updateLpTokenMetaData
- createTranchePass
- updateTranchePass
- depositTranche
- withdrawFromDeal
- burnTranche
- withdrawTranche
- initializeProgramState
- updateGlobalMarketCredixFees
- updateProgramState
- updateGlobalMarketState
- updateDeal
- updateMarketAdmins

## Accounts

- borrowerInfo
- credixPass
- dealTranches
- deal
- globalMarketState
- investorTranche
- marketAdmins
- programState
- repaymentSchedule
- tranchePass

## Types

- TrancheConfig
- Fraction
- DealTranche
- RepaymentPeriod
- RepaymentPeriodInput
- DealStatus

## Events

- DealCreationEvent
- DealActivationEvent
- BurnTrancheTokensEvent
- CreateCredixPassEvent
- CreateTranchePassEvent
- DealRepaymentEvent
- DepositEvent
- DepositTrancheEvent
- OpenDealEvent
- SetTranchesEvent
- SetRepaymentScheduleEvent
- UpdateCredixPassEvent
- FreezeGlobalMarketStateEvent
- ThawGlobalMarketStateEvent
- UpdateTranchePassEvent
- DealWithdrawEvent
- WithdrawEvent
- WithdrawTrancheEvent
