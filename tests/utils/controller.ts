import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Token } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { MANGO_PROGRAM_ID } from "./mango";
import { utils } from "./utils";

enum ControllerPDASeed {
  State = "STATE",
  UXD = "STABLECOIN",
  Depository = "DEPOSITORY",
  Passthrough = "PASSTHROUGH",
}

export class ControllerUXD {
  public static ProgramId: PublicKey = anchor.workspace.Controller.programId;
  public static rpc: anchor.RpcNamespace = (anchor.workspace.Controller as Program).rpc;

  // Pda
  public static statePda: PublicKey = ControllerUXD.findControllerPda(ControllerPDASeed.State);
  public static mintPda: PublicKey = ControllerUXD.findControllerPda(ControllerPDASeed.UXD);

  public static depositoryPda(collateralMint: PublicKey): PublicKey {
    return utils.findProgramAddressSync(ControllerUXD.ProgramId, [
      Buffer.from(ControllerPDASeed.Depository),
      collateralMint.toBuffer(),
    ])[0];
  }

  // This pda is function of the depository mint
  public static coinPassthroughPda(collateralMint: PublicKey): PublicKey {
    return utils.findProgramAddressSync(ControllerUXD.ProgramId, [
      Buffer.from(ControllerPDASeed.Passthrough),
      collateralMint.toBuffer(),
    ])[0];
  }

  // Find the depository program PDA adresse for a given seed
  private static findControllerPda(seed: ControllerPDASeed): PublicKey {
    return utils.findProgramAddressSync(ControllerUXD.ProgramId, [Buffer.from(seed.toString())])[0];
  }

  public info() {
    console.log(`\
      [Controller debug info]
        * statePda:                                     ${ControllerUXD.statePda}
        * mintPda:                                      ${ControllerUXD.mintPda}`);
  }
}
