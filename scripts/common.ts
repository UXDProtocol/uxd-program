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
  CredixLpDepository,
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
    console.error('Failed to initialize mercurial depository');
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
    console.error('Failed to initialize credix depository');
    throw error;
  }
}

// CI script payer on mainnet: 4NtUyktW5evy1Ez4BgfnRBdU7PLsmDRiZiH5HfBPGRSs
export const payer = Keypair.fromSecretKey(
  Buffer.from(JSON.parse(require('fs').readFileSync('./payer.json', 'utf-8')))
);
console.log('payer', payer.publicKey.toBase58());
