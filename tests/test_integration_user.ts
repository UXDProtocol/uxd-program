import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
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
  WSOL,
  BTC,
  UXD_DECIMAL,
  provider,
  createAssocTokenIx,
  BTC_DECIMAL,
  connection,
} from "./utils/utils";
import { Depository } from "./utils/depository";
import { depositoryBTC, depositoryWSOL } from "./test_integration_admin";
import { MANGO_PROGRAM_ID } from "./utils/mango";
import { ZERO_I80F48 } from "@blockworks-foundation/mango-client";

// User's SPL Accounts
let userBTCTokenAccount: PublicKey;
let userWSOLTokenAccount: PublicKey;
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
        *     SOL:                                        ${await getBalance(userWSOLTokenAccount)}
        *     UXD:                                        ${await getBalance(userUXDTokenAccount)}`);
}

before("Configure user accounts", async () => {
  // Find every user adresses
  userBTCTokenAccount = utils.findAssocTokenAddressSync(user, BTC)[0];
  userWSOLTokenAccount = utils.findAssocTokenAddressSync(user, WSOL)[0];
  userUXDTokenAccount = utils.findAssocTokenAddressSync(user, ControllerUXD.mintPda)[0];

  console.log(`\
    * payer:                              ${wallet.publicKey.toString()}
    * ---- 
    * BTC mint:                           ${BTC.toString()}
    * WSOL mint:                          ${WSOL.toString()}
    * UXD mint:                           ${ControllerUXD.mintPda.toString()}
    * ---- 
    * user's BTC tokenAcc                 ${userBTCTokenAccount.toString()}
    * user's SOL tokenAcc                 ${userWSOLTokenAccount.toString()} 
    * user's UXD tokenAcc                 ${userUXDTokenAccount.toString()} (uninit)`);
});

describe("Test user standard interactions with a Depository (BTC)", () => {
  afterEach("[General balances info]", async () => {
    await printUserBalance();
    await printSystemBalance(depositoryBTC);
    await printMangoPDAInfo(depositoryBTC);
  });

  it("Mint UXD worth 0.4 BTC", async () => {
    // GIVEN
    const amountToConvert = 0.4;
    // const _expectedDepositoryBTCBalance =
    //   (await getBalance(ControllerUXD.depositoryPda(depositoryBTC.collateralMint))) + amountToConvert;

    // WHEN
    const depositedTokenIndex = utils.mango.group.getTokenIndex(depositoryBTC.collateralMint);
    const mangoCacheAccount = utils.mango.getMangoCache();
    const mangoRootBankAccount = utils.mango.getRootBankForToken(depositedTokenIndex);
    const mangoNodeBankAccount = utils.mango.getNodeBankFor(depositedTokenIndex);
    const mangoDepositedVaultAccount = utils.mango.getVaultFor(depositedTokenIndex);
    const mangoPerpMarketConfig = utils.mango.getPerpMarketConfigFor(depositoryBTC.collateralSymbol);

    let ixs = undefined;
    if (!(await provider.connection.getAccountInfo(userUXDTokenAccount))) {
      // Create the token account for user's UXD if not exists
      ixs = [createAssocTokenIx(user.publicKey, userUXDTokenAccount, ControllerUXD.mintPda)];
    }
    let slippage = 10; // point based (1000 <=> 100%, 0.1% granularity)
    let coinAmount = new anchor.BN(amountToConvert * 10 ** BTC_DECIMAL);
    await ControllerUXD.rpc.mintUxd(coinAmount, slippage, {
      accounts: {
        user: user.publicKey,
        state: ControllerUXD.statePda,
        depository: ControllerUXD.depositoryPda(depositoryBTC.collateralMint),
        coinMint: depositoryBTC.collateralMint,
        coinPassthrough: ControllerUXD.coinPassthroughPda(depositoryBTC.collateralMint),
        userCoin: userBTCTokenAccount,
        userUxd: userUXDTokenAccount,
        uxdMint: ControllerUXD.mintPda,
        // mango stuff
        mangoGroup: utils.mango.group.publicKey,
        mangoAccount: ControllerUXD.mangoPda(depositoryBTC.collateralMint),
        mangoCache: mangoCacheAccount,
        // -- for the deposit
        mangoRootBank: mangoRootBankAccount,
        mangoNodeBank: mangoNodeBankAccount,
        mangoVault: mangoDepositedVaultAccount,
        // -- for the position perp opening
        mangoPerpMarket: mangoPerpMarketConfig.publicKey,
        mangoBids: mangoPerpMarketConfig.bidsKey,
        mangoAsks: mangoPerpMarketConfig.asksKey,
        mangoEventQueue: mangoPerpMarketConfig.eventsKey,
        //
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        mangoProgram: MANGO_PROGRAM_ID,
        //
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [user.payer],
      options: TXN_OPTS,
      instructions: ixs,
    });

    // Then
    // const _depositoryBTCTokenAccountBalance = await getBalance(
    //   ControllerUXD.depositoryPda(depositoryBTC.collateralMint)
    // );
    // expect(_depositoryBTCTokenAccountBalance).to.be.closeTo(_expectedDepositoryBTCBalance, 0.000_000_000_1);

    // TODO check that there was an order that opened the short position

    // TODO check user UXD balance
  });

  // it("Redeem 500 UXD", async () => {
  //   // GIVEN
  //   const amountUXD = 500;

  //   // WHEN
  //   await ControllerUXD.rpc.redeemUxd(new anchor.BN(amountUXD * 10 ** UXD_DECIMAL), {
  //     accounts: {
  //       user: user.publicKey,
  //       state: ControllerUXD.statePda,
  //       depository: ControllerUXD.depositoryPda(depositoryBTC.collateralMint),
  //       coinMint: depositoryBTC.collateralMint,
  //       coinPassthrough: ControllerUXD.coinPassthroughPda(depositoryBTC.collateralMint),
  //       userCoin: userBTCTokenAccount,
  //       userUxd: userUXDTokenAccount,
  //       uxdMint: ControllerUXD.mintPda,
  //       rent: SYSVAR_RENT_PUBKEY,
  //       systemProgram: SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       oracle: depositoryBTC.oraclePriceAccount,
  //     },
  //     signers: [user.payer],
  //     options: TXN_OPTS,
  //   });

  //   // THEN

  //   // TODO
  // });

  // it("Redeem all remaining UXD", async () => {
  //   // GIVEN
  //   let _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
  //   const amountUXD = _userUXDTokenAccountBalance;
  //   const _expectedUserUXDBalance = 0;

  //   // WHEN
  //   await ControllerUXD.rpc.redeemUxd(new anchor.BN(amountUXD * 10 ** UXD_DECIMAL), {
  //     accounts: {
  //       user: user.publicKey,
  //       state: ControllerUXD.statePda,
  //       depository: ControllerUXD.depositoryPda(depositoryBTC.collateralMint),
  //       coinMint: depositoryBTC.collateralMint,
  //       coinPassthrough: ControllerUXD.coinPassthroughPda(depositoryBTC.collateralMint),
  //       userCoin: userBTCTokenAccount,
  //       userUxd: userUXDTokenAccount,
  //       uxdMint: ControllerUXD.mintPda,
  //       rent: SYSVAR_RENT_PUBKEY,
  //       systemProgram: SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       oracle: depositoryBTC.oraclePriceAccount,
  //     },
  //     signers: [user.payer],
  //     options: TXN_OPTS,
  //   });

  //   // THEN
  //   _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
  //   expect(_userUXDTokenAccountBalance).to.equal(_expectedUserUXDBalance);
  // });
});
