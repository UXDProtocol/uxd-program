import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Token } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { findAddr } from "./utils";

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
  static ProgramId: PublicKey = anchor.workspace.Depository.programId;

  // The Depository Solana program (pointer)
  public program: Program;
  // The collateral
  public mint: Token;
  public mintName: string;  // For debug purpose mostly
  public oraclePriceAccount: PublicKey;
  // Depository PDAs
  public statePda: PublicKey;
  public redeemableMintPda: PublicKey;
  public depositPda: PublicKey;

  public constructor(mint: Token, mintName: string, oraclePriceAccount: PublicKey) {
    this.program = anchor.workspace.Depository;

    this.mint = mint;
    this.mintName = mintName;
    this.oraclePriceAccount = oraclePriceAccount;

    this.statePda = this.findDepositoryPda(DepositoryPDASeed.State);
    this.redeemableMintPda = this.findDepositoryPda(DepositoryPDASeed.RedeemableMint);
    this.depositPda = this.findDepositoryPda(DepositoryPDASeed.Deposit);
  }

  // Find the depository program PDA adresse for a given seed - derived from the mint
  private findDepositoryPda(seed: DepositoryPDASeed): PublicKey {
    return findAddr(
      [Buffer.from(seed.toString()), this.mint.publicKey.toBuffer()],
      Depository.ProgramId
    );
  }
}
