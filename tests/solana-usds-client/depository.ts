import { PublicKey } from "@solana/web3.js";
import { ControllerUXD } from "./controller";

enum DepositoryPDASeed {
  State = "STATE",
  RedeemableMint = "REDEEMABLE",
  Deposit = "DEPOSIT",
}

// XXX this is rather messed up and anemic now but we need something to hold the mints still.
export class Depository {
  public collateralMint: PublicKey;
  public collateralSymbol: string;
  public decimals: number;

  public constructor(mint: PublicKey, mintName: string, decimals: number) {
    this.collateralMint = mint;
    this.collateralSymbol = mintName;
    this.decimals = decimals;
  }

  public info() {
    console.log(`\
      [Depository debug info - Collateral mint: ${this.collateralSymbol}]
        * mint (collateral):                            ${this.collateralMint.toString()}
        * controller's associated depositoryRecordPda:  ${ControllerUXD.depositoryPda(this.collateralMint).toString()}
        * controller's associated coinPassthroughPda:   ${ControllerUXD.collateralPassthroughPda(
          this.collateralMint
        ).toString()}
        * controller's associated MangoAccountPda:      ${ControllerUXD.mangoPda(this.collateralMint).toString()}
        `);
  }
}
