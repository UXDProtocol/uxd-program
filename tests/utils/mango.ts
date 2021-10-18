import {
  IDS,
  MangoClient,
  Config,
  Cluster,
  MangoGroup,
  GroupConfig,
  TokenConfig,
  makeUpdateRootBankInstruction,
  zeroKey,
} from "@blockworks-foundation/mango-client";
import { PublicKey, Connection, Account, Transaction } from "@solana/web3.js";

// This has devnet name, but we copy devnet to localnet with the ./configure_local_validator.sh
export const devnetCluster: Cluster = "devnet";
export const devnetGroup = "devnet.2";

// Devnet, but also localnet since we clone with the script ./vonfigure_local_validator.sh
export const MANGO_PROGRAM_ID = new PublicKey("4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA");

// READ https://mirror.xyz/0x9fEcc73Da3f8bd2aC436547a72f8Dd32326D90dc/u05KU4oE4tnlI4Z5Yj-TAQSq8bgZuu4Mv2s3wblpkTs
export class Mango {
  mangoGroupKey: PublicKey;
  groupConfig: GroupConfig;
  public client: MangoClient;
  public group: MangoGroup;

  public constructor(cluster: Cluster, group: string) {
    const config = new Config(IDS);
    this.groupConfig = config.getGroup(cluster, group);
    if (!this.groupConfig) {
      throw new Error("unable to get mango group config");
    }
    this.mangoGroupKey = this.groupConfig.publicKey;

    const clusterData = IDS.groups.find((g) => {
      return g.name == group && g.cluster == cluster;
    });
    const mangoProgramIdPk = new PublicKey(clusterData.mangoProgramId);
    const clusterUrl = IDS.cluster_urls[cluster];
    const connection = new Connection(clusterUrl, "singleGossip");
    this.client = new MangoClient(connection, mangoProgramIdPk);
  }

  async setupMangoGroup() {
    this.group = await this.client.getMangoGroup(this.mangoGroupKey);
    this.group.rootBankAccounts = await this.group.loadRootBanks(this.client.connection);
    this.group.mangoCache = (await this.group.loadCache(this.client.connection)).publicKey;
  }

  getTokenIndex(tokenMint: PublicKey): number {
    // look up token index by mint public key
    return this.group.getTokenIndex(tokenMint);
  }

  getTokenConfig(tokenIndex: number): TokenConfig {
    return this.groupConfig.tokens.find((_config, index) => {
      return index == tokenIndex;
    });
  }

  getRootBankForToken(tokenIndex: number): PublicKey {
    const rootBank = this.group.rootBankAccounts[tokenIndex];
    if (!rootBank) {
      throw new Error("vault is undefined");
    }
    return rootBank.publicKey;
  }

  getNodeBankFor(tokenIndex: number): PublicKey {
    const tokenConfig = this.getTokenConfig(tokenIndex);
    const nodeBank = this.group.rootBankAccounts[tokenIndex].nodeBankAccounts.find((node, _index) => {
      if (!node) {
        return false;
      }
      return node.publicKey.toBase58 == tokenConfig.rootKey.toBase58;
    });
    if (!nodeBank) {
      throw new Error("nodeBank is undefined");
    }
    return nodeBank.publicKey;
  }

  getVaultFor(tokenIndex: number): PublicKey {
    const vault = this.group.rootBankAccounts[tokenIndex].nodeBankAccounts[0].vault;
    if (!vault) {
      throw new Error("vault is undefined");
    }
    return vault;
  }

  getMangoCache(): PublicKey {
    return this.group.mangoCache;
  }

  async createUpdateRootBankTx(): Promise<Transaction> {
    const rootBanks = await this.group.loadRootBanks(this.client.connection);

    const updateRootBankTransaction = new Transaction();
    this.FIXED_IDS.forEach((token, i) => {
      if (rootBanks[i]) {
        updateRootBankTransaction.add(
          makeUpdateRootBankInstruction(
            MANGO_PROGRAM_ID,
            this.group.publicKey,
            this.group.mangoCache,
            rootBanks[i]!.publicKey,
            rootBanks[i]!.nodeBanks.filter((n) => !n.equals(zeroKey))
          )
        );
      }
    });
    return updateRootBankTransaction;
  }

  // //https://github.com/blockworks-foundation/mango-client-v3/blob/ce76884d7899c5502e50d05d5bba513dd28bdd6b/test/TestGroup.ts#L272
  // async runKeeper() {
  //   console.log("runKeeper");
  //   // if (!this.log) {
  //   //   console.log = function () {};
  //   // }
  //   await this.updateCache();
  //   await this.updateBanksAndMarkets();
  //   // if (!this.log) {
  //   //   console.log = this.logger;
  //   // }

  //   await this.consumeEvents();
  // }

  FIXED_IDS: any[] = [
    {
      symbol: "MNGO",
      decimals: 6,
      baseLot: 1000000,
      quoteLot: 100,
      initLeverage: 1.25,
      maintLeverage: 2.5,
      liquidationFee: 0.2,
      oracleProvider: "switchboard",
      mint: "Bb9bsTQa1bGEtQ5KagGkvSHyuLqDWumFUcRqFusFNJWC",
    },
    {
      symbol: "BTC",
      decimals: 6,
      baseLot: 100,
      quoteLot: 10,
      price: 45000,
      mint: "3UNBZ6o52WTWwjac2kPUb4FyodhU1vFkRJheu1Sh2TvU",
    },
    {
      symbol: "ETH",
      decimals: 6,
      baseLot: 1000,
      quoteLot: 10,
      oracleProvider: "pyth",
      mint: "Cu84KB3tDL6SbFgToHMLYVDJJXdJjenNzSKikeAvzmkA",
    },
    {
      symbol: "SOL",
      decimals: 9,
      baseLot: 100000000,
      quoteLot: 100,
      oracleProvider: "pyth",
      mint: "So11111111111111111111111111111111111111112",
    },
    {
      symbol: "SRM",
      decimals: 6,
      baseLot: 100000,
      quoteLot: 100,
      oracleProvider: "pyth",
      mint: "AvtB6w9xboLwA145E221vhof5TddhqsChYcx7Fy3xVMH",
    },
    {
      symbol: "RAY",
      decimals: 6,
      baseLot: 100000,
      quoteLot: 100,
      oracleProvider: "pyth",
      mint: "3YFQ7UYJ7sNGpXTKBxM3bYLVxKpzVudXAe4gLExh5b3n",
      initLeverage: 3,
      maintLeverage: 6,
      liquidationFee: 0.0833,
    },
    {
      symbol: "USDT",
      decimals: 6,
      baseLot: 1000000,
      quoteLot: 100,
      oracleProvider: "pyth",
      mint: "DAwBSXe6w9g37wdE2tCrFbho3QHKZi4PjuBytQCULap2",
      initLeverage: 10,
      maintLeverage: 20,
      liquidationFee: 0.025,
    },
    {
      symbol: "USDC",
      decimals: 6,
      mint: "8FRFC6MoGGkMFQwngccyu69VnYbzykGeez7ignHVAFSN",
    },
  ];

  // async updateBanksAndMarkets() {
  //   const payer = new Account(wallet.payer.secretKey);
  //   console.log("processKeeperTransactions");
  //   const promises: Promise<string>[] = [];
  //   const mangoGroup = await this.client.getMangoGroup(this.mangoGroupKey);
  //   const perpMarkets = await Promise.all(
  //     [1, 3].map((marketIndex) => {
  //       return mangoGroup.loadPerpMarket(this.client.connection, marketIndex, 6, 6);
  //     })
  //   );
  //   const rootBanks = await mangoGroup.loadRootBanks(this.client.connection);

  //   const updateRootBankTransaction = new Transaction();
  //   this.FIXED_IDS.forEach((token, i) => {
  //     if (rootBanks[i]) {
  //       updateRootBankTransaction.add(
  //         makeUpdateRootBankInstruction(
  //           MANGO_PROGRAM_ID,
  //           mangoGroup.publicKey,
  //           mangoGroup.mangoCache,
  //           rootBanks[i]!.publicKey,
  //           rootBanks[i]!.nodeBanks.filter((n) => !n.equals(zeroKey))
  //         )
  //       );
  //     }
  //   });

  //   const updateFundingTransaction = new Transaction();
  //   perpMarkets.forEach((market) => {
  //     if (market) {
  //       updateFundingTransaction.add(
  //         makeUpdateFundingInstruction(
  //           MANGO_PROGRAM_ID,
  //           mangoGroup.publicKey,
  //           mangoGroup.mangoCache,
  //           market.publicKey,
  //           market.bids,
  //           market.asks
  //         )
  //       );
  //     }
  //   });

  //   if (updateRootBankTransaction.instructions.length > 0) {
  //     promises.push(this.client.sendTransaction(updateRootBankTransaction, payer, []));
  //   }
  //   if (updateFundingTransaction.instructions.length > 0) {
  //     promises.push(this.client.sendTransaction(updateFundingTransaction, payer, []));
  //   }

  //   await Promise.all(promises);
  // }

  // async consumeEvents() {
  //   const payer = new Account(wallet.payer.secretKey);
  //   console.log("processConsumeEvents");
  //   const mangoGroup = await this.client.getMangoGroup(this.mangoGroupKey);
  //   const perpMarkets = await Promise.all(
  //     [1, 3].map((marketIndex) => {
  //       return mangoGroup.loadPerpMarket(this.client.connection, marketIndex, 6, 6);
  //     })
  //   );
  //   const eventQueuePks = perpMarkets.map((mkt) => mkt.eventQueue);
  //   const eventQueueAccts = await getMultipleAccounts(this.client.connection, eventQueuePks);

  //   const perpMktAndEventQueue = eventQueueAccts.map(({ publicKey, accountInfo }) => {
  //     const parsed = PerpEventQueueLayout.decode(accountInfo?.data);
  //     const eventQueue = new PerpEventQueue(parsed);
  //     const perpMarket = perpMarkets.find((mkt) => mkt.eventQueue.equals(publicKey));
  //     if (!perpMarket) {
  //       throw new Error("PerpMarket not found");
  //     }
  //     return { perpMarket, eventQueue };
  //   });

  //   for (let i = 0; i < perpMktAndEventQueue.length; i++) {
  //     const { perpMarket, eventQueue } = perpMktAndEventQueue[i];

  //     const events = eventQueue.getUnconsumedEvents();
  //     if (events.length === 0) {
  //       console.log("No events to consume");
  //       continue;
  //     }

  //     const accounts: Set<string> = new Set();
  //     for (const event of events) {
  //       if (event.fill) {
  //         accounts.add(event.fill.maker.toBase58());
  //         accounts.add(event.fill.taker.toBase58());
  //       } else if (event.out) {
  //         accounts.add(event.out.owner.toBase58());
  //       }

  //       // Limit unique accounts to first 10
  //       if (accounts.size >= 10) {
  //         break;
  //       }
  //     }

  //     await this.client.consumeEvents(
  //       mangoGroup,
  //       perpMarket,
  //       Array.from(accounts)
  //         .map((s) => new PublicKey(s))
  //         .sort(),
  //       payer,
  //       new BN(5)
  //     );
  //     console.log(`Consumed up to ${events.length} events`);
  //   }
  // }

  // async updateCache() {
  //   const payer = new Account(wallet.payer.secretKey);
  //   console.log("processUpdateCache");
  //   const batchSize = 8;
  //   const promises: Promise<string>[] = [];
  //   const mangoGroup = await this.client.getMangoGroup(this.mangoGroupKey);
  //   const rootBanks = mangoGroup.tokens.map((t) => t.rootBank).filter((t) => !t.equals(zeroKey));
  //   const oracles = mangoGroup.oracles.filter((o) => !o.equals(zeroKey));
  //   const perpMarkets = mangoGroup.perpMarkets.filter((pm) => !pm.isEmpty()).map((pm) => pm.perpMarket);

  //   for (let i = 0; i < rootBanks.length / batchSize; i++) {
  //     const startIndex = i * batchSize;
  //     const endIndex = i * batchSize + batchSize;
  //     const cacheTransaction = new Transaction();
  //     cacheTransaction.add(
  //       makeCacheRootBankInstruction(
  //         MANGO_PROGRAM_ID,
  //         mangoGroup.publicKey,
  //         mangoGroup.mangoCache,
  //         rootBanks.slice(startIndex, endIndex)
  //       )
  //     );

  //     cacheTransaction.add(
  //       makeCachePricesInstruction(
  //         MANGO_PROGRAM_ID,
  //         mangoGroup.publicKey,
  //         mangoGroup.mangoCache,
  //         oracles.slice(startIndex, endIndex)
  //       )
  //     );

  //     cacheTransaction.add(
  //       makeCachePerpMarketsInstruction(
  //         MANGO_PROGRAM_ID,
  //         mangoGroup.publicKey,
  //         mangoGroup.mangoCache,
  //         perpMarkets.slice(startIndex, endIndex)
  //       )
  //     );
  //     if (cacheTransaction.instructions.length > 0) {
  //       promises.push(this.client.sendTransaction(cacheTransaction, payer, []));
  //     }
  //   }

  //   await Promise.all(promises);
  // }
}
