import {
  Controller,
  CredixLpDepository,
  IdentityDepository,
  MercurialVaultDepository,
  UXDClient,
} from '@uxd-protocol/uxd-client';
import {
  Commitment,
  Connection,
  Keypair,
  PublicKey,
  Signer,
  Transaction,
} from '@solana/web3.js';
import { web3 } from '@project-serum/anchor';

const TXN_COMMIT: Commitment = 'confirmed';
const connectionConfig = {
  commitment: TXN_COMMIT,
  disableRetryOnRateLimit: false,
  confirmTransactionInitialTimeout: 10000,
};
const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: true,
};

const uxdProgramId = new PublicKey(
  'UXD8m9cvwk4RcSxnX2HZ9VudQCEeDH6fRnB4CAP57Dr'
);
const usdcMint = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');

function getConnection(): Connection {
  const connection = new Connection(
    'https://api.mainnet-beta.solana.com',
    connectionConfig
  );
  return connection;
}

function createIdentityDepository(): IdentityDepository {
  return new IdentityDepository(usdcMint, 'USDC', 6, uxdProgramId);
}

async function createMercurialVaultDepository(): Promise<MercurialVaultDepository> {
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

async function createCredixLpDepository(): Promise<CredixLpDepository> {
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

async function main() {
  const controller = new Controller('UXD', 6, uxdProgramId);

  const identityDepository = createIdentityDepository();
  const mercurialVaultDepository = await createMercurialVaultDepository();
  const credixLpDepository = await createCredixLpDepository();

  credixLpDepository.info();

  const payer = Keypair.fromSeed(
    Uint8Array.from([
      1, 56, 76, 89, 32, 55, 1, 128, 98, 23, 56, 22, 30, 12, 76, 23, 2, 9, 3, 5,
      1, 22, 120, 109, 0, 8, 5, 3, 2, 7, 6, 8,
    ])
  );

  const uxdClient = new UXDClient(uxdProgramId);

  const profitsBeneficiaryCollateral = new PublicKey(
    'EbmvhGxvvMQhCx8PrWRHScMTbq6tMuwdeigh5muT4bwr'
  );

  const rebalanceCreateInstruction =
    uxdClient.createRebalanceCreateWithdrawRequestFromCredixLpDepositoryInstruction(
      controller,
      identityDepository,
      mercurialVaultDepository,
      credixLpDepository,
      payer.publicKey,
      TXN_OPTS
    );

  const rebalanceRedeemInstruction =
    uxdClient.createRebalanceRedeemWithdrawRequestFromCredixLpDepositoryInstruction(
      controller,
      identityDepository,
      mercurialVaultDepository,
      credixLpDepository,
      payer.publicKey,
      profitsBeneficiaryCollateral,
      TXN_OPTS
    );

  const signers: Signer[] = [payer];
  const transaction = new Transaction();
  transaction.add(rebalanceRedeemInstruction);
  transaction.feePayer = payer.publicKey;

  console.log('payer', payer.publicKey.toBase58());
  console.log('signers', signers);
  console.log('transaction', transaction);

  const result = await web3.sendAndConfirmTransaction(
    getConnection(),
    transaction,
    signers,
    TXN_OPTS
  );
  console.log('result', result);
}

main();
