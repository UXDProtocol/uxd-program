import {
  Controller,
  CredixLpDepository,
  UXDClient,
  nativeToUi,
} from '@uxd-protocol/uxd-client';
import { Transaction, ComputeBudgetProgram } from '@solana/web3.js';
import { BN, web3 } from '@project-serum/anchor';
import {
  createCredixLpDepository,
  createIdentityDepository,
  createMercurialVaultDepository,
  getConnection,
  payer,
  TXN_OPTS,
  uxdProgramId,
} from './common';

async function main() {
  console.log();
  console.log('------------------------------ ------------------------------');
  console.log('------------------------ PREPARATION ------------------------');
  console.log('------------------------------ ------------------------------');
  console.log();

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
  console.log('> overflow_value:', overflow_value.toString());

  console.log();
}

function compute_value_for_shares_amount_floor(
  shares_amount: BN,
  total_shares_supply: BN,
  total_shares_value: BN
) {
  return shares_amount.mul(total_shares_value).div(total_shares_supply);
}

main();
