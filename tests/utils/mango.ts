import {
  IDS,
  MangoClient,
  Config,
  I80F48,
  Cluster,
  MangoGroup,
  MangoCache,
  RootBank,
  GroupConfig,
  TokenInfo,
  TokenConfig,
  NodeBank,
} from "@blockworks-foundation/mango-client";
import { PublicKey, Connection } from "@solana/web3.js";

// This has devnet name, but we copy devnet to localnet with the ./configure_local_validator.sh
export const devnetCluster: Cluster = "devnet";
export const devnetGroup = "devnet.2";

// Devnet, but also localnet since we clone with the script ./vonfigure_local_validator.sh
export const MANGO_PROGRAM_ID = new PublicKey("4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA");

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
  async getRootBank(tokenConfig: TokenConfig): Promise<RootBank> {
    const rootBanks = await this.group.loadRootBanks(this.client.connection);
    return rootBanks.find((bank) => {
      if (!bank) {
        return false;
      }
      return bank.publicKey.toBase58 == tokenConfig.rootKey.toBase58;
    });
  }

  async getNodeBank(tokenConfig: TokenConfig, rootBank: RootBank): Promise<NodeBank> {
    const nodeBankAccounts = await rootBank.loadNodeBanks(this.client.connection);
    return nodeBankAccounts.find((node, index) => {
      if (!node) {
        return false;
      }
      return node.publicKey.toBase58 == tokenConfig.rootKey.toBase58;
    });
  }

  async getRootBankForToken(tokenMint: PublicKey): Promise<RootBank> {
    const tokenIndex = this.getTokenIndex(tokenMint);
    const tokenConfig = this.getTokenConfig(tokenIndex);
    const rootBank = await this.getRootBank(tokenConfig);

    if (!rootBank) {
      throw new Error("rootBanks is undefined");
    }
    return rootBank;
  }

  async getNodeBankFor(tokenMint: PublicKey, rootBank: RootBank): Promise<NodeBank> {
    const tokenIndex = this.getTokenIndex(tokenMint);
    const tokenConfig = this.getTokenConfig(tokenIndex);
    const nodeBank = await this.getNodeBank(tokenConfig, rootBank);

    if (!nodeBank) {
      throw new Error("nodeBank is undefined");
    }
    return nodeBank;
  }
}
