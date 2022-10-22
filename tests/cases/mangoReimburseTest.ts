import { MangoV3ReimbursementClient } from "@blockworks-foundation/mango-v3-reimbursement-lib/dist/client";
import { AnchorProvider } from "@project-serum/anchor";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, findATAAddrSync, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { mangoReimbursement } from "../api";
import { CLUSTER } from "../constants";
import { getBalance } from "../utils";

async function getExpectedTokenAmountToReceiveFromTable({
    tokenMint,
    mangoReimbursementGroup,
    depository,
}: {
    tokenMint: PublicKey;
    mangoReimbursementGroup: PublicKey;
    depository: MangoDepository;
}) {
    const mangoV3ReimbursementClient = new MangoV3ReimbursementClient(AnchorProvider.local("https://mango.devnet.rpcpool.com"));
    const onChainMangoReimbursementGroup = await mangoV3ReimbursementClient.program.account.group.fetch(mangoReimbursementGroup);

    const tokenIndex = onChainMangoReimbursementGroup.mints.findIndex(mint => mint.equals(tokenMint));

    const rows = await mangoV3ReimbursementClient.decodeTable(onChainMangoReimbursementGroup);
    const row = rows.find((row) => row.owner.equals(depository.pda));

    return row.balances[tokenIndex];
}

export const mangoReimburseTest = async function (
    authority: Signer,
    payer: Signer,
    controller: Controller,
    depository: MangoDepository,
    mangoReimbursementGroup: PublicKey,
    tokenMint: PublicKey,
    tokenDecimals: number,
    tokenMintSymbol: string,
): Promise<void> {
    console.group("🧭 mangoReimburseTest");
    try {
        // GIVEN
        const [authorityTokenAccount] = findATAAddrSync(authority.publicKey, tokenMint);
        const [depositoryTokenAccount] = findATAAddrSync(depository.pda, tokenMint);

        const [
            authorityTokenAccountBalance_pre,
            depositoryTokenAccountBalance_pre,
        ] = await Promise.all([
            getBalance(authorityTokenAccount),
            getBalance(depositoryTokenAccount),
        ]);

        const expectedReceivedTokenAmount = await getExpectedTokenAmountToReceiveFromTable({
            tokenMint,
            mangoReimbursementGroup,
            depository,
        });

        // WHEN
        const txId = await mangoReimbursement(authority, payer, controller, depository, mangoReimbursementGroup, tokenMint);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const [
            authorityTokenAccountBalance_post,
            depositoryTokenAccountBalance_post,
        ] = await Promise.all([
            getBalance(authorityTokenAccount),
            getBalance(depositoryTokenAccount),
        ]);

        const receivedTokenAmount = authorityTokenAccountBalance_post - authorityTokenAccountBalance_pre;
        const uiReceivedTokenAmount = nativeToUi(receivedTokenAmount, tokenDecimals);

        console.log(
            `🧾 Received`, uiReceivedTokenAmount.toLocaleString(), tokenMintSymbol,
        );
        expect(receivedTokenAmount.toString()).equals(expectedReceivedTokenAmount.toString(), "The amount of received tokens should be bigger than 0");
        expect((depositoryTokenAccountBalance_post - depositoryTokenAccountBalance_pre).toString()).equals(0, "Depository ATA balance should not change");

        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}