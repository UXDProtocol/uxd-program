import {
  PublicKey,
  Connection,
  ConnectionConfig,
  ConfirmOptions,
  Keypair,
} from '@solana/web3.js';
import {
  IdentityDepository,
  MercurialVaultDepository,
  AlloyxVaultDepository,
  CredixLpDepository,
  Controller,
  UXDClient,
} from '@uxd-protocol/uxd-client';

const TXN_COMMIT = 'confirmed';
const connectionConfig = {
  commitment: TXN_COMMIT,
  disableRetryOnRateLimit: false,
  confirmTransactionInitialTimeout: 10000,
} as ConnectionConfig;
export const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: true,
} as ConfirmOptions;

export const uxdProgramId = new PublicKey(
  'UXD8m9cvwk4RcSxnX2HZ9VudQCEeDH6fRnB4CAP57Dr'
);
const usdcMint = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');

export function getConnection() {
  const connection = new Connection(
    'https://api.mainnet-beta.solana.com',
    connectionConfig
  );
  return connection;
}

export function createClient() {
  return new UXDClient(uxdProgramId);
}

export function createController() {
  return new Controller('UXD', 6, uxdProgramId);
}

export function createIdentityDepository() {
  return new IdentityDepository(usdcMint, 'USDC', 6, uxdProgramId);
}

export async function createMercurialVaultDepository() {
  try {
    return await MercurialVaultDepository.initialize({
      connection: getConnection(),
      collateralMint: {
        mint: usdcMint,
        name: 'USDC',
        symbol: 'USDC',
        decimals: 6,
      },
      uxdProgramId,
    });
  } catch (error) {
    console.error('Failed to initialize mercurial_vault_depository');
    throw error;
  }
}

export async function createCredixLpDepository() {
  try {
    return await CredixLpDepository.initialize({
      connection: getConnection(),
      uxdProgramId: uxdProgramId,
      collateralMint: usdcMint,
      collateralSymbol: 'USDC',
      credixProgramId: new PublicKey(
        'CRDx2YkdtYtGZXGHZ59wNv1EwKHQndnRc1gT4p8i2vPX'
      ),
    });
  } catch (error) {
    console.error('Failed to initialize credix_lp_depository');
    throw error;
  }
}

export async function createAlloyxVaultDepository() {
  try {
    return await AlloyxVaultDepository.initialize({
      connection: getConnection(),
      uxdProgramId: uxdProgramId,
      collateralMint: usdcMint,
      collateralSymbol: 'USDC',
      alloyxVaultId: 'uxd-debug', // TODO - which vault_id to use on mainnet?
      alloyxVaultMint: new PublicKey(
        'CBQcnyoVjdCyPf2nnhPjbMJL18FEtTuPA9nQPrS7wJPF' // TODO - which vault mint to use on mainnet?
      ),
      alloyxProgramId: new PublicKey(
        '8U29WVwDFLxFud36okhqrngUquaZqVnVL9uE5G8DzX5c'
      ),
    });
  } catch (error) {
    console.error('Failed to initialize alloyx_vault_depository');
    throw error;
  }
}

// Dummy payer for mainnet tooling E7N44oZ3APNFjzv95xL6kSxSLgw3wVP3ixM7dgsMApzZ
export const payer = Keypair.fromSeed(
  Uint8Array.from([
    1, 56, 76, 89, 32, 55, 1, 128, 98, 23, 56, 22, 30, 12, 76, 23, 2, 9, 3, 5,
    1, 22, 120, 109, 0, 8, 5, 3, 2, 7, 6, 8,
  ])
);

console.log('payer', payer.publicKey.toBase58());
