import { MangoDepository, Mango, SOL_DECIMALS, findATAAddrSync, Controller } from "@uxdprotocol/uxd-client";
import { PublicKey } from "@solana/web3.js";
import { MANGO_QUOTE_DECIMALS, uxdHelpers } from "./constants";
import { nativeI80F48ToUi, nativeToUi } from "@blockworks-foundation/mango-client";
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
    const controllerAccount = await uxdHelpers.getControllerAccount(provider, controller, TXN_OPTS);
    const depositoryAccount = await uxdHelpers.getMangoDepositoryAccount(provider, depository, TXN_OPTS);
    const mangoAccount = await mango.load(depository.mangoAccountPda);
    const pmi = mango.getPerpMarketConfig(SYM).marketIndex;
    const pa = mangoAccount.perpAccounts[pmi];
    const pm = await mango.getPerpMarket(SYM);
    const cache = await mango.group.loadCache(provider.connection);
    const accountValue = mangoAccount.computeValue(mango.group, cache).toBig();
    const accountingInsuranceDepositedValue = nativeToUi(depositoryAccount.insuranceAmountDeposited.toNumber(), depository.insuranceMintDecimals);
    // 
    const collateralSpotAmount = await uxdHelpers.getMangoDepositoryCollateralBalance(depository, mango);
    const insuranceSpotAmount = await uxdHelpers.getMangoDepositoryInsuranceBalance(depository, mango);
    //
    const collateralDepositInterests = collateralSpotAmount.toBig().sub(depositoryAccount.collateralAmountDeposited);
    const insuranceDepositInterests = insuranceSpotAmount.toBig().sub(depositoryAccount.insuranceAmountDeposited);
    //
    const accountValueMinusTotalInsuranceDeposited = accountValue - accountingInsuranceDepositedValue;
    const redeemableUnderManagement = nativeToUi(depositoryAccount.redeemableAmountUnderManagement.toNumber(), controller.redeemableMintDecimals);

    // await mango.printAccountInfo(mangoAccount);

    console.group("[Depository", SYM, "]");
    console.log("collateralPassthrough", "\t\t\t\t\t", await getBalance(depository.collateralPassthroughPda));
    console.log("insurancePassthroughPda", "\t\t\t\t\t", await getBalance(depository.insurancePassthroughPda));
    console.groupEnd()

    console.group("[Derived Information from onchain accounting and mango account] :");
    console.table({
        ["PnL (Quote)"]: Number((accountValueMinusTotalInsuranceDeposited - redeemableUnderManagement).toFixed(MANGO_QUOTE_DECIMALS)),
        [`collateral deposit interests (${SYM})`]: Number(nativeToUi(collateralDepositInterests, depository.collateralMintDecimals).toFixed(depository.collateralMintDecimals)),
        [`insurance deposit interests (${depository.insuranceMintSymbol})`]: Number(nativeToUi(insuranceDepositInterests.toNumber(), depository.insuranceMintDecimals).toFixed(depository.insuranceMintDecimals)),
    });
    console.groupEnd();

    console.group("[OnChain Accounting (Program)] :");
    console.table({
        [`insuranceAmountDeposited (${depository.insuranceMintSymbol})`]: accountingInsuranceDepositedValue,
        [`collateralAmountDeposited (${SYM})`]: nativeToUi(depositoryAccount.collateralAmountDeposited.toNumber(), depository.collateralMintDecimals),
        [`depository.redeemableAmountUnderManagement (${controller.redeemableMintSymbol})`]: redeemableUnderManagement,
        [`controller.redeemableCirculatingSupply (${controller.redeemableMintSymbol})`]: nativeToUi(controllerAccount.redeemableCirculatingSupply.toNumber(), controller.redeemableMintDecimals),
        ["totalAmountPaidTakerFee (Quote)"]: nativeToUi(depositoryAccount.totalAmountPaidTakerFee.toNumber(), MANGO_QUOTE_DECIMALS)
    });
    console.groupEnd()

    console.group("[MangoAccount (Program owned)] :");
    console.table({
        [`spot_base_position (${SYM})`]: Number(nativeI80F48ToUi(collateralSpotAmount, depository.collateralMintDecimals).toFixed(depository.collateralMintDecimals)),
        ["account value (minus insurance) (Quote)"]: Number(accountValueMinusTotalInsuranceDeposited.toFixed(MANGO_QUOTE_DECIMALS)),
        ["account value (Quote)"]: Number(accountValue.toFixed(MANGO_QUOTE_DECIMALS))
    });
    console.groupEnd()

    console.group("[MangoAccount's PerpAccount (Program owned)] :");
    console.table({
        ["perp_base_position"]: Number(nativeToUi(pm.baseLotsToNative(pa.basePosition).toNumber(), depository.collateralMintDecimals).toFixed(depository.collateralMintDecimals)),
        ["perp_quote_position"]: Number(nativeI80F48ToUi(pa.quotePosition, MANGO_QUOTE_DECIMALS).toFixed(MANGO_QUOTE_DECIMALS)),
        ["perp_taker_base"]: Number(nativeToUi(pm.baseLotsToNative(pa.takerBase).toNumber(), depository.collateralMintDecimals).toFixed(depository.collateralMintDecimals)),
        ["perp_taker_quote"]: Number(nativeToUi(pa.takerQuote.toNumber(), MANGO_QUOTE_DECIMALS).toFixed(MANGO_QUOTE_DECIMALS)),
        ["perp_unsettled_funding (Quote)"]: Number(nativeI80F48ToUi(pa.getUnsettledFunding(cache.perpMarketCache[pmi]), MANGO_QUOTE_DECIMALS).toFixed(MANGO_QUOTE_DECIMALS)),
    });
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
                // @ts-expect-error not sure why but it's not in their interface
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