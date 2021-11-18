import { Provider, setProvider } from "@project-serum/anchor";
import { Commitment } from "@solana/web3.js";

// TXN prefight checks options
export const TXN_COMMIT: Commitment = "singleGossip";

export const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: false,
};

// Provider
export const provider = Provider.local("https://api.devnet.solana.com", TXN_OPTS); // should use provider from env
setProvider(provider);
