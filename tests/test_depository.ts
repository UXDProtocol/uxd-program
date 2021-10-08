import { TOKEN_PROGRAM_ID, Token, AccountInfo } from "@solana/spl-token";
import assert from "assert";
import * as anchor from "@project-serum/anchor";
import { PublicKey, Keypair, SystemProgram, SYSVAR_RENT_PUBKEY, TransactionInstruction } from "@solana/web3.js";

let payer: Keypair;
// Add what's needed here

// // Despository Unit Tests
// ///////////////////////////////////////////////////////////////////////////////
// describe("Depository Unit Tests", () => {
//   it("Setup", async () => {
//     //
//   });

//   it("Testing A equals B", async () => {
//     // GIVEN
//     let a = 1;
//     let b = 1;
//     let expectedResult = true;

//     // WHEN
//     let result = a == b;

//     // THEN
//     assert(result == expectedResult, "A and B should be equal");
//   });

//   it("Unit Test B", async () => { });

//   //...
// });
