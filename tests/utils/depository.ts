import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Token } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { ControllerUXD } from "./controller";
import { testUtils } from "./utils";

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

  public collateralMint: Token;
  public collateralName: string; // For debug purpose
  public oraclePriceAccount: PublicKey;
  // PDAs
  public statePda: PublicKey;
  public redeemableMintPda: PublicKey;
  public depositPda: PublicKey;

  public constructor(mint: Token, mintName: string, oraclePriceAccount: PublicKey) {
    this.collateralMint = mint;
    this.collateralName = mintName;
    this.oraclePriceAccount = oraclePriceAccount;

    this.statePda = this.findDepositoryPda(DepositoryPDASeed.State);
    this.redeemableMintPda = this.findDepositoryPda(DepositoryPDASeed.RedeemableMint);
    this.depositPda = this.findDepositoryPda(DepositoryPDASeed.Deposit);
  }

  // Find the depository program PDA adresse for a given seed - derived from the mint
  private findDepositoryPda(seed: DepositoryPDASeed): PublicKey {
    return testUtils.findProgramAddressSync(Depository.ProgramId, [
      Buffer.from(seed.toString()),
      this.collateralMint.publicKey.toBuffer(),
    ])[0];
  }

  public info() {
    console.log(`\
      [Depository debug info - Collateral mint: ${this.collateralName}]
        * mint (collateral):                            ${this.collateralMint.publicKey.toString()}
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
