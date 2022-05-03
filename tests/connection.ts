import { AnchorProvider } from "@project-serum/anchor";
import { Commitment, Connection } from "@solana/web3.js";

// TXN preflight checks options
export const TXN_COMMIT: Commitment = "processed";

export const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: true,
};

export function getConnection(): Connection {
  const provider = AnchorProvider.env();
  return provider.connection;
}