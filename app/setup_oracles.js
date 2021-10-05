"use strict";

const anchor = require("@project-serum/anchor");

//const DEVNET = "https://api.devnet.solana.com";
// const DEVNET = "http://128.0.113.156";

// To pull data from instead of devnet, more reliable for local testing
const TESTNET = "https://api.testnet.solana.com";
const TXN_COMMIT = "processed";
const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: true,
};

const provider = anchor.Provider.local();
anchor.setProvider(provider);
const oracle = anchor.workspace.Oracle;

// BTC
const btcUsdSeed = "BTCUSD";
// const devnetOracle = new anchor.web3.PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J");
const testnetBTCOraclePriceAccountKey = new anchor.web3.PublicKey(
  "DJW6f4ZVqCnpYNN9rNuzqUcCvkVtBgixo8mq9FKSsCbJ"
);
const localBTCOraclePriceAccountKey =
  anchor.utils.publicKey.findProgramAddressSync(
    [btcUsdSeed],
    oracle.programId
  )[0];

// SOL
const solUsdSeed = "SOLUSD";
// const devnetOracle = new anchor.web3.PublicKey("3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E");
const testnetSOLOraclePriceAccountKey = new anchor.web3.PublicKey(
  "GcnU8V2WKq5QXLhJwBQdt2ankU6GRGh6b1bLCxdykWnP"
);
const localSOLOraclePriceAccountKey =
  anchor.utils.publicKey.findProgramAddressSync(
    [solUsdSeed],
    oracle.programId
  )[0];

async function main() {
  await create_local_oracle(
    btcUsdSeed,
    testnetBTCOraclePriceAccountKey,
    localBTCOraclePriceAccountKey
  );
  await create_local_oracle(
    solUsdSeed,
    testnetSOLOraclePriceAccountKey,
    localSOLOraclePriceAccountKey
  );
}

async function create_local_oracle(
  seed_pair,
  testnetOraclePriceAccountKey,
  localOraclePriceAccountKey
) {
  console.log(
    `testnet ${seed_pair} price key:`,
    testnetOraclePriceAccountKey.toString()
  );
  console.log(
    `local ${seed_pair} price key:`,
    localOraclePriceAccountKey.toString()
  );

  let testnetConn = new anchor.web3.Connection(TESTNET);
  let testnet_oracle_account = await testnetConn.getAccountInfo(
    testnetOraclePriceAccountKey
  );

  console.log(`DATA FROM TESTNET for ${seed_pair}`);
  console.log(testnet_oracle_account);

  console.log("init");
  await oracle.rpc
    .init(Buffer.from(seed_pair), {
      accounts: {
        wallet: provider.wallet.publicKey,
        buffer: localOraclePriceAccountKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [provider.wallet.payer],
      options: TXN_OPTS,
    })
    .catch(() => null);

  var i = 0;
  while (true) {
    let j =
      i + 512 > testnet_oracle_account.data.length
        ? testnet_oracle_account.data.length
        : i + 512;

    console.log(`put [${i}..${j}]`);
    await oracle.rpc.put(
      new anchor.BN(i),
      testnet_oracle_account.data.slice(i, j),
      {
        accounts: {
          buffer: localOraclePriceAccountKey,
        },
        options: TXN_OPTS,
      }
    );

    i += 512;
    if (i > testnet_oracle_account.data.length) break;
  }
}

main();
