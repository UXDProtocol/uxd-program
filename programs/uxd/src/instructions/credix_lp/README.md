
# Deposit

### Deposit Funds

##### Raw param

- investor
- globalMarketState (pda seed: "global_market_seed")
- signingAuthority (pda seed: globalMarketState)
- investorTokenAccount
- liquidityPoolTokenAccount
- lpTokenMint (pda seed: globalMarketState + "lp-token-mint")
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
generateSigningAuthorityPDA() {
    return (0, pda_utils_1.findSigningAuthorityPDA)(this.address, this.programId);
}
findBaseTokenAccount(pk) {
    return spl_token_1.Token.getAssociatedTokenAddress(spl_token_1.ASSOCIATED_TOKEN_PROGRAM_ID, spl_token_1.TOKEN_PROGRAM_ID, this.baseMintPK, pk, true);
}
findLiquidityPoolTokenAccount() {
    return __awaiter(this, void 0, void 0, function* () {
        const [signingAuthorityPK] = yield this.generateSigningAuthorityPDA();
        return this.findBaseTokenAccount(signingAuthorityPK);
    });
}
findLPTokenAccount(pk) {
    return spl_token_1.Token.getAssociatedTokenAddress(spl_token_1.ASSOCIATED_TOKEN_PROGRAM_ID, spl_token_1.TOKEN_PROGRAM_ID, this.lpMintPK, pk, true);
}
generateCredixPassPDA(pk) {
    const credixSeed = (0, pda_utils_1.encodeSeedString)("credix-pass");
    const seed = [this.address.toBuffer(), pk.toBuffer(), credixSeed];
    return web3_js_1.PublicKey.findProgramAddress(seed, this.programId);
}```

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
- investorLpTokenAccount
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
