import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { ControllerUXD } from "./utils/controller";
import { provider, TXN_OPTS } from "./utils/utils";
import { Depository } from "./utils/depository";
import { createTokenEnv, BTC_DECIMAL, SOL_DECIMAL, createTestUser, TestUser, TokenEnv } from "./utils/utils";

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
  btc = await createTokenEnv(BTC_DECIMAL, 45000n);
  sol = await createTokenEnv(SOL_DECIMAL, 180n);
  admin = await createTestUser([]);

  depositoryBTC = new Depository(btc.token, "BTC", btc.pythPrice.publicKey);
  depositorySOL = new Depository(sol.token, "SOL", sol.pythPrice.publicKey);
});

before(
  "[Integration tests ADMIN] - What we would do once to setup the UXD controllers and associated depositories",
  () => {
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

    it("Create SOL depository (admin)", async () => {
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
      await ControllerUXD.rpc.registerDepository(depositoryBTC.oraclePriceAccount, {
        accounts: {
          authority: admin.wallet.publicKey,
          state: ControllerUXD.statePda,
          depositoryRecord: ControllerUXD.depositoryRecordPda(depositoryBTC.collateralMint),
          depositoryState: depositoryBTC.statePda,
          coinMint: depositoryBTC.collateralMint.publicKey,
          coinPassthrough: ControllerUXD.coinPassthroughPda(depositoryBTC.collateralMint),
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [admin.wallet],
        options: TXN_OPTS,
      });
      // Add some asserts ...
    });

    it("Register SOL Depository with Controller", async () => {
      await ControllerUXD.rpc.registerDepository(depositorySOL.oraclePriceAccount, {
        accounts: {
          authority: admin.wallet.publicKey,
          state: ControllerUXD.statePda,
          depositoryRecord: ControllerUXD.depositoryRecordPda(depositorySOL.collateralMint),
          depositoryState: depositorySOL.statePda,
          coinMint: depositorySOL.collateralMint.publicKey,
          coinPassthrough: ControllerUXD.coinPassthroughPda(depositorySOL.collateralMint),
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [admin.wallet],
        options: TXN_OPTS,
      });
      // Add some asserts ...
    });
  }
);
