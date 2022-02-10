describe("Rebalancing tests", () => {
    const user = new Keypair();
    console.log("USER =>", user.publicKey.toString());


    before("Transfer 2000 USDC from bank to test user", async () => {
        const usdcToken = new Token(getConnection(), USDC_DEVNET, TOKEN_PROGRAM_ID, bank);
        const sender = await usdcToken.getOrCreateAssociatedAccountInfo(bank.publicKey);
        const receiver = await usdcToken.getOrCreateAssociatedAccountInfo(user.publicKey);
        const transferTokensIx = Token.createTransferInstruction(TOKEN_PROGRAM_ID, sender.address, receiver.address, bank.publicKey, [], uiToNative(2000, USDC_DECIMALS).toNumber());
        const transaction = new web3.Transaction().add(transferTokensIx);
        await web3.sendAndConfirmTransaction(getConnection(), transaction, [
            bank,
        ]);
    });

    // TEST REBALANCING
    it(`Rebalance 50 ${.quoteMintSymbol} (${params.slippage} slippage)`, async () => {
        const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
        console.log("unrealizedPnl", unrealizedPnl);
        const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;
        const rebalancedAmount = await rebalanceMangoDepositoryLiteTest(50, polarity, params.slippage, user, controller, depository, mango, bank);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });


    after("Return remaining USDC balance to the bank", async () => {
        const usdcToken = new Token(getConnection(), USDC_DEVNET, TOKEN_PROGRAM_ID, bank);
        const sender = await usdcToken.getOrCreateAssociatedAccountInfo(user.publicKey);
        const receiver = await usdcToken.getOrCreateAssociatedAccountInfo(bank.publicKey);
        const amount = await getBalance(sender.address);
        const transferTokensIx = Token.createTransferInstruction(TOKEN_PROGRAM_ID, sender.address, receiver.address, user.publicKey, [], uiToNative(amount, USDC_DECIMALS).toNumber());
        const transaction = new web3.Transaction().add(transferTokensIx);
        await web3.sendAndConfirmTransaction(getConnection(), transaction, [
            user,
        ]);
    });
});