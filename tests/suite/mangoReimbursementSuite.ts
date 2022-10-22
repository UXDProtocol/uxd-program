import { AnchorProvider } from "@project-serum/anchor";
import { PublicKey, sendAndConfirmTransaction, Signer, Transaction } from "@solana/web3.js";
import { Controller, createAssocTokenIx, findATAAddrSync } from "@uxd-protocol/uxd-client";
import { MangoV3ReimbursementClient, ID as MangoV3ReimbursementProgramId } from '@blockworks-foundation/mango-v3-reimbursement-lib/dist/client';
import { MANGO_REIMBURSEMENT_TABLE, uxdClient } from "../constants";
import { MangoDepository } from "@uxd-protocol/uxd-client";
import { getConnection, TXN_OPTS } from "../connection";
import { mangoReimburseTest } from "../cases/mangoReimburseTest";

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

export const mangoReimbursementSuite = function (
    authority: Signer,
    payer: Signer,
    depository: MangoDepository,
    controller: Controller,
) {
    /*const mangoV3ReimbursementClient = new MangoV3ReimbursementClient(AnchorProvider.local("https://mango.devnet.rpcpool.com"));
 
     it('Setup new mango reimbursement group', async () => {
         const claimTransferDestination = new PublicKey('mdcXrm2NkzXYvHNcKXzCLXT58R4UN8Rzd1uzD4h8338');
         const testing = 1;
 
         // get the new group number by checking existing groups
         const groupIndex = await getNewGroupIndex(mangoV3ReimbursementClient);
 
         console.log('groups', JSON.stringify((await mangoV3ReimbursementClient.program.account.group.all(), null, 3)));
 
         console.log('New groupIndex', groupIndex);
 
         // create the new group
         const signature = await mangoV3ReimbursementClient.program.methods
             .createGroup(
                 groupIndex,
                 claimTransferDestination,
                 testing,
             )
             .accounts({
                 table: MANGO_REIMBURSEMENT_TABLE,
                 payer: payer.publicKey,
                 authority: authority.publicKey,
             })
             .signers([authority, payer])
             .rpc({ skipPreflight: true });
 
         console.log(
             `created group, sig https://explorer.solana.com/tx/${signature}?cluster=devnet`
         );
 
         let group = await getGroupOnChain(mangoV3ReimbursementClient, groupIndex);
 
         console.log('group', group);
     });*/

    it(`Proceed to reimbursement of SOL token`, () => mangoReimburseTest(
        authority,
        payer,
        controller,
        depository,
        MANGO_REIMBURSEMENT_GROUP,
        new PublicKey('So11111111111111111111111111111111111111112'),
        6,
        'WSOL',
    ));
};