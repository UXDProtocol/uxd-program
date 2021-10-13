import { IDS, MangoClient, Config, I80F48, Cluster, MangoGroup } from "@blockworks-foundation/mango-client";
import { PublicKey, Connection } from "@solana/web3.js";

// This has devnet name, but we copy devnet to localnet with the ./configure_local_validator.sh
export const devnetCluster: Cluster = "devnet";
export const devnetGroup = "devnet.2";

// Devnet, but also localnet since we clone with the script ./vonfigure_local_validator.sh
export const MANGO_PROGRAM = new PublicKey("4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA");

export class Mango {
  mangoGroupKey: PublicKey;
  public client: MangoClient;
  public mangoGroup: MangoGroup;

  public constructor(cluster: Cluster, group: string) {
    const config = new Config(IDS);
    const groupConfig = config.getGroup(cluster, group);
    if (!groupConfig) {
      throw new Error("unable to get mango group config");
    }
    this.mangoGroupKey = groupConfig.publicKey;

    const clusterData = IDS.groups.find((g) => {
      return g.name == group && g.cluster == cluster;
    });
    const mangoProgramIdPk = new PublicKey(clusterData.mangoProgramId);
    const clusterUrl = IDS.cluster_urls[cluster];
    const connection = new Connection(clusterUrl, "singleGossip");
    this.client = new MangoClient(connection, mangoProgramIdPk);
  }

  async setupMangoGroup() {
    this.mangoGroup = await this.client.getMangoGroup(this.mangoGroupKey);
  }
}
