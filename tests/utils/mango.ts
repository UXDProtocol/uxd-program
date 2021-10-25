import {
  IDS,
  MangoClient,
  Config,
  Cluster,
  MangoGroup,
  GroupConfig,
  TokenConfig,
  PerpMarketConfig,
} from "@blockworks-foundation/mango-client";
import { PublicKey, Connection } from "@solana/web3.js";

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

  getPerpMarketConfigFor(baseSymbol: string): PerpMarketConfig {
    const perpMarketConfig = this.groupConfig.perpMarkets.find((p) => p.baseSymbol === baseSymbol);
    if (!perpMarketConfig) {
      throw new Error(`Could not find perpMarketConfig for symbol ${baseSymbol}`);
    }
    return perpMarketConfig;
  }

  // They are the same - not just on devnet right?
  // getSpotMarketConfigFor(baseSymbol: string): PerpMarketConfig {
  //   const spotMarketConfig = this.groupConfig.spotMarkets.find((p) => p.baseSymbol === baseSymbol);
  //   if (!spotMarketConfig) {
  //     throw new Error(`Could not find spotMarketConfig for symbol ${baseSymbol}`);
  //   }
  //   return spotMarketConfig;
  // }
}
