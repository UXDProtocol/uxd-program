import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { expect } from "chai";
import * as anchor from "@project-serum/anchor";
import { SystemProgram, SYSVAR_RENT_PUBKEY, PublicKey } from "@solana/web3.js";
import { ControllerUXD } from "./utils/controller";
import { createAssocTokenIx, getBalance, provider, testUtils, TXN_OPTS, wallet } from "./utils/utils";
import { Depository } from "./utils/depository";
import { BTC_DECIMAL, createTestUser, SOL_DECIMAL, TestUser, UXD_DECIMAL } from "./utils/utils";
import { btc, depositoryBTC, depositorySOL, sol } from "./test_integration_admin";

// Identities
let user: TestUser;

// User accounts
let userBTCDepRedeemableTokenAccount: PublicKey;
let userSOLDepRedeemableTokenAccount: PublicKey;

// User's SPL Accounts
let userBTCTokenAccount: PublicKey;
let userSOLTokenAccount: PublicKey;
let userUXDTokenAccount: PublicKey;

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
        *     SOL:                                        ${await getBalance(userSOLTokenAccount)}
        *     redeemable from depositoryBTC:              ${await getBalance(userBTCDepRedeemableTokenAccount)}
        *     redeemable from depositorySOL:              ${await getBalance(userSOLDepRedeemableTokenAccount)}
        *     UXD:                                        ${await getBalance(userUXDTokenAccount)}`);
}

before("Create user identity", async () => {
  user = await createTestUser([btc, sol]);
});

before("Configure user accounts", async () => {
  // GIVEN
  const expectedUserBTCBalance = 1;
  const expectedUserSOLBalance = 15;
  // Bind every user adresses
  userBTCTokenAccount = (await btc.token.getAccountInfo(user.tokenAccounts[btc.token.publicKey.toBase58()])).address;
  userSOLTokenAccount = (await sol.token.getAccountInfo(user.tokenAccounts[sol.token.publicKey.toBase58()])).address;
  // Find user AssocToken derived adresses (not created yet), for convenience
  userBTCDepRedeemableTokenAccount = testUtils.findAssocTokenAddressSync(wallet, depositoryBTC.redeemableMintPda)[0];
  userSOLDepRedeemableTokenAccount = testUtils.findAssocTokenAddressSync(wallet, depositorySOL.redeemableMintPda)[0];
  userUXDTokenAccount = testUtils.findAssocTokenAddressSync(wallet, ControllerUXD.mintPda)[0];

  // WHEN
  await btc.token.mintTo(
    user.tokenAccounts[btc.token.publicKey.toBase58()],
    wallet.publicKey,
    [],
    1 * 10 ** BTC_DECIMAL
  );
  await sol.token.mintTo(
    user.tokenAccounts[sol.token.publicKey.toBase58()],
    wallet.publicKey,
    [],
    15 * 10 ** SOL_DECIMAL
  );

  // THEN
  const _userBTCTokenAccountBalance = await getBalance(userBTCTokenAccount);
  const _userSOLTokenAccountBalance = await getBalance(userSOLTokenAccount);

  expect(_userBTCTokenAccountBalance).to.equal(expectedUserBTCBalance);
  expect(_userSOLTokenAccountBalance).to.equal(expectedUserSOLBalance);

  console.log(`\
    * payer:                              ${wallet.publicKey.toString()}
    * ---- 
    * BTC mint:                           ${btc.token.publicKey.toString()}
    * SOL mint:                           ${sol.token.publicKey.toString()}
    * UXD mint:                           ${ControllerUXD.mintPda.toString()}
    * ----
    * BTC pyth price acc:                 ${btc.pythPrice.publicKey.toString()}
    * SOL pyth price acc:                 ${sol.pythPrice.publicKey.toString()}
    * ---- 
    * user's BTC tokenAcc                 ${userBTCTokenAccount.toString()}
    * user's SOL tokenAcc                 ${userSOLTokenAccount.toString()} 
    * user's BTCDR tokenAcc               ${userBTCDepRedeemableTokenAccount.toString()} (uninit)
    * user's SOLDR tokenAcc               ${userSOLDepRedeemableTokenAccount.toString()} (uninit)
    * user's UXD tokenAcc                 ${userUXDTokenAccount.toString()} (uninit)`);
});

describe("Test user standard interactions with a Depository (BTC)", () => {
  it("[General balances info] /\\", async () => {
    await printUserBalance();
    await printSystemBalance(depositoryBTC);
  });

  it("User Deposit BTC collateral first time", async () => {
    // Given
    const depositAmountBTC = new anchor.BN(0.45 * 10 ** BTC_DECIMAL);
    const expectedUserBTCBalance = 0.55;
    const expectedDepositoryBTCBalance = 0.45;
    const expectedRedeemableBalance = 0.45;

    // When
    // CREATE HIS account cause first time - can be offloaded in the program. I tried, but that force to have a
    // new program call like "init_user_token_acc", cause this nice idl language don't supper get_or_init behaviour yet
    // This is not tied to here actually. Should call it elsewhere with a "if not exist do it"
    const ix = createAssocTokenIx(
      user.wallet.publicKey,
      userBTCDepRedeemableTokenAccount,
      depositoryBTC.redeemableMintPda
    );

    await Depository.rpc.deposit(depositAmountBTC, {
      accounts: {
        user: user.wallet.publicKey,
        state: depositoryBTC.statePda,
        programCoin: depositoryBTC.depositPda,
        redeemableMint: depositoryBTC.redeemableMintPda,
        userCoin: userBTCTokenAccount,
        userRedeemable: userBTCDepRedeemableTokenAccount,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [user.wallet],
      options: TXN_OPTS,
      instructions: [ix],
    });

    // Then
    const _userBTCTokenAccountBalance = await getBalance(userBTCTokenAccount);
    const _depositoryBTCTokenAccountBalance = await getBalance(depositoryBTC.depositPda);
    const _userBTCDepRedeemableBalance = await getBalance(userBTCDepRedeemableTokenAccount);

    expect(_userBTCTokenAccountBalance).to.equal(expectedUserBTCBalance);
    expect(_depositoryBTCTokenAccountBalance).to.equal(expectedDepositoryBTCBalance);
    expect(_userBTCDepRedeemableBalance).to.equal(expectedRedeemableBalance);
  });

  it("[General balances info] /\\", async () => {
    await printUserBalance();
    await printSystemBalance(depositoryBTC);
  });

  it("Mint UXD (BTC depository)", async () => {
    // GIVEN
    const redeemableAmountToConvert = new anchor.BN(0.4 * 10 ** BTC_DECIMAL);
    // XXX TODO get oracle price, and substract value here -- JANKY need to think
    // const expectedUserUXDBalance = userUXDBalance; //.sub(convertCollateralAmount);// * oraclePrice);
    // const expectedDepositoryBTCBalance = depositoryBTCBalance.sub(convertCollateralAmount);

    // WHEN
    // Same as above
    const ix = createAssocTokenIx(user.wallet.publicKey, userUXDTokenAccount, ControllerUXD.mintPda);
    await ControllerUXD.rpc.mintUxd(redeemableAmountToConvert, {
      accounts: {
        user: user.wallet.publicKey,
        state: ControllerUXD.statePda,
        depositoryRecord: ControllerUXD.depositoryRecordPda(depositoryBTC.collateralMint),
        depositoryState: depositoryBTC.statePda,
        depositoryCoin: depositoryBTC.depositPda,
        coinMint: depositoryBTC.collateralMint.publicKey,
        coinPassthrough: ControllerUXD.coinPassthroughPda(depositoryBTC.collateralMint),
        redeemableMint: depositoryBTC.redeemableMintPda,
        userRedeemable: userBTCDepRedeemableTokenAccount,
        userUxd: userUXDTokenAccount,
        uxdMint: ControllerUXD.mintPda,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        depositoryProgram: Depository.ProgramId,
        oracle: depositoryBTC.oraclePriceAccount,
      },
      signers: [user.wallet],
      options: TXN_OPTS,
      instructions: [ix],
    });

    // Then
    // const _userUXDBalance = await getUserTokenBalance(controllerUXD.uxdMintPda);
    // const _depositoryBTCTokenAccount = await mintBTC.getAccountInfo(depositoryBTC.depositPda);
    // // assert(_userUXDBalance == expectedUserUXDBalance.toNumber(), "Invalid user's UXD balance");
    // assert.equal(
    //   _depositoryBTCTokenAccount.amount.toNumber(),
    //   expectedDepositoryBTCBalance.toNumber(),
    //   "Invalid depository's BTC balance"
    // );
  });

  it("[General balances info] /\\", async () => {
    await printUserBalance();
    await printSystemBalance(depositoryBTC);
  });

  it("Redeem UXD", async () => {
    let amountUXD = new anchor.BN(500 * 10 ** UXD_DECIMAL);
    await ControllerUXD.rpc.redeemUxd(amountUXD, {
      accounts: {
        user: user.wallet.publicKey,
        state: ControllerUXD.statePda,
        depositoryRecord: ControllerUXD.depositoryRecordPda(depositoryBTC.collateralMint),
        depositoryState: depositoryBTC.statePda,
        depositoryCoin: depositoryBTC.depositPda,
        coinMint: depositoryBTC.collateralMint.publicKey,
        coinPassthrough: ControllerUXD.coinPassthroughPda(depositoryBTC.collateralMint),
        redeemableMint: depositoryBTC.redeemableMintPda,
        userRedeemable: userBTCDepRedeemableTokenAccount,
        userUxd: userUXDTokenAccount,
        uxdMint: ControllerUXD.mintPda,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        depositoryProgram: Depository.ProgramId,
        associatedSystemProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        oracle: depositoryBTC.oraclePriceAccount,
      },
      signers: [user.wallet],
      options: TXN_OPTS,
    });
  });

  it("[General balances info] /\\", async () => {
    await printUserBalance();
    await printSystemBalance(depositoryBTC);
  });

  it("Withdraw UXD (all) (BTC depository)", async () => {
    // GIVEN
    // const _userUXDBalance = await getUserTokenBalance(controllerUXD.uxdMintPda);
    // const _depositoryBTCTokenAccount = await mintBTC.getAccountInfo(depositoryBTC.depositPda);
    // // XXX TODO get oracle price, and substract value here -- JANKY need to think
    // const expectedUserUXDBalance = userUXDBalance; //.sub(convertCollateralAmount);// * oraclePrice);
    // const expectedDepositoryBTCBalance = depositoryBTCBalance.sub(convertCollateralAmount);

    // WHEN
    await Depository.rpc.withdraw(null, {
      accounts: {
        user: user.wallet.publicKey,
        state: depositoryBTC.statePda,
        programCoin: depositoryBTC.depositPda,
        redeemableMint: depositoryBTC.redeemableMintPda,
        userCoin: userBTCTokenAccount,
        userRedeemable: userBTCDepRedeemableTokenAccount,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [user.wallet],
      options: TXN_OPTS,
    });
  });

  it("[General balances info] /\\", async () => {
    await printUserBalance();
    await printSystemBalance(depositoryBTC);
  });
});
// USER SPACE ends ------------------------------------------------------------
