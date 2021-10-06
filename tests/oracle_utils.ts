import * as anchor from "@project-serum/anchor";

const TXN_COMMIT = "processed";
const TXN_OPTS = { commitment: TXN_COMMIT, preflightCommitment: TXN_COMMIT, skipPreflight: false };

const oracle = anchor.workspace.Oracle;

const TESTNET = "https://api.testnet.solana.com";

// ORACLE CONSTS
//SOL
export const solUsdSeed = Buffer.from("SOLUSD");
// const devnetOracle = new anchor.web3.PublicKey("3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E");
export const testnetSOLOraclePriceAccountKey = new anchor.web3.PublicKey("GcnU8V2WKq5QXLhJwBQdt2ankU6GRGh6b1bLCxdykWnP");
export const localSOLOraclePriceAccountKey = anchor.utils.publicKey.findProgramAddressSync([solUsdSeed], oracle.programId)[0];
//BTC
export const btcUsdSeed = Buffer.from("BTCUSD");
// const devnetOracle = new anchor.web3.PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J");
export const testnetBTCOraclePriceAccountKey = new anchor.web3.PublicKey("DJW6f4ZVqCnpYNN9rNuzqUcCvkVtBgixo8mq9FKSsCbJ");
export const localBTCOraclePriceAccountKey =
anchor.utils.publicKey.findProgramAddressSync([btcUsdSeed], oracle.programId)[0];

export async function create_localnet_oracle_mirrored_from_testnet(
    seed_pair: String,
    testnetOraclePriceAccountKey: anchor.web3.PublicKey,
    localOraclePriceAccountKey: anchor.web3.PublicKey,
    wallet: anchor.Provider.Wallet
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
  // Should add a get or init if already exists
  // Also typescript error to fix
  await oracle.rpc
    .init(Buffer.from(seed_pair), {
      accounts: {
        wallet: wallet.publicKey,
        buffer: localOraclePriceAccountKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [wallet.payer],
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
};