import { TOKEN_PROGRAM_ID, Token, AccountInfo } from "@solana/spl-token";
import assert from "assert";
import * as anchor from "@project-serum/anchor";
import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  TransactionInstruction,
} from "@solana/web3.js";

const TXN_COMMIT = "processed";
const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: false,
};

// THE WALLET
const provider = anchor.Provider.env();
anchor.setProvider(provider);

// THE PROGRAM
const program = anchor.workspace.Controller;

let payer: Keypair;
// Add what's needed here

// Controller Unit Tests
///////////////////////////////////////////////////////////////////////////////
describe("Controller Unit Tests", () => {
  it("Setup", async () => {
    //
  });

  it("Testing A equals B", async () => {
    // GIVEN
    const a = 1;
    const b = 1;
    const expectedResult = true;

    // WHEN
    const result = a == b;

    // THEN
    assert(result == expectedResult, "A and B should be equal");
  });

  it("Unit Test B", async () => { });

  //...
});
