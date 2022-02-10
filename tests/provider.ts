import { Provider } from "@project-serum/anchor";
import { Commitment, Connection } from "@solana/web3.js";

// TXN preflight checks options
export const TXN_COMMIT: Commitment = "processed";

export const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: false,
};

export function getConnection(): Connection {
  const provider = Provider.env();
  return provider.connection;
}