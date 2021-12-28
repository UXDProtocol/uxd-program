import { MangoDepository, Mango, SOL_DECIMALS, findATAAddrSync, Controller } from "@uxdprotocol/uxd-client";
import { PublicKey } from "@solana/web3.js";
import { uxdClient, uxdHelpers } from "./constants";
import { nativeI80F48ToUi, nativeToUi } from "@blockworks-foundation/mango-client";
import { BN } from "@project-serum/anchor";
import * as anchor from "@project-serum/anchor";
import { ASSOCIATED_TOKEN_PROGRAM_ID, NATIVE_MINT, Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { provider, TXN_COMMIT, TXN_OPTS } from "./provider";

export function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function getSolBalance(wallet: PublicKey): Promise<number> {
    const lamports = await provider.connection
        .getBalance(wallet, TXN_COMMIT);
    return nativeToUi(lamports, SOL_DECIMALS);
}

export function getBalance(tokenAccount: PublicKey): Promise<number> {
    return provider.connection
        .getTokenAccountBalance(tokenAccount, TXN_COMMIT)
        .then((o) => o["value"]["uiAmount"])
        .catch(() => 0);
}

export async function printUserInfo(user: PublicKey, controller: Controller, depository: MangoDepository) {
    const userCollateralATA: PublicKey = findATAAddrSync(user, depository.collateralMint)[0];
    const userRedeemableATA: PublicKey = findATAAddrSync(user, controller.redeemableMintPda)[0];

    console.group("[User balances]");
    console.log("Native SOL", `\t\t\t\t\t\t\t`, await getSolBalance(user));
    console.log(`${depository.collateralMintSymbol}`, `\t\t\t\t\t\t\t\t`, await getBalance(userCollateralATA));
    console.log(`${controller.redeemableMintSymbol}`, `\t\t\t\t\t\t\t\t`, await getBalance(userRedeemableATA));
    console.groupEnd()
}

export async function printDepositoryInfo(controller: Controller, depository: MangoDepository, mango: Mango) {
    const SYM = depository.collateralMintSymbol;
    const controllerAccount = await uxdHelpers.getControllerAccount(provider, uxdClient.program, controller, TXN_OPTS);
    const depositoryAccount = await uxdHelpers.getMangoDepositoryAccount(provider, uxdClient.program, depository, TXN_OPTS);
    const mangoAccount = await mango.load(depository.mangoAccountPda);
    const pmi = mango.getPerpMarketConfig(SYM).marketIndex;
    const smi = mango.getSpotMarketConfig(SYM).marketIndex;
    const sti = mango.getTokenIndex(depository.collateralMint);
    const pa = mangoAccount.perpAccounts[pmi];
    const pm = await mango.getPerpMarket(SYM);
    const cache = await mango.group.loadCache(provider.connection);
    const accountValue = mangoAccount.computeValue(mango.group, cache).toBig();
    const accountingInsuranceDepositedValue = nativeToUi(depositoryAccount.insuranceAmountDeposited.toNumber(), 6);
    const collateralSpotAmount = mangoAccount.getNet(cache.rootBankCache[smi], sti);
    const collateralDepositInterests = new BN(collateralSpotAmount.toNumber()).sub(depositoryAccount.collateralAmountDeposited);
    const accountValueMinusInsurance = accountValue - accountingInsuranceDepositedValue;
    const redeemableUnderManagement = nativeToUi(depositoryAccount.redeemableAmountUnderManagement.toNumber(), 6);

    // await mango.printAccountInfo(mangoAccount);

    console.group("[Depository", SYM, "]");
    console.log("collateralPassthrough", "\t\t\t\t\t", await getBalance(depository.collateralPassthroughPda));
    console.log("insurancePassthroughPda", "\t\t\t\t\t", await getBalance(depository.insurancePassthroughPda));

    console.group("[Derived Information]");
    console.log("depository PnL", "\t\t\t\t\t\t", (accountValueMinusInsurance - redeemableUnderManagement));
    console.log("collateral deposit interests ", "\t\t\t\t", nativeToUi(collateralDepositInterests.toNumber(), 9));
    console.groupEnd();

    console.group("[On Chain Accounting]");
    console.log("insuranceAmountDeposited", "\t\t\t\t\t", accountingInsuranceDepositedValue);
    console.log("collateralAmountDeposited", "\t\t\t\t\t", nativeToUi(depositoryAccount.collateralAmountDeposited.toNumber(), 9));
    console.log("redeemableAmountUnderManagement", "\t\t\t\t", redeemableUnderManagement, "/", nativeToUi(controllerAccount.redeemableCirculatingSupply.toNumber(), 6), "(controller.redeemableCirculatingSupply)");
    console.log("totalAmountPaidTakerFee", "\t\t\t\t\t", nativeToUi(depositoryAccount.totalAmountPaidTakerFee.toNumber(), 6));
    console.groupEnd();

    console.group("[Depository mango account (Program owned)]");
    console.log(`${SYM}-SPOT BASE Pos`, "\t\t\t\t\t\t", nativeI80F48ToUi(collateralSpotAmount, 9).toNumber());
    console.groupCollapsed(`${SYM}-PERP BASE Pos`);
    console.table([
        {
            base_position: nativeToUi(pm.baseLotsToNative(pa.basePosition).toNumber(), 9),
            quote_position: nativeI80F48ToUi(pa.quotePosition, 6).toNumber(),
            taker_base: nativeToUi(pm.baseLotsToNative(pa.takerBase).toNumber(), 9),
            taker_quote: nativeToUi(pa.takerQuote.toNumber(), 6),
            unsettled_funding: nativeI80F48ToUi(pa.getUnsettledFunding(cache.perpMarketCache[pmi]), 6).toNumber(),
        }
    ]);
    console.groupEnd()
    console.groupEnd();
    console.groupEnd();
}

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