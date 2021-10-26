import { PublicKey } from "@solana/web3.js";
import { ControllerUXD } from "./controller";

enum DepositoryPDASeed {
  State = "STATE",
  RedeemableMint = "REDEEMABLE",
  Deposit = "DEPOSIT",
}

// Cleaner API to interact with the Depository program
// To reason in term of collateral, less confusing.
//
// Mints act as Sub Domains for each PDA-cluster, managee by the single stateless head, the Depository program.
//
// == (One)             |    Depository Program
// == (# mints)         |     Mint A                    Mint B                    Mint C
// == (# mints * # PDA) |      XaPDA + YaPDA + ZaPDA     XbPDA + YbPDA + ZbPDA     XcPDA + YcPDA + ZcPDA
//
// FS analogie :
// Depository program is the Zip program
// PDAs derived from a given Mint are a folder of files
//

// XXX this is rather messed up and anemic now but we need something to hold the mints still
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
