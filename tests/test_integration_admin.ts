import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { SystemProgram, SYSVAR_RENT_PUBKEY, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { ControllerUXD } from "./utils/controller";
import { Depository } from "./utils/depository";
import { TXN_OPTS, utils, BTC, WSOL, admin, provider, connection, BTC_DECIMALS } from "./utils/utils";
import { expect, util } from "chai";
import { MANGO_PROGRAM_ID } from "./utils/mango";

// Depositories - They represent the business object that tie a mint to a depository
export let depositoryBTC: Depository;

before("Setup mints and depositories", async () => {
  // GIVEN
  await utils.setupMango(); // Async fetch of mango group

  let sig = await connection.requestAirdrop(admin.publicKey, 10 * LAMPORTS_PER_SOL);
  await connection.confirmTransaction(sig);

  depositoryBTC = new Depository(BTC, "BTC", BTC_DECIMALS);
});

before("Standard Administrative flow for UXD Controller and depositories", () => {
  it("Create UXD Controller", async () => {
    // WHEN
    if (await provider.connection.getAccountInfo(ControllerUXD.statePda)) {
      console.log("already initialized.");
    } else {
      await ControllerUXD.rpc.new({
        accounts: {
          authority: admin.publicKey,
          state: ControllerUXD.statePda,
          uxdMint: ControllerUXD.mintPda,
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [admin.payer],
        options: TXN_OPTS,
      });
    }

    // THEN
    // XXX add asserts
  });

  it("Register BTC Depository with Controller", async () => {
    // GIVEN
    const depositoryPda = ControllerUXD.depositoryPda(depositoryBTC.collateralMint);

    // WHEN
    if (await provider.connection.getAccountInfo(ControllerUXD.coinPassthroughPda(depositoryBTC.collateralMint))) {
      console.log("already registered.");
    } else {
      await ControllerUXD.rpc.registerDepository({
        accounts: {
          authority: admin.publicKey,
          state: ControllerUXD.statePda,
          depository: ControllerUXD.depositoryPda(depositoryBTC.collateralMint),
          coinMint: depositoryBTC.collateralMint,
          coinPassthrough: ControllerUXD.coinPassthroughPda(depositoryBTC.collateralMint),
          mangoGroup: utils.mango.group.publicKey,
          mangoAccount: ControllerUXD.mangoPda(depositoryBTC.collateralMint),
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          mangoProgram: MANGO_PROGRAM_ID,
        },
        signers: [admin.payer],
        options: TXN_OPTS,
      });
    }
  });
});
