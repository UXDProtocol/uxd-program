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
import { Provider } from "@project-serum/anchor";
import { PublicKey, Connection } from "@solana/web3.js";

export const devnetGroup = "devnet.2";
export const mainnetGroup = "mainnet.1";
export const MANGO_PROGRAM_ID_DEVNET = new PublicKey("4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA");
export const MANGO_PROGRAM_ID_MAINNET = new PublicKey("mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68");

export class Mango {
  mangoGroupKey: PublicKey;
  groupConfig: GroupConfig | undefined;
  public client: MangoClient;
  public group: MangoGroup;
  public programId: PublicKey;

  public constructor(provider: Provider, cluster: Cluster) {
    let mangoGroup: string;
    switch (cluster) {
      case "devnet":
        mangoGroup = devnetGroup;
        this.programId = MANGO_PROGRAM_ID_DEVNET;
        break;
      case "mainnet":
        mangoGroup = mainnetGroup;
        this.programId = MANGO_PROGRAM_ID_MAINNET;
        break;
    }
    const config = new Config(IDS);
    this.groupConfig = config.getGroup(cluster, mangoGroup);
    if (!this.groupConfig) {
      throw new Error("unable to get mango group config");
    }
    this.mangoGroupKey = this.groupConfig.publicKey;

    const clusterData = IDS.groups.find((g) => {
      return g.name == mangoGroup && g.cluster == cluster;
    });
    const mangoProgramIdPk = new PublicKey(clusterData.mangoProgramId);
    // const clusterUrl = IDS.cluster_urls[cluster];
    // const connection = new Connection(clusterUrl, "singleGossip");
    this.client = new MangoClient(provider.connection, mangoProgramIdPk);
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
}
