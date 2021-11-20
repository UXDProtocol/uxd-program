import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";
import { createAndInitializeMango, Depository, findATAAddrSync, Mango } from "@uxdprotocol/uxd-client";
import { user, BTC, WSOL } from "./identities";
import { controllerUXD, depositoryBTC, depositoryWSOL, mintWithMangoDepository, redeemFromMangoDepository } from "./uxdApi";
import { TXN_COMMIT, provider } from "./provider";

// Util object to interact with Mango, dependency for MangoDepositories
let mango: Mango;

// User's SPL Accounts
let userBTCTokenAccount: PublicKey = findATAAddrSync(user, BTC)[0];
let userWSOLTokenAccount: PublicKey = findATAAddrSync(user, WSOL)[0];
let userUXDTokenAccount: PublicKey = findATAAddrSync(user, controllerUXD.mintPda)[0];

before("initialize Mango + print world state", async () => {
  mango = await createAndInitializeMango(provider, `devnet`);
  printWorldInfo();
  await printUserBalances();
});

describe("Mint then redeem all BTC", () => {

  let uxdLeftOver;

  // afterEach("Balances info", async () => {
  //   await printUserBalances();
  //   await printDepositoryInfo(depositoryBTC, mango);
  //   console.log("\n\n");
  // });

  it("Fetch initial UXD balance", async () => {
    uxdLeftOver = await getBalance(userUXDTokenAccount);
  });

  it("Mint UXD worth 0.01 BTC with 1% max slippage", async () => {
    // GIVEN
    const caller = user;
    const slippage = 10; // <=> 1%
    const collateralAmount = 0.01; // in BTC
    const depository = depositoryBTC;
    const controller = controllerUXD;

    // WHEN
    await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);

    // Then
    await printUserBalances();
    await printDepositoryInfo(depositoryBTC, mango);
    console.log("\n");
  });

  it("Redeem all remaining UXD with 1% max slippage", async () => {
    // GIVEN
    const caller = user;
    const slippage = 10; // <=> 1%
    const amountRedeemable = (await getBalance(userUXDTokenAccount)) - uxdLeftOver; // In UXD
    const depository = depositoryBTC;
    const controller = controllerUXD;

    // WHEN
    await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

    // THEN
    await printUserBalances();
    await printDepositoryInfo(depositoryBTC, mango);
    console.log("\n");
    // _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
    // expect(_userUXDTokenAccountBalance).to.equal(_expectedUserUXDBalance);
  });
});

describe("Mint then redeem all WSOL", () => {

  let uxdLeftOver;

  afterEach("Balances info", async () => {
    await printUserBalances();
    await printDepositoryInfo(depositoryWSOL, mango);
    console.log("\n\n");
  });

  it("Fetch initial UXD balances", async () => {
    uxdLeftOver = await getBalance(userUXDTokenAccount);
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
    await printUserBalances();
    await printDepositoryInfo(depositoryWSOL, mango);
    console.log("\n");
  });

  it("Redeem all remaining UXD with 1% max slippage", async () => {
    // GIVEN
    const caller = user;
    const slippage = 10; // <=> 1%
    const amountRedeemable = (await getBalance(userUXDTokenAccount)) - uxdLeftOver; // In UXD
    const depository = depositoryWSOL;
    const controller = controllerUXD;

    // WHEN
    console.log(`     > reedeem amount : ${amountRedeemable}`);
    await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

    // THEN
    await printUserBalances();
    await printDepositoryInfo(depositoryWSOL, mango);
    console.log("\n");
    // _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
    // expect(_userUXDTokenAccountBalance).to.equal(_expectedUserUXDBalance);
  });
});

function getBalance(tokenAccount: PublicKey): Promise<number> {
  return provider.connection
    .getTokenAccountBalance(tokenAccount, TXN_COMMIT)
    .then((o) => o["value"]["uiAmount"])
    .catch(() => null);
}

async function printDepositoryInfo(depository: Depository, mango: Mango) {
  const SYM = depository.collateralMintSymbol;
  console.log(`\
        * [Depository ${SYM}]
        *     collateral_passthrough:                 ${await getBalance(depository.collateralPassthroughPda)}`);
  console.log("------------------------------------------------------");
  let mangoAccount = await mango.load(depository.mangoAccountPda); // might do that in the TS object then reload idk
  await mango.printAccountInfo(mangoAccount);
  console.log("------------------------------------------------------");
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
    * BTC mint:                           ${BTC.toString()}
    * WSOL mint:                          ${WSOL.toString()}
    * UXD mint:                           ${controllerUXD.mintPda.toString()}`);
}