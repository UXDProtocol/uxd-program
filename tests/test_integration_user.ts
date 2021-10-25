import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { expect } from "chai";
import * as anchor from "@project-serum/anchor";
import { SystemProgram, SYSVAR_RENT_PUBKEY, PublicKey } from "@solana/web3.js";
import { ControllerUXD } from "./utils/controller";
import {
  getBalance,
  utils,
  TXN_OPTS,
  wallet,
  user,
  BTC,
  provider,
  createAssocTokenIx,
  BTC_DECIMALS,
} from "./utils/utils";
import { Depository } from "./utils/depository";
import { depositoryBTC } from "./test_integration_admin";
import { MANGO_PROGRAM_ID } from "./utils/mango";

// User's SPL Accounts
let userBTCTokenAccount: PublicKey;
let userUXDTokenAccount: PublicKey;

// TODO add multi users tests, how will the depository behave when managing several users.
// TODO add should fail tests

async function printSystemBalance(depository: Depository) {
  const SYM = depository.collateralSymbol;
  const passthroughPda = ControllerUXD.coinPassthroughPda(depository.collateralMint);
  const depositoryPda = ControllerUXD.depositoryPda(depository.collateralMint);
  console.log(`\
        * [depository ${depository.collateralSymbol}]:
        *     ${SYM}:                                        ${await getBalance(depositoryPda)}
        * [controller]
        *     associated ${SYM} passthrough:                 ${await getBalance(passthroughPda)}`);
}

async function printMangoPDAInfo(depository: Depository) {
  const mangoPda = ControllerUXD.mangoPda(depository.collateralMint);
  const mangoGroup = utils.mango.group;
  const mangoCache = await mangoGroup.loadCache(utils.mango.client.connection);
  const mangoAccount = await utils.mango.client.getMangoAccount(mangoPda, MANGO_PROGRAM_ID);
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
  userBTCTokenAccount = utils.findAssocTokenAddressSync(user, BTC)[0];
  userUXDTokenAccount = utils.findAssocTokenAddressSync(user, ControllerUXD.mintPda)[0];

  console.log(`\
    * payer:                              ${wallet.publicKey.toString()}
    * ---- 
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

  it("Initial balances", async () => {});

  it("Mint UXD worth 0.1 BTC", async () => {
    // GIVEN
    const collateralAmount = 0.1;
    const slippage = 10; // <=> 1%
    // const _expectedDepositoryBTCBalance = (await getBalance(ControllerUXD.depositoryPda(depositoryBTC.collateralMint))) + amountToConvert;

    // WHEN
    await ControllerUXD.mintUXD(collateralAmount, slippage, depositoryBTC, user);

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
    await ControllerUXD.redeemUXD(amountUXD, slippage, depositoryBTC, user);
  });

  // it("Redeem all remaining UXD", async () => {
  //   // GIVEN
  //   let _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
  //   const amountUXD = _userUXDTokenAccountBalance;
  //   const slippage = 10; // <=> 1%
  //   const _expectedUserUXDBalance = 0;

  //   // WHEN
  //   await ControllerUXD.redeemUXD(amountUXD, slippage, depositoryBTC, user);

  //   // THEN
  //   _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
  //   expect(_userUXDTokenAccountBalance).to.equal(_expectedUserUXDBalance);
  // });
});
