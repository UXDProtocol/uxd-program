import * as anchor from "@project-serum/anchor";
import { NodeWallet } from "@project-serum/anchor/dist/cjs/provider";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, Token, AccountInfo } from "@solana/spl-token";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { Controller } from "./controller_utils";
import { Depository } from "./depository_utils";

///////////////////////////////////////////////////////////////////////////////

// WALLET
const provider = anchor.Provider.local();
anchor.setProvider(provider);
export const wallet = provider.wallet as NodeWallet;
export const connection = provider.connection;

// CLUSTERS
export const MAINNET = "https://api.mainnet-beta.solana.com";
export const DEVNET = "https://api.devnet.solana.com";
export const TESTNET = "https://api.testnet.solana.com";
export const LOCALNET = "https://api.testnet.solana.com";

// TXN prefight checks options
export const TXN_COMMIT = "processed";
export const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: false,
};

///////////////////////////////////////////////////////////////////////////////

// SHORTHANDS and UTILS
export function findAddr(seeds: (Buffer | Uint8Array)[], programId: PublicKey): PublicKey {
  return anchor.utils.publicKey.findProgramAddressSync(seeds, programId)[0];
}

// derives the canonical token account address for a given wallet and mint
export function findAssocTokenAddr(walletKey: PublicKey, mintKey: PublicKey): PublicKey {
  return findAddr([walletKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mintKey.toBuffer()], ASSOCIATED_TOKEN_PROGRAM_ID);
}

// returns an instruction to create the associated account for a wallet and mint
export function createAssocIxn(walletKey: PublicKey, mintKey: PublicKey): TransactionInstruction {
  let assocKey = findAssocTokenAddr(walletKey, mintKey);

  return new anchor.web3.TransactionInstruction({
    keys: [
      { pubkey: walletKey, isSigner: true, isWritable: true },
      { pubkey: assocKey, isSigner: false, isWritable: true },
      { pubkey: walletKey, isSigner: false, isWritable: false },
      { pubkey: mintKey, isSigner: false, isWritable: false },
      {
        pubkey: anchor.web3.SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      {
        pubkey: anchor.web3.SYSVAR_RENT_PUBKEY,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId: ASSOCIATED_TOKEN_PROGRAM_ID,
    data: Buffer.alloc(0),
  });
}

export async function printBalances(depository: Depository, controller: Controller, wallet: NodeWallet) {
  // XXX why this one doesn't work. Weird stuff I suspect using connection.xyz and mintX.dostuff is not compatible
  const userCollateralTokenAccount = findAssocTokenAddr(wallet.publicKey, depository.mint.publicKey);
  const userRedeemableTokenAccount = findAssocTokenAddr(wallet.publicKey, depository.redeemableMintPda);
  const userUXDTokenAccount = findAssocTokenAddr(wallet.publicKey, controller.uxdMintPda);

  const userCollateralAmount = await getUserTokenBalance(userCollateralTokenAccount);
  const userRedeemableAmount = await getUserTokenBalance(userRedeemableTokenAccount);
  const userUXDAmount = await getUserTokenBalance(userUXDTokenAccount);
  const depositoryCollateralAmount = await getUserTokenBalance(depository.depositPda);
  const controllerCollateralAmount = await getUserTokenBalance(controller.coinPassthroughPda(depository));

  console.log(`\
      [General balances info - Collateral: ${depository.mintName}]
        * user balance:
        *     collateral:                   ${userCollateralAmount}
        *     redeemable:                   ${userRedeemableAmount}
        *     UXD:                          ${userUXDAmount}
        * -- 
        * depository collateral balance:    ${depositoryCollateralAmount}
        * controller collateral balance:    ${controllerCollateralAmount} (Passthrough acc)`);
}

export function printDepositoryInfo(depository: Depository, controller: Controller) {
  console.log(`\
      [Depository debug info - Collateral: ${depository.mintName}]
        * mint (collateral):                            ${depository.mint.publicKey.toString()}
        * statePda:                                     ${depository.statePda.toString()}
        * redeemableMintPda:                            ${depository.redeemableMintPda.toString()}
        * depositPda:                                   ${depository.depositPda.toString()}
        * controller's associated depositoryRecordPda:  ${controller.depositoryRecordPda(depository).toString()}
        * controller's associated coinPassthroughPda:   ${controller.coinPassthroughPda(depository).toString()}`);
}

// returns an instruction to create the associated account for a wallet and mint
export function createAssocTokenIx(wallet: NodeWallet, account: PublicKey, mint: PublicKey): TransactionInstruction {
  return Token.createAssociatedTokenAccountInstruction(
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    mint, // reedeemable mint PDA
    account, // user's reedeemable associated token account
    wallet.publicKey, // owner
    wallet.publicKey // payer
  );
}

export function getUserTokenBalance(mint: PublicKey): Promise<number> {
  return provider.connection
    .getTokenAccountBalance(mint, TXN_COMMIT)
    .then((o) => o["value"]["uiAmount"])
    .catch(() => null);
}
