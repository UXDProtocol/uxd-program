const {
  Controller,
  CredixLpDepository,
  IdentityDepository,
  MercurialVaultDepository,
  UXDClient,
} = require('@uxd-protocol/uxd-client');
const {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
} = require('@solana/web3.js');
const { web3 } = require('@project-serum/anchor');

const TXN_COMMIT = 'confirmed';
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

function getConnection() {
  const connection = new Connection(
    'https://api.mainnet-beta.solana.com',
    connectionConfig
  );
  return connection;
}

function createIdentityDepository() {
  return new IdentityDepository(usdcMint, 'USDC', 6, uxdProgramId);
}

async function createMercurialVaultDepository() {
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

async function createCredixLpDepository() {
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
  // Dummy payer for mainnet tooling E7N44oZ3APNFjzv95xL6kSxSLgw3wVP3ixM7dgsMApzZ
  const payer = Keypair.fromSeed(
    Uint8Array.from([
      1, 56, 76, 89, 32, 55, 1, 128, 98, 23, 56, 22, 30, 12, 76, 23, 2, 9, 3, 5,
      1, 22, 120, 109, 0, 8, 5, 3, 2, 7, 6, 8,
    ])
  );
  console.log('payer', payer.publicKey.toBase58());

  const controller = new Controller('UXD', 6, uxdProgramId);
  const identityDepository = createIdentityDepository();
  const mercurialVaultDepository = await createMercurialVaultDepository();
  const credixLpDepository = await createCredixLpDepository();

  controller.info();
  identityDepository.info();
  mercurialVaultDepository.info();
  credixLpDepository.info();

  const profitsBeneficiaryCollateral = (
    await credixLpDepository.getOnchainAccount(getConnection(), TXN_OPTS)
  ).profitsBeneficiaryCollateral;
  console.log(
    'profitsBeneficiaryCollateral',
    profitsBeneficiaryCollateral.toBase58()
  );

  const uxdClient = new UXDClient(uxdProgramId);

  const rebalanceCreateInstruction =
    uxdClient.createRebalanceCreateWithdrawRequestFromCredixLpDepositoryInstruction(
      controller,
      identityDepository,
      mercurialVaultDepository,
      credixLpDepository,
      payer.publicKey,
      TXN_OPTS
    );
  const rebalanceCreateTransaction = new Transaction();
  rebalanceCreateTransaction.add(rebalanceCreateInstruction);
  rebalanceCreateTransaction.feePayer = payer.publicKey;
  console.log('rebalanceCreateTransaction', rebalanceCreateTransaction);

  try {
    const rebalanceCreateResult = await web3.sendAndConfirmTransaction(
      getConnection(),
      rebalanceCreateTransaction,
      [payer],
      TXN_OPTS
    );
    console.log('rebalanceCreateResult', rebalanceCreateResult);
  } catch (rebalanceCreateError) {
    console.log('rebalanceCreateError', rebalanceCreateError);
  }

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
  const rebalanceRedeemTransaction = new Transaction();
  rebalanceRedeemTransaction.add(rebalanceRedeemInstruction);
  rebalanceRedeemTransaction.feePayer = payer.publicKey;
  console.log('rebalanceRedeemTransaction', rebalanceRedeemTransaction);

  try {
    const rebalanceRedeemResult = await web3.sendAndConfirmTransaction(
      getConnection(),
      rebalanceCreateTransaction,
      [payer],
      TXN_OPTS
    );
    console.log('rebalanceRedeemResult', rebalanceRedeemResult);
  } catch (rebalanceRedeemError) {
    console.log('rebalanceRedeemError', rebalanceRedeemError);
  }
}

main();
