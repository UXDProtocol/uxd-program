import { Provider } from "@project-serum/anchor";
import { Commitment } from "@solana/web3.js";

// TXN preflight checks options
export const TXN_COMMIT: Commitment = "processed";

export const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: false,
};

// Return how much Insurance the mango account holds (SPOT)
export function getProvider(): Provider {
  const provider = Provider.env();
  return provider;
}