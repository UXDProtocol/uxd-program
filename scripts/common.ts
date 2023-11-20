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
  AlloyxVaultDepository,
  Controller,
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

export const uxdProgramIdMainnet = new PublicKey(
  'UXD8m9cvwk4RcSxnX2HZ9VudQCEeDH6fRnB4CAP57Dr'
);
const usdcMintMainnet = new PublicKey(
  'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v'
);

export function getConnectionMainnet() {
  const connection = new Connection(
    'https://api.mainnet-beta.solana.com',
    connectionConfig
  );
  return connection;
}

export function createControllerMainnet() {
  return new Controller('UXD', 6, uxdProgramIdMainnet);
}

export function createIdentityDepositoryMainnet() {
  return new IdentityDepository(
    usdcMintMainnet,
    'USDC',
    6,
    uxdProgramIdMainnet
  );
}

export async function createMercurialVaultDepositoryMainnet() {
  try {
    return await MercurialVaultDepository.initialize({
      connection: getConnectionMainnet(),
      collateralMint: {
        mint: usdcMintMainnet,
        name: 'USDC',
        symbol: 'USDC',
        decimals: 6,
      },
      uxdProgramId: uxdProgramIdMainnet,
    });
  } catch (error) {
    console.error('Failed to initialize mercurial depository');
    throw error;
  }
}

export async function createCredixLpDepositoryMainnet() {
  try {
    return await CredixLpDepository.initialize({
      connection: getConnectionMainnet(),
      uxdProgramId: uxdProgramIdMainnet,
      collateralMint: usdcMintMainnet,
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

export async function createAlloyxVaultDepositoryMainnet() {
  try {
    return await AlloyxVaultDepository.initialize({
      connection: getConnectionMainnet(),
      uxdProgramId: uxdProgramIdMainnet,
      collateralMint: usdcMintMainnet,
      collateralSymbol: 'USDC',
      alloyxVaultId: 'diversified_public_credit',
      alloyxVaultMint: new PublicKey(
        'CBQcnyoVjdCyPf2nnhPjbMJL18FEtTuPA9nQPrS7wJPF' // TODO - need to wait for alloyx to supply the correct mint address
      ),
      alloyxProgramId: new PublicKey(
        '8U29WVwDFLxFud36okhqrngUquaZqVnVL9uE5G8DzX5c'
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
