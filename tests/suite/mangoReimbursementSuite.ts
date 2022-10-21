import { AnchorProvider } from "@project-serum/anchor";
import { PublicKey, sendAndConfirmTransaction, Signer, Transaction } from "@solana/web3.js";
import { Controller, createAssocTokenIx, findATAAddrSync } from "@uxd-protocol/uxd-client";
import { MangoV3ReimbursementClient, ID as MangoV3ReimbursementProgramId } from '@blockworks-foundation/mango-v3-reimbursement-lib/dist/client';
import { uxdClient } from "../constants";
import { MangoDepository } from "@uxd-protocol/uxd-client";
import { getConnection, TXN_OPTS } from "../connection";

const CLAIM_TRANSFER_DESTINATION = new PublicKey('mdcXrm2NkzXYvHNcKXzCLXT58R4UN8Rzd1uzD4h8338');
const MANGO_REIMBURSEMENT_TABLE = new PublicKey('tab2GSQhmstsCiPmPABk1F8QnffSaFEXnqbef7AkEnB');
const MANGO_REIMBURSEMENT_GROUP = new PublicKey('4vSjJeDnJY3edWizzPgNuWRSCnxNoSYHm7zQ3xgTcBKB');

async function getGroupOnChain(mangoV3ReimbursementClient: MangoV3ReimbursementClient, groupIndex: number) {
    return (
        await mangoV3ReimbursementClient.program.account.group.all()
    ).find((group) => group.account.groupNum === groupIndex);
}

// Return an uninitialized group index
async function getNewGroupIndex(mangoV3ReimbursementClient: MangoV3ReimbursementClient) {
    const groups = await mangoV3ReimbursementClient.program.account.group.all();

    return groups.length;
}

async function findMangoReimbursementAccountAddress(mangoReimbursementGroup: PublicKey, mangoAccountOwner: PublicKey) {
    return (
        await PublicKey.findProgramAddress(
            [
                Buffer.from("ReimbursementAccount"),
                mangoReimbursementGroup.toBuffer()!,
                mangoAccountOwner.toBuffer(),
            ],
            MangoV3ReimbursementProgramId
        )
    )[0];
}

export const mangoReimbursementSuite = function (
    authority: Signer,
    payer: Signer,
    depository: MangoDepository,
    controller: Controller,
    provider: AnchorProvider,
) {
    const mangoV3ReimbursementClient = new MangoV3ReimbursementClient(provider);

    /*
    it('Setup mango reimbursement group', async () => {
        // get the new group number by checking existing groups
        const groupIndex = await getNewGroupIndex(mangoV3ReimbursementClient);

        // create the new group
        const signature = await mangoV3ReimbursementClient.program.methods
            .createGroup(
                groupIndex,
                CLAIM_TRANSFER_DESTINATION,
                1 // testing
            )
            .accounts({
                table: TABLE,
                payer: payer.publicKey,
                authority: authority.publicKey,
            })
            .rpc({ skipPreflight: true });

        console.log(
            `created group, sig https://explorer.solana.com/tx/${signature}?cluster=devnet`
        );

        let group = await getGroupOnChain(mangoV3ReimbursementClient, groupIndex);
    });
    */

    it(`Proceed to reimbursement`, async () => {
        const onChainMangoReimbursementGroup = await mangoV3ReimbursementClient.program.account.group.fetch(MANGO_REIMBURSEMENT_GROUP);

        // Look for specific mint in group mints
        const tokenMint = new PublicKey('So11111111111111111111111111111111111111112');
        const tokenIndex = onChainMangoReimbursementGroup.mints.findIndex(mint => mint.equals(tokenMint));

        const mangoReimbursementVault = onChainMangoReimbursementGroup.vaults[tokenIndex];
        const mangoReimbursementClaimMint = onChainMangoReimbursementGroup.claimMints[tokenIndex];

        const claimTransferDestination = onChainMangoReimbursementGroup.claimTransferDestination;

        const [mangoReimbursementClaimMintTokenAccount] = findATAAddrSync(claimTransferDestination, mangoReimbursementClaimMint);

        // Setup the reimbursement account
        const mangoReimbursementAccount = await findMangoReimbursementAccountAddress(MANGO_REIMBURSEMENT_GROUP, depository.pda);

        if (!(await getConnection().getAccountInfo(mangoReimbursementAccount))) {
            const signature = await mangoV3ReimbursementClient.program.methods
                .createReimbursementAccount()
                .accounts({
                    group: MANGO_REIMBURSEMENT_GROUP,
                    mangoAccountOwner: depository.pda,
                    payer: payer.publicKey,
                })
                .signers([payer])
                .rpc({ skipPreflight: true });
            console.log(
                `created reimbursement account for ${authority.publicKey}, sig https://explorer.solana.com/tx/${signature}?cluster=devnet`
            );
        } else {
            console.log('Reimbursement account already set up', mangoReimbursementAccount.toBase58());
        }

        // Find at the mango_account_owner within the table
        const rows = await mangoV3ReimbursementClient.decodeTable(onChainMangoReimbursementGroup);

        console.log('Rows', rows);

        const depositoryRow = rows.find((row) => row.owner.equals(depository.pda));
        const indexToTable = rows.findIndex((row) => row.owner.equals(depository.pda));

        console.log('depositoryRow', depositoryRow);

        console.log('depositoryRow', {
            owner: depositoryRow.owner.toBase58(),
            balances: depositoryRow.balances.map(x => x.toString()),
        });

        // Claim the first token
        const mangoReimburseIx = uxdClient.createMangoReimburseInstruction(
            controller,
            depository,
            authority.publicKey,
            tokenMint,
            tokenIndex,
            MangoV3ReimbursementProgramId,
            MANGO_REIMBURSEMENT_GROUP,
            mangoReimbursementVault,
            mangoReimbursementAccount,
            mangoReimbursementClaimMintTokenAccount,
            mangoReimbursementClaimMint,
            MANGO_REIMBURSEMENT_TABLE,
            indexToTable,
            TXN_OPTS,
        );

        const signers = [];
        const tx = new Transaction();

        // Init the authority token account if not initialized
        const [authorityTokenAccount] = findATAAddrSync(authority.publicKey, tokenMint);
        if (!(await getConnection().getAccountInfo(authorityTokenAccount))) {
            const createAuthorityTokenATAIx = createAssocTokenIx(authority.publicKey, authorityTokenAccount, tokenMint, payer.publicKey);
            tx.add(createAuthorityTokenATAIx);
        }

        // Init the depository token account if not initialized
        const [depositoryTokenAccount] = findATAAddrSync(depository.pda, tokenMint);
        if (!(await getConnection().getAccountInfo(authorityTokenAccount))) {
            const createDepositoryTokenATAIx = createAssocTokenIx(depository.pda, depositoryTokenAccount, tokenMint, payer.publicKey);
            tx.add(createDepositoryTokenATAIx);
        }

        tx.instructions.push(mangoReimburseIx);
        signers.push(authority);
        if (payer) {
            signers.push(payer);
        }
        tx.feePayer = payer.publicKey;
        tx.recentBlockhash = (await getConnection().getLatestBlockhash()).blockhash;
        tx.sign(authority, payer);

        console.log('TX', tx.serialize().toString('base64'));

        // const signature = await sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
        // console.log(
        //     `reimburse, sig https://explorer.solana.com/tx/${signature}?cluster=devnet`
        // );
    });
};