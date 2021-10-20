import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Token } from "@solana/spl-token";
import { PublicKey, Keypair } from "@solana/web3.js";
import { ControllerUXD } from "./controller";
import { utils } from "./utils";

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
  public collateralName: string; // For debug purpose
  public oraclePriceAccount: PublicKey; // TO remove
  // PDAs
  // Mango account
  public mangoAccount: Keypair; // Now using a keypair cause the init must be client side, the span of the account is too big to do so in anchor

  public constructor(mint: PublicKey, mintName: string, oraclePriceAccount: PublicKey) {
    this.collateralMint = mint;
    this.collateralName = mintName;
    this.oraclePriceAccount = oraclePriceAccount; // To remove
    this.mangoAccount = new Keypair();
  }

  public info() {
    console.log(`\
      [Depository debug info - Collateral mint: ${this.collateralName}]
        * mint (collateral):                            ${this.collateralMint.toString()}
        * controller's associated depositoryRecordPda:  ${ControllerUXD.depositoryRecordPda(
          this.collateralMint
        ).toString()}
        * controller's associated coinPassthroughPda:   ${ControllerUXD.coinPassthroughPda(
          this.collateralMint
        ).toString()}`);
  }
}
