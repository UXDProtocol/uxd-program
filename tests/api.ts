import { getConnection, TXN_OPTS } from "./connection";
import { uxdClient } from "./constants";
import { Signer, Transaction } from "@solana/web3.js";
import {
  Controller,
  createAssocTokenIx,
  findATAAddrSync,
  MercurialVaultDepository,
  IdentityDepository,
} from "@uxd-protocol/uxd-client";
import { BN, web3 } from "@project-serum/anchor";

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

export async function mintWithMercurialVaultDepository(
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: MercurialVaultDepository,
  collateralAmount: number
): Promise<string> {
  const mintWithMercurialVaultDepositoryIx = uxdClient.createMintWithMercurialVaultDepositoryInstruction(
    controller,
    depository,
    authority.publicKey,
    collateralAmount,
    TXN_OPTS,
    payer.publicKey
  );
  let signers = [];
  let tx = new Transaction();

  const [authorityRedeemableAta] = findATAAddrSync(authority.publicKey, controller.redeemableMintPda);
  if (!(await getConnection().getAccountInfo(authorityRedeemableAta))) {
    const createUserRedeemableAtaIx = createAssocTokenIx(
      authority.publicKey,
      authorityRedeemableAta,
      controller.redeemableMintPda
    );
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
  redeemableAmount: number
): Promise<string> {
  const redeemFromMercurialVaultDepositoryIx = uxdClient.createRedeemFromMercurialVaultDepositoryInstruction(
    controller,
    depository,
    authority.publicKey,
    redeemableAmount,
    TXN_OPTS,
    payer.publicKey
  );
  let signers = [];
  let tx = new Transaction();

  const [authorityRedeemableAta] = findATAAddrSync(authority.publicKey, controller.redeemableMintPda);
  if (!(await getConnection().getAccountInfo(authorityRedeemableAta))) {
    const createUserRedeemableAtaIx = createAssocTokenIx(
      authority.publicKey,
      authorityRedeemableAta,
      controller.redeemableMintPda
    );
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
  redeemableAmountUnderManagementCap: number
): Promise<string> {
  const registerMercurialVaultDepositoryIx = uxdClient.createRegisterMercurialVaultDepositoryInstruction(
    controller,
    depository,
    authority.publicKey,
    mintingFeeInBps,
    redeemingFeeInBps,
    redeemableAmountUnderManagementCap,
    TXN_OPTS,
    payer.publicKey
  );
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

export async function editController(
  authority: Signer,
  controller: Controller,
  uiFields: {
    quoteMintAndRedeemSoftCap?: {
      value: number;
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

export async function editMercurialVaultDepository(
  authority: Signer,
  controller: Controller,
  depository: MercurialVaultDepository,
  uiFields: {
    redeemableAmountUnderManagementCap?: BN;
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

export async function initializeIdentityDepository(
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: IdentityDepository,
): Promise<string> {
  const initializeIdentityDepositoryIx = uxdClient.createInitializeIdentityDepositoryInstruction(
    controller,
    depository,
    authority.publicKey,
    TXN_OPTS,
    payer.publicKey
  );
  let signers = [];
  let tx = new Transaction();

  tx.instructions.push(initializeIdentityDepositoryIx);
  signers.push(authority);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}


export async function mintWithIdentityDepository(
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: IdentityDepository,
  collateralAmount: number
): Promise<string> {
  const mintWithIdentityDepositoryIx = uxdClient.createMintWithIdentityDepositoryInstruction(
    controller,
    depository,
    authority.publicKey,
    collateralAmount,
    TXN_OPTS,
    payer.publicKey
  );
  let signers = [];
  let tx = new Transaction();

  const [authorityRedeemableAta] = findATAAddrSync(authority.publicKey, controller.redeemableMintPda);
  if (!(await getConnection().getAccountInfo(authorityRedeemableAta))) {
    const createUserRedeemableAtaIx = createAssocTokenIx(
      authority.publicKey,
      authorityRedeemableAta,
      controller.redeemableMintPda
    );
    tx.add(createUserRedeemableAtaIx);
  }

  tx.add(mintWithIdentityDepositoryIx);
  signers.push(authority);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function redeemFromIdentityDepository(
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: IdentityDepository,
  redeemableAmount: number
): Promise<string> {
  const redeemFromIdentityDepositoryIx = uxdClient.createRedeemFromIdentityDepositoryInstruction(
    controller,
    depository,
    authority.publicKey,
    redeemableAmount,
    TXN_OPTS,
    payer.publicKey
  );
  let signers = [];
  let tx = new Transaction();

  const [authorityRedeemableAta] = findATAAddrSync(authority.publicKey, controller.redeemableMintPda);
  if (!(await getConnection().getAccountInfo(authorityRedeemableAta))) {
    const createUserRedeemableAtaIx = createAssocTokenIx(
      authority.publicKey,
      authorityRedeemableAta,
      controller.redeemableMintPda
    );
    tx.add(createUserRedeemableAtaIx);
  }

  tx.add(redeemFromIdentityDepositoryIx);
  signers.push(authority);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}