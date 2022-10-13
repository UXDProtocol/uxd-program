import { getConnection, TXN_OPTS } from "./connection";
import { uxdClient } from "./constants";
import { Keypair, Signer, Transaction } from "@solana/web3.js";
import { NATIVE_MINT } from "@solana/spl-token";
import { createAssociatedTokenAccountItx, prepareWrappedSolTokenAccount } from "./utils";
import {
  MangoDepository,
  Mango,
  Controller,
  PnLPolarity,
  createAssocTokenIx,
  findATAAddrSync,
  uiToNative,
  MercurialVaultDepository,
} from "@uxd-protocol/uxd-client";
import { BN, web3 } from "@project-serum/anchor";
import { Payer } from "@blockworks-foundation/mango-client";

export async function initializeController(authority: Signer, payer: Signer, controller: Controller): Promise<string> {
  const initControllerIx = uxdClient.createInitializeControllerInstruction(
    controller,
    authority.publicKey,
    TXN_OPTS,
    payer.publicKey
  );

  const signers = [];
  const tx = new Transaction();

  tx.instructions.push(initControllerIx);
  signers.push(authority);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function mintWithMercurialVaultDepository(authority: Signer, payer: Signer, controller: Controller, depository: MercurialVaultDepository, collateralAmount: number): Promise<string> {
  const mintWithMercurialVaultDepositoryIx = uxdClient.createMintWithMercurialVaultDepositoryInstruction(controller, depository, authority.publicKey, collateralAmount, TXN_OPTS, payer.publicKey);
  let signers = [];
  let tx = new Transaction();

  const [authorityRedeemableAta] = findATAAddrSync(authority.publicKey, controller.redeemableMintPda);
  if (!await getConnection().getAccountInfo(authorityRedeemableAta)) {
    const createUserRedeemableAtaIx = createAssocTokenIx(authority.publicKey, authorityRedeemableAta, controller.redeemableMintPda);
    tx.add(createUserRedeemableAtaIx);
  }

  tx.add(mintWithMercurialVaultDepositoryIx);
  signers.push(authority);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function redeemFromMercurialVaultDepository(
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: MercurialVaultDepository,
  redeemableAmount: number,
): Promise<string> {
  const redeemFromMercurialVaultDepositoryIx = uxdClient.createRedeemFromMercurialVaultDepositoryInstruction(controller, depository, authority.publicKey, redeemableAmount, TXN_OPTS, payer.publicKey);
  let signers = [];
  let tx = new Transaction();

  const [authorityRedeemableAta] = findATAAddrSync(authority.publicKey, controller.redeemableMintPda);
  if (!await getConnection().getAccountInfo(authorityRedeemableAta)) {
    const createUserRedeemableAtaIx = createAssocTokenIx(authority.publicKey, authorityRedeemableAta, controller.redeemableMintPda);
    tx.add(createUserRedeemableAtaIx);
  }

  tx.add(redeemFromMercurialVaultDepositoryIx);
  signers.push(authority);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function registerMercurialVaultDepository(
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: MercurialVaultDepository,
  mintingFeeInBps: number,
  redeemingFeeInBps: number,
  redeemableDepositorySupplyCap: number,
): Promise<string> {
  const registerMercurialVaultDepositoryIx = uxdClient.createRegisterMercurialVaultDepositoryInstruction(controller, depository, authority.publicKey, mintingFeeInBps, redeemingFeeInBps, redeemableDepositorySupplyCap, TXN_OPTS, payer.publicKey);
  let signers = [];
  let tx = new Transaction();

  tx.instructions.push(registerMercurialVaultDepositoryIx);
  signers.push(authority);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function registerMangoDepository(
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: MangoDepository,
  mango: Mango,
  redeemableDepositorySupplyCap: number,
): Promise<string> {
  const registerMangoDepositoryIx = uxdClient.createRegisterMangoDepositoryInstruction(
    controller,
    depository,
    mango,
    authority.publicKey,
    redeemableDepositorySupplyCap,
    TXN_OPTS,
    payer.publicKey
  );
  let signers = [];
  let tx = new Transaction();

  tx.instructions.push(registerMangoDepositoryIx);
  signers.push(authority);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function depositInsuranceToMangoDepository(
  authority: Signer,
  amount: number,
  controller: Controller,
  depository: MangoDepository,
  mango: Mango
): Promise<string> {
  const depositInsuranceToMangoDepositoryIx = uxdClient.createDepositInsuranceToMangoDepositoryInstruction(
    amount,
    controller,
    depository,
    mango,
    authority.publicKey,
    TXN_OPTS
  );
  let signers = [];
  let tx = new Transaction();

  tx.instructions.push(depositInsuranceToMangoDepositoryIx);
  signers.push(authority);

  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function withdrawInsuranceFromMangoDepository(
  amount: number,
  authority: Signer,
  controller: Controller,
  depository: MangoDepository,
  mango: Mango
): Promise<string> {
  const withdrawInsuranceFromMangoDepository = uxdClient.createWithdrawInsuranceFromMangoDepositoryInstruction(
    amount,
    controller,
    depository,
    mango,
    authority.publicKey,
    TXN_OPTS
  );
  let signers = [];
  let tx = new Transaction();

  const authorityQuoteAta = findATAAddrSync(authority.publicKey, depository.quoteMint)[0];
  if (!(await getConnection().getAccountInfo(authorityQuoteAta))) {
    const createUserQuoteAtaIx = createAssocTokenIx(authority.publicKey, authorityQuoteAta, depository.quoteMint);
    tx.add(createUserQuoteAtaIx);
  }

  tx.instructions.push(withdrawInsuranceFromMangoDepository);
  signers.push(authority);

  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function setRedeemableGlobalSupplyCap(
  authority: Signer,
  controller: Controller,
  supplyCapUiAmount: number
): Promise<string> {
  const setRedeemableGlobalSupplyCapIx = uxdClient.createSetRedeemableGlobalSupplyCapInstruction(
    controller,
    authority.publicKey,
    supplyCapUiAmount,
    TXN_OPTS
  );
  let signers = [];
  let tx = new Transaction();

  tx.instructions.push(setRedeemableGlobalSupplyCapIx);
  signers.push(authority);

  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function setMangoDepositoriesRedeemableSoftCap(
  authority: Signer,
  controller: Controller,
  supplySoftCapUiAmount: number
): Promise<string> {
  const setMangoDepositoriesRedeemableSoftCapIx = uxdClient.createSetMangoDepositoriesRedeemableSoftCapInstruction(
    controller,
    authority.publicKey,
    supplySoftCapUiAmount,
    TXN_OPTS
  );
  let signers = [];
  let tx = new Transaction();

  tx.instructions.push(setMangoDepositoriesRedeemableSoftCapIx);
  signers.push(authority);

  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function mintWithMangoDepository(
  user: Signer,
  payer: Signer,
  slippage: number,
  collateralAmount: number,
  controller: Controller,
  depository: MangoDepository,
  mango: Mango
): Promise<string> {
  const mintWithMangoDepositoryIx = await uxdClient.createMintWithMangoDepositoryInstruction(
    collateralAmount,
    slippage,
    controller,
    depository,
    mango,
    user.publicKey,
    TXN_OPTS,
    payer.publicKey
  );
  let signers = [];
  let tx = new Transaction();

  const userRedeemableAta = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
  if (!(await getConnection().getAccountInfo(userRedeemableAta))) {
    const createUserRedeemableAtaIx = createAssociatedTokenAccountItx(
      payer.publicKey,
      user.publicKey,
      controller.redeemableMintPda
    );
    tx.add(createUserRedeemableAtaIx);
  }

  if (depository.collateralMint.equals(NATIVE_MINT)) {
    const nativeAmount = uiToNative(collateralAmount, depository.collateralMintDecimals);
    const prepareWrappedSolIxs = await prepareWrappedSolTokenAccount(
      getConnection(),
      payer.publicKey,
      user.publicKey,
      nativeAmount.toNumber()
    );
    tx.add(...prepareWrappedSolIxs);
  } else {
    const userCollateralAta = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
    if (!(await getConnection().getAccountInfo(userCollateralAta))) {
      const createUserCollateralAtaIx = createAssocTokenIx(
        user.publicKey,
        userCollateralAta,
        depository.collateralMint
      );
      tx.add(createUserCollateralAtaIx);
    }
  }

  tx.add(mintWithMangoDepositoryIx);
  signers.push(user);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function quoteMintWithMangoDepository(
  user: Signer,
  payer: Signer,
  quoteAmount: number,
  controller: Controller,
  depository: MangoDepository,
  mango: Mango
): Promise<string> {
  const quoteMintWithMangoDepositoryIx = uxdClient.createQuoteMintWithMangoDepositoryInstruction(
    quoteAmount,
    controller,
    depository,
    mango,
    user.publicKey,
    TXN_OPTS,
    payer.publicKey
  );
  let signers = [];
  let tx = new Transaction();

  const userRedeemableAta = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
  if (!(await getConnection().getAccountInfo(userRedeemableAta))) {
    const createRedeemableAtaIx = createAssocTokenIx(user.publicKey, userRedeemableAta, controller.redeemableMintPda);
    tx.add(createRedeemableAtaIx);
  }

  tx.add(quoteMintWithMangoDepositoryIx);
  signers.push(user);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function redeemFromMangoDepository(
  user: Signer,
  payer: Signer,
  slippage: number,
  amountRedeemable: number,
  controller: Controller,
  depository: MangoDepository,
  mango: Mango
): Promise<string> {
  const redeemFromMangoDepositoryIx = await uxdClient.createRedeemFromMangoDepositoryInstruction(
    amountRedeemable,
    slippage,
    controller,
    depository,
    mango,
    user.publicKey,
    TXN_OPTS,
    payer.publicKey
  );

  let signers = [];
  let tx = new Transaction();

  const userCollateralAta = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
  if (!(await getConnection().getAccountInfo(userCollateralAta))) {
    const createUserCollateralAtaIx = createAssocTokenIx(user.publicKey, userCollateralAta, depository.collateralMint);
    tx.add(createUserCollateralAtaIx);
  }

  const userRedeemableAta = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
  if (!(await getConnection().getAccountInfo(userRedeemableAta))) {
    const createUserRedeemableAtaIx = createAssocTokenIx(
      user.publicKey,
      userRedeemableAta,
      controller.redeemableMintPda
    );
    tx.add(createUserRedeemableAtaIx);
  }

  tx.add(redeemFromMangoDepositoryIx);
  signers.push(user);
  if (payer) {
    signers.push(payer);
  }

  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function quoteRedeemFromMangoDepository(
  user: Signer,
  payer: Signer,
  redeemableAmount: number,
  controller: Controller,
  depository: MangoDepository,
  mango: Mango
): Promise<string> {
  const quoteRedeemFromMangoDepositoryIx = uxdClient.createQuoteRedeemWithMangoDepositoryInstruction(
    redeemableAmount,
    controller,
    depository,
    mango,
    user.publicKey,
    TXN_OPTS,
    payer.publicKey
  );
  let signers = [];
  let tx = new Transaction();

  const userRedeemableAta = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
  if (!(await getConnection().getAccountInfo(userRedeemableAta))) {
    const createUserRedeemableAtaIx = createAssocTokenIx(
      user.publicKey,
      userRedeemableAta,
      controller.redeemableMintPda
    );
    tx.add(createUserRedeemableAtaIx);
  }

  const userQuoteATA = findATAAddrSync(user.publicKey, depository.quoteMint)[0];
  if (!(await getConnection().getAccountInfo(userQuoteATA))) {
    const createUserQuoteAtaIx = createAssocTokenIx(user.publicKey, userQuoteATA, depository.quoteMint);
    tx.add(createUserQuoteAtaIx);
  }

  await depository.settleMangoDepositoryMangoAccountPnl(payer as Payer, mango);

  tx.add(quoteRedeemFromMangoDepositoryIx);
  signers.push(user);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function setMangoDepositoryQuoteMintAndRedeemFee(
  authority: Signer,
  controller: Controller,
  depository: MangoDepository,
  quoteFee: number
): Promise<string> {
  const setMangoDepositoryQuoteMintAndRedeemFeeIx = uxdClient.createSetMangoDepositoryQuoteMintAndRedeemFeeInstruction(
    quoteFee,
    controller,
    depository,
    authority.publicKey,
    TXN_OPTS
  );
  let signers = [];
  let tx = new Transaction();

  tx.instructions.push(setMangoDepositoryQuoteMintAndRedeemFeeIx);
  signers.push(authority);

  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function setMangoDepositoryQuoteMintAndRedeemSoftCap(
  authority: Signer,
  controller: Controller,
  depository: MangoDepository,
  quoteMintAndRedeemSoftCap: number
): Promise<string> {
  const setMangoDepositoryQuoteMintAndRedeemSoftCapIx =
    uxdClient.createSetMangoDepositoryQuoteMintAndRedeemSoftCapInstruction(
      quoteMintAndRedeemSoftCap,
      controller,
      depository,
      authority.publicKey,
      TXN_OPTS
    );
  let signers = [];
  let tx = new Transaction();

  tx.instructions.push(setMangoDepositoryQuoteMintAndRedeemSoftCapIx);
  signers.push(authority);

  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function editController(
  authority: Signer,
  controller: Controller,
  uiFields: {
    quoteMintAndRedeemSoftCap?: {
      value: number;
      depository: MangoDepository;
    };
    redeemableSoftCap?: number;
    redeemableGlobalSupplyCap?: number;
  }
): Promise<string> {
  const editControllerIx = uxdClient.createEditControllerInstruction(
    controller,
    authority.publicKey,
    uiFields,
    TXN_OPTS
  );
  let signers = [];
  let tx = new Transaction();

  tx.instructions.push(editControllerIx);
  signers.push(authority);

  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function rebalanceMangoDepositoryLite(
  user: Signer,
  payer: Signer,
  rebalancingMaxAmountQuote: number,
  polarity: PnLPolarity,
  slippage: number,
  controller: Controller,
  depository: MangoDepository,
  mango: Mango
): Promise<string> {
  const rebalanceMangoDepositoryLiteIx = await uxdClient.createRebalanceMangoDepositoryLiteInstruction(
    rebalancingMaxAmountQuote,
    slippage,
    polarity,
    controller,
    depository,
    mango,
    user.publicKey,
    TXN_OPTS,
    payer.publicKey
  );
  let signers = [];
  let tx = new Transaction();

  // Only when polarity is positive this is required
  // - Negative polarity sends QUOTE, gets COLLATERAL back.
  // - Positive polarity sends COLLATERAL, gets QUOTE back.
  if (polarity == PnLPolarity.Positive && depository.collateralMint.equals(NATIVE_MINT)) {
    const mangoPerpPrice = await depository.getCollateralPerpPriceUI(mango);
    // Transfer extra collateral to ensure there are enough due to price change
    const rebalancingMaxAmountCollateral = (rebalancingMaxAmountQuote / mangoPerpPrice) * 1.01;
    const nativeAmount = uiToNative(rebalancingMaxAmountCollateral, depository.collateralMintDecimals);
    const prepareWrappedSolIxs = await prepareWrappedSolTokenAccount(
      getConnection(),
      payer.publicKey,
      user.publicKey,
      nativeAmount.toNumber()
    );
    tx.add(...prepareWrappedSolIxs);
  }

  const userCollateralAta = findATAAddrSync(user.publicKey, depository.collateralMint)[0];

  if (!(await getConnection().getAccountInfo(userCollateralAta)) && !depository.collateralMint.equals(NATIVE_MINT)) {
    const createUserCollateralAtaIx = createAssocTokenIx(user.publicKey, userCollateralAta, depository.collateralMint);
    tx.add(createUserCollateralAtaIx);
  }

  const userQuoteATA = findATAAddrSync(user.publicKey, depository.quoteMint)[0];

  if (!(await getConnection().getAccountInfo(userQuoteATA))) {
    const createUserQuoteAtaIx = createAssocTokenIx(user.publicKey, userQuoteATA, depository.quoteMint);
    tx.add(createUserQuoteAtaIx);
  }

  signers.push(user);
  if (payer) {
    signers.push(payer);
  }
  tx.add(rebalanceMangoDepositoryLiteIx);
  tx.feePayer = payer.publicKey;
  let txId = web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);

  // PNL should be settled afterward to ensure we have no "borrow" to prevent paying interests
  // const settlePnlTxID = await settleDepositoryPnl(payer, depository, mango);
  // console.log("🔗 depository PnL settlement Tx:", `'https://explorer.solana.com/tx/${settlePnlTxID}?cluster=${CLUSTER}'`);

  return txId;
}

export async function editMangoDepository(
  authority: Signer,
  controller: Controller,
  depository: MangoDepository,
  uiFields: {
    quoteMintAndRedeemFee?: number;
    redeemableDepositorySupplyCap?: BN;
  }
): Promise<string> {
  const editMangoDepositoryIx = uxdClient.createEditMangoDepositoryInstruction(
    controller,
    depository,
    authority.publicKey,
    uiFields,
    TXN_OPTS
  );
  let signers = [];
  let tx = new Transaction();

  tx.instructions.push(editMangoDepositoryIx);
  signers.push(authority);

  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function editMercurialVaultDepository(
  authority: Signer,
  controller: Controller,
  depository: MercurialVaultDepository,
  uiFields: {
    redeemableDepositorySupplyCap?: BN;
    mintingFeeInBps?: number;
    redeemingFeeInBps?: number;
    mintingDisabled?: boolean;
  }
): Promise<string> {
  const editMercurialVaultDepositoryIx = uxdClient.createEditMercurialVaultDepositoryInstruction(
    controller,
    depository,
    authority.publicKey,
    uiFields,
    TXN_OPTS
  );
  let signers = [];
  let tx = new Transaction();

  tx.instructions.push(editMercurialVaultDepositoryIx);
  signers.push(authority);

  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function disableDepositoryRegularMinting(
  authority: Signer,
  controller: Controller,
  depository: MangoDepository,
  disableMinting: boolean
): Promise<string> {
  const disableDepositoryMintingIx = uxdClient.createDisableDepositoryRegularMintingInstruction(
    disableMinting,
    controller,
    depository,
    authority.publicKey,
    TXN_OPTS
  );
  let signers = [];
  let tx = new Transaction();

  tx.instructions.push(disableDepositoryMintingIx);
  signers.push(authority);

  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

// Non UXD API calls ----------------------------------------------------------

export async function settleDepositoryPnl(payer: Signer, depository: MangoDepository, mango: Mango): Promise<string> {
  return depository.settleMangoDepositoryMangoAccountPnl(payer as Keypair, mango);
}
