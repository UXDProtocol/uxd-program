"use strict";

const anchor = require("@project-serum/anchor");
const spl = require("@solana/spl-token");

const FAKE_BTC_MINT = process.argv[2];
if(!FAKE_BTC_MINT) throw "need fake btc mint";

const BTC_DECIMAL = 6;
const SOL_DECIMAL = 9;
const UXD_DECIMAL = 6;

const TXN_COMMIT = "processed";
const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: false,
};

// whether to run the mint/redeem cycle or not
const DEPLOY_ONLY = false;

const btcMintKey = new anchor.web3.PublicKey(FAKE_BTC_MINT);
const solMintKey = new anchor.web3.PublicKey(spl.NATIVE_MINT);
const tokenProgramKey = spl.TOKEN_PROGRAM_ID;
const assocTokenProgramKey = spl.ASSOCIATED_TOKEN_PROGRAM_ID;

const provider = anchor.Provider.local();
anchor.setProvider(provider);

const controller = anchor.workspace.Controller;
const depository = anchor.workspace.Depository;
const oracle = anchor.workspace.Oracle;

const btcOraclePriceAccountKey = findAddr(
  [Buffer.from("BTCUSD")],
  oracle.programId
);
const solOraclePriceAccountKey = findAddr(
  [Buffer.from("SOLUSD")],
  oracle.programId
);

// controller keys
const controllerStateKey = findAddr(
  [Buffer.from("STATE")],
  controller.programId
);
const uxdMintKey = findAddr([Buffer.from("STABLECOIN")], controller.programId);

const depositoryStates = {};

depositoryStates[FAKE_BTC_MINT] = makeDepositoryState(
  btcMintKey,
  btcOraclePriceAccountKey
);
depositoryStates[spl.NATIVE_MINT.toString()] = makeDepositoryState(
  solMintKey,
  solOraclePriceAccountKey
);

// internal function to conveniently init depository key objects
// record and passthrough are controller keys but they depend on the depository
function makeDepositoryState(mint, oraclePriceAccountKey) {
  return {
    // XXX THIS IS useless now that we use only one program, and several states
    // TODO remove and also in the depository program. It's One program Mult states, derived from the mint
    key: depository.programId,
    stateKey: findAddr(
      [Buffer.from("STATE"), mint.toBuffer()],
      depository.programId
    ),
    coinMintKey: mint,
    redeemableMintKey: findAddr(
      [Buffer.from("REDEEMABLE"), mint.toBuffer()],
      depository.programId
    ),
    depositAccountKey: findAddr(
      [Buffer.from("DEPOSIT"), mint.toBuffer()],
      depository.programId
    ),
    recordKey: findAddr(
      [Buffer.from("RECORD"), depository.programId.toBuffer(), mint.toBuffer()],
      controller.programId
    ),
    coinPassthroughKey: findAddr(
      [Buffer.from("PASSTHROUGH"), mint.toBuffer()],
      controller.programId
    ),
    oraclePriceAccountKey: oraclePriceAccountKey,
  };
}

// simple shorthand
function findAddr(seeds, programId) {
  return anchor.utils.publicKey.findProgramAddressSync(seeds, programId)[0];
}

// derives the canonical token account address for a given wallet and mint
function findAssocTokenAddr(walletKey, mintKey) {
  return findAddr(
    [walletKey.toBuffer(), tokenProgramKey.toBuffer(), mintKey.toBuffer()],
    assocTokenProgramKey
  );
}

// returns an instruction to create the associated account for a wallet and mint
function createAssocIxn(walletKey, mintKey) {
  let assocKey = findAssocTokenAddr(walletKey, mintKey);

  return new anchor.web3.TransactionInstruction({
    keys: [
      { pubkey: walletKey, isSigner: true, isWritable: true },
      { pubkey: assocKey, isSigner: false, isWritable: true },
      { pubkey: walletKey, isSigner: false, isWritable: false },
      { pubkey: mintKey, isSigner: false, isWritable: false },
      {
        pubkey: anchor.web3.SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
      { pubkey: tokenProgramKey, isSigner: false, isWritable: false },
      {
        pubkey: anchor.web3.SYSVAR_RENT_PUBKEY,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId: assocTokenProgramKey,
    data: Buffer.alloc(0),
  });
}

// handle the error when an account is uninitialized...
function getTokenBalance(tokenKey) {
  return provider.connection
    .getTokenAccountBalance(tokenKey, TXN_COMMIT)
    .then((o) => o["value"]["uiAmount"])
    .catch(() => null);
}

async function main() {
  // standard spl associated account
  let userUxdKey = findAssocTokenAddr(provider.wallet.publicKey, uxdMintKey);

  console.log("payer:", provider.wallet.publicKey.toString());
  console.log("uxd mint:", uxdMintKey.toString());
  console.log("controller id:", controller.programId.toString());
  console.log("controller state:", controllerStateKey.toString());
  console.log("depository id", depository.programId.toString());
  // BTC depository PDAs. One depository program managing several states clusters derived from a mint
  console.log(
    " * BTC stateKey :",
    depositoryStates[FAKE_BTC_MINT].stateKey.toString()
  );
  console.log(
    " * BTC coinMintKey :",
    depositoryStates[FAKE_BTC_MINT].coinMintKey.toString()
  );
  console.log(
    " * BTC depositAccountKey :",
    depositoryStates[FAKE_BTC_MINT].depositAccountKey.toString()
  );
  // SOL depository PDAs.
  console.log(
    " * SOL stateKey :",
    depositoryStates[spl.NATIVE_MINT].stateKey.toString()
  );
  console.log(
    " * SOL coinMintKey :",
    depositoryStates[spl.NATIVE_MINT].coinMintKey.toString()
  );
  console.log(
    " * SOL depositAccountKey :",
    depositoryStates[spl.NATIVE_MINT].depositAccountKey.toString()
  );
  console.log("\n");

  // set up the controller
  if (await provider.connection.getAccountInfo(controllerStateKey)) {
    console.log("controller already initialized...");
  } else {
    await controller.rpc.new({
      accounts: {
        authority: provider.wallet.publicKey,
        state: controllerStateKey,
        uxdMint: uxdMintKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: tokenProgramKey,
      },
      signers: [provider.wallet.payer],
      options: TXN_OPTS,
    });

    console.log("controller initialized!");
  }

  // and set up the depositories states
  for (let depositoryState of Object.values(depositoryStates)) {
    if (await provider.connection.getAccountInfo(depositoryState.stateKey)) {
      console.log("depository already initialized...");
    } else {
      await depository.rpc.new(controller.programId, {
        accounts: {
          payer: provider.wallet.publicKey,
          state: depositoryState.stateKey,
          redeemableMint: depositoryState.redeemableMintKey,
          programCoin: depositoryState.depositAccountKey,
          coinMint: depositoryState.coinMintKey,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: tokenProgramKey,
        },
        signers: [provider.wallet.payer],
        options: TXN_OPTS,
      });

      console.log("depository initialized!");
    }

    // aaand register with the controller
    if (await provider.connection.getAccountInfo(depositoryState.recordKey)) {
      console.log("depository already registered...");
    } else {
      await controller.rpc.registerDepository(
        depositoryState.key,
        depositoryState.oraclePriceAccountKey,
        {
          accounts: {
            authority: provider.wallet.publicKey,
            state: controllerStateKey,
            depositoryRecord: depositoryState.recordKey,
            depositoryState: depositoryState.stateKey,
            coinMint: depositoryState.coinMintKey,
            coinPassthrough: depositoryState.coinPassthroughKey,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: tokenProgramKey,
          },
          signers: [provider.wallet.payer],
          options: TXN_OPTS,
        }
      );

      console.log("depository registered!");
    }
  }

  if (!DEPLOY_ONLY) {
    let depositoryState = depositoryStates[FAKE_BTC_MINT];

    let userCoinKey = findAssocTokenAddr(
      provider.wallet.publicKey,
      depositoryState.coinMintKey
    );
    let userRedeemableKey = findAssocTokenAddr(
      provider.wallet.publicKey,
      depositoryState.redeemableMintKey
    );

    async function printBalances() {
      let userCoin = await getTokenBalance(userCoinKey);
      let depositCoin = await getTokenBalance(
        depositoryState.depositAccountKey
      );
      let coinPassthrough = await getTokenBalance(
        depositoryState.coinPassthroughKey
      );
      let userRedeemable = await getTokenBalance(userRedeemableKey);
      let userUxd = await getTokenBalance(userUxdKey);

      console.log(
        `* user balance: ${userCoin}
* depository balance: ${depositCoin}
* controller balance: ${coinPassthrough}
* user redeemable: ${userRedeemable}
* user uxd: ${userUxd}
`
      );
    }

    // create user account for redeemables if it doesnt exist
    // note anchor will error if you pass [] or null for the extra ixns
    let depositIxns = (await provider.connection.getAccountInfo(
      userRedeemableKey
    ))
      ? undefined
      : [
          createAssocIxn(
            provider.wallet.publicKey,
            depositoryState.redeemableMintKey
          ),
        ];

    console.log("BEFORE DEPOSIT");
    await printBalances();

    let dsig = await depository.rpc.deposit(
      new anchor.BN(1 * 10 ** BTC_DECIMAL),
      {
        accounts: {
          user: provider.wallet.publicKey,
          state: depositoryState.stateKey,
          programCoin: depositoryState.depositAccountKey,
          redeemableMint: depositoryState.redeemableMintKey,
          userCoin: userCoinKey,
          userRedeemable: userRedeemableKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: tokenProgramKey,
        },
        signers: [provider.wallet.payer],
        options: TXN_OPTS,
        instructions: depositIxns,
      }
    );

    console.log("AFTER DEPOSIT", dsig);
    await printBalances();

    // create user account for uxd if needed
    let mintIxns = (await provider.connection.getAccountInfo(userUxdKey))
      ? undefined
      : [createAssocIxn(provider.wallet.publicKey, uxdMintKey)];

    let msig = await controller.rpc.mintUxd(
      new anchor.BN(1 * 10 ** BTC_DECIMAL),
      {
        accounts: {
          user: provider.wallet.publicKey,
          state: controllerStateKey,
          depository: depositoryState.key,
          depositoryRecord: depositoryState.recordKey,
          depositoryState: depositoryState.stateKey,
          depositoryCoin: depositoryState.depositAccountKey,
          coinMint: depositoryState.coinMintKey,
          coinPassthrough: depositoryState.coinPassthroughKey,
          redeemableMint: depositoryState.redeemableMintKey,
          userRedeemable: userRedeemableKey,
          userUxd: userUxdKey,
          uxdMint: uxdMintKey,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: tokenProgramKey,
          // XXX FIXME temp
          oracle: depositoryState.oraclePriceAccountKey,
        },
        signers: [provider.wallet.payer],
        options: TXN_OPTS,
        instructions: mintIxns,
      }
    );

    console.log("AFTER MINT", msig);
    await printBalances();

    let rsig = await controller.rpc.redeemUxd(
      new anchor.BN(20000 * 10 ** UXD_DECIMAL),
      {
        accounts: {
          user: provider.wallet.publicKey,
          state: controllerStateKey,
          depository: depositoryState.key,
          depositoryRecord: depositoryState.recordKey,
          depositoryState: depositoryState.stateKey,
          depositoryCoin: depositoryState.depositAccountKey,
          coinMint: depositoryState.coinMintKey,
          coinPassthrough: depositoryState.coinPassthroughKey,
          redeemableMint: depositoryState.redeemableMintKey,
          userRedeemable: userRedeemableKey,
          userUxd: userUxdKey,
          uxdMint: uxdMintKey,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: tokenProgramKey,
          // XXX FIXME temp
          oracle: depositoryState.oraclePriceAccountKey,
        },
        signers: [provider.wallet.payer],
        options: TXN_OPTS,
      }
    );

    console.log("AFTER REDEEM", rsig);
    await printBalances();

    let wsig = await depository.rpc.withdraw(null, {
      accounts: {
        user: provider.wallet.publicKey,
        state: depositoryState.stateKey,
        programCoin: depositoryState.depositAccountKey,
        redeemableMint: depositoryState.redeemableMintKey,
        userCoin: userCoinKey,
        userRedeemable: userRedeemableKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: tokenProgramKey,
      },
      signers: [provider.wallet.payer],
      options: TXN_OPTS,
    });

    console.log("AFTER WITHDRAW", wsig);
    await printBalances();
  }
}

main();
