import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";
import { Controller, createAndInitializeMango, Depository, findATAAddrSync, Mango } from "@uxdprotocol/uxd-client";
import { user, BTC, WSOL } from "./identities";
import { controllerUXD, depositoryBTC, depositoryWSOL, mintWithMangoDepository, redeemFromMangoDepository } from "./uxdApi";
import { TXN_COMMIT, TXN_OPTS, provider } from "./provider";

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

  afterEach("Balances info", async () => {
    await printUserBalances();
    await printDepositoryInfo(depositoryBTC, mango);
    console.log("\n\n");
  });

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
    // _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
    // expect(_userUXDTokenAccountBalance).to.equal(_expectedUserUXDBalance);
  });
});

// describe("Mint then redeem, a bit, then redeem all", () => {
//   afterEach("[General balances info]", async () => {
//     // seems we have unreliable result sometimes, idk if I need to update a cache or sleep or what
//     await sleep(1000);
//     // Get fresh cash and info from mango
//     await controller.mango.setupMangoGroup();
//     await printUserBalance();
//     await printSystemBalance(depositoryBTC);
//     await printMangoPDAInfo(depositoryBTC);
//     console.log("\n\n\n");
//   });

//   it("Initial balances", async () => {
//     /* noop - prints after each */
//   });

//   it("Mint UXD worth 0.001 BTC with 1% max slippage", async () => {
//     // GIVEN
//     const collateralAmount = 0.001;
//     const slippage = 10; // <=> 1%
//     // WHEN
//     await controller.mintUXD(provider, collateralAmount, slippage, depositoryBTC, user, TXN_OPTS);

//     // Then
//     // const userUXDBalance = await getBalance(userUXDTokenAccount);
//   });

//   it("Redeem 25 UXD with 1% max slippage", async () => {
//     // GIVEN
//     const amountUXD = 25;
//     const slippage = 10; // <=> 1%

//     // WHEN
//     await controller.redeemUXD(amountUXD, slippage, depositoryBTC, user, TXN_OPTS);
//   });

//   it("Redeem all remaining UXD", async () => {
//     // GIVEN
//     let _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
//     const amountUXD = _userUXDTokenAccountBalance;
//     const slippage = 10; // <=> 1%
//     const _expectedUserUXDBalance = 0;

//     console.log(`     > reedeem amount : ${amountUXD}`);

//     // WHEN
//     await controller.redeemUXD(amountUXD, slippage, depositoryBTC, user, TXN_OPTS);

//     // THEN
//     _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
//     expect(_userUXDTokenAccountBalance).to.equal(_expectedUserUXDBalance);
//   });
// });

// function sleep(ms) {
//   return new Promise((resolve) => setTimeout(resolve, ms));
// }

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
  mango.printAccountInfo(mangoAccount);
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