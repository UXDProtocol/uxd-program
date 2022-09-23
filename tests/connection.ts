import { AnchorProvider } from "@project-serum/anchor";
import { Commitment, Connection } from "@solana/web3.js";

// TXN preflight checks options
export const TXN_COMMIT: Commitment = "confirmed";

export const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: true,
};

export function getConnection(): Connection {
  const provider = AnchorProvider.local("https://api.devnet.solana.com");
  return provider.connection;
}
