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
  static ProgramId: PublicKey = anchor.workspace.Controller.programId;

  // The controller Solana program
  public program: Program;
  // Pda
  public statePda: PublicKey;
  public uxdMintPda: PublicKey;

  public constructor() {
    this.program = anchor.workspace.Controller;

    this.statePda = this.findControllerPda(ControllerPDASeed.State);
    this.uxdMintPda = this.findControllerPda(ControllerPDASeed.UXD);

  };

  public depositoryRecordPda(depository: Depository): PublicKey {
    return findAddr(
      [Buffer.from(ControllerPDASeed.Record), Depository.ProgramId.toBuffer(), depository.mint.publicKey.toBuffer()],
      Controller.ProgramId
    );
  }

  // This pda is function of the depository mint
  public coinPassthroughPda(depositoryMint: Token): PublicKey {
    return findAddr(
      [Buffer.from(ControllerPDASeed.Passthrough), depositoryMint.publicKey.toBuffer()],
      Controller.ProgramId
    );
  };

  // Find the depository program PDA adresse for a given seed
  private findControllerPda(seed: ControllerPDASeed): PublicKey {
    return findAddr([Buffer.from(seed.toString())], Controller.ProgramId);
  };
}
