import { getConnection, TXN_OPTS } from "./connection";
import { uxdClient } from "./constants";
import { Signer, Transaction } from "@solana/web3.js";
import {
  MangoDepository,
  Controller,
  createAssocTokenIx,
  findATAAddrSync,
  MercurialVaultDepository,
  MaplePoolDepository,
  CredixLpDepository,
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

export async function mintWithMaplePoolDepository(
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: MaplePoolDepository,
  collateralAmount: number
): Promise<string> {
  const mintWithMaplePoolDepositoryIx = uxdClient.createMintWithMaplePoolDepositoryInstruction(
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

  tx.add(mintWithMaplePoolDepositoryIx);
  signers.push(authority);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function mintWithCredixLpDepository(
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: CredixLpDepository,
  collateralAmount: number
): Promise<string> {
  const mintWithCredixLpDepositoryIx = uxdClient.createMintWithCredixLpDepositoryInstruction(
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

  tx.add(mintWithCredixLpDepositoryIx);
  signers.push(authority);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function redeemFromCredixLpDepository(
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: CredixLpDepository,
  redeemableAmount: number
): Promise<string> {
  const redeemFromCredixLpDepositoryIx = uxdClient.createRedeemFromCredixLpDepositoryInstruction(
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

  tx.add(redeemFromCredixLpDepositoryIx);
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

export async function registerMaplePoolDepository(
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: MaplePoolDepository,
  mintingFeeInBps: number,
  redeemingFeeInBps: number,
  redeemableAmountUnderManagementCap: number
): Promise<string> {
  const registerMaplePoolDepositoryIx = uxdClient.createRegisterMaplePoolDepositoryInstruction(
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

  tx.instructions.push(registerMaplePoolDepositoryIx);
  signers.push(authority);
  if (payer) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function registerCredixLpDepository(
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: CredixLpDepository,
  mintingFeeInBps: number,
  redeemingFeeInBps: number,
  redeemableAmountUnderManagementCap: number
): Promise<string> {
  const registerCredixLpDepositoryIx = uxdClient.createRegisterCredixLpDepositoryInstruction(
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

  tx.instructions.push(registerCredixLpDepositoryIx);
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

export async function editMaplePoolDepository(
  authority: Signer,
  controller: Controller,
  depository: MaplePoolDepository,
  uiFields: {
    redeemableAmountUnderManagementCap?: BN;
    mintingFeeInBps?: number;
    redeemingFeeInBps?: number;
    mintingDisabled?: boolean;
  }
): Promise<string> {
  const editMaplePoolDepositoryIx = uxdClient.createEditMaplePoolDepositoryInstruction(
    controller,
    depository,
    authority.publicKey,
    uiFields,
    TXN_OPTS
  );
  let signers = [];
  let tx = new Transaction();

  tx.instructions.push(editMaplePoolDepositoryIx);
  signers.push(authority);

  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function editCredixLpDepository(
  authority: Signer,
  controller: Controller,
  depository: CredixLpDepository,
  uiFields: {
    redeemableAmountUnderManagementCap?: BN;
    mintingFeeInBps?: number;
    redeemingFeeInBps?: number;
    mintingDisabled?: boolean;
  }
): Promise<string> {
  const editCredixLpDepositoryIx = uxdClient.createEditCredixLpDepositoryInstruction(
    controller,
    depository,
    authority.publicKey,
    uiFields,
    TXN_OPTS
  );
  let signers = [];
  let tx = new Transaction();

  tx.instructions.push(editCredixLpDepositoryIx);
  signers.push(authority);

  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}
