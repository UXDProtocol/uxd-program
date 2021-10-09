import { TOKEN_PROGRAM_ID, Token, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { expect } from "chai";
import * as anchor from "@project-serum/anchor";
import { PublicKey, Keypair, SystemProgram, SYSVAR_RENT_PUBKEY, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { ControllerUXD } from "./utils/controller";
import { createAssocTokenIx, getBalance, TestToken, testUtils, TXN_OPTS, wallet } from "./utils/testutils";
import { Depository } from "./utils/depository";

// Constants
const BTC_DECIMAL = 6;
const SOL_DECIMAL = 9;
const UXD_DECIMAL = 6;

// The test user, can instantiate several for tests
let user: TestUser;

// Mints
let btc: TokenEnv;
let sol: TokenEnv;

// Depositories - They represent the business object that tie a mint to a depository
let depositoryBTC: Depository;
let depositorySOL: Depository;

// User's SPL Accounts
let userBTCTokenAccount: PublicKey;
let userSOLTokenAccount: PublicKey;
let userBTCDepRedeemableTokenAccount: PublicKey;
let userSOLDepRedeemableTokenAccount: PublicKey;
let userUXDTokenAccount: PublicKey;

// HELPERS
async function createTokenEnv(decimals: number, price: bigint) {
  let pythPrice = await testUtils.pyth.createPriceAccount();
  let pythProduct = await testUtils.pyth.createProductAccount();

  await testUtils.pyth.updatePriceAccount(pythPrice, {
    exponent: -9,
    aggregatePriceInfo: {
      price: price * 1000000000n,
    },
  });
  await testUtils.pyth.updateProductAccount(pythProduct, {
    priceAccount: pythPrice.publicKey,
    attributes: {
      quote_currency: "USD",
    },
  });

  return {
    token: await testUtils.createToken(decimals),
    pythPrice,
    pythProduct,
  } as TokenEnv;
}
interface TokenEnv {
  token: TestToken;
  pythPrice: Keypair;
  pythProduct: Keypair;
}

async function createTestUser(assets: Array<TokenEnv>): Promise<TestUser> {
  const userWallet = wallet.payer; //await testUtils.createWallet(1 * LAMPORTS_PER_SOL); I WISH TO use that.... but idk it doesn't sign
  // I think it would be neat to have 2 wallets to encure things are tighs, to not have only one GOD wallet that's also the user

  const createUserTokens = async (asset: TokenEnv) => {
    const tokenAccount = await asset.token.getOrCreateAssociatedAccountInfo(userWallet.publicKey);
    return tokenAccount.address;
  };

  let tokenAccounts: Record<string, PublicKey> = {};
  for (const asset of assets) {
    tokenAccounts[asset.token.publicKey.toBase58()] = await createUserTokens(asset);
  }

  return {
    wallet: userWallet,
    tokenAccounts,
  };
}
interface TestUser {
  wallet: Keypair;
  tokenAccounts: Record<string, PublicKey>;
}

export async function printUserBalance() {
  console.log(`\
        * [user]:
        *     BTC:                                        ${await getBalance(userBTCTokenAccount)}
        *     SOL:                                        ${await getBalance(userSOLTokenAccount)}
        *     redeemable from depositoryBTC:              ${await getBalance(userBTCDepRedeemableTokenAccount)}
        *     redeemable from depositorySOL:              ${await getBalance(userSOLDepRedeemableTokenAccount)}
        *     UXD:                                        ${await getBalance(userUXDTokenAccount)}`);
}

export async function printSystemBalance(depository: Depository) {
  const SYM = depository.collateralName;
  const passthroughPda = ControllerUXD.coinPassthroughPda(depository.collateralMint);
  console.log(`\
        * [depository ${depository.collateralName}]:
        *     ${SYM}:                                        ${await getBalance(depository.depositPda)}
        * [controller]
        *     associated ${SYM} passthrough:                 ${await getBalance(passthroughPda)}`);
}

// This is cool! -- use for fail tests
// await expect(
//   userA.client.deposit(
//     usdc.reserve,
//     userA.tokenAccounts[usdc.token.publicKey.toBase58()],
//     Amount.tokens(1)
//   )
// ).to.be.rejectedWith("0x142");

// UXD ADMINISTRATION SPACE starts --------------------------------------------
before("setup", async () => {
  // GIVEN
  btc = await createTokenEnv(BTC_DECIMAL, 45000n);
  sol = await createTokenEnv(SOL_DECIMAL, 180n);
  user = await createTestUser([btc, sol]);
  //
  userBTCTokenAccount = (await btc.token.getAccountInfo(user.tokenAccounts[btc.token.publicKey.toBase58()])).address;
  userSOLTokenAccount = (await sol.token.getAccountInfo(user.tokenAccounts[sol.token.publicKey.toBase58()])).address;
  const expectedUserBTCBalance = 1;
  const expectedUserSOLBalance = 15;

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

  // Unrelated setup
  // save that for later, convenience

  depositoryBTC = new Depository(btc.token, "BTC", btc.pythPrice.publicKey);
  depositorySOL = new Depository(sol.token, "SOL", sol.pythPrice.publicKey);
});

describe("UXD ADMINISTRATION SPACE - What we would do once to setup the UXD controllers and associated depositories", () => {
  it("Create UXD Controller", async () => {
    // WHEN
    await ControllerUXD.rpc.new({
      accounts: {
        authority: wallet.publicKey,
        state: ControllerUXD.statePda,
        uxdMint: ControllerUXD.mintPda,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [wallet.payer],
      options: TXN_OPTS,
    });

    // THEN
    // XXX add asserts
  });

  it("Create BTC depository", async () => {
    await Depository.rpc.new(ControllerUXD.ProgramId, {
      accounts: {
        payer: wallet.publicKey,
        state: depositoryBTC.statePda,
        redeemableMint: depositoryBTC.redeemableMintPda,
        programCoin: depositoryBTC.depositPda,
        coinMint: depositoryBTC.collateralMint.publicKey,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [wallet.payer],
      options: TXN_OPTS,
    });
    // Add some asserts ...
    depositoryBTC.info();
  });

  it("Create SOL depository (admin)", async () => {
    await Depository.rpc.new(ControllerUXD.ProgramId, {
      accounts: {
        payer: wallet.publicKey,
        state: depositorySOL.statePda,
        redeemableMint: depositorySOL.redeemableMintPda,
        programCoin: depositorySOL.depositPda,
        coinMint: depositorySOL.collateralMint.publicKey,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [wallet.payer],
      options: TXN_OPTS,
    });
    // Add some asserts ...
    depositorySOL.info();
  });

  it("Register BTC Depository with Controller", async () => {
    await ControllerUXD.rpc.registerDepository(depositoryBTC.oraclePriceAccount, {
      accounts: {
        authority: wallet.publicKey,
        state: ControllerUXD.statePda,
        depositoryRecord: ControllerUXD.depositoryRecordPda(depositoryBTC.collateralMint),
        depositoryState: depositoryBTC.statePda,
        coinMint: depositoryBTC.collateralMint.publicKey,
        coinPassthrough: ControllerUXD.coinPassthroughPda(depositoryBTC.collateralMint),
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [wallet.payer],
      options: TXN_OPTS,
    });
    // Add some asserts ...
  });

  it("Register SOL Depository with Controller", async () => {
    await ControllerUXD.rpc.registerDepository(depositorySOL.oraclePriceAccount, {
      accounts: {
        authority: wallet.publicKey,
        state: ControllerUXD.statePda,
        depositoryRecord: ControllerUXD.depositoryRecordPda(depositorySOL.collateralMint),
        depositoryState: depositorySOL.statePda,
        coinMint: depositorySOL.collateralMint.publicKey,
        coinPassthrough: ControllerUXD.coinPassthroughPda(depositorySOL.collateralMint),
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [wallet.payer],
      options: TXN_OPTS,
    });
    // Add some asserts ...
  });
});
// UXD ADMINISTRATION SPACE ends ----------------------------------------------

// USER SPACE starts ----------------------------------------------------------
before("Configure user accounts and print information before starting USER SPACE tests", async () => {
  // Find user AssocToken derived adresses (not created yet), for convenience
  userBTCDepRedeemableTokenAccount = testUtils.findAssocTokenAddressSync(wallet, depositoryBTC.redeemableMintPda)[0];
  userSOLDepRedeemableTokenAccount = testUtils.findAssocTokenAddressSync(wallet, depositorySOL.redeemableMintPda)[0];
  userUXDTokenAccount = testUtils.findAssocTokenAddressSync(wallet, ControllerUXD.mintPda)[0];

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

describe("USER SPACE - Test interacting with a Depository (BTC)", () => {
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
