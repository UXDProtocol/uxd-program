import * as anchor from "@project-serum/anchor";
import { ASSOCIATED_TOKEN_PROGRAM_ID, NATIVE_MINT, Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";

//

/**
 *
 * @param {*} connection
 * @param {anchor.web3.PublicKey} userKey
 * @param {number} amountNative
 * @returns {Promise<anchor.web3.TransactionInstruction[]>}
 */
export const prepareWrappedSolTokenAccount = async (
    connection,
    userKey,
    amountNative
) => {
    const wsolTokenKey = findAssociatedTokenAddress(userKey, NATIVE_MINT);
    const tokenAccount = await connection.getParsedAccountInfo(wsolTokenKey);
    if (tokenAccount.value) {
        const balanceNative = Number(
            tokenAccount.value.data.parsed.info.tokenAmount.amount
        );
        if (balanceNative < amountNative) {
            return [
                transferSolItx(userKey, wsolTokenKey, amountNative - balanceNative),
                Token.createSyncNativeInstruction(TOKEN_PROGRAM_ID, wsolTokenKey),
            ];
        } else {
            // no-op we have everything we need
        }
    } else {
        return createWrappedSolTokenAccount(connection, userKey, amountNative);
    }
    return [];
};

// derives the canonical token account address for a given wallet and mint
function findAssociatedTokenAddress(walletKey, mintKey) {
    if (!walletKey || !mintKey) return;
    return findAddr(
        [walletKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mintKey.toBuffer()],
        ASSOCIATED_TOKEN_PROGRAM_ID
    );
}

// simple shorthand
function findAddr(seeds, programId) {
    return anchor.utils.publicKey.findProgramAddressSync(seeds, programId)[0];
}

/**
 *
 * @param {anchor.web3.PublicKey} fromKey
 * @param {anchor.web3.PublicKey} toKey
 * @param {number} amountNative
 * @returns {anchor.web3.TransactionInstruction}
 */
const transferSolItx = (fromKey, toKey, amountNative) =>
    anchor.web3.SystemProgram.transfer({
        fromPubkey: fromKey,
        toPubkey: toKey,
        lamports: amountNative,
    });

/**
*
* @param {*} connection
* @param {anchor.web3.PublicKey} userKey
* @param {number} amountNative
* @returns {Promise<anchor.web3.TransactionInstruction[]>}
*/
const createWrappedSolTokenAccount = async (
    connection,
    userKey,
    amountNative = 0
) => {
    const assocTokenKey = findAssociatedTokenAddress(userKey, NATIVE_MINT);
    const balanceNeeded = await Token.getMinBalanceRentForExemptAccount(
        connection
    );

    const transferItx = transferSolItx(
        userKey,
        assocTokenKey,
        amountNative + balanceNeeded
    );
    const createItx = createAssociatedTokenAccountItx(userKey, NATIVE_MINT);

    return [transferItx, createItx];
};

function createAssociatedTokenAccountItx(walletKey, mintKey) {
    const assocKey = findAssociatedTokenAddress(walletKey, mintKey);

    return new anchor.web3.TransactionInstruction({
        keys: [
            { pubkey: walletKey, isSigner: true, isWritable: true },
            { pubkey: assocKey, isSigner: false, isWritable: true },
            { pubkey: walletKey, isSigner: false, isWritable: false },
            { pubkey: mintKey, isSigner: false, isWritable: false },
            {
                pubkey: anchor.web3.SystemProgram.programId,
                isSigner: false,
                isWritable: false,
            },
            { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
            {
                pubkey: anchor.web3.SYSVAR_RENT_PUBKEY,
                isSigner: false,
                isWritable: false,
            },
        ],
        programId: ASSOCIATED_TOKEN_PROGRAM_ID,
        data: Buffer.alloc(0),
    });
}