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
export class Depository {
  public static ProgramId: PublicKey = anchor.workspace.Depository.programId;
  public static rpc: anchor.RpcNamespace = (anchor.workspace.Depository as Program).rpc;

  public collateralMint: PublicKey;
  public collateralSymbol: string; 
  public oraclePriceAccount: PublicKey;
  // PDAs
  public statePda: PublicKey;
  public redeemableMintPda: PublicKey;
  public depositPda: PublicKey;
  // Mango account
  public mangoAccount: Keypair; // Now using a keypair cause the init must be client side, the span of the account is too big to do so in anchor

  public constructor(mint: PublicKey, mintName: string, oraclePriceAccount: PublicKey) {
    this.collateralMint = mint;
    this.collateralSymbol = mintName;
    this.oraclePriceAccount = oraclePriceAccount; // To remove
    this.mangoAccount = new Keypair();

    this.statePda = this.findDepositoryPda(DepositoryPDASeed.State);
    this.redeemableMintPda = this.findDepositoryPda(DepositoryPDASeed.RedeemableMint);
    this.depositPda = this.findDepositoryPda(DepositoryPDASeed.Deposit);
  }

  // Find the depository program PDA adresse for a given seed - derived from the mint
  private findDepositoryPda(seed: DepositoryPDASeed): PublicKey {
    return utils.findProgramAddressSync(Depository.ProgramId, [
      Buffer.from(seed.toString()),
      this.collateralMint.toBuffer(),
    ])[0];
  }

  public info() {
    console.log(`\
      [Depository debug info - Collateral mint: ${this.collateralSymbol}]
        * mint (collateral):                            ${this.collateralMint.toString()}
        * statePda:                                     ${this.statePda.toString()}
        * redeemableMintPda:                            ${this.redeemableMintPda.toString()}
        * depositPda:                                   ${this.depositPda.toString()}
        * controller's associated depositoryRecordPda:  ${ControllerUXD.depositoryRecordPda(
          this.collateralMint
        ).toString()}
        * controller's associated coinPassthroughPda:   ${ControllerUXD.coinPassthroughPda(
          this.collateralMint
        ).toString()}`);
  }
}
