import {
  AlloyxVaultDepository,
  UXDClient,
  nativeToUi,
} from '@uxd-protocol/uxd-client';
import { Transaction, ComputeBudgetProgram } from '@solana/web3.js';
import { web3 } from '@project-serum/anchor';
import {
  createCredixLpDepositoryMainnet,
  createIdentityDepositoryMainnet,
  createMercurialVaultDepositoryMainnet,
  createAlloyxVaultDepositoryMainnet,
  getConnectionMainnet,
  payer,
  TXN_OPTS,
  createControllerMainnet,
  uxdProgramIdMainnet,
} from './common';

async function main() {
  console.log();
  console.log('------------------------------ ------------------------------');
  console.log('------------------------ PREPARATION ------------------------');
  console.log('------------------------------ ------------------------------');
  console.log();

  const controller = createControllerMainnet();
  const identityDepository = createIdentityDepositoryMainnet();
  const mercurialVaultDepository =
    await createMercurialVaultDepositoryMainnet();
  const credixLpDepository = await createCredixLpDepositoryMainnet();
  const alloyxVaultDepository = await createAlloyxVaultDepositoryMainnet();

  controller.info();
  identityDepository.info();
  mercurialVaultDepository.info();
  credixLpDepository.info();
  alloyxVaultDepository.info();

  const alloyxVaultDepositoryAccount =
    await alloyxVaultDepository.getOnchainAccount(
      getConnectionMainnet(),
      TXN_OPTS
    );

  const profitsBeneficiaryCollateral =
    alloyxVaultDepositoryAccount.profitsBeneficiaryCollateral;
  console.log(
    'profitsBeneficiaryCollateral',
    profitsBeneficiaryCollateral.toBase58()
  );

  const uxdClient = new UXDClient(uxdProgramIdMainnet);

  console.log();
  console.log('------------------------------ ------------------------------');
  console.log('---------------------- ALLOYX REBALANCE ---------------------');
  console.log('------------------------------ ------------------------------');
  console.log();

  const rebalanceInstruction =
    uxdClient.createRebalanceAlloyxVaultDepositoryInstruction(
      controller,
      identityDepository,
      mercurialVaultDepository,
      credixLpDepository,
      alloyxVaultDepository,
      payer.publicKey,
      profitsBeneficiaryCollateral,
      TXN_OPTS
    );
  const rebalanceCreateTransaction = new Transaction();
  rebalanceCreateTransaction.add(rebalanceInstruction);
  rebalanceCreateTransaction.add(
    ComputeBudgetProgram.setComputeUnitLimit({
      units: 400_000,
    })
  );
  rebalanceCreateTransaction.feePayer = payer.publicKey;

  try {
    const rebalanceResult = await web3.sendAndConfirmTransaction(
      getConnectionMainnet(),
      rebalanceCreateTransaction,
      [payer],
      TXN_OPTS
    );
    console.log('rebalanceResult', rebalanceResult);
  } catch (rebalanceError) {
    console.log('rebalanceError', rebalanceError);
  }

  console.log();
  console.log('------------------------------ ------------------------------');
  console.log('------------------- LATEST ON-CHAIN STATE -------------------');
  console.log('------------------------------ ------------------------------');
  console.log();

  const alloyxProgram = AlloyxVaultDepository.getAlloyxProgram(
    getConnectionMainnet(),
    alloyxVaultDepository.alloyxProgramId
  );

  const alloyxVaultInfo = await AlloyxVaultDepository.getAlloyxVaultInfoAccount(
    alloyxProgram,
    alloyxVaultDepository.alloyxVaultInfo
  );
  console.log('> alloyxVaultInfo');
  console.log(
    'alloyxVaultInfo.walletDeskUsdcValue',
    nativeToUi(
      alloyxVaultInfo.walletDeskUsdcValue,
      alloyxVaultDepository.collateralDecimals
    )
  );
  console.log();

  const alloyxVaultPass = await AlloyxVaultDepository.getAlloyxVaultPassAccount(
    alloyxProgram,
    alloyxVaultDepository.alloyxVaultPass
  );
  console.log('> alloyxVaultPass');
  console.log('alloyxVaultPass.investor', alloyxVaultPass.investor.toBase58());
  console.log();

  const alloyxVaultMint = await getConnectionMainnet().getTokenSupply(
    alloyxVaultInfo.alloyxMint
  );
  console.log('> alloyxVaultMint');
  console.log('alloyxVaultMint.supply', alloyxVaultMint.value.uiAmount);
  console.log();

  const alloyxVaultCollateral =
    await getConnectionMainnet().getTokenAccountBalance(
      alloyxVaultDepository.alloyxVaultCollateral
    );
  console.log('> alloyxVaultCollateral');
  console.log(
    'alloyxVaultCollateral.amount',
    alloyxVaultCollateral.value.uiAmount
  );
  console.log();

  const depositoryCollateralAccount =
    await getConnectionMainnet().getTokenAccountBalance(
      alloyxVaultDepository.depositoryCollateral
    );
  console.log('> depositoryCollateral');
  console.log(
    'depositoryCollateral.amount',
    depositoryCollateralAccount.value.uiAmount
  );
  console.log();

  const depositorySharesAccount =
    await getConnectionMainnet().getTokenAccountBalance(
      alloyxVaultDepository.depositoryShares
    );
  console.log('> depositoryShares');
  console.log(
    'depositoryShares.amount',
    depositorySharesAccount.value.uiAmount
  );
  console.log();
}

main();
