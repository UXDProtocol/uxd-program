import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";
import { ControllerUXD, Depository, findAssocTokenAddressSync } from "@uxdprotocol/solana-usds-client";
import { user, BTC } from "./identities";
import { controller, depositoryBTC } from "./test_integration_admin";
import { TXN_COMMIT, TXN_OPTS, provider } from "./provider";
import { printMangoPDAInfo } from "./debug_printers";

// User's SPL Accounts
let userBTCTokenAccount: PublicKey;
let userUXDTokenAccount: PublicKey;

// TODO add multi users tests, how will the depository behave when managing several users.
// TODO add should fail tests

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function getBalance(tokenAccount: PublicKey): Promise<number> {
  return provider.connection
    .getTokenAccountBalance(tokenAccount, TXN_COMMIT)
    .then((o) => o["value"]["uiAmount"])
    .catch(() => null);
}

async function printSystemBalance(depository: Depository) {
  const SYM = depository.collateralSymbol;
  const passthroughPda = controller.collateralPassthroughPda(depository.collateralMint);
  console.log(`\
        * [controller]
        *     associated ${SYM} passthrough:                 ${await getBalance(passthroughPda)}`);
}

async function printUserBalance() {
  console.log(`\
        * [user]:
        *     BTC:                                        ${await getBalance(userBTCTokenAccount)}
        *     UXD:                                        ${await getBalance(userUXDTokenAccount)}`);
}

before("Configure user accounts", async () => {
  // Find every user adresses
  userBTCTokenAccount = findAssocTokenAddressSync(user, BTC)[0];
  userUXDTokenAccount = findAssocTokenAddressSync(user, controller.mintPda)[0];

  console.log(`\
    * BTC mint:                           ${BTC.toString()}
    * UXD mint:                           ${controller.mintPda.toString()}
    * ---- 
    * user's BTC tokenAcc                 ${userBTCTokenAccount.toString()}
    * user's UXD tokenAcc                 ${userUXDTokenAccount.toString()} (uninit)`);
});

describe("Mint then redeem all", () => {
  afterEach("[General balances info]", async () => {
    // seems we have unreliable result sometimes, idk if I need to update a cache or sleep or what
    await sleep(2500);
    // Get fresh cash and info from mango
    await controller.mango.setupMangoGroup();
    await printUserBalance();
    await printSystemBalance(depositoryBTC);
    await printMangoPDAInfo(depositoryBTC, controller);
    console.log("\n\n\n");
  });

  it("Initial balances", async () => {
    /* no-op - prints after each */
  });

  it("Mint UXD worth 0.001 BTC with 1% max slippage", async () => {
    // GIVEN
    const collateralAmount = 0.001;
    const slippage = 10; // <=> 1%
    // WHEN
    await controller.mintUXD(provider, collateralAmount, slippage, depositoryBTC, user, TXN_OPTS);

    // Then
  });

  it("Redeem all remaining UXD", async () => {
    // GIVEN
    let _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
    const amountUXD = _userUXDTokenAccountBalance;
    const slippage = 10; // <=> 1%
    const _expectedUserUXDBalance = 0;

    console.log(`     > reedeem amount : ${amountUXD}`);

    // WHEN
    await controller.redeemUXD(provider, amountUXD, slippage, depositoryBTC, user, TXN_OPTS);

    // THEN
    _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
    expect(_userUXDTokenAccountBalance).to.equal(_expectedUserUXDBalance);
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
