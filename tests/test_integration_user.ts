import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";
import { ControllerUXD, Depository, findATAAddrSync } from "@uxdprotocol/solana-usds-client";
import { user, BTC, WSOL } from "./identities";
import { controllerUXD, depositoryBTC, depositoryWSOL } from "./test_integration_permissionned";
import { TXN_COMMIT, TXN_OPTS, provider } from "./provider";
import { printMangoPDAInfo } from "./debug_printers";

// User's SPL Accounts
let userBTCTokenAccount: PublicKey = findATAAddrSync(user, BTC)[0];
let userWSOLTokenAccount: PublicKey = findATAAddrSync(user, WSOL)[0];
let userUXDTokenAccount: PublicKey = findATAAddrSync(user, controllerUXD.uxdMintPda)[0];

before("World state", async () => {
  printEnvironment();
});

describe("Mint then redeem all BTC", () => {
  let uxdLeftOver;

  afterEach("[General balances info]", async () => {
    // seems we have unreliable result sometimes, idk if I need to update a cache or sleep or what
    await sleep(3000);
    // Get fresh cash and info from mango
    await controllerUXD.mango.setupMangoGroup();
    await printUserBalance();
    await printSystemBalance(depositoryBTC);
    await printMangoPDAInfo(depositoryBTC, controllerUXD);
    console.log("\n\n\n");
  });

  it("Take initial UXD balance to offset the checks", async () => {
    uxdLeftOver = await getBalance(userUXDTokenAccount);
    /* no-op */
  });

  it("Mint UXD worth 0.01 BTC with 1% max slippage", async () => {
    // GIVEN
    const collateralAmount = 0.01;
    const slippage = 10; // <=> 1%
    // WHEN
    await controllerUXD.mintUXD(collateralAmount, slippage, depositoryBTC, user, TXN_OPTS);

    // Then
  });

  it("Redeem all remaining UXD with 1% max slippage", async () => {
    // GIVEN
    let _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
    const amountUXD = _userUXDTokenAccountBalance - uxdLeftOver;
    const slippage = 10; // <=> 1%
    // const _expectedUserUXDBalance = 0;

    console.log(`     > reedeem amount : ${amountUXD}`);
    // WHEN
    await controllerUXD.redeemUXD(amountUXD, slippage, depositoryBTC, user, TXN_OPTS);

    // THEN
    // _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
    // expect(_userUXDTokenAccountBalance).to.equal(_expectedUserUXDBalance);
  });
});

describe("Mint then redeem all WSOL", () => {
  let uxdLeftOver;

  afterEach("[General balances info]", async () => {
    // seems we have unreliable result sometimes, idk if I need to update a cache or sleep or what
    await sleep(3000);
    // Get fresh cash and info from mango
    await controllerUXD.mango.setupMangoGroup();
    await printUserBalance();
    await printSystemBalance(depositoryWSOL);
    await printMangoPDAInfo(depositoryWSOL, controllerUXD);
    console.log("\n\n\n");
  });

  it("Initial balances", async () => {
    uxdLeftOver = await getBalance(userUXDTokenAccount);
    /* no-op - prints after each */
  });

  it("Mint UXD worth 1 WSOL with 1% max slippage", async () => {
    // GIVEN
    const collateralAmount = 1;
    const slippage = 10; // <=> 1%
    // WHEN
    await controllerUXD.mintUXD(collateralAmount, slippage, depositoryWSOL, user, TXN_OPTS);

    // Then
  });

  it("Redeem all remaining UXD with 1% max slippage", async () => {
    // GIVEN
    let _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
    const amountUXD = _userUXDTokenAccountBalance - uxdLeftOver;
    const slippage = 10; // <=> 1%
    // const _expectedUserUXDBalance = 0;

    console.log(`     > reedeem amount : ${amountUXD}`);

    // WHEN
    await controllerUXD.redeemUXD(amountUXD, slippage, depositoryWSOL, user, TXN_OPTS);

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

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function getBalance(tokenAccount: PublicKey): Promise<number> {
  return provider.connection
    .getTokenAccountBalance(tokenAccount, TXN_COMMIT)
    .then((o) => o["value"]["uiAmount"])
    .catch(() => null);
}

function printEnvironment() {
  console.log(`\
    * BTC mint:                           ${BTC.toString()}
    * WSOL mint:                          ${WSOL.toString()}
    * UXD mint:                           ${controllerUXD.uxdMintPda.toString()}
    * ---- 
    * user's BTC tokenAcc                 ${userBTCTokenAccount.toString()}
    * user's WSOL tokenAcc                ${userWSOLTokenAccount.toString()}
    * user's UXD tokenAcc                 ${userUXDTokenAccount.toString()}`);
}

async function printSystemBalance(depository: Depository) {
  const SYM = depository.collateralSymbol;
  const passthroughPda = controllerUXD.collateralPassthroughPda(depository.collateralMint)[0];
  console.log(`\
        * [controller]
        *     associated ${SYM} passthrough:                 ${await getBalance(passthroughPda)}`);
}

async function printUserBalance() {
  console.log(`\
        * [user]:
        *     BTC:                                        ${await getBalance(userBTCTokenAccount)}
        *     WSOL:                                       ${await getBalance(userWSOLTokenAccount)}
        *     UXD:                                        ${await getBalance(userUXDTokenAccount)}`);
}
