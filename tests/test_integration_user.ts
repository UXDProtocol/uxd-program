import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";
import { Depository, findATAAddrSync, Mango } from "@uxdprotocol/uxd-client";
import { user, BTC, WSOL } from "./identities";
import { controllerUXD, depositoryBTC, depositoryWSOL, mintWithMangoDepository, redeemFromMangoDepository, mango } from "./uxdApi";
import { TXN_COMMIT, provider } from "./provider";

// User's SPL Accounts
const userBTCTokenAccount: PublicKey = findATAAddrSync(user, BTC)[0];
const userWSOLTokenAccount: PublicKey = findATAAddrSync(user, WSOL)[0];
const userUXDTokenAccount: PublicKey = findATAAddrSync(user, controllerUXD.redeemableMintPda)[0];

before("Initial world state", async () => {
  printWorldInfo();
  await printUserBalances();
});

afterEach("", () => {
  console.log("\n=====================================\n");
});

describe("Mint then redeem all BTC", () => {

  let redeemablesLeftOver: number;

  afterEach("", async () => {
    await printUserBalances();
    await printDepositoryInfo(depositoryBTC, mango);
  });

  it("Initial state", async () => {
    redeemablesLeftOver = await getBalance(userUXDTokenAccount);
  });

  it("Mint UXD worth 0.01 BTC with 1% max slippage", async () => {
    // GIVEN
    const caller = user;
    const slippage = 10; // <=> 1%
    const collateralAmount = 0.01; // in BTC
    const controller = controllerUXD;
    const depository = depositoryBTC;

    // WHEN
    await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);

    // Then
  });

  it("Redeem all remaining UXD with 1% max slippage", async () => {
    // GIVEN
    const caller = user;
    const slippage = 10; // <=> 1%
    const amountRedeemable = (await getBalance(userUXDTokenAccount)) - redeemablesLeftOver; // In UXD
    const controller = controllerUXD;
    const depository = depositoryBTC;

    // WHEN
    await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

    // THEN
    // _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
    // expect(_userUXDTokenAccountBalance).to.equal(_expectedUserUXDBalance);
  });
});

describe("Mint then redeem all WSOL", () => {

  let redeemablesLeftOver: number;

  afterEach("", async () => {
    await printUserBalances();
    await printDepositoryInfo(depositoryWSOL, mango);
  });

  it("Initial state", async () => {
    redeemablesLeftOver = await getBalance(userUXDTokenAccount);
  });

  it("Mint UXD worth 1 WSOL with 1% max slippage", async () => {
    // GIVEN
    const caller = user;
    const slippage = 10; // <=> 1%
    const collateralAmount = 1; // in WSol
    const depository = depositoryWSOL;
    const controller = controllerUXD;

    // WHEN
    await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);

    // Then
  });

  it("Redeem all remaining UXD with 1% max slippage", async () => {
    // GIVEN
    const caller = user;
    const slippage = 10; // <=> 1%
    const amountRedeemable = (await getBalance(userUXDTokenAccount)) - redeemablesLeftOver; // In UXD
    const depository = depositoryWSOL;
    const controller = controllerUXD;

    // WHEN
    console.log(`     > reedeem amount : ${amountRedeemable}`);
    await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

    // THEN
    // _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
    // expect(_userUXDTokenAccountBalance).to.equal(_expectedUserUXDBalance);
  });
});

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function getBalance(tokenAccount: PublicKey): Promise<number> {
  return provider.connection
    .getTokenAccountBalance(tokenAccount, TXN_COMMIT)
    .then((o) => o["value"]["uiAmount"])
    .catch(() => null);
}

async function printDepositoryInfo(depository: Depository, mango: Mango) {
  // Sleep waiting for mango market update
  await sleep(3000);
  const SYM = depository.collateralMintSymbol;
  console.log(`\
        * [Depository ${SYM}]
        *     collateral_passthrough:                     ${await getBalance(depository.collateralPassthroughPda)}`);
  const mangoAccount = await mango.load(depository.mangoAccountPda); // might do that in the TS object then reload idk
  await mango.printAccountInfo(mangoAccount);
}

async function printUserBalances() {
  console.log(`\
        * [user]:
        *     BTC:                                        ${await getBalance(userBTCTokenAccount)}
        *     WSOL:                                       ${await getBalance(userWSOLTokenAccount)}
        *     UXD:                                        ${await getBalance(userUXDTokenAccount)}`);
}

function printWorldInfo() {
  console.log(`\
        * BTC mint:                                       ${BTC.toString()}
        * WSOL mint:                                      ${WSOL.toString()}
        * UXD mint:                                       ${controllerUXD.redeemableMintPda.toString()}`);
}