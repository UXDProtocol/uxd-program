import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { SystemProgram, SYSVAR_RENT_PUBKEY, Keypair } from "@solana/web3.js";
import { ControllerUXD } from "./utils/controller";
import { Depository } from "./utils/depository";
import {
  createTokenEnv,
  BTC_DECIMAL,
  SOL_DECIMAL,
  createTestUser,
  TestUser,
  TokenEnv,
  TXN_OPTS,
  utils,
  getRentExemption,
} from "./utils/utils";
import { expect } from "chai";
import { MANGO_PROGRAM_ID } from "./utils/mango";
import { MangoAccountLayout } from "@blockworks-foundation/mango-client";

// Identities
let admin: TestUser; // This is us, the UXD deployment admins

// Mints
export let btc: TokenEnv;
export let sol: TokenEnv;

// Depositories - They represent the business object that tie a mint to a depository
export let depositoryBTC: Depository;
export let depositorySOL: Depository;

before("Setup mints and depositories", async () => {
  // GIVEN
  await utils.setupMango(); // Async fetch of mango group
  btc = await createTokenEnv(BTC_DECIMAL, 45000n);
  sol = await createTokenEnv(SOL_DECIMAL, 180n);
  admin = await createTestUser([]);

  depositoryBTC = new Depository(btc.token, "BTC", btc.pythPrice.publicKey);
  depositorySOL = new Depository(sol.token, "SOL", sol.pythPrice.publicKey);
});

before("Standard Administrative flow for UXD Controller and depositories", () => {
  it("Create UXD Controller", async () => {
    // WHEN
    await ControllerUXD.rpc.new({
      accounts: {
        authority: admin.wallet.publicKey,
        state: ControllerUXD.statePda,
        uxdMint: ControllerUXD.mintPda,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [admin.wallet],
      options: TXN_OPTS,
    });

    // THEN
    // XXX add asserts
  });

  it("Create BTC depository", async () => {
    await Depository.rpc.new(ControllerUXD.ProgramId, {
      accounts: {
        payer: admin.wallet.publicKey,
        state: depositoryBTC.statePda,
        redeemableMint: depositoryBTC.redeemableMintPda,
        programCoin: depositoryBTC.depositPda,
        coinMint: depositoryBTC.collateralMint.publicKey,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [admin.wallet],
      options: TXN_OPTS,
    });
    // Add some asserts ...
    depositoryBTC.info();
  });

  it("Create SOL depository", async () => {
    await Depository.rpc.new(ControllerUXD.ProgramId, {
      accounts: {
        payer: admin.wallet.publicKey,
        state: depositorySOL.statePda,
        redeemableMint: depositorySOL.redeemableMintPda,
        programCoin: depositorySOL.depositPda,
        coinMint: depositorySOL.collateralMint.publicKey,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [admin.wallet],
      options: TXN_OPTS,
    });
    // Add some asserts ...
    depositorySOL.info();
  });

  it("Register BTC Depository with Controller", async () => {
    // Given
    const [depositoryRecordPda, _n1] = ControllerUXD.depositoryRecordPda(depositoryBTC.collateralMint);
    const mangoAccount = new Keypair();

    const createMangoAccountIx = SystemProgram.createAccount({
      programId: MANGO_PROGRAM_ID,
      space: MangoAccountLayout.span,
      lamports: await getRentExemption(MangoAccountLayout.span),
      fromPubkey: admin.wallet.publicKey,
      newAccountPubkey: mangoAccount.publicKey,
    });
    MangoAccountLayout;
    // WHEN
    await ControllerUXD.rpc.registerDepository(depositoryBTC.oraclePriceAccount, {
      accounts: {
        authority: admin.wallet.publicKey,
        state: ControllerUXD.statePda,
        depositoryRecord: depositoryRecordPda,
        depositoryState: depositoryBTC.statePda,
        coinMint: depositoryBTC.collateralMint.publicKey,
        coinPassthrough: ControllerUXD.coinPassthroughPda(depositoryBTC.collateralMint),
        mangoGroup: utils.mango.mangoGroup.publicKey,
        mangoAccount: mangoAccount.publicKey,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        mangoProgram: MANGO_PROGRAM_ID,
      },
      signers: [admin.wallet, mangoAccount],
      options: TXN_OPTS,
      instructions: [createMangoAccountIx],
    });
  });

  // it("Register SOL Depository with Controller", async () => {
  //   await ControllerUXD.rpc.registerDepository(depositorySOL.oraclePriceAccount, {
  //     accounts: {
  //       authority: admin.wallet.publicKey,
  //       state: ControllerUXD.statePda,
  //       depositoryRecord: ControllerUXD.depositoryRecordPda(depositorySOL.collateralMint),
  //       depositoryState: depositorySOL.statePda,
  //       coinMint: depositorySOL.collateralMint.publicKey,
  //       coinPassthrough: ControllerUXD.coinPassthroughPda(depositorySOL.collateralMint),
  //       rent: SYSVAR_RENT_PUBKEY,
  //       systemProgram: SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       mangoProgram: MANGO_PROGRAM_ID,
  //     },
  //     signers: [admin.wallet],
  //     options: TXN_OPTS,
  //   });
  //   // Add some asserts ...
  // });
});

// It does fail, but how to play nice with Moche/Chai...
// it("Create BTC depository when already there should fail", async () => {
//   await Depository.rpc.new(ControllerUXD.ProgramId, {
//     accounts: {
//       payer: admin.wallet.publicKey,
//       state: depositoryBTC.statePda,
//       redeemableMint: depositoryBTC.redeemableMintPda,
//       programCoin: depositoryBTC.depositPda,
//       coinMint: depositoryBTC.collateralMint.publicKey,
//       rent: SYSVAR_RENT_PUBKEY,
//       systemProgram: SystemProgram.programId,
//       tokenProgram: TOKEN_PROGRAM_ID,
//     },
//     signers: [admin.wallet],
//     options: TXN_OPTS,
//   });
//   // Add some asserts ...
//   expect.fail("Should fail because the BTC depository is already initialized.");
// });
