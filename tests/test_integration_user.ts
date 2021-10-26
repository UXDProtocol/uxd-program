import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";
import { ControllerUXD } from "./solana-usds-client/controller";
import { user, BTC } from "./identities";
import { Depository } from "./solana-usds-client/depository";
import { controller, depositoryBTC } from "./test_integration_admin";
import { TXN_COMMIT, TXN_OPTS, provider } from "./provider";
import { findAssocTokenAddressSync } from "./solana-usds-client/utils";

// User's SPL Accounts
let userBTCTokenAccount: PublicKey;
let userUXDTokenAccount: PublicKey;

// TODO add multi users tests, how will the depository behave when managing several users.
// TODO add should fail tests

function getBalance(tokenAccount: PublicKey): Promise<number> {
  return provider.connection
    .getTokenAccountBalance(tokenAccount, TXN_COMMIT)
    .then((o) => o["value"]["uiAmount"])
    .catch(() => null);
}

async function printSystemBalance(depository: Depository) {
  const SYM = depository.collateralSymbol;
  const passthroughPda = ControllerUXD.collateralPassthroughPda(depository.collateralMint);
  const depositoryPda = ControllerUXD.depositoryPda(depository.collateralMint);
  console.log(`\
        * [controller]
        *     associated ${SYM} passthrough:                 ${await getBalance(passthroughPda)}`);
}

async function printMangoPDAInfo(depository: Depository) {
  const mangoPda = ControllerUXD.mangoPda(depository.collateralMint);
  const mangoGroup = controller.mango.group;
  const mangoCache = await mangoGroup.loadCache(controller.mango.client.connection);
  const mangoAccount = await controller.mango.client.getMangoAccount(mangoPda, controller.mango.programId);
  const mangoHealthRatio = mangoAccount.getHealthRatio(mangoGroup, mangoCache, "Maint");
  const mangoLeverage = mangoAccount.getLeverage(mangoGroup, mangoCache).toNumber();
  const assetsVal = mangoAccount.getAssetsVal(mangoGroup, mangoCache, "Maint").toNumber();
  const computeValue = mangoAccount.computeValue(mangoGroup, mangoCache).toNumber();
  const liabsValue = mangoAccount.getLiabsVal(mangoGroup, mangoCache, "Maint").toNumber();

  console.log(`\
        * [MangoPDA (for ${depository.collateralSymbol} depository)]
        *     value                                       ${computeValue}
        *     assets value                                ${assetsVal}
        *     liabilities value                           ${liabsValue}
        *     leverage                                    x${mangoLeverage}
        *     health ratio:                               ${mangoHealthRatio.toNumber()} `);
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
  userUXDTokenAccount = findAssocTokenAddressSync(user, ControllerUXD.mintPda)[0];

  console.log(`\
    * BTC mint:                           ${BTC.toString()}
    * UXD mint:                           ${ControllerUXD.mintPda.toString()}
    * ---- 
    * user's BTC tokenAcc                 ${userBTCTokenAccount.toString()}
    * user's UXD tokenAcc                 ${userUXDTokenAccount.toString()} (uninit)`);
});

describe("Test user standard interactions with a Depository (BTC)", () => {
  afterEach("[General balances info]", async () => {
    await printUserBalance();
    await printSystemBalance(depositoryBTC);
    await printMangoPDAInfo(depositoryBTC);
  });

  it("Initial balances", async () => {
    /* noop - prints after each */
  });

  it("Mint UXD worth 0.1 BTC", async () => {
    // GIVEN
    const collateralAmount = 0.1;
    const slippage = 10; // <=> 1%
    // const _expectedDepositoryBTCBalance = (await getBalance(ControllerUXD.depositoryPda(depositoryBTC.collateralMint))) + amountToConvert;

    // WHEN
    // await controller.mintUXD(provider, collateralAmount, slippage, depositoryBTC, user, TXN_OPTS);

    // Then
    // const _depositoryBTCTokenAccountBalance = await getBalance(ControllerUXD.depositoryPda(depositoryBTC.collateralMint));
    // expect(_depositoryBTCTokenAccountBalance).to.be.closeTo(_expectedDepositoryBTCBalance, 0.000_000_000_1);
    // TODO check user UXD balance
  });

  it("Redeem 200 UXD", async () => {
    // GIVEN
    const amountUXD = 200;
    const slippage = 10; // <=> 1%

    // WHEN
    await controller.redeemUXD(amountUXD, slippage, depositoryBTC, user, TXN_OPTS);
  });

  // it("Redeem all remaining UXD", async () => {
  //   // GIVEN
  //   let _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
  //   const amountUXD = _userUXDTokenAccountBalance;
  //   const slippage = 20; // <=> 1%
  //   const _expectedUserUXDBalance = 0;

  //   // WHEN
  //   await controller.redeemUXD(amountUXD, slippage, depositoryBTC, user, TXN_OPTS);

  //   // THEN
  //   _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
  //   expect(_userUXDTokenAccountBalance).to.equal(_expectedUserUXDBalance);
  // });
});
