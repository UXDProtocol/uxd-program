"use strict";

const anchor = require("@project-serum/anchor");
const spl = require("@solana/spl-token");

const CONTROLLER = process.env.CONTROLLER;
const BTC_DEPOSITORY = process.env.BTC_DEPOSITORY;
const SOL_DEPOSITORY = process.env.SOL_DEPOSITORY;
const COIN_MINT = process.env.COIN_MINT;
if (!(CONTROLLER && BTC_DEPOSITORY && SOL_DEPOSITORY && COIN_MINT))
  throw "controller two depositories and a coin mint pls";

const BTC_DECIMAL = 6;
const SOL_DECIMAL = 9;
const UXD_DECIMAL = 6;

const DEVNET =
  process.env.DEVNET == "devnet" ? "https://api.devnet.solana.com" : false;

const TXN_COMMIT = "processed";
const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: false,
};

// whether to run the mint/redeem cycle or not
const DEPLOY_ONLY = false;

const btcMintKey = new anchor.web3.PublicKey(COIN_MINT);
const solMintKey = new anchor.web3.PublicKey(spl.NATIVE_MINT);
const tokenProgramKey = spl.TOKEN_PROGRAM_ID;
const assocTokenProgramKey = spl.ASSOCIATED_TOKEN_PROGRAM_ID;

// we should not need this on mainnet but note the addresses change per cluster
// oracleprogram is for if we copied data to localnet
const oracleProgramKey = new anchor.web3.PublicKey(
  require("../target/idl/oracle.json").metadata.address
);
const devnetBtcOracleKey = new anchor.web3.PublicKey(
  "HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J"
);
const localBtcOracleKey = findAddr([Buffer.from("BTCUSD")], oracleProgramKey);
const btcOracleKey = DEVNET ? devnetBtcOracleKey : localBtcOracleKey;
const solOracleKey = new anchor.web3.PublicKey(
  "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix"
);

const provider = anchor.Provider.local(DEVNET || undefined);
anchor.setProvider(provider);

const controllerIdl = require("../target/idl/controller.json");
const controllerKey = new anchor.web3.PublicKey(CONTROLLER);
const controller = new anchor.Program(controllerIdl, controllerKey);

// controller keys
const controlStateKey = findAddr([Buffer.from("STATE")], controllerKey);
const uxdMintKey = findAddr([Buffer.from("STABLECOIN")], controllerKey);

const depositoryIdl = require("../target/idl/depository.json");
const btcDepositoryKey = new anchor.web3.PublicKey(BTC_DEPOSITORY);
const solDepositoryKey = new anchor.web3.PublicKey(SOL_DEPOSITORY);

// XXX im using the btc oracle for sol but it doesnt matter here im just testing
const depositories = {};
depositories[COIN_MINT] = mkDepository(
  btcDepositoryKey,
  btcMintKey,
  btcOracleKey
);
depositories[spl.NATIVE_MINT.toString()] = mkDepository(
  solDepositoryKey,
  solMintKey,
  solOracleKey
);

// internal function to conveniently init depository key objects
// record and passthrough are controller keys but they depend on the depository
function mkDepository(depositoryKey, mintKey, oracleKey) {
  return {
    key: depositoryKey,
    program: new anchor.Program(depositoryIdl, depositoryKey),
    stateKey: findAddr([Buffer.from("STATE")], depositoryKey),
    coinMintKey: mintKey,
    redeemableMintKey: findAddr([Buffer.from("REDEEMABLE")], depositoryKey),
    depositAccountKey: findAddr([Buffer.from("DEPOSIT")], depositoryKey),
    recordKey: findAddr(
      [Buffer.from("RECORD"), depositoryKey.toBuffer()],
      controllerKey
    ),
    coinPassthroughKey: findAddr(
      [Buffer.from("PASSTHROUGH"), mintKey.toBuffer()],
      controllerKey
    ),
    oracleKey: oracleKey,
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
  console.log("controller id:", controllerKey.toString());
  console.log("controller state:", controlStateKey.toString());
  console.log("btc depository id:", depositories[COIN_MINT].key.toString());
  console.log(
    "sol depository id:",
    depositories[spl.NATIVE_MINT.toString()].key.toString()
  );
  console.log("\n");

  // set up the controller
  if (await provider.connection.getAccountInfo(controlStateKey)) {
    console.log("controller already initialized...");
  } else {
    await controller.rpc.new({
      accounts: {
        authority: provider.wallet.publicKey,
        state: controlStateKey,
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

  // and set up the depositories
  for (let depository of Object.values(depositories)) {
    if (await provider.connection.getAccountInfo(depository.stateKey)) {
      console.log("depository already initialized...");
    } else {
      await depository.program.rpc.new(controllerKey, {
        accounts: {
          payer: provider.wallet.publicKey,
          state: depository.stateKey,
          redeemableMint: depository.redeemableMintKey,
          programCoin: depository.depositAccountKey,
          coinMint: depository.coinMintKey,
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
    if (await provider.connection.getAccountInfo(depository.recordKey)) {
      console.log("depository already registered...");
    } else {
      await controller.rpc.registerDepository(
        depository.key,
        depository.oracleKey,
        {
          accounts: {
            authority: provider.wallet.publicKey,
            state: controlStateKey,
            depositoryRecord: depository.recordKey,
            depositoryState: depository.stateKey,
            coinMint: depository.coinMintKey,
            coinPassthrough: depository.coinPassthroughKey,
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
    let depository = depositories[COIN_MINT];

    let userCoinKey = findAssocTokenAddr(
      provider.wallet.publicKey,
      depository.coinMintKey
    );
    let userRedeemableKey = findAssocTokenAddr(
      provider.wallet.publicKey,
      depository.redeemableMintKey
    );

    async function printBalances() {
      let userCoin = await getTokenBalance(userCoinKey);
      let depositCoin = await getTokenBalance(depository.depositAccountKey);
      let coinPassthrough = await getTokenBalance(
        depository.coinPassthroughKey
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
            depository.redeemableMintKey
          ),
        ];

    console.log("BEFORE DEPOSIT");
    await printBalances();

    let dsig = await depository.program.rpc.deposit(
      new anchor.BN(1 * 10 ** BTC_DECIMAL),
      {
        accounts: {
          user: provider.wallet.publicKey,
          state: depository.stateKey,
          programCoin: depository.depositAccountKey,
          redeemableMint: depository.redeemableMintKey,
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
          state: controlStateKey,
          depository: depository.key,
          depositoryRecord: depository.recordKey,
          depositoryState: depository.stateKey,
          depositoryCoin: depository.depositAccountKey,
          coinMint: depository.coinMintKey,
          coinPassthrough: depository.coinPassthroughKey,
          redeemableMint: depository.redeemableMintKey,
          userRedeemable: userRedeemableKey,
          userUxd: userUxdKey,
          uxdMint: uxdMintKey,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: tokenProgramKey,
          // XXX FIXME temp
          oracle: depository.oracleKey,
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
          state: controlStateKey,
          depository: depository.key,
          depositoryRecord: depository.recordKey,
          depositoryState: depository.stateKey,
          depositoryCoin: depository.depositAccountKey,
          coinMint: depository.coinMintKey,
          coinPassthrough: depository.coinPassthroughKey,
          redeemableMint: depository.redeemableMintKey,
          userRedeemable: userRedeemableKey,
          userUxd: userUxdKey,
          uxdMint: uxdMintKey,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: tokenProgramKey,
          // XXX FIXME temp
          oracle: depository.oracleKey,
        },
        signers: [provider.wallet.payer],
        options: TXN_OPTS,
      }
    );

    console.log("AFTER REDEEM", rsig);
    await printBalances();

    let wsig = await depository.program.rpc.withdraw(null, {
      accounts: {
        user: provider.wallet.publicKey,
        state: depository.stateKey,
        programCoin: depository.depositAccountKey,
        redeemableMint: depository.redeemableMintKey,
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
