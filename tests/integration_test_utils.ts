import { MangoDepository, findATAAddrSync, Mango } from "@uxdprotocol/uxd-client";
import { BTC, user, WSOL } from "./identities";
import { provider, TXN_COMMIT, TXN_OPTS } from "./provider";
import { PublicKey } from "@solana/web3.js";
import { controllerUXD, uxdClient, uxdHelpers } from "./test_0_consts";

// User's SPL Accounts
export const userBTCATA: PublicKey = findATAAddrSync(user.publicKey, BTC)[0];
export const userWSOLATA: PublicKey = findATAAddrSync(user.publicKey, WSOL)[0];
export const userUXDATA: PublicKey = findATAAddrSync(user.publicKey, controllerUXD.redeemableMintPda)[0];

export function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

export function getBalance(tokenAccount: PublicKey): Promise<number> {
    return provider.connection
        .getTokenAccountBalance(tokenAccount, TXN_COMMIT)
        .then((o) => o["value"]["uiAmount"])
        .catch(() => null);
}

export async function printDepositoryInfo(depository: MangoDepository, mango: Mango) {
    // Sleep waiting for mango market update
    // await sleep(accountUpdateSleepingInterval);

    const SYM = depository.collateralMintSymbol;
    console.log(`\
        * [Depository ${SYM}]
        *     collateral_passthrough:                     ${await getBalance(depository.collateralPassthroughPda)}`);
    const mangoAccount = await mango.load(depository.mangoAccountPda); // might do that in the TS object then reload idk
    await mango.printAccountInfo(mangoAccount);

    let depositoryAccount = await uxdHelpers.getMangoDepositoryAccount(provider, uxdClient.program, depository, TXN_OPTS);
    console.log("=================");
    console.log(`Depository insuranceAmountDeposited            ${depositoryAccount.insuranceAmountDeposited.toNumber()}`);
    console.log(`Depository collateralAmountDeposited           ${depositoryAccount.collateralAmountDeposited.toNumber()}`);
    console.log(`Depository redeemableAmountUnderManagement     ${depositoryAccount.redeemableAmountUnderManagement.toNumber()}`);

    let controllerAccount = await uxdHelpers.getControllerAccount(provider, uxdClient.program, controllerUXD, TXN_OPTS);
    console.log("=================");
    console.log(`Controller redeemableCirculatingSupply         ${controllerAccount.redeemableCirculatingSupply.toNumber()}`);
}

export async function printUserBalances() {
    console.log(`\
        * [user]:
        *     BTC:                                        ${await getBalance(userBTCATA)}
        *     WSOL:                                       ${await getBalance(userWSOLATA)}
        *     UXD:                                        ${await getBalance(userUXDATA)}`);
}

export function printWorldInfo() {
    console.log(`\
        * BTC mint:                                       ${BTC.toString()}
        * WSOL mint:                                      ${WSOL.toString()}
        * UXD mint:                                       ${controllerUXD.redeemableMintPda.toString()}`);
}