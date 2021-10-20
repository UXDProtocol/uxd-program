import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { expect } from "chai";
import * as anchor from "@project-serum/anchor";
import { SystemProgram, SYSVAR_RENT_PUBKEY, PublicKey, Account, sendAndConfirmTransaction } from "@solana/web3.js";
import { ControllerUXD } from "./utils/controller";
import {
  createAssocTokenIx,
  getBalance,
  utils,
  TXN_OPTS,
  wallet,
  user,
  WSOL,
  BTC,
  UXD_DECIMAL,
  connection,
  admin,
} from "./utils/utils";
import { Depository } from "./utils/depository";
import { BTC_DECIMAL, SOL_DECIMAL } from "./utils/utils";
import { depositoryBTC, depositoryWSOL } from "./test_integration_admin";
import {} from "@blockworks-foundation/mango-client";
import { MANGO_PROGRAM_ID } from "./utils/mango";

// User's SPL Accounts
let userBTCDepRedeemableTokenAccount: PublicKey;
let userSOLDepRedeemableTokenAccount: PublicKey;
let userBTCTokenAccount: PublicKey;
let userWSOLTokenAccount: PublicKey;
let userUXDTokenAccount: PublicKey;

// TODO add multi users tests, how will the depository behave when managing several users.
// TODO add should fail tests

async function printSystemBalance(depository: Depository) {
  const SYM = depository.collateralName;
  const passthroughPda = ControllerUXD.coinPassthroughPda(depository.collateralMint);
  console.log(`\
        * [depository ${depository.collateralName}]:
        *     ${SYM}:                                        ${await getBalance(depository.depositPda)}
        * [controller]
        *     associated ${SYM} passthrough:                 ${await getBalance(passthroughPda)}`);
}

async function printUserBalance() {
  console.log(`\
        * [user]:
        *     BTC:                                        ${await getBalance(userBTCTokenAccount)}
        *     SOL:                                        ${await getBalance(userWSOLTokenAccount)}
        *     redeemable from depositoryBTC:              ${await getBalance(userBTCDepRedeemableTokenAccount)}
        *     redeemable from depositorySOL:              ${await getBalance(userSOLDepRedeemableTokenAccount)}
        *     UXD:                                        ${await getBalance(userUXDTokenAccount)}`);
}

before("Configure user accounts", async () => {
  // Find every user adresses
  userBTCTokenAccount = utils.findAssocTokenAddressSync(user, BTC)[0];
  userWSOLTokenAccount = utils.findAssocTokenAddressSync(user, WSOL)[0];
  userBTCDepRedeemableTokenAccount = utils.findAssocTokenAddressSync(user, depositoryBTC.redeemableMintPda)[0];
  userSOLDepRedeemableTokenAccount = utils.findAssocTokenAddressSync(user, depositoryWSOL.redeemableMintPda)[0];
  userUXDTokenAccount = utils.findAssocTokenAddressSync(user, ControllerUXD.mintPda)[0];

  console.log(`\
    * payer:                              ${wallet.publicKey.toString()}
    * ---- 
    * BTC mint:                           ${BTC.toString()}
    * WSOL mint:                           ${WSOL.toString()}
    * UXD mint:                           ${ControllerUXD.mintPda.toString()}
    * ---- 
    * user's BTC tokenAcc                 ${userBTCTokenAccount.toString()}
    * user's SOL tokenAcc                 ${userWSOLTokenAccount.toString()} 
    * user's BTCDR tokenAcc               ${userBTCDepRedeemableTokenAccount.toString()} (uninit)
    * user's SOLDR tokenAcc               ${userSOLDepRedeemableTokenAccount.toString()} (uninit)
    * user's UXD tokenAcc                 ${userUXDTokenAccount.toString()} (uninit)`);
});

describe("Test user standard interactions with a Depository (BTC)", () => {
  afterEach("[General balances info]", async () => {
    await printUserBalance();
    await printSystemBalance(depositoryBTC);
  });

/*
  it("User Deposit 0.45 BTC collateral", async () => {
    // Given
    let _userBTCTokenAccountBalance = await getBalance(userBTCTokenAccount);
    const depositAmountBTC = 0.45;
    const _expectedUserBTCBalance = _userBTCTokenAccountBalance - 0.45;
    const _expectedDepositoryBTCBalance = depositAmountBTC;
    const _expectedUserRedeemableBalance = depositAmountBTC;

    // When
    const ix = createAssocTokenIx(user.publicKey, userBTCDepRedeemableTokenAccount, depositoryBTC.redeemableMintPda);

    await Depository.rpc.deposit(new anchor.BN(depositAmountBTC * 10 ** BTC_DECIMAL), {
      accounts: {
        user: user.publicKey,
        state: depositoryBTC.statePda,
        programCoin: depositoryBTC.depositPda,
        redeemableMint: depositoryBTC.redeemableMintPda,
        userCoin: userBTCTokenAccount,
        userRedeemable: userBTCDepRedeemableTokenAccount,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [user.payer],
      options: TXN_OPTS,
      instructions: [ix],
    });

    // Then
    _userBTCTokenAccountBalance = await getBalance(userBTCTokenAccount);
    const _depositoryBTCTokenAccountBalance = await getBalance(depositoryBTC.depositPda);
    const _userBTCDepRedeemableBalance = await getBalance(userBTCDepRedeemableTokenAccount);

    // Check that the balances are correct
    expect(_userBTCTokenAccountBalance).to.equal(_expectedUserBTCBalance);
    expect(_depositoryBTCTokenAccountBalance).to.equal(_expectedDepositoryBTCBalance);
    expect(_userBTCDepRedeemableBalance).to.equal(_expectedUserRedeemableBalance);
  });
*/

  it("Mint UXD worth 0.4 BTC", async () => {
    // GIVEN
    const amountToConvert = 0.4;

    // WHEN
    const depositedTokenMint = depositoryBTC.collateralMint;
    // Improve with this later
    const depositedTokenIndex = utils.mango.group.getTokenIndex(depositedTokenMint);
    const mangoCacheAccount = await utils.mango.getMangoCache();
    const mangoRootBankAccount = await utils.mango.getRootBankForToken(depositedTokenIndex);
    const mangoNodeBankAccount = await utils.mango.getNodeBankFor(depositedTokenIndex);
    const mangoDepositedVaultAccount = utils.mango.getVaultFor(depositedTokenIndex);
    console.log(`* token mint ${depositedTokenMint}`);
    console.log(`* mango group ${utils.mango.group.publicKey}`);
    console.log(`* mango acc ${depositoryBTC.mangoAccount.publicKey}`);
    console.log(`* mango cache ${mangoCacheAccount}`);
    console.log(`* rootbank ${mangoRootBankAccount}`);
    console.log(`* nodebank ${mangoNodeBankAccount}`);
    console.log(`* nodeBank vault ${mangoDepositedVaultAccount}`);

    // This is just for localnet, on dev/main net the Keeper does it?
    // await utils.mango.runKeeper();
    const updateRootBanksTx = await utils.mango.createUpdateRootBankTx();
    await sendAndConfirmTransaction(connection, updateRootBanksTx, [admin.payer], TXN_OPTS);

    const ix = createAssocTokenIx(user.publicKey, userUXDTokenAccount, ControllerUXD.mintPda);
    await ControllerUXD.rpc.mintUxd(new anchor.BN(amountToConvert * 10 ** BTC_DECIMAL), {
      accounts: {
        user: user.publicKey,
        state: ControllerUXD.statePda,
        depository: ControllerUXD.depositoryPda(depositoryBTC.collateralMint),
        coinMint: depositoryBTC.collateralMint,
        coinPassthrough: ControllerUXD.coinPassthroughPda(depositoryBTC.collateralMint),
        userCoin: userBTCTokenAccount,
        userUxd: userUXDTokenAccount,
        uxdMint: ControllerUXD.mintPda,
        oracle: depositoryBTC.oraclePriceAccount,
        // mango stuff
        mangoGroup: utils.mango.group.publicKey,
        mangoAccount: depositoryBTC.mangoAccount.publicKey,
        mangoCache: mangoCacheAccount,
        mangoRootBank: mangoRootBankAccount,
        mangoNodeBank: mangoNodeBankAccount,
        mangoVault: mangoDepositedVaultAccount,
        //
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        mangoProgram: MANGO_PROGRAM_ID,
        //
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [user.payer],
      options: TXN_OPTS,
      instructions: [ix],
    });

    // Then

    // TODO check user UXD balance
    // TODO check mango account balance grew
  });

  it("Redeem 500 UXD", async () => {
    // GIVEN
    const amountUXD = 500;

    // WHEN
    await ControllerUXD.rpc.redeemUxd(new anchor.BN(amountUXD * 10 ** UXD_DECIMAL), {
      accounts: {
        user: user.publicKey,
        state: ControllerUXD.statePda,
        depository: ControllerUXD.depositoryPda(depositoryBTC.collateralMint),
        coinMint: depositoryBTC.collateralMint,
        coinPassthrough: ControllerUXD.coinPassthroughPda(depositoryBTC.collateralMint),
        userCoin: userBTCTokenAccount,
        userUxd: userUXDTokenAccount,
        uxdMint: ControllerUXD.mintPda,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        oracle: depositoryBTC.oraclePriceAccount,
      },
      signers: [user.payer],
      options: TXN_OPTS,
    });

    // THEN

    // TODO
  });

  // it("Withdraw 0.02 BTC from depository", async () => {
  //   // GIVEN
  //   const amountBTC = 0.02;
  //   let _userBTCTokenAccountBalance = await getBalance(userBTCTokenAccount);
  //   let _depositoryBTCTokenAccountBalance = await getBalance(depositoryBTC.depositPda);
  //   const _expectedUserBTCBalance = _userBTCTokenAccountBalance + amountBTC;
  //   const _expectedDepositoryBTCBalance = _depositoryBTCTokenAccountBalance - amountBTC;

  //   // WHEN
  //   await Depository.rpc.withdraw(new anchor.BN(amountBTC * 10 ** BTC_DECIMAL), {
  //     accounts: {
  //       user: user.publicKey,
  //       state: depositoryBTC.statePda,
  //       programCoin: depositoryBTC.depositPda,
  //       redeemableMint: depositoryBTC.redeemableMintPda,
  //       userCoin: userBTCTokenAccount,
  //       userRedeemable: userBTCDepRedeemableTokenAccount,
  //       systemProgram: SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //     },
  //     signers: [user],
  //     options: TXN_OPTS,
  //   });

  //   // THEN
  //   _userBTCTokenAccountBalance = await getBalance(userBTCTokenAccount);
  //   _depositoryBTCTokenAccountBalance = await getBalance(depositoryBTC.depositPda);
  //   expect(_userBTCTokenAccountBalance).to.closeTo(_expectedUserBTCBalance, 0.000_000_000_1);
  //   expect(_depositoryBTCTokenAccountBalance).to.closeTo(_expectedDepositoryBTCBalance, 0.000_000_000_1);
  // });

  // it("Withdraw 0.01 BTC from depository", async () => {
  //   // GIVEN
  //   const amountBTC = 0.01; // <=> to withdraw all
  //   let _userBTCTokenAccountBalance = await getBalance(userBTCTokenAccount);
  //   let _depositoryBTCTokenAccountBalance = await getBalance(depositoryBTC.depositPda);
  //   const _expectedUserBTCBalance = _userBTCTokenAccountBalance + amountBTC;
  //   const _expectedDepositoryBTCBalance = _depositoryBTCTokenAccountBalance - amountBTC;

  //   // WHEN
  //   await Depository.rpc.withdraw(new anchor.BN(amountBTC * 10 ** BTC_DECIMAL), {
  //     accounts: {
  //       user: user.publicKey,
  //       state: depositoryBTC.statePda,
  //       programCoin: depositoryBTC.depositPda,
  //       redeemableMint: depositoryBTC.redeemableMintPda,
  //       userCoin: userBTCTokenAccount,
  //       userRedeemable: userBTCDepRedeemableTokenAccount,
  //       systemProgram: SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //     },
  //     signers: [user],
  //     options: TXN_OPTS,
  //   });

  //   // THEN
  //   _userBTCTokenAccountBalance = await getBalance(userBTCTokenAccount);
  //   _depositoryBTCTokenAccountBalance = await getBalance(depositoryBTC.depositPda);
  //   expect(_userBTCTokenAccountBalance).to.closeTo(_expectedUserBTCBalance, 0.000_000_000_1);
  //   expect(_depositoryBTCTokenAccountBalance).to.closeTo(_expectedDepositoryBTCBalance, 0.000_000_000_1);
  // });

  it("Redeem all remaining UXD", async () => {
    // GIVEN
    let _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
    const amountUXD = _userUXDTokenAccountBalance;
    const _expectedUserUXDBalance = 0;

    // WHEN
    await ControllerUXD.rpc.redeemUxd(new anchor.BN(amountUXD * 10 ** UXD_DECIMAL), {
      accounts: {
        user: user.publicKey,
        state: ControllerUXD.statePda,
        depository: ControllerUXD.depositoryPda(depositoryBTC.collateralMint),
        coinMint: depositoryBTC.collateralMint,
        coinPassthrough: ControllerUXD.coinPassthroughPda(depositoryBTC.collateralMint),
        userCoin: userBTCTokenAccount,
        userUxd: userUXDTokenAccount,
        uxdMint: ControllerUXD.mintPda,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        oracle: depositoryBTC.oraclePriceAccount,
      },
      signers: [user.payer],
      options: TXN_OPTS,
    });

    // THEN
    _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
    expect(_userUXDTokenAccountBalance).to.equal(_expectedUserUXDBalance);
  });

  // it("Withdraw BTC (all) from depository", async () => {
  //   // GIVEN
  //   const amountBTC = null; // <=> to withdraw all
  //   // let _userBTCTokenAccountBalance = await getBalance(userUXDTokenAccount);
  //   // let _depositoryBTCTokenAccountBalance = await getBalance(depositoryBTC.depositPda);
  //   // const _expectedUserBTCBalance = _userBTCTokenAccountBalance + ;
  //   // const _expectedDepositoryBTCBalance = _depositoryBTCTokenAccountBalance - amountBTC.toNumber();

  //   // WHEN
  //   await Depository.rpc.withdraw(amountBTC, {
  //     accounts: {
  //       user: user.publicKey,
  //       state: depositoryBTC.statePda,
  //       programCoin: depositoryBTC.depositPda,
  //       redeemableMint: depositoryBTC.redeemableMintPda,
  //       userCoin: userBTCTokenAccount,
  //       userRedeemable: userBTCDepRedeemableTokenAccount,
  //       systemProgram: SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //     },
  //     signers: [user],
  //     options: TXN_OPTS,
  //   });
  // });
});
