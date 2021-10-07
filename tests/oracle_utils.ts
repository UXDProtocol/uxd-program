import * as anchor from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { findAddr, TESTNET, TXN_OPTS, wallet } from "./utils";

// The oracle program
export const oracle = anchor.workspace.Oracle;

// ORACLE CONSTS
//SOL
export const SOL_USD = "SOLUSD";
const solUsdSeed = Buffer.from(SOL_USD);
// const devnetOracle = new anchor.web3.PublicKey("3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E");
export const testnetSOLOraclePriceAccountKey = new PublicKey("GcnU8V2WKq5QXLhJwBQdt2ankU6GRGh6b1bLCxdykWnP");
export const localSOLOraclePriceAccountKey = anchor.utils.publicKey.findProgramAddressSync([solUsdSeed], oracle.programId)[0];
//BTC
export const BTC_USD = "BTCUSD";
const btcUsdSeed = Buffer.from(BTC_USD);
// const devnetOracle = new anchor.web3.PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J");
export const testnetBTCOraclePriceAccountKey = new PublicKey("DJW6f4ZVqCnpYNN9rNuzqUcCvkVtBgixo8mq9FKSsCbJ");
export const localBTCOraclePriceAccountKey =
  anchor.utils.publicKey.findProgramAddressSync([btcUsdSeed], oracle.programId)[0];

//

export const btcOraclePriceAccountKey = findAddr(
  [btcUsdSeed],
  oracle.programId
);
export const solOraclePriceAccountKey = findAddr(
  [solUsdSeed],
  oracle.programId
);

//

export async function create_localnet_oracle_mirrored_from_testnet(
  seed_pair: String,
  testnetOraclePriceAccountKey: PublicKey,
  localOraclePriceAccountKey: PublicKey
) {
  // console.log(
  //   `testnet ${seed_pair} price key:`,
  //   testnetOraclePriceAccountKey.toString()
  // );
  // console.log(
  //   `local ${seed_pair} price key:`,
  //   localOraclePriceAccountKey.toString()
  // );

  const testnetConn = new anchor.web3.Connection(TESTNET);
  const testnet_oracle_account = await testnetConn.getAccountInfo(
    testnetOraclePriceAccountKey
  );

  // console.log(`DATA FROM TESTNET for ${seed_pair}`);
  // console.log(testnet_oracle_account);

  // console.log("init");
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

  let i = 0;
  while (true) {
    const j =
      i + 512 > testnet_oracle_account.data.length
        ? testnet_oracle_account.data.length
        : i + 512;

    // console.log(`put [${i}..${j}]`);
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