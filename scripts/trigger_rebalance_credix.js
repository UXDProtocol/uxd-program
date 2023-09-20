const {
  Controller,
  CredixLpDepository,
  IdentityDepository,
  MercurialVaultDepository,
  UXDClient,
  nativeToUi,
} = require('@uxd-protocol/uxd-client');
const {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  ComputeBudgetProgram,
} = require('@solana/web3.js');
const { web3 } = require('@project-serum/anchor');
const { BN } = require('bn.js');

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
  console.log();
  console.log('------------------------------ ------------------------------');
  console.log('------------------------ PREPARATION ------------------------');
  console.log('------------------------------ ------------------------------');
  console.log();

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

  const controllerAccount = await controller.getOnchainAccount(
    getConnection(),
    TXN_OPTS
  );
  const credixLpDepositoryAccount = await credixLpDepository.getOnchainAccount(
    getConnection(),
    TXN_OPTS
  );

  const profitsBeneficiaryCollateral =
    credixLpDepositoryAccount.profitsBeneficiaryCollateral;
  console.log(
    'profitsBeneficiaryCollateral',
    profitsBeneficiaryCollateral.toBase58()
  );

  const uxdClient = new UXDClient(uxdProgramId);

  console.log();
  console.log('------------------------------ ------------------------------');
  console.log('------------------ CREDIX REBALANCE CREATE ------------------');
  console.log('------------------------------ ------------------------------');
  console.log();

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
  rebalanceCreateTransaction.add(
    ComputeBudgetProgram.setComputeUnitLimit({
      units: 400_000,
    })
  );
  rebalanceCreateTransaction.feePayer = payer.publicKey;

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

  console.log();
  console.log('------------------------------ ------------------------------');
  console.log('----------------- CREDIX REBALANCE REDEEM -------------------');
  console.log('------------------------------ ------------------------------');
  console.log();

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
  rebalanceRedeemTransaction.add(
    ComputeBudgetProgram.setComputeUnitLimit({
      units: 400_000,
    })
  );
  rebalanceRedeemTransaction.feePayer = payer.publicKey;

  try {
    const rebalanceRedeemResult = await web3.sendAndConfirmTransaction(
      getConnection(),
      rebalanceRedeemTransaction,
      [payer],
      TXN_OPTS
    );
    console.log('rebalanceRedeemResult', rebalanceRedeemResult);
  } catch (rebalanceRedeemError) {
    console.log('rebalanceRedeemError', rebalanceRedeemError);
  }

  console.log();
  console.log('------------------------------ ------------------------------');
  console.log('------------------- LATEST ON-CHAIN STATE -------------------');
  console.log('------------------------------ ------------------------------');
  console.log();

  const credixBaseDecimals = 6;

  const credixProgramId = credixLpDepository.credixProgramId;
  const credixGlobalMarketState = credixLpDepository.credixGlobalMarketState;
  const credixPass = credixLpDepository.credixPass;
  const credixWithdrawEpoch = credixLpDepository.credixWithdrawEpoch;
  const credixWithdrawRequest = credixLpDepository.credixWithdrawRequest;
  const credixProgram = CredixLpDepository.getCredixProgram(
    getConnection(),
    credixProgramId
  );

  const credixGlobalMarketStateAccount =
    await CredixLpDepository.getCredixGlobalMarketStateAccount(
      credixProgram,
      credixGlobalMarketState
    );
  console.log('> credixGlobalMarketStateAccount');
  console.log(
    'credixGlobalMarketStateAccount.credixFeePercentage',
    credixGlobalMarketStateAccount.credixFeePercentage
  );
  console.log(
    'credixGlobalMarketStateAccount.withdrawalFee',
    credixGlobalMarketStateAccount.withdrawalFee
  );
  console.log(
    'credixGlobalMarketStateAccount.latestWithdrawEpochEnd:',
    new Date(credixGlobalMarketStateAccount.latestWithdrawEpochEnd * 1000)
  );
  console.log(
    'credixGlobalMarketStateAccount.poolOutstandingCredit',
    nativeToUi(
      credixGlobalMarketStateAccount.poolOutstandingCredit,
      credixBaseDecimals
    )
  );
  console.log(
    'credixGlobalMarketStateAccount.lockedLiquidity',
    nativeToUi(
      credixGlobalMarketStateAccount.lockedLiquidity,
      credixBaseDecimals
    )
  );
  console.log(
    'credixGlobalMarketStateAccount.totalRedeemedBaseAmount',
    nativeToUi(
      credixGlobalMarketStateAccount.totalRedeemedBaseAmount,
      credixBaseDecimals
    )
  );
  console.log();

  const credixPassAccount = await CredixLpDepository.getCredixPassAccount(
    credixProgram,
    credixPass
  );
  console.log('> credixPassAccount');
  console.log('credixPassAccount.active', credixPassAccount.active);
  console.log('credixPassAccount.isBorrower', credixPassAccount.isBorrower);
  console.log('credixPassAccount.isInvestor', credixPassAccount.isInvestor);
  console.log(
    'credixPassAccount.disableWithdrawalFee',
    credixPassAccount.disableWithdrawalFee
  );
  console.log(
    'credixPassAccount.bypassWithdrawEpochs',
    credixPassAccount.bypassWithdrawEpochs
  );
  console.log(
    'credixPassAccount.releaseTimestamp:',
    new Date(credixPassAccount.releaseTimestamp * 1000)
  );
  console.log();

  const credixWithdrawEpochAccount =
    await CredixLpDepository.getCredixWithdrawEpochAccount(
      credixProgram,
      credixWithdrawEpoch
    );
  console.log('> credixWithdrawEpochAccount');
  console.log(
    'credixWithdrawEpochAccount.goLive:',
    new Date(credixWithdrawEpochAccount.goLive * 1000)
  );
  console.log(
    'credixWithdrawEpochAccount.goLive+request:',
    new Date(
      credixWithdrawEpochAccount.goLive.addn(
        credixWithdrawEpochAccount.requestSeconds
      ) * 1000
    )
  );
  console.log(
    'credixWithdrawEpochAccount.goLive+request+redeem:',
    new Date(
      credixWithdrawEpochAccount.goLive
        .addn(credixWithdrawEpochAccount.requestSeconds)
        .addn(credixWithdrawEpochAccount.redeemSeconds) * 1000
    )
  );
  console.log(
    'credixWithdrawEpochAccount.goLive+request+redeem+availableLiquidity:',
    new Date(
      credixWithdrawEpochAccount.goLive
        .addn(credixWithdrawEpochAccount.requestSeconds)
        .addn(credixWithdrawEpochAccount.redeemSeconds)
        .addn(credixWithdrawEpochAccount.availableLiquiditySeconds) * 1000
    )
  );
  console.log(
    'credixWithdrawEpochAccount.totalRequestedBaseAmount',
    nativeToUi(
      credixWithdrawEpochAccount.totalRequestedBaseAmount,
      credixBaseDecimals
    )
  );
  console.log(
    'credixWithdrawEpochAccount.participatingInvestorsTotalLpAmount',
    nativeToUi(
      credixWithdrawEpochAccount.participatingInvestorsTotalLpAmount,
      credixBaseDecimals
    )
  );
  console.log();

  const credixSharesMintAccount = await getConnection().getTokenSupply(
    credixLpDepository.credixSharesMint
  );
  console.log('> credixSharesMint');
  console.log(
    'credixSharesMint.supply',
    credixSharesMintAccount.value.uiAmount
  );
  console.log();

  const credixLiquidityCollateralAccount =
    await getConnection().getTokenAccountBalance(
      credixLpDepository.credixLiquidityCollateral
    );
  console.log('> credixLiquidityCollateral');
  console.log(
    'credixLiquidityCollateral.amount',
    credixLiquidityCollateralAccount.value.uiAmount
  );
  console.log();

  const depositoryCollateralAccount =
    await getConnection().getTokenAccountBalance(
      credixLpDepository.depositoryCollateral
    );
  console.log('> depositoryCollateral');
  console.log(
    'depositoryCollateral.amount',
    depositoryCollateralAccount.value.uiAmount
  );
  console.log();

  const depositorySharesAccount = await getConnection().getTokenAccountBalance(
    credixLpDepository.depositoryShares
  );
  console.log('> depositoryShares');
  console.log(
    'depositoryShares.amount',
    depositorySharesAccount.value.uiAmount
  );
  console.log();

  console.log();
  console.log('------------------------------ ------------------------------');
  console.log('----------------- REBALANCING REDEEM MATHS ------------------');
  console.log('------------------------------ ------------------------------');
  console.log();

  const total_shares_supply_before = new BN(
    credixSharesMintAccount.value.amount
  );
  console.log(
    '> total_shares_supply_before:',
    total_shares_supply_before.toString()
  );

  const total_shares_value_before = new BN(
    credixLiquidityCollateralAccount.value.amount
  ).add(credixGlobalMarketStateAccount.poolOutstandingCredit);
  console.log(
    '> total_shares_value_before:',
    total_shares_value_before.toString()
  );

  const owned_shares_amount_before = new BN(
    depositorySharesAccount.value.amount
  );
  console.log(
    '> owned_shares_amount_before:',
    owned_shares_amount_before.toString()
  );

  const owned_shares_value_before = compute_value_for_shares_amount_floor(
    owned_shares_amount_before,
    total_shares_supply_before,
    total_shares_value_before
  );
  console.log(
    '> owned_shares_value_before:',
    owned_shares_value_before.toString()
  );

  const redeemable_amount_under_management =
    credixLpDepositoryAccount.redeemableAmountUnderManagement;
  console.log(
    '> redeemable_amount_under_management:',
    redeemable_amount_under_management.toString()
  );

  const profits_collateral_amount = owned_shares_value_before.sub(
    redeemable_amount_under_management
  );
  console.log(
    '> profits_collateral_amount:',
    profits_collateral_amount.toString()
  );

  const redeemable_amount_under_management_target_amount =
    controllerAccount.redeemableCirculatingSupply
      .muln(controllerAccount.credixLpDepositoryWeightBps)
      .divn(10_000);
  let overflow_value = redeemable_amount_under_management.sub(
    redeemable_amount_under_management_target_amount
  );
  if (overflow_value < 0) {
    overflow_value = new BN(0);
  }
  console.log('> overflow_value:', overflow_value.toString());

  console.log();
}

function compute_value_for_shares_amount_floor(
  shares_amount,
  total_shares_supply,
  total_shares_value
) {
  return shares_amount.mul(total_shares_value).div(total_shares_supply);
}

function compute_shares_amount_for_value_floor(
  value,
  total_shares_supply,
  total_shares_value
) {
  return value.mul(total_shares_supply).div(total_shares_value);
}

main();
