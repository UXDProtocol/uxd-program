import { Commitment, Connection } from '@solana/web3.js';

// TXN preflight checks options
export const TXN_COMMIT: Commitment = 'confirmed';

export const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: true,
};

export function getConnection(): Connection {
  const connectionConfig = {
    commitment: TXN_COMMIT,
    disableRetryOnRateLimit: false,
    confirmTransactionInitialTimeout: 60000,
  };
  const connection = new Connection(
    'https://api.devnet.solana.com',
    connectionConfig
  );
  return connection;
}
