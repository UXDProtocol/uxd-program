import { getConnection, TXN_OPTS } from './connection';
import { uxdClient } from './constants';
import { PublicKey, Signer, Transaction } from '@solana/web3.js';
import {
  Controller,
  createAssocTokenIx,
  findATAAddrSync,
  MercurialVaultDepository,
  IdentityDepository,
  CredixLpDepository,
} from '@uxd-protocol/uxd-client';
import { web3 } from '@project-serum/anchor';

export async function initializeController({
  authority,
  payer,
  controller,
}: {
  authority: Signer;
  payer: Signer;
  controller: Controller;
}): Promise<string> {
  const initControllerIx = uxdClient.createInitializeControllerInstruction(
    controller,
    authority.publicKey,
    TXN_OPTS,
    payer.publicKey
  );

  const signers: Signer[] = [];
  const tx = new Transaction();

  tx.instructions.push(initControllerIx);
  signers.push(authority);
  if (payer != authority) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function mintWithMercurialVaultDepository({
  user,
  payer,
  controller,
  depository,
  collateralAmount,
}: {
  user: Signer;
  payer: Signer;
  controller: Controller;
  depository: MercurialVaultDepository;
  collateralAmount: number;
}): Promise<string> {
  const mintWithMercurialVaultDepositoryIx =
    uxdClient.createMintWithMercurialVaultDepositoryInstruction(
      controller,
      depository,
      user.publicKey,
      collateralAmount,
      TXN_OPTS,
      payer.publicKey
    );
  let signers: Signer[] = [];
  let tx = new Transaction();

  const [userRedeemableAta] = findATAAddrSync(
    user.publicKey,
    controller.redeemableMintPda
  );
  if (!(await getConnection().getAccountInfo(userRedeemableAta))) {
    const createUserRedeemableAtaIx = createAssocTokenIx(
      user.publicKey,
      userRedeemableAta,
      controller.redeemableMintPda
    );
    tx.add(createUserRedeemableAtaIx);
  }

  tx.add(mintWithMercurialVaultDepositoryIx);
  signers.push(user);
  if (payer != user) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function redeemFromMercurialVaultDepository({
  user,
  payer,
  controller,
  depository,
  redeemableAmount,
}: {
  user: Signer;
  payer: Signer;
  controller: Controller;
  depository: MercurialVaultDepository;
  redeemableAmount: number;
}): Promise<string> {
  const redeemFromMercurialVaultDepositoryIx =
    uxdClient.createRedeemFromMercurialVaultDepositoryInstruction(
      controller,
      depository,
      user.publicKey,
      redeemableAmount,
      TXN_OPTS,
      payer.publicKey
    );
  let signers: Signer[] = [];
  let tx = new Transaction();

  const [userRedeemableAta] = findATAAddrSync(
    user.publicKey,
    controller.redeemableMintPda
  );
  if (!(await getConnection().getAccountInfo(userRedeemableAta))) {
    const createUserRedeemableAtaIx = createAssocTokenIx(
      user.publicKey,
      userRedeemableAta,
      controller.redeemableMintPda
    );
    tx.add(createUserRedeemableAtaIx);
  }

  tx.add(redeemFromMercurialVaultDepositoryIx);
  signers.push(user);
  if (payer != user) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function registerMercurialVaultDepository({
  authority,
  payer,
  controller,
  depository,
  mintingFeeInBps,
  redeemingFeeInBps,
  redeemableAmountUnderManagementCap,
}: {
  authority: Signer;
  payer: Signer;
  controller: Controller;
  depository: MercurialVaultDepository;
  mintingFeeInBps: number;
  redeemingFeeInBps: number;
  redeemableAmountUnderManagementCap: number;
}): Promise<string> {
  const registerMercurialVaultDepositoryIx =
    uxdClient.createRegisterMercurialVaultDepositoryInstruction(
      controller,
      depository,
      authority.publicKey,
      mintingFeeInBps,
      redeemingFeeInBps,
      redeemableAmountUnderManagementCap,
      TXN_OPTS,
      payer.publicKey
    );
  let signers: Signer[] = [];
  let tx = new Transaction();

  tx.instructions.push(registerMercurialVaultDepositoryIx);
  signers.push(authority);
  if (payer != authority) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function editController({
  authority,
  controller,
  uiFields,
}: {
  authority: Signer;
  controller: Controller;
  uiFields: {
    redeemableGlobalSupplyCap?: number;
  };
}): Promise<string> {
  const editControllerIx = uxdClient.createEditControllerInstruction(
    controller,
    authority.publicKey,
    uiFields,
    TXN_OPTS
  );
  let signers: Signer[] = [];
  let tx = new Transaction();

  tx.instructions.push(editControllerIx);
  signers.push(authority);

  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function editMercurialVaultDepository({
  authority,
  controller,
  depository,
  uiFields,
}: {
  authority: Signer;
  controller: Controller;
  depository: MercurialVaultDepository;
  uiFields: {
    redeemableAmountUnderManagementCap?: number;
    mintingFeeInBps?: number;
    redeemingFeeInBps?: number;
    mintingDisabled?: boolean;
    profitsBeneficiaryCollateral?: PublicKey;
  };
}): Promise<string> {
  const editMercurialVaultDepositoryIx =
    uxdClient.createEditMercurialVaultDepositoryInstruction(
      controller,
      depository,
      authority.publicKey,
      uiFields,
      TXN_OPTS
    );
  let signers: Signer[] = [];
  let tx = new Transaction();

  tx.instructions.push(editMercurialVaultDepositoryIx);
  signers.push(authority);

  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function editIdentityDepository({
  authority,
  controller,
  depository,
  uiFields,
}: {
  authority: Signer;
  controller: Controller;
  depository: IdentityDepository;
  uiFields: {
    redeemableAmountUnderManagementCap?: number;
    mintingDisabled?: boolean;
  };
}): Promise<string> {
  const editIdentityDepositoryIx =
    uxdClient.createEditIdentityDepositoryInstruction(
      controller,
      depository,
      authority.publicKey,
      uiFields,
      TXN_OPTS
    );
  let signers: Signer[] = [];
  let tx = new Transaction();

  tx.instructions.push(editIdentityDepositoryIx);
  signers.push(authority);

  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function initializeIdentityDepository({
  authority,
  payer,
  controller,
  depository,
}: {
  authority: Signer;
  payer: Signer;
  controller: Controller;
  depository: IdentityDepository;
}): Promise<string> {
  const initializeIdentityDepositoryIx =
    uxdClient.createInitializeIdentityDepositoryInstruction(
      controller,
      depository,
      authority.publicKey,
      TXN_OPTS,
      payer.publicKey
    );
  let signers: Signer[] = [];
  let tx = new Transaction();

  tx.instructions.push(initializeIdentityDepositoryIx);
  signers.push(authority);
  if (payer != authority) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function mintWithIdentityDepository({
  user,
  payer,
  controller,
  depository,
  collateralAmount,
}: {
  user: Signer;
  payer: Signer;
  controller: Controller;
  depository: IdentityDepository;
  collateralAmount: number;
}): Promise<string> {
  const mintWithIdentityDepositoryIx =
    uxdClient.createMintWithIdentityDepositoryInstruction(
      controller,
      depository,
      user.publicKey,
      collateralAmount,
      TXN_OPTS,
      payer.publicKey
    );
  let signers: Signer[] = [];
  let tx = new Transaction();

  const [userRedeemableAta] = findATAAddrSync(
    user.publicKey,
    controller.redeemableMintPda
  );
  if (!(await getConnection().getAccountInfo(userRedeemableAta))) {
    const createUserRedeemableAtaIx = createAssocTokenIx(
      user.publicKey,
      userRedeemableAta,
      controller.redeemableMintPda
    );
    tx.add(createUserRedeemableAtaIx);
  }

  tx.add(mintWithIdentityDepositoryIx);
  signers.push(user);
  if (payer != user) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function redeemFromIdentityDepository({
  user,
  payer,
  controller,
  depository,
  redeemableAmount,
}: {
  user: Signer;
  payer: Signer;
  controller: Controller;
  depository: IdentityDepository;
  redeemableAmount: number;
}): Promise<string> {
  const redeemFromIdentityDepositoryIx =
    uxdClient.createRedeemFromIdentityDepositoryInstruction(
      controller,
      depository,
      user.publicKey,
      redeemableAmount,
      TXN_OPTS,
      payer.publicKey
    );
  let signers: Signer[] = [];
  let tx = new Transaction();

  const [userRedeemableAta] = findATAAddrSync(
    user.publicKey,
    controller.redeemableMintPda
  );
  if (!(await getConnection().getAccountInfo(userRedeemableAta))) {
    const createUserRedeemableAtaIx = createAssocTokenIx(
      user.publicKey,
      userRedeemableAta,
      controller.redeemableMintPda
    );
    tx.add(createUserRedeemableAtaIx);
  }

  tx.add(redeemFromIdentityDepositoryIx);
  signers.push(user);
  if (payer != user) {
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
  const registerCredixLpDepositoryIx =
    uxdClient.createRegisterCredixLpDepositoryInstruction(
      controller,
      depository,
      authority.publicKey,
      mintingFeeInBps,
      redeemingFeeInBps,
      redeemableAmountUnderManagementCap,
      TXN_OPTS,
      payer.publicKey
    );
  let signers: Signer[] = [];
  let tx = new Transaction();

  tx.instructions.push(registerCredixLpDepositoryIx);
  signers.push(authority);
  if (payer != authority) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function mintWithCredixLpDepository(
  user: Signer,
  payer: Signer,
  controller: Controller,
  depository: CredixLpDepository,
  collateralAmount: number
): Promise<string> {
  const mintWithCredixLpDepositoryIx =
    uxdClient.createMintWithCredixLpDepositoryInstruction(
      controller,
      depository,
      user.publicKey,
      collateralAmount,
      TXN_OPTS,
      payer.publicKey
    );
  let signers: Signer[] = [];
  let tx = new Transaction();

  const [userRedeemableAta] = findATAAddrSync(
    user.publicKey,
    controller.redeemableMintPda
  );
  if (!(await getConnection().getAccountInfo(userRedeemableAta))) {
    const createUserRedeemableAtaIx = createAssocTokenIx(
      user.publicKey,
      userRedeemableAta,
      controller.redeemableMintPda
    );
    tx.add(createUserRedeemableAtaIx);
  }

  tx.add(mintWithCredixLpDepositoryIx);
  signers.push(user);
  if (payer != user) {
    signers.push(payer);
  }
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function editCredixLpDepository(
  authority: Signer,
  controller: Controller,
  depository: CredixLpDepository,
  uiFields: {
    redeemableAmountUnderManagementCap?: number;
    mintingFeeInBps?: number;
    redeemingFeeInBps?: number;
    mintingDisabled?: boolean;
    profitsBeneficiaryCollateral?: PublicKey;
  }
): Promise<string> {
  const editCredixLpDepositoryIx =
    uxdClient.createEditCredixLpDepositoryInstruction(
      controller,
      depository,
      authority.publicKey,
      uiFields,
      TXN_OPTS
    );
  let signers: Signer[] = [];
  let tx = new Transaction();

  tx.instructions.push(editCredixLpDepositoryIx);
  signers.push(authority);

  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function freezeProgram(
  authority: Signer,
  controller: Controller,
  freeze: boolean
): Promise<string> {
  const freezeProgramIx = uxdClient.createFreezeProgramInstruction(
    freeze,
    controller,
    authority.publicKey,
    TXN_OPTS
  );
  let signers: Signer[] = [];
  let tx = new Transaction();

  tx.instructions.push(freezeProgramIx);
  signers.push(authority);

  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function collectProfitsOfMercurialVaultDepository({
  payer,
  controller,
  depository,
  profitsBeneficiaryCollateral,
}: {
  payer: Signer;
  controller: Controller;
  depository: MercurialVaultDepository;
  profitsBeneficiaryCollateral: PublicKey;
}): Promise<string> {
  const collectInterestsAndFeesFromMercurialVaultDepositoryIx =
    uxdClient.createCollectProfitsOfMercurialVaultDepositoryInstruction(
      controller,
      depository,
      profitsBeneficiaryCollateral,
      TXN_OPTS,
      payer.publicKey
    );

  let signers: Signer[] = [];
  let tx = new Transaction();

  tx.add(collectInterestsAndFeesFromMercurialVaultDepositoryIx);
  signers.push(payer);
  tx.feePayer = payer.publicKey;
  return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}
