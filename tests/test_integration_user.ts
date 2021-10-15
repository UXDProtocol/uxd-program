import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { expect } from "chai";
import * as anchor from "@project-serum/anchor";
import { SystemProgram, SYSVAR_RENT_PUBKEY, PublicKey } from "@solana/web3.js";
import { ControllerUXD } from "./utils/controller";
import { createAssocTokenIx, getBalance, utils, TXN_OPTS, wallet, connection } from "./utils/utils";
import { Depository } from "./utils/depository";
import { BTC_DECIMAL, createTestUser, SOL_DECIMAL, TestUser, UXD_DECIMAL } from "./utils/utils";
import { btc, depositoryBTC, depositorySOL, sol } from "./test_integration_admin";
import {} from "@blockworks-foundation/mango-client";
import { MANGO_PROGRAM_ID } from "./utils/mango";

// Identities
let user: TestUser;

// User accounts
let userBTCDepRedeemableTokenAccount: PublicKey;
let userSOLDepRedeemableTokenAccount: PublicKey;

// User's SPL Accounts
let userBTCTokenAccount: PublicKey;
let userSOLTokenAccount: PublicKey;
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
  userBTCDepRedeemableTokenAccount = utils.findAssocTokenAddressSync(wallet, depositoryBTC.redeemableMintPda)[0];
  userSOLDepRedeemableTokenAccount = utils.findAssocTokenAddressSync(wallet, depositorySOL.redeemableMintPda)[0];
  userUXDTokenAccount = utils.findAssocTokenAddressSync(wallet, ControllerUXD.mintPda)[0];

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
  afterEach("[General balances info]", async () => {
    await printUserBalance();
    await printSystemBalance(depositoryBTC);
  });

  it("User Deposit 0.45 BTC collateral", async () => {
    // Given
    let _userBTCTokenAccountBalance = await getBalance(userBTCTokenAccount);
    const depositAmountBTC = 0.45;
    const _expectedUserBTCBalance = _userBTCTokenAccountBalance - 0.45;
    const _expectedDepositoryBTCBalance = depositAmountBTC;
    const _expectedUserRedeemableBalance = depositAmountBTC;

    // When
    const ix = createAssocTokenIx(
      user.wallet.publicKey,
      userBTCDepRedeemableTokenAccount,
      depositoryBTC.redeemableMintPda
    );

    await Depository.rpc.deposit(new anchor.BN(depositAmountBTC * 10 ** BTC_DECIMAL), {
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
    _userBTCTokenAccountBalance = await getBalance(userBTCTokenAccount);
    const _depositoryBTCTokenAccountBalance = await getBalance(depositoryBTC.depositPda);
    const _userBTCDepRedeemableBalance = await getBalance(userBTCDepRedeemableTokenAccount);

    // Check that the balances are correct
    expect(_userBTCTokenAccountBalance).to.equal(_expectedUserBTCBalance);
    expect(_depositoryBTCTokenAccountBalance).to.equal(_expectedDepositoryBTCBalance);
    expect(_userBTCDepRedeemableBalance).to.equal(_expectedUserRedeemableBalance);
  });

  it("Mint UXD worth 0.4 BTC", async () => {
    // GIVEN
    const redeemableAmountToConvert = 0.4;
    let _depositoryBTCTokenAccountBalance = await getBalance(depositoryBTC.depositPda);
    const _expectedDepositoryBTCBalance = _depositoryBTCTokenAccountBalance - redeemableAmountToConvert;

    // WHEN
    const depositedTokenMint = depositoryBTC.collateralMint.publicKey;
    console.log(`* token mint ${depositedTokenMint}`);
    console.log(`* mango group ${utils.mango.group.publicKey}`);
    console.log(`* mango acc ${depositoryBTC.mangoAccount.publicKey}`);
    console.log(`* mango cache ${utils.mango.group.mangoCache}`);
    const rootBank = await utils.mango.getRootBankForToken(depositedTokenMint);
    const nodeBank = await utils.mango.getNodeBankFor(depositedTokenMint, rootBank);
    console.log(`* rootbank ${rootBank.publicKey}`);
    console.log(`* nodebank ${nodeBank.publicKey}`);
    console.log(`* vault ${nodeBank.vault}`);

    const ix = createAssocTokenIx(user.wallet.publicKey, userUXDTokenAccount, ControllerUXD.mintPda);
    await ControllerUXD.rpc.mintUxd(new anchor.BN(redeemableAmountToConvert * 10 ** BTC_DECIMAL), {
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
        oracle: depositoryBTC.oraclePriceAccount,
        // mango stuff
        mangoGroup: utils.mango.group.publicKey,
        mangoAccount: depositoryBTC.mangoAccount.publicKey,
        mangoCache: utils.mango.group.mangoCache,
        mangoRootBank: rootBank.publicKey,
        mangoNodeBank: nodeBank.publicKey,
        mangoVault: nodeBank.vault,
        //
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        depositoryProgram: Depository.ProgramId,
        mango_program: MANGO_PROGRAM_ID,
        //
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [user.wallet],
      options: TXN_OPTS,
      instructions: [ix],
    });

    // Then
    _depositoryBTCTokenAccountBalance = await getBalance(depositoryBTC.depositPda);
    expect(_depositoryBTCTokenAccountBalance).to.closeTo(_expectedDepositoryBTCBalance, 0.000_000_000_1);

    // TODO check user UXD balance
  });

  // it("Redeem 500 UXD", async () => {
  //   // GIVEN
  //   const amountUXD = 500;
  //   let _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
  //   const _expectedUserUXDBalance = _userUXDTokenAccountBalance - amountUXD;

  //   // WHEN
  //   await ControllerUXD.rpc.redeemUxd(new anchor.BN(amountUXD * 10 ** UXD_DECIMAL), {
  //     accounts: {
  //       user: user.wallet.publicKey,
  //       state: ControllerUXD.statePda,
  //       depositoryRecord: ControllerUXD.depositoryRecordPda(depositoryBTC.collateralMint),
  //       depositoryState: depositoryBTC.statePda,
  //       depositoryCoin: depositoryBTC.depositPda,
  //       coinMint: depositoryBTC.collateralMint.publicKey,
  //       coinPassthrough: ControllerUXD.coinPassthroughPda(depositoryBTC.collateralMint),
  //       redeemableMint: depositoryBTC.redeemableMintPda,
  //       userRedeemable: userBTCDepRedeemableTokenAccount,
  //       userUxd: userUXDTokenAccount,
  //       uxdMint: ControllerUXD.mintPda,
  //       rent: SYSVAR_RENT_PUBKEY,
  //       systemProgram: SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       depositoryProgram: Depository.ProgramId,
  //       mangoProgram: MANGO_PROGRAM_ID,
  //       associatedSystemProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //       oracle: depositoryBTC.oraclePriceAccount,
  //     },
  //     signers: [user.wallet],
  //     options: TXN_OPTS,
  //   });

  //   // THEN
  //   _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
  //   expect(_userUXDTokenAccountBalance).to.closeTo(_expectedUserUXDBalance, 0.000_000_000_1);
  // });

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
  //       user: user.wallet.publicKey,
  //       state: depositoryBTC.statePda,
  //       programCoin: depositoryBTC.depositPda,
  //       redeemableMint: depositoryBTC.redeemableMintPda,
  //       userCoin: userBTCTokenAccount,
  //       userRedeemable: userBTCDepRedeemableTokenAccount,
  //       systemProgram: SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //     },
  //     signers: [user.wallet],
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
  //       user: user.wallet.publicKey,
  //       state: depositoryBTC.statePda,
  //       programCoin: depositoryBTC.depositPda,
  //       redeemableMint: depositoryBTC.redeemableMintPda,
  //       userCoin: userBTCTokenAccount,
  //       userRedeemable: userBTCDepRedeemableTokenAccount,
  //       systemProgram: SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //     },
  //     signers: [user.wallet],
  //     options: TXN_OPTS,
  //   });

  //   // THEN
  //   _userBTCTokenAccountBalance = await getBalance(userBTCTokenAccount);
  //   _depositoryBTCTokenAccountBalance = await getBalance(depositoryBTC.depositPda);
  //   expect(_userBTCTokenAccountBalance).to.closeTo(_expectedUserBTCBalance, 0.000_000_000_1);
  //   expect(_depositoryBTCTokenAccountBalance).to.closeTo(_expectedDepositoryBTCBalance, 0.000_000_000_1);
  // });

  // it("Redeem all remaining UXD", async () => {
  //   // GIVEN
  //   let _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
  //   const amountUXD = _userUXDTokenAccountBalance;
  //   const _expectedUserUXDBalance = 0;

  //   // WHEN
  //   await ControllerUXD.rpc.redeemUxd(new anchor.BN(amountUXD * 10 ** UXD_DECIMAL), {
  //     accounts: {
  //       user: user.wallet.publicKey,
  //       state: ControllerUXD.statePda,
  //       depositoryRecord: ControllerUXD.depositoryRecordPda(depositoryBTC.collateralMint),
  //       depositoryState: depositoryBTC.statePda,
  //       depositoryCoin: depositoryBTC.depositPda,
  //       coinMint: depositoryBTC.collateralMint.publicKey,
  //       coinPassthrough: ControllerUXD.coinPassthroughPda(depositoryBTC.collateralMint),
  //       redeemableMint: depositoryBTC.redeemableMintPda,
  //       userRedeemable: userBTCDepRedeemableTokenAccount,
  //       userUxd: userUXDTokenAccount,
  //       uxdMint: ControllerUXD.mintPda,
  //       rent: SYSVAR_RENT_PUBKEY,
  //       systemProgram: SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       depositoryProgram: Depository.ProgramId,
  //       mangoProgram: MANGO_PROGRAM_ID,
  //       associatedSystemProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //       oracle: depositoryBTC.oraclePriceAccount,
  //     },
  //     signers: [user.wallet],
  //     options: TXN_OPTS,
  //   });

  //   // THEN
  //   _userUXDTokenAccountBalance = await getBalance(userUXDTokenAccount);
  //   expect(_userUXDTokenAccountBalance).to.equal(_expectedUserUXDBalance);
  // });

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
  //       user: user.wallet.publicKey,
  //       state: depositoryBTC.statePda,
  //       programCoin: depositoryBTC.depositPda,
  //       redeemableMint: depositoryBTC.redeemableMintPda,
  //       userCoin: userBTCTokenAccount,
  //       userRedeemable: userBTCDepRedeemableTokenAccount,
  //       systemProgram: SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //     },
  //     signers: [user.wallet],
  //     options: TXN_OPTS,
  //   });
  // });
});
