import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Token } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { Depository } from "./depository_utils";
import { findAddr } from "./utils";

enum ControllerPDASeed {
  State = "STATE",
  UXD = "STABLECOIN",
  Record = "RECORD",
  Passthrough = "PASSTHROUGH",
}

export class Controller {
  // keeping this in both class to convey the meaning that there is only ONE of each program,
  //  and this is just an abstraction layer
  static ProgramId: PublicKey = anchor.workspace.Controller.programId;

  // The controller Solana program
  public program: Program;
  // Pda
  public statePda: PublicKey;
  public mintPda: PublicKey;

  public constructor() {
    this.program = anchor.workspace.Controller;

    this.statePda = Controller.findControllerPda(ControllerPDASeed.State);
    this.mintPda = Controller.findControllerPda(ControllerPDASeed.UXD);
  }

  public static depositoryRecordPda(collateralMint: Token): PublicKey {
    return findAddr(
      // XXX should remove depository from this
      [Buffer.from(ControllerPDASeed.Record), collateralMint.publicKey.toBuffer()],
      Controller.ProgramId
    );
  }

  // This pda is function of the depository mint
  public static coinPassthroughPda(collateralMint: Token): PublicKey {
    return findAddr(
      [Buffer.from(ControllerPDASeed.Passthrough), collateralMint.publicKey.toBuffer()],
      Controller.ProgramId
    );
  }

  // Find the depository program PDA adresse for a given seed
  private static findControllerPda(seed: ControllerPDASeed): PublicKey {
    return findAddr([Buffer.from(seed.toString())], Controller.ProgramId);
  }

  public info() {
    console.log(`\
      [Controller debug info]
        * statePda:                                     ${this.statePda}
        * mintPda:                                      ${this.mintPda}`);
  }
}
