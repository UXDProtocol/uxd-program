import * as anchor from "@project-serum/anchor";
import { findAddr, TXN_OPTS, wallet } from "./utils";
import {
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";

// The oracle program
export const oracle = anchor.workspace.Oracle;

// ORACLE CONSTS
//SOL
export const SOL_USD = "SOLUSD";
const solUsdSeed = Buffer.from(SOL_USD);
// const devnetSOLOraclePriceAccountKey = new anchor.web3.PublicKey("3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E");
// export const testnetSOLOraclePriceAccountKey = new PublicKey("GcnU8V2WKq5QXLhJwBQdt2ankU6GRGh6b1bLCxdykWnP");
export const mainnetSOLOraclePriceAccountKey = new PublicKey("H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG");
export const localSOLOraclePriceAccountKey = anchor.utils.publicKey.findProgramAddressSync([solUsdSeed], oracle.programId)[0];
//BTC
export const BTC_USD = "BTCUSD";
const btcUsdSeed = Buffer.from(BTC_USD);
// export const devnetBTCOraclePriceAccountKey = new anchor.web3.PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J");
// export const testnetBTCOraclePriceAccountKey = new PublicKey("DJW6f4ZVqCnpYNN9rNuzqUcCvkVtBgixo8mq9FKSsCbJ");
export const mainnetBTCOraclePriceAccountKey = new PublicKey("GVXRSBjFk6e6J3NbVPXohDJetcTjaeeuykUpbQF8UoMU");
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

export async function create_localnet_oracle_mirrored(
  seed_pair: string,
  endpoint: string,
  oraclePriceAccountKey: PublicKey,
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

  const connection = new anchor.web3.Connection(endpoint);
  const priceOracleAccount = await connection.getAccountInfo(
    oraclePriceAccountKey
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
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
      },
      signers: [wallet.payer],
      options: TXN_OPTS,
    })
    .catch(() => null);

  let i = 0;
  while (true) {
    const j =
      i + 512 > priceOracleAccount.data.length
        ? priceOracleAccount.data.length
        : i + 512;

    // console.log(`put [${i}..${j}]`);
    await oracle.rpc.put(
      new anchor.BN(i),
      priceOracleAccount.data.slice(i, j),
      {
        accounts: {
          buffer: localOraclePriceAccountKey,
        },
        options: TXN_OPTS,
      }
    );

    i += 512;
    if (i > priceOracleAccount.data.length) break;
  }
};