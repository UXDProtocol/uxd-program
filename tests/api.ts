import { getConnection, TXN_OPTS } from "./connection";
import { uxdClient } from "./constants";
import { Keypair, PublicKey, Signer, Transaction } from "@solana/web3.js";
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
  WSOL,
  MSOL,
} from "@uxd-protocol/uxd-client";
import { web3 } from "@project-serum/anchor";
import { Payer } from "@blockworks-foundation/mango-client";
import { MarinadeConfig } from "@marinade.finance/marinade-ts-sdk";

// Permissionned Calls --------------------------------------------------------

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

export async function registerMangoDepository(
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: MangoDepository,
  mango: Mango
): Promise<string> {
  const registerMangoDepositoryIx = uxdClient.createRegisterMangoDepositoryInstruction(
    controller,
    depository,
    mango,
    authority.publicKey,
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
  const depositInsuranceToMangoDepositoryIx = await uxdClient.createDepositInsuranceToMangoDepositoryInstruction(
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

// Permissionless Calls -------------------------------------------------------

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
    if (prepareWrappedSolIxs.length > 0) {
      tx.add(...prepareWrappedSolIxs);
    }
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
  const quoteMintWithMangoDepositoryIx = await uxdClient.createQuoteMintWithMangoDepositoryInstruction(
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
    const createUserCollateralAtaIx = createAssocTokenIx(
      user.publicKey,
      userRedeemableAta,
      controller.redeemableMintPda
    );
    tx.add(createUserCollateralAtaIx);
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
    const createUserCollateralAtaIx = createAssocTokenIx(
      user.publicKey,
      userRedeemableAta,
      controller.redeemableMintPda
    );
    tx.add(createUserCollateralAtaIx);
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
  const quoteRedeemFromMangoDepositoryIx = await uxdClient.createQuoteRedeemWithMangoDepositoryInstruction(
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

  const userQuoteATA = findATAAddrSync(user.publicKey, depository.quoteMint)[0];
  if (!(await getConnection().getAccountInfo(userQuoteATA))) {
    const createUserCollateralAtaIx = createAssocTokenIx(user.publicKey, userQuoteATA, depository.collateralMint);
    tx.add(createUserCollateralAtaIx);
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
  const setMangoDepositoryQuoteMintAndRedeemFeeIx =
    await uxdClient.createSetMangoDepositoryQuoteMintAndRedeemFeeInstruction(
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
    const rebalancingMaxAmountCollateral = rebalancingMaxAmountQuote / mangoPerpPrice;
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

  tx.add(rebalanceMangoDepositoryLiteIx);
  signers.push(user);
  if (payer) {
    signers.push(payer);
  }

  tx.feePayer = payer.publicKey;
  let txId = web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);

  // PNL should be settled afterward to ensure we have no "borrow" to prevent paying interests
  // const settlePnlTxID = await settleDepositoryPnl(payer, depository, mango);
  // console.log("🔗 depository PnL settlement Tx:", `'https://explorer.solana.com/tx/${settlePnlTxID}?cluster=${CLUSTER}'`);

  return txId;
}

export async function disableDepositoryMinting(
  authority: Signer,
  controller: Controller,
  depository: MangoDepository,
  disableMinting: boolean
): Promise<string> {
  const disableDepositoryMintingIx = await uxdClient.createDisableDepositoryMintingInstruction(
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

export async function createDepositoryMsolConfig(
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: MangoDepository,
  msolConfigPda: PublicKey,
  targetLiquidityRatio: number
): Promise<string> {
  const ix = uxdClient.createDepositoryMsolConfigInstruction(
    targetLiquidityRatio,
    controller,
    depository,
    msolConfigPda,
    authority.publicKey,
    TXN_OPTS,
    payer.publicKey
  );

  const signers = [];
  const tx = new Transaction();

  tx.instructions.push(ix);
  signers.push(authority);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function swapDepositoryMsol(
  user: Signer,
  payer: Signer,
  controller: Controller,
  depository: MangoDepository,
  msolConfigPda: PublicKey,
  mango: Mango
): Promise<string> {
  const marinadeConfig = new MarinadeConfig({
    connection: getConnection(),
    publicKey: user.publicKey,
  });

  const tx = new Transaction();

  // const userWSolAta = findATAAddrSync(user.publicKey, WSOL)[0];
  // if (!(await getConnection().getAccountInfo(userWSolAta))) {
  //   const createUserWSolAtaIx = createAssocTokenIx(user.publicKey, userWSolAta, WSOL);
  //   tx.add(createUserWSolAtaIx);
  // }

  // const userMSolAta = findATAAddrSync(user.publicKey, MSOL)[0];
  // if (!(await getConnection().getAccountInfo(userMSolAta))) {
  //   const createUserMSolAtaIx = createAssocTokenIx(user.publicKey, userMSolAta, MSOL);
  //   tx.add(createUserMSolAtaIx);
  // }

  const ix = await uxdClient.swapDepositoryMsolInstruction(
    controller,
    depository,
    msolConfigPda,
    mango,
    marinadeConfig,
    TXN_OPTS,
    user.publicKey,
    payer.publicKey
  );

  const signers = [];

  tx.instructions.push(ix);
  signers.push(user);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function enableMsolSwap(
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: MangoDepository,
  msolConfigPda: PublicKey,
  enable: boolean
): Promise<string> {
  const ix = uxdClient.enableMsolSwapInstruction(
    enable,
    controller,
    depository,
    msolConfigPda,
    authority.publicKey,
    TXN_OPTS,
    payer.publicKey
  );

  const signers = [];
  const tx = new Transaction();

  tx.instructions.push(ix);
  signers.push(authority);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

// Non UXD API calls ----------------------------------------------------------

export async function settleDepositoryPnl(payer: Signer, depository: MangoDepository, mango: Mango): Promise<string> {
  return depository.settleMangoDepositoryMangoAccountPnl(payer as Keypair, mango);
}

export async function createAta(user: Signer, payer: Signer, mint: PublicKey): Promise<string> {
  const tx = new Transaction();
  const userAta = findATAAddrSync(user.publicKey, mint)[0];
  if (!(await getConnection().getAccountInfo(userAta))) {
    const createUserAtaIx = createAssocTokenIx(user.publicKey, userAta, mint);
    tx.add(createUserAtaIx);
  } else {
    throw new Error("has created")
  }
  const signers = [];

  signers.push(user);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}
