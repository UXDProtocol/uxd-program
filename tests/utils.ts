import {
  SOL_DECIMALS,
  findATAAddrSync,
  nativeToUi,
  uiToNative,
  CredixLpDepository,
  MercurialVaultDepository,
  IdentityDepository,
  USDC_DECIMALS,
  USDC_DEVNET,
} from '@uxd-protocol/uxd-client';
import {
  Connection,
  ParsedAccountData,
  PublicKey,
  Signer,
  Transaction,
} from '@solana/web3.js';
import { web3, BN } from '@project-serum/anchor';
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  NATIVE_MINT,
  getMinimumBalanceForRentExemptAccount,
  getOrCreateAssociatedTokenAccount,
  createTransferInstruction,
  TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import { getConnection, TXN_COMMIT, TXN_OPTS } from './connection';
import {
  MERCURIAL_USDC_DEVNET,
  MERCURIAL_USDC_DEVNET_DECIMALS,
  uxdProgramId,
} from './constants';

const SOLANA_FEES_LAMPORT: number = 1238880;

export function ceilAtDecimals(number: number, decimals: number): number {
  return Number(
    (Math.ceil(number * 10 ** decimals) / 10 ** decimals).toFixed(decimals)
  );
}

export async function sendAndConfirmTransaction(
  transaction: Transaction,
  signers: Signer[]
): Promise<string> {
  const result = await web3.sendAndConfirmTransaction(
    getConnection(),
    transaction,
    signers,
    TXN_OPTS
  );
  // As a temporary fix to the flakyness of the solana devnet RPC
  // We add an artificial delay to ensure the transaction went through
  // And to make sure we can actually proceed with the next transaction
  // Without risking race conditions, the extra delay remains small tho
  await new Promise((resolve) => setTimeout(resolve, 2000));
  return result;
}

export function transferSol(
  amountUi: number,
  from: Signer,
  to: PublicKey
): Promise<string> {
  const transaction = new web3.Transaction().add(
    web3.SystemProgram.transfer({
      fromPubkey: from.publicKey,
      toPubkey: to,
      lamports: web3.LAMPORTS_PER_SOL * amountUi,
    })
  );
  return sendAndConfirmTransaction(transaction, [from]);
}

export async function transferAllSol(
  from: Signer,
  to: PublicKey
): Promise<string> {
  const fromBalance = await getSolBalance(from.publicKey);
  const transaction = new web3.Transaction().add(
    web3.SystemProgram.transfer({
      fromPubkey: from.publicKey,
      toPubkey: to,
      lamports: web3.LAMPORTS_PER_SOL * fromBalance - SOLANA_FEES_LAMPORT,
    })
  );
  return sendAndConfirmTransaction(transaction, [from]);
}

export async function transferTokens(
  amountUi: number,
  mint: PublicKey,
  decimals: number,
  from: Signer,
  to: PublicKey
): Promise<string> {
  const sender = await getOrCreateAssociatedTokenAccount(
    getConnection(),
    from,
    mint,
    from.publicKey
  );
  const receiver = await getOrCreateAssociatedTokenAccount(
    getConnection(),
    from,
    mint,
    to
  );
  const transferTokensIx = createTransferInstruction(
    sender.address,
    receiver.address,
    from.publicKey,
    uiToNative(amountUi, decimals).toNumber()
  );
  const transaction = new web3.Transaction().add(transferTokensIx);
  return sendAndConfirmTransaction(transaction, [from]);
}

export async function transferAllTokens(
  mint: PublicKey,
  decimals: number,
  from: Signer,
  to: PublicKey
): Promise<string> {
  const sender = findATAAddrSync(from.publicKey, mint)[0];
  if (!(await getConnection().getAccountInfo(sender))) {
    return 'No account';
  }
  const amountUi = await getBalance(sender);
  return transferTokens(amountUi, mint, decimals, from, to);
}

export async function getSolBalance(wallet: PublicKey): Promise<number> {
  const lamports = await getConnection().getBalance(wallet, TXN_COMMIT);
  return nativeToUi(new BN(lamports), SOL_DECIMALS);
}

export async function getBalance(tokenAccount: PublicKey): Promise<number> {
  try {
    const o = await getConnection().getTokenAccountBalance(
      tokenAccount,
      TXN_COMMIT
    );
    return o.value.uiAmount ?? 0;
  } catch {
    return 0;
  }
}

export const prepareWrappedSolTokenAccount = async (
  connection: Connection,
  payerKey: PublicKey,
  userKey: PublicKey,
  amountNative: number
) => {
  const wsolTokenKey = findAssociatedTokenAddress(userKey, NATIVE_MINT);
  const tokenAccount = await connection.getParsedAccountInfo(wsolTokenKey);
  if (tokenAccount.value) {
    const balanceNative = Number(
      (tokenAccount.value.data as ParsedAccountData).parsed.info.tokenAmount
        .amount
    );
    if (balanceNative < amountNative) {
      return [
        transferSolItx(userKey, wsolTokenKey, amountNative - balanceNative),
        // @ts-expect-error not sure why but it's not in their interface
        createSyncNativeInstruction(TOKEN_PROGRAM_ID, wsolTokenKey),
      ];
    } else {
      // no-op we have everything we need
    }
  } else {
    return createWrappedSolTokenAccount(
      connection,
      payerKey,
      userKey,
      amountNative
    );
  }
  return [];
};

// derives the canonical token account address for a given wallet and mint
function findAssociatedTokenAddress(
  walletKey: PublicKey,
  mintKey: PublicKey
): PublicKey {
  return findAddr(
    [walletKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mintKey.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID
  );
}

// simple shorthand
function findAddr(seeds: Buffer[], programId: PublicKey) {
  return PublicKey.findProgramAddressSync(seeds, programId)[0];
}

/**
 *
 * @param {web3.PublicKey} fromKey
 * @param {web3.PublicKey} toKey
 * @param {number} amountNative
 * @returns {web3.TransactionInstruction}
 */
const transferSolItx = (
  fromKey: PublicKey,
  toKey: PublicKey,
  amountNative: number
) =>
  web3.SystemProgram.transfer({
    fromPubkey: fromKey,
    toPubkey: toKey,
    lamports: amountNative,
  });

const createWrappedSolTokenAccount = async (
  connection: Connection,
  payerKey: PublicKey,
  userKey: PublicKey,
  amountNative: number = 0
) => {
  const assocTokenKey = findAssociatedTokenAddress(userKey, NATIVE_MINT);
  const balanceNeeded = await getMinimumBalanceForRentExemptAccount(connection);

  const transferItx = transferSolItx(
    userKey,
    assocTokenKey,
    amountNative + balanceNeeded
  );
  const createItx = createAssociatedTokenAccountItx(
    payerKey,
    userKey,
    NATIVE_MINT
  );

  return [transferItx, createItx];
};

export function createAssociatedTokenAccountItx(
  payerKey: PublicKey,
  walletKey: PublicKey,
  mintKey: PublicKey
) {
  const assocKey = findAssociatedTokenAddress(walletKey, mintKey);

  return new web3.TransactionInstruction({
    keys: [
      { pubkey: payerKey, isSigner: true, isWritable: true },
      { pubkey: assocKey, isSigner: false, isWritable: true },
      { pubkey: walletKey, isSigner: false, isWritable: false },
      { pubkey: mintKey, isSigner: false, isWritable: false },
      {
        pubkey: web3.SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      {
        pubkey: web3.SYSVAR_RENT_PUBKEY,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId: ASSOCIATED_TOKEN_PROGRAM_ID,
    data: Buffer.alloc(0),
  });
}

export async function createCredixLpDepositoryDevnetUSDC(): Promise<CredixLpDepository> {
  try {
    return await CredixLpDepository.initialize({
      connection: getConnection(),
      uxdProgramId: uxdProgramId,
      collateralMint: new PublicKey(
        'Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr'
      ),
      collateralSymbol: 'USDC(CredixDevnet)',
      credixProgramId: new PublicKey(
        'crdRi38zEhQdzpsxnKur73WHBM9BSvXMSfGcbLyJCdP'
      ),
    });
  } catch (error) {
    console.error('Failed to initialize devnet credix depository');
    throw error;
  }
}

export async function createMercurialVaultDepositoryDevnet(): Promise<MercurialVaultDepository> {
  try {
    return await MercurialVaultDepository.initialize({
      connection: getConnection(),
      collateralMint: {
        mint: MERCURIAL_USDC_DEVNET,
        name: 'USDC',
        symbol: 'USDC',
        decimals: MERCURIAL_USDC_DEVNET_DECIMALS,
      },
      uxdProgramId,
    });
  } catch (error) {
    console.error('Failed to initialize devnet mercurial depository');
    throw error;
  }
}

export function createIdentityDepositoryDevnet(): IdentityDepository {
  return new IdentityDepository(
    USDC_DEVNET,
    'USDC',
    USDC_DECIMALS,
    uxdProgramId
  );
}
