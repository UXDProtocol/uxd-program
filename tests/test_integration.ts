import { TOKEN_PROGRAM_ID, Token, MintLayout, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { assert } from "chai";
import * as anchor from "@project-serum/anchor";
import { PublicKey, Keypair, SystemProgram, SYSVAR_RENT_PUBKEY, Transaction } from "@solana/web3.js";

import {
  BTC_USD,
  create_localnet_oracle_mirrored,
  localBTCOraclePriceAccountKey,
  localSOLOraclePriceAccountKey,
  mainnetBTCOraclePriceAccountKey,
  mainnetSOLOraclePriceAccountKey,
  oracle,
  SOL_USD,
} from "./oracle_utils";
import { Controller } from "./controller_utils";
import {
  connection,
  createAssocTokenIx,
  findAssocTokenAddr,
  getUserTokenBalance,
  MAINNET,
  printBalances,
  printDepositoryInfo,
  TXN_COMMIT,
  TXN_OPTS,
  wallet,
} from "./utils";
import { Depository } from "./depository_utils";

// Constants
const BTC_DECIMAL = 6;
const SOL_DECIMAL = 9;
const UXD_DECIMAL = 6;

// Keypairs
let payer: Keypair;
let mintAuthority: Keypair;

// Mints
let mintBTC: Token;
let mintSOL: Token;

// Depositories - They represent the business object that tie a mint to a depository
let depositoryBTC: Depository;
let depositorySOL: Depository;
// Controller
let controllerUXD: Controller;

// Accounts
let userBTCTokenAccount: PublicKey;
let userSOLTokenAccount: PublicKey;
let userBTCDepositoryRedeemableTokenAccount: PublicKey;
let userSOLDepositoryRedeemableTokenAccount: PublicKey;
let userUXDTokenAccount: PublicKey;

// For asserts, a bit janky. Better idea welcome
let userBTCBalance: anchor.BN;
let userSOLBalance: anchor.BN;
let userUXDBalance: anchor.BN;
let depositoryBTCBalance: anchor.BN;
let depositorySOLBalance: anchor.BN;

// Setup Mint Redeem flow with BTC and SOL
///////////////////////////////////////////////////////////////////////////////
describe("UXD full flow with BTC and SOL collaterals", () => {
  it("Setup - Payer", async () => {
    payer = anchor.web3.Keypair.generate();
    const solAidropAmountForPayer = 100 * 10 ** SOL_DECIMAL;
    // Airdropping tokens to the payer.
    await connection.confirmTransaction(
      await connection.requestAirdrop(payer.publicKey, solAidropAmountForPayer),
      "confirmed"
    );
  });

  it("Setup - Create mints and accounts", async () => {
    mintAuthority = anchor.web3.Keypair.generate();
    // Setup BTC mint
    mintBTC = await Token.createMint(connection, payer, mintAuthority.publicKey, null, BTC_DECIMAL, TOKEN_PROGRAM_ID);
    // Setup SOL mint
    mintSOL = await Token.createMint(connection, payer, mintAuthority.publicKey, null, SOL_DECIMAL, TOKEN_PROGRAM_ID);
    // create token accounts
    userBTCTokenAccount = await mintBTC.createAccount(wallet.publicKey);
    userSOLTokenAccount = await mintSOL.createAccount(wallet.publicKey);
  });

  it("Setup - Airdrops", async () => {
    // GIVEN

    // WHEN
    // For asserts, a bit janky. Better idea welcome
    userBTCBalance = new anchor.BN(100 * 10 ** BTC_DECIMAL);
    userSOLBalance = new anchor.BN(100 * 10 ** SOL_DECIMAL);
    userUXDBalance = new anchor.BN(0);
    depositoryBTCBalance = new anchor.BN(0);
    depositorySOLBalance = new anchor.BN(0);

    // mint tokens
    await mintBTC.mintTo(userBTCTokenAccount, mintAuthority.publicKey, [mintAuthority], userBTCBalance.toNumber());
    await mintSOL.mintTo(userSOLTokenAccount, mintAuthority.publicKey, [mintAuthority], userSOLBalance.toNumber());

    // THEN
    const _userBTCTokenAccount = await mintBTC.getAccountInfo(userBTCTokenAccount);
    const _userSOLTokenAccount = await mintSOL.getAccountInfo(userSOLTokenAccount);

    assert.equal(_userBTCTokenAccount.amount.toNumber(), userBTCBalance.toNumber(), "Invalid BTC amount minted");
    assert.equal(_userSOLTokenAccount.amount.toNumber(), userSOLBalance.toNumber(), "Invalid SOL amount minted");
  });

  it("Setup - Oracles", async () => {
    // BTC
    await create_localnet_oracle_mirrored(
      BTC_USD,
      MAINNET,
      mainnetBTCOraclePriceAccountKey,
      localBTCOraclePriceAccountKey
    );

    // SOL
    await create_localnet_oracle_mirrored(
      SOL_USD,
      MAINNET,
      mainnetSOLOraclePriceAccountKey,
      localSOLOraclePriceAccountKey
    );
  });

  it("Setup - Depositories (BTC and SOL)", async () => {
    depositoryBTC = new Depository(mintBTC, "BTC", localBTCOraclePriceAccountKey);

    depositorySOL = new Depository(mintSOL, "SOL", localSOLOraclePriceAccountKey);

    // Find user AssocToken derived adresses (not created yet)
    userBTCDepositoryRedeemableTokenAccount = findAssocTokenAddr(wallet.publicKey, depositoryBTC.redeemableMintPda);
    userSOLDepositoryRedeemableTokenAccount = findAssocTokenAddr(wallet.publicKey, depositorySOL.redeemableMintPda);
  });

  it("Initializing controller", async () => {
    // GIVEN
    controllerUXD = new Controller();

    // WHEN
    await controllerUXD.program.rpc.new({
      accounts: {
        authority: wallet.publicKey,
        state: controllerUXD.statePda,
        uxdMint: controllerUXD.uxdMintPda,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [wallet.payer], // Payer does exist, just a problem of discovery?
      options: TXN_OPTS,
    });

    // Find user AssocToken derived adresses (not created yet)
    userUXDTokenAccount = findAssocTokenAddr(wallet.publicKey, controllerUXD.uxdMintPda);

    // THEN
    // XXX add asserts
    console.log(`\
        * payer:                              ${wallet.publicKey.toString()}
        * BTC mint:                           ${mintBTC.publicKey.toString()}
        * SOL mint:                           ${mintSOL.publicKey.toString()}
        * UXD mint:                           ${controllerUXD.uxdMintPda.toString()}
        * user's BTC tokenAcc                 ${userBTCTokenAccount.toString()}
        * user's SOL tokenAcc                 ${userSOLTokenAccount.toString()}
        * ----  (Below 3 not created yet)
        * user's BTCDR tokenAcc               ${userBTCDepositoryRedeemableTokenAccount.toString()}
        * user's SOLDR tokenAcc               ${userSOLDepositoryRedeemableTokenAccount.toString()}
        * user's UXD tokenAcc                 ${userUXDTokenAccount.toString()}
        * ----
        * Controller program:                 ${Controller.ProgramId}
        *     statePda:                       ${controllerUXD.statePda}
        * Depository program:                 ${Depository.ProgramId}`);
  });

  it("Create BTC depository", async () => {
    await depositoryBTC.program.rpc.new(Controller.ProgramId, {
      accounts: {
        payer: wallet.publicKey,
        state: depositoryBTC.statePda,
        redeemableMint: depositoryBTC.redeemableMintPda,
        programCoin: depositoryBTC.depositPda,
        coinMint: depositoryBTC.mint.publicKey,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [wallet.payer],
      options: TXN_OPTS,
    });
    // Add some asserts ...
    printDepositoryInfo(depositoryBTC, controllerUXD);
  });

  it("Create SOL depository", async () => {
    await depositorySOL.program.rpc.new(Controller.ProgramId, {
      accounts: {
        payer: wallet.publicKey,
        state: depositorySOL.statePda,
        redeemableMint: depositorySOL.redeemableMintPda,
        programCoin: depositorySOL.depositPda,
        coinMint: depositorySOL.mint.publicKey,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [wallet.payer],
      options: TXN_OPTS,
    });
    // Add some asserts ...

    printDepositoryInfo(depositorySOL, controllerUXD);
  });

  it("Register BTC depository with Controller", async () => {
    await controllerUXD.program.rpc.registerDepository(Depository.ProgramId, depositoryBTC.oraclePriceAccount, {
      accounts: {
        authority: wallet.publicKey,
        state: controllerUXD.statePda,
        depositoryRecord: controllerUXD.depositoryRecordPda(depositoryBTC),
        depositoryState: depositoryBTC.statePda,
        coinMint: depositoryBTC.mint.publicKey,
        coinPassthrough: controllerUXD.coinPassthroughPda(depositoryBTC),
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [wallet.payer],
      options: TXN_OPTS,
    });
    // Add some asserts ...
  });

  it("Register SOL depository with Controller", async () => {
    await controllerUXD.program.rpc.registerDepository(Depository.ProgramId, depositorySOL.oraclePriceAccount, {
      accounts: {
        authority: wallet.publicKey,
        state: controllerUXD.statePda,
        depositoryRecord: controllerUXD.depositoryRecordPda(depositorySOL),
        depositoryState: depositorySOL.statePda,
        coinMint: depositorySOL.mint.publicKey,
        coinPassthrough: controllerUXD.coinPassthroughPda(depositorySOL),
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [wallet.payer],
      options: TXN_OPTS,
    });
    // Add some asserts ...
  });

  it("Balances before deposit /\\", async () => {
    await printBalances(depositoryBTC, controllerUXD, wallet);
  });

  it("Deposit BTC (BTC depository)", async () => {
    // Given
    const depositAmountBTC = new anchor.BN(1 * 10 ** BTC_DECIMAL);
    const expectedUserBTCBalance = userBTCBalance.sub(depositAmountBTC);
    const expectedDepositoryBTCBalance = depositoryBTCBalance.add(depositAmountBTC);

    // When
    const ix = createAssocTokenIx(wallet, userBTCDepositoryRedeemableTokenAccount, depositoryBTC.redeemableMintPda);
    const dsig = await depositoryBTC.program.rpc.deposit(depositAmountBTC, {
      accounts: {
        user: wallet.publicKey,
        state: depositoryBTC.statePda,
        programCoin: depositoryBTC.depositPda,
        redeemableMint: depositoryBTC.redeemableMintPda,
        userCoin: userBTCTokenAccount,
        userRedeemable: userBTCDepositoryRedeemableTokenAccount,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [wallet.payer],
      options: TXN_OPTS,
      instructions: [ix],
    });

    // Then
    const _userBTCTokenAccount = await mintBTC.getAccountInfo(userBTCTokenAccount);
    const _depositoryBTCTokenAccount = await mintBTC.getAccountInfo(depositoryBTC.depositPda);
    assert(_userBTCTokenAccount.amount.eq(expectedUserBTCBalance), "Invalid user's BTC balance");
    assert(_depositoryBTCTokenAccount.amount.eq(expectedDepositoryBTCBalance), "Invalid depository's BTC balance");
  });

  it("Balances after deposit /\\", async () => {
    await printBalances(depositoryBTC, controllerUXD, wallet);
  });

  it("Mint UXD (BTC depository)", async () => {
    // GIVEN
    const convertCollateralAmount = new anchor.BN(1 * 10 ** BTC_DECIMAL);
    // XXX TODO get oracle price, and substract value here -- JANKY need to think
    // const expectedUserUXDBalance = userUXDBalance; //.sub(convertCollateralAmount);// * oraclePrice);
    // const expectedDepositoryBTCBalance = depositoryBTCBalance.sub(convertCollateralAmount);

    // WHEN
    const ix = createAssocTokenIx(wallet, userUXDTokenAccount, controllerUXD.uxdMintPda);
    let msig = await controllerUXD.program.rpc.mintUxd(convertCollateralAmount, {
      accounts: {
        user: wallet.publicKey,
        state: controllerUXD.statePda,
        depository: Depository.ProgramId,
        depositoryRecord: controllerUXD.depositoryRecordPda(depositoryBTC),
        depositoryState: depositoryBTC.statePda,
        depositoryCoin: depositoryBTC.depositPda,
        coinMint: depositoryBTC.mint.publicKey,
        coinPassthrough: controllerUXD.coinPassthroughPda(depositoryBTC),
        redeemableMint: depositoryBTC.redeemableMintPda,
        userRedeemable: userBTCDepositoryRedeemableTokenAccount,
        userUxd: userUXDTokenAccount,
        uxdMint: controllerUXD.uxdMintPda,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        oracle: depositoryBTC.oraclePriceAccount,
      },
      signers: [wallet.payer],
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

  it("Balances after mint /\\", async () => {
    await printBalances(depositoryBTC, controllerUXD, wallet);
  });

  it("Withdraw UXD (all) (BTC depository)", async () => {
    // GIVEN
    // const _userUXDBalance = await getUserTokenBalance(controllerUXD.uxdMintPda);
    // const _depositoryBTCTokenAccount = await mintBTC.getAccountInfo(depositoryBTC.depositPda);
    // // XXX TODO get oracle price, and substract value here -- JANKY need to think
    // const expectedUserUXDBalance = userUXDBalance; //.sub(convertCollateralAmount);// * oraclePrice);
    // const expectedDepositoryBTCBalance = depositoryBTCBalance.sub(convertCollateralAmount);

    // WHEN
    let wsig = await depositoryBTC.program.rpc.withdraw(null, {
      accounts: {
        user: wallet.publicKey,
        state: depositoryBTC.statePda,
        programCoin: depositoryBTC.depositPda,
        redeemableMint: depositoryBTC.redeemableMintPda,
        userCoin: userBTCTokenAccount,
        userRedeemable: userBTCDepositoryRedeemableTokenAccount,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [wallet.payer],
      options: TXN_OPTS,
    });
  });

  it("Balances after mint /\\", async () => {
    await printBalances(depositoryBTC, controllerUXD, wallet);
  });

  // XXX to work on. Split BTC and SOL. Write

  // it("", async () => {

  //   // Add some asserts ...
  // });

  // it("", async () => {

  //   // Add some asserts ...
  // });

  // it("", async () => {

  //   // Add some asserts ...
  // });
});
