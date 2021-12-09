import { Provider, setProvider } from "@project-serum/anchor";
import { Commitment } from "@solana/web3.js";

// TXN prefight checks options
export const TXN_COMMIT: Commitment = "processed";

export const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: false,
};

// Provider
export const provider = Provider.env();
setProvider(provider);
