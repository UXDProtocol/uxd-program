import * as anchor from "@project-serum/anchor";
import { Keypair, PublicKey, Signer, SystemProgram, SYSVAR_RENT_PUBKEY, Transaction } from "@solana/web3.js";
import {
  Controller,
  MangoDepository,
  SOL_DECIMALS,
  USDC_DECIMALS,
  UXD_DECIMALS,
  WSOL,
  USDC_DEVNET,
  BTC_DECIMALS,
  BTC_DEVNET,
  ETH_DECIMALS,
  ETH_DEVNET,
  findATAAddrSync,
  createAssocTokenIx,
} from "@uxd-protocol/uxd-client";
import { authority, bank, slippageBase, uxdProgramId } from "./constants";
import {
  createAssociatedTokenAccountItx,
  findAssociatedTokenAddress,
  getBalance,
  getSolBalance,
  printDepositoryInfo,
  printUserInfo,
  transferAllSol,
  transferAllTokens,
  transferSol,
  transferTokens,
} from "./utils";
import { depositInsuranceMangoDepositoryTest } from "./cases/depositInsuranceMangoDepositoryTest";
import { registerMangoDepositoryTest } from "./cases/registerMangoDepositoryTest";
import { mango } from "./fixtures";
import { withdrawInsuranceMangoDepositoryTest } from "./cases/withdrawInsuranceMangoDepositoryTest";
import { mintWithMangoDepositoryTest } from "./cases/mintWithMangoDepositoryTest";
import { redeemFromMangoDepositoryTest } from "./cases/redeemFromMangoDepositoryTest";
import { initializeControllerTest } from "./cases/initializeControllerTest";
import {
  MangoDepositoryRebalancingSuiteParameters,
  mangoDepositoryRebalancingSuite,
} from "./suite/mangoDepositoryRebalancingSuite";
import { Uxd, IDL } from "../target/types/uxd";
import { getConnection, TXN_OPTS } from "./connection";
import { ASSOCIATED_TOKEN_PROGRAM_ID, NATIVE_MINT, TOKEN_PROGRAM_ID } from "@solana/spl-token";

console.log(uxdProgramId.toString());
const mangoDepositorySOL = new MangoDepository(
  WSOL,
  "SOL",
  SOL_DECIMALS,
  USDC_DEVNET,
  "USDC",
  USDC_DECIMALS,
  uxdProgramId
);
const mangoDepositoryBTC = new MangoDepository(
  BTC_DEVNET,
  "BTC",
  BTC_DECIMALS,
  USDC_DEVNET,
  "USDC",
  USDC_DECIMALS,
  uxdProgramId
);
const mangoDepositoryETH = new MangoDepository(
  ETH_DEVNET,
  "ETH",
  ETH_DECIMALS,
  USDC_DEVNET,
  "USDC",
  USDC_DECIMALS,
  uxdProgramId
);
const controller = new Controller("UXD", UXD_DECIMALS, uxdProgramId);
const payer = bank;
const slippage = 500; // 5%

// console.log(`SOL 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositorySOL.mangoAccountPda}'`);

beforeEach("\n", function () {
  console.log("=============================================\n\n");
});

// Use SOL as it's the special case using more computing
describe("Integration tests SOL", function () {
  const user: Signer = new Keypair();

  this.beforeAll("Init and fund user (1 SOL and 1k usdc)", async function () {
    console.log("USER =>", user.publicKey.toString());
    await transferSol(1, bank, user.publicKey);
    await transferTokens(1000, USDC_DEVNET, USDC_DECIMALS, bank, user.publicKey);
  });

  describe("Init", async function () {
    it.skip("Initialize Controller", async function () {
      await initializeControllerTest(authority, controller, payer);
    });

    it.skip(`Initialize ${mangoDepositorySOL.collateralMintSymbol} Depository`, async function () {
      await registerMangoDepositoryTest(authority, controller, mangoDepositorySOL, mango, payer);
    });
    it.skip(`Initialize ${mangoDepositoryBTC.collateralMintSymbol} Depository`, async function () {
      await registerMangoDepositoryTest(authority, controller, mangoDepositoryBTC, mango, payer);
    });
    it.skip(`Initialize ${mangoDepositoryETH.collateralMintSymbol} Depository`, async function () {
      await registerMangoDepositoryTest(authority, controller, mangoDepositoryETH, mango, payer);
    });

    it.skip(`Deposit 100 USDC of insurance`, async function () {
      await depositInsuranceMangoDepositoryTest(100, authority, controller, mangoDepositorySOL, mango);
    });

    it.skip(`Withdraw 1 USDC of insurance`, async function () {
      await withdrawInsuranceMangoDepositoryTest(1, authority, controller, mangoDepositorySOL, mango);
    });

    // it(`Mint 80 ${controller.redeemableMintSymbol} then redeem the outcome (${slippage / slippageBase * 100} % slippage)`, async function () {
    //     const mintedAmount = await mintWithMangoDepositoryTest(80, slippage, user, controller, depository, mango, payer);
    // });
  });

  describe("Test minting/redeeming", async function () {
    it.skip(`Mint 1 ${controller.redeemableMintSymbol} then redeem the outcome (${
      (slippage / slippageBase) * 100
    } % slippage)`, async function () {
      const perpPrice = await mangoDepositorySOL.getCollateralPerpPriceUI(mango);
      const amount = 1 / perpPrice;
      console.log("[🧾 amount", amount, mangoDepositorySOL.collateralMintSymbol, "]");
      const mintedAmount = await mintWithMangoDepositoryTest(
        amount,
        slippage,
        user,
        controller,
        mangoDepositorySOL,
        mango,
        payer
      );
      await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, mangoDepositorySOL, mango, payer);
    });

    it.skip(`Mint twice min mint trading size, then redeem them (${
      (slippage / slippageBase) * 100
    }% slippage)`, async function () {
      const minRedeemAmount = await mangoDepositorySOL.getMinRedeemSizeQuoteUI(mango);
      const minTradingSize = await mangoDepositorySOL.getMinTradingSizeCollateralUI(mango);

      await mintWithMangoDepositoryTest(
        minTradingSize * 2,
        slippage,
        user,
        controller,
        mangoDepositorySOL,
        mango,
        payer
      );
      await redeemFromMangoDepositoryTest(
        minRedeemAmount,
        slippage,
        user,
        controller,
        mangoDepositorySOL,
        mango,
        payer
      );
    });
  });

  describe("Test mSOL", async function () {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = new anchor.Program(IDL, "H4fDUuiTmRNrUVCaswDNFXAe1vR2UEgpdV8iQkzEn2C3");

    const msolMint = new anchor.web3.PublicKey("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So");

    const userMSolAta = findATAAddrSync(user.publicKey, msolMint)[0];
    console.log(`userMSolAta = ${userMSolAta}`);

    const userWSolAta = findATAAddrSync(user.publicKey, WSOL)[0];
    console.log(`userWSolAta = ${userWSolAta}`);

    const msolPda = PublicKey.findProgramAddressSync(
      [Buffer.from("MSOLCONFIG"), mangoDepositorySOL.pda.toBuffer(), new anchor.BN(2).toArrayLike(Buffer, "le", 1)],
      program.programId
    )[0];
    console.log(`msolPda = ${msolPda}`);

    it.skip("register msol config", async function () {
      const txId = await program.rpc.createDepositoryMsolConfig(new anchor.BN(5000), {
        accounts: {
          authority: authority.publicKey,
          payer: bank.publicKey,
          controller: controller.pda,
          depository: mangoDepositorySOL.pda,
          msolConfig: msolPda,
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PUBKEY,
        },
        options: TXN_OPTS,
        signers: [authority, payer],
      });
      console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=devnet'`);
    });

    // describe.only("Quote Mint And Redeem Suite", async function () {
    //     quoteMintAndRedeemSuite(authority, user, payer, controller, mangoDepositoryBTC);
    // });

    // describe("Quote mint and redeem", async function () {
    //     it("Mint 10 BTC", async function() {
    //         await mintWithMangoDepositoryTest(10, slippage, user, controller, mangoDepositoryBTC, mango, payer);
    //     });
    // });

    // describe.skip("Test minting/redeeming SOL", async function () {
    //     it(`Mint 10 ${controller.redeemableMintSymbol} then redeem the outcome (${slippage / slippageBase * 100} % slippage)`, async function () {
    //         const perpPrice = await mangoDepositorySOL.getCollateralPerpPriceUI(mango);
    //         const amount = 10 / perpPrice;
    //         console.log("[🧾 amount", amount, mangoDepositorySOL.collateralMintSymbol, "]");
    //         const mintedAmount = await mintWithMangoDepositoryTest(amount, slippage, user, controller, mangoDepositorySOL, mango, payer);
    //         await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, mangoDepositorySOL, mango, payer);
    //     });

    //     it(`Mint twice min mint trading size, then redeem them (${slippage / slippageBase * 100}% slippage)`, async function () {
    //         const minRedeemAmount = await mangoDepositorySOL.getMinRedeemSizeQuoteUI(mango);
    //         const minTradingSize = await mangoDepositorySOL.getMinTradingSizeCollateralUI(mango);

    //         await mintWithMangoDepositoryTest(minTradingSize * 2, slippage, user, controller, mangoDepositorySOL, mango, payer);
    //         await redeemFromMangoDepositoryTest(minRedeemAmount, slippage, user, controller, mangoDepositorySOL, mango, payer);
    //     });
    // });
    it("prepare user's wsol ata", async function () {
      let signers = [];
      let tx = new Transaction();

      if (!(await getConnection().getAccountInfo(userWSolAta))) {
        const createUserRedeemableAtaIx = createAssocTokenIx(user.publicKey, userWSolAta, WSOL);
        tx.add(createUserRedeemableAtaIx);
      }
      signers.push(user);
      if (payer) {
        signers.push(payer);
      }
      tx.feePayer = payer.publicKey;
      const txAtaId = await anchor.web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
      console.log(`🔗 'https://explorer.solana.com/tx/${txAtaId}?cluster=devnet'`);
    });

    it("prepare user's msol ata", async function () {
      let signers = [];
      let tx = new Transaction();

      if (!(await getConnection().getAccountInfo(userMSolAta))) {
        const createUserRedeemableAtaIx = createAssocTokenIx(user.publicKey, userMSolAta, msolMint);
        tx.add(createUserRedeemableAtaIx);
      }
      signers.push(user);
      if (payer) {
        signers.push(payer);
      }
      tx.feePayer = payer.publicKey;
      const txAtaId = await anchor.web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
      console.log(`🔗 'https://explorer.solana.com/tx/${txAtaId}?cluster=devnet'`);
    });

    it.skip("print mango group", async function () {
      const tokens = mango.group.tokens;
      console.log(`mSOL`, `depositedTokenIndex = `, JSON.stringify(tokens));
    });

    it("reading state / deposit cpi", async function () {
      const mangoCacheAccount = mango.getMangoCacheAccount();

      const msolTokenIndex = mango.group.getTokenIndex(msolMint);
      const mangoMsolRootBankAccount = mango.getRootBankForToken(msolTokenIndex);
      const mangoMsolNodeBankAccount = mango.getNodeBankFor(msolTokenIndex, msolMint);
      const mangoMsolDepositedVaultAccount = mango.getVaultFor(msolTokenIndex);

      const wsolTokenIndex = mango.group.getTokenIndex(mangoDepositorySOL.collateralMint);
      const mangoWsolRootBankAccount = mango.getRootBankForToken(wsolTokenIndex);
      const mangoWsolNodeBankAccount = mango.getNodeBankFor(wsolTokenIndex, mangoDepositorySOL.collateralMint);
      const mangoWsolDepositedVaultAccount = mango.getVaultFor(wsolTokenIndex);

      const txId = await program.rpc.swapDepositoryMsol({
        accounts: {
          user: user.publicKey,
          payer: payer.publicKey,
          controller: controller.pda,
          depository: mangoDepositorySOL.pda,
          msolConfig: msolPda,
          mangoAccount: mangoDepositorySOL.mangoAccountPda,
          mangoGroup: mango.group.publicKey,
          mangoCache: mangoCacheAccount,
          mangoSigner: mango.group.signerKey,
          mangoSolRootBank: mangoWsolRootBankAccount,
          mangoSolNodeBank: mangoWsolNodeBankAccount,
          mangoSolVault: mangoWsolDepositedVaultAccount,
          mangoMsolRootBank: mangoMsolRootBankAccount,
          mangoMsolNodeBank: mangoMsolNodeBankAccount,
          mangoMsolVault: mangoMsolDepositedVaultAccount,
          mangoProgram: mango.programId,
          marinadeState: new anchor.web3.PublicKey("8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC"),
          msolMint: msolMint,
          msolMintAuthority: new anchor.web3.PublicKey("3JLPCS1qM2zRw3Dp6V4hZnYHd4toMNPkNesXdX9tg6KM"),
          liqPoolSolLegPda: new anchor.web3.PublicKey("UefNb6z6yvArqe4cJHTXCqStRsKmWhGxnZzuHbikP5Q"),
          liqPoolMsolLeg: new anchor.web3.PublicKey("7GgPYjS5Dza89wV6FpZ23kUJRG5vbQ1GM25ezspYFSoE"),
          liqPoolMsolLegAuthority: new anchor.web3.PublicKey("EyaSjUtSgo9aRD1f8LWXwdvkpDTmXAW54yoSHZRF14WL"),
          treasuryMsolAccount: new anchor.web3.PublicKey("8ZUcztoAEhpAeC2ixWewJKQJsSUGYSGPVAjkhDJYf5Gd"),
          reservePda: new anchor.web3.PublicKey("Du3Ysj1wKbxPKkuPPnvzQLQh8oMSVifs3jGZjJWXFmHN"),
          solPassthroughAta: userWSolAta,
          msolPassthroughAta: userMSolAta,
          marinadeFinanceProgram: new anchor.web3.PublicKey("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD"),
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        options: TXN_OPTS,
        signers: [user, payer],
      });
      console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=devnet'`);
      console.log(`SOL`, `\t\t\t\t\t\t\t\t`, await getSolBalance(user.publicKey));
      console.log(`mSOL`, `\t\t\t\t\t\t\t\t`, await getBalance(userMSolAta));
    });

  });

  describe.skip("info", async function () {
    it("info", async function () {
      await printUserInfo(user.publicKey, controller, mangoDepositorySOL);
      await printDepositoryInfo(controller, mangoDepositorySOL, mango);
    });
  });

  this.afterAll("Transfer funds back to bank", async function () {
    await transferAllTokens(USDC_DEVNET, USDC_DECIMALS, user, bank.publicKey);
    await transferAllSol(user, bank.publicKey);
  });
});
