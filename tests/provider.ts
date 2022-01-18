import { Provider } from "@project-serum/anchor";
import { Commitment } from "@solana/web3.js";

// TXN preflight checks options
export const TXN_COMMIT: Commitment = "confirmed";

export const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: false,
};

// Provider
// export const provider = Provider.env();
// setProvider(provider);

// Return how much Insurance the mango account holds (SPOT)
export function getProvider(): Provider {
  const provider = Provider.env();
  return provider;
}