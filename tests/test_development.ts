import { Connection, Keypair, PublicKey, sendAndConfirmTransaction, Signer, SystemProgram, SYSVAR_RENT_PUBKEY, Transaction } from "@solana/web3.js";
import { Controller, MangoDepository, MercurialVaultDepository, USDC, SOL_DECIMALS, USDC_DECIMALS, UXD_DECIMALS, WSOL, USDC_DEVNET, BTC_DECIMALS, BTC_DEVNET, ETH_DECIMALS, ETH_DEVNET, UXD_DEVNET, WSOL_DEVNET, createAndInitializeMango, createAssocTokenIx } from "@uxd-protocol/uxd-client";
import VaultImpl, { getVaultPdas, PROGRAM_ID as MERCURIAL_VAULT_PROGRAM_ID } from '@mercurial-finance/vault-sdk';
import { authority, bank, slippageBase, uxdProgramId } from "./constants";
import { getBalance, prepareWrappedSolTokenAccount, printDepositoryInfo, printUserInfo, transferAllSol, transferAllTokens, transferSol, transferTokens } from "./utils";
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
import { quoteMintAndRedeemSuite } from "./suite/quoteMintAndRedeemSuite";
import { setMangoDepositoriesRedeemableSoftCap } from "./api";
import { registerMercurialVaultDepositoryTest } from "./cases/registerMercurialVaultDepositoryTest";
import { getConnection, TXN_OPTS } from "./connection";
import { StaticTokenListResolutionStrategy, TokenInfo } from "@solana/spl-token-registry";
import { mintWithMercurialVaultDepositoryTest } from "./cases/mintWithMercurialVaultDepositoryTest";
import { redeemFromMercurialVaultDepositoryTest } from "./cases/redeemFromMercurialVaultDepositoryTest";
import { NATIVE_MINT, Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { VAULT_BASE_KEY } from "@mercurial-finance/vault-sdk/src/vault/constants";
import { ASSOCIATED_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";
import { findATAAddrSync } from "@uxd-protocol/uxd-client";

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
// const mangoDepositoryBTC = new MangoDepository(BTC_DEVNET, "BTC", BTC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
// const mangoDepositoryETH = new MangoDepository(ETH_DEVNET, "ETH", ETH_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);

let mercurialVaultDepositoryUSDC: MercurialVaultDepository;
let mercurialVault: VaultImpl;

const controller = new Controller("UXD", UXD_DECIMALS, uxdProgramId);
const payer = bank;
const slippage = 50; // 5%

const SOLEND_USDC_DEVNET = new PublicKey('zVzi5VAf4qMEwzv7NXECVx5v2pQ7xnqVVjCXZwS9XzA');
const SOLEND_USDC_DEVNET_DECIMALS = 6;

// console.log(`SOL 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositorySOL.mangoAccountPda}'`);

beforeEach("\n", function () {
  console.log("=============================================\n\n");
});

// Use SOL as it's the special case using more computing
describe("Integration tests", function () {
  const user: Signer = new Keypair();

  this.beforeAll("Init and fund user (10 SOL and 100 usdc)", async function () {
    console.log("USER =>", user.publicKey.toString());
    await transferSol(10, bank, user.publicKey);
    await transferTokens(200, USDC_DEVNET, USDC_DECIMALS, bank, user.publicKey);

    // ========================================================================
    // Transfer 1 WSOL to the user
    const instructions = await prepareWrappedSolTokenAccount(
      getConnection(),
      bank.publicKey,
      user.publicKey,
      1_000_000_000,
    );

    let tx = new Transaction();
    tx.instructions.push(...instructions);
    tx.feePayer = bank.publicKey;
    await sendAndConfirmTransaction(getConnection(), tx, [user, bank], TXN_OPTS);
    // ========================================================================
  });

  describe("Init", async function () {

    // TODO
    // We should create the USDC vault if it doesn't exist and use it.
    // Because the target is USDC.
    // We should take a look at the "initialize" instruction

    // Do not create the vault. We are building an object with utilities methods.
    it("Initialize Mercurial Vault", async function () {
      // const wsolDevnet = await getVaultPdas(WSOL_DEVNET, new PublicKey(PROGRAM_ID));
      const tokenMap = new StaticTokenListResolutionStrategy().resolve();
      const solTokenInfo = tokenMap.find(token => token.symbol === 'SOL') as TokenInfo;

      mercurialVault = await VaultImpl.create(
        getConnection(),
        solTokenInfo,
        {
          cluster: 'devnet'
        },
      );

      console.log('Mercurial Vault PDA =>', mercurialVault.vaultPda.toBase58());
      console.log('Mercurial Vault LP SUPPLY =>', mercurialVault.lpSupply.toString());
      console.log('Mercurial Vault Token Vault PDA =>', mercurialVault.tokenVaultPda.toBase58());
      console.log('Mercurial Vault Token Info Address =>', mercurialVault.tokenInfo.address);

      console.log('Vault State =>', mercurialVault.vaultState);

      console.log('Vault State =>', {
        tokenMint: mercurialVault.vaultState.tokenMint.toBase58(),
        lpMint: mercurialVault.vaultState.lpMint.toBase58(),
        feeVault: mercurialVault.vaultState.feeVault.toBase58(),
        tokenVault: mercurialVault.vaultState.tokenVault.toBase58(),
        base: mercurialVault.vaultState.base.toString(),
        admin: mercurialVault.vaultState.admin.toString(),
        operator: mercurialVault.vaultState.operator.toString(),
      });
    });

    it("Initialize Controller", async function () {
      await initializeControllerTest(authority, controller, payer);
    });

    it(`Initialize Mercurial Vault SOL Depository`, async function () {
      const tokenMap = new StaticTokenListResolutionStrategy().resolve();
      const solTokenInfo = tokenMap.find(token => token.symbol === 'SOL') as TokenInfo;

      console.log("The collateral mint is =>", solTokenInfo.address);

      mercurialVaultDepositorySOL = await MercurialVaultDepository.initialize({
        collateralMintName: "SOL",
        collateralMintSymbol: "SOL",
        collateralMint: new PublicKey(solTokenInfo.address),
        collateralMintDecimals: SOL_DECIMALS,
        uxdProgramId,
        cluster: 'devnet',
        connection: getConnection(),
      });

      await registerMercurialVaultDepositoryTest(authority, controller, mercurialVaultDepositorySOL, payer);

      await mintWithMercurialVaultDepositoryTest(0.1, 0, user, controller, mercurialVaultDepositorySOL, payer);
    });

    /*
    it(`Initialize ${mangoDepositorySOL.collateralMintSymbol} Depository`, async function () {
        await registerMangoDepositoryTest(authority, controller, mangoDepositorySOL, mango, payer);
    });

    it(`Deposit 100 USDC of insurance`, async function () {
        await depositInsuranceMangoDepositoryTest(100, authority, controller, mangoDepositorySOL, mango);
    });

    it("Increase soft cap", async function () {
        await setMangoDepositoriesRedeemableSoftCap(authority, controller, 10_000_000);
    });

    it("Mint 1 SOL", async function () {
        await mintWithMangoDepositoryTest(1, slippage, user, controller, mangoDepositorySOL, mango, payer);
    });

    it(`Withdraw 10 USDC of insurance`, async function () {
        await withdrawInsuranceMangoDepositoryTest(10, authority, controller, mangoDepositorySOL, mango);
    });*/

    it(`Initialize ${mangoDepositorySOL.collateralMintSymbol} Depository`, async function () {
      await registerMangoDepositoryTest(authority, controller, mangoDepositorySOL, mango, payer);
    });

    it(`Deposit 100 USDC of insurance`, async function () {
      await depositInsuranceMangoDepositoryTest(100, authority, controller, mangoDepositorySOL, mango);
    });

    it("Increase mango depositories redeemable soft cap to 10_000_000", async function () {
      await setMangoDepositoriesRedeemableSoftCap(authority, controller, 10_000_000);
    });

    it("Mint 1 SOL", async function () {
      await mintWithMangoDepositoryTest(1, slippage, user, controller, mangoDepositorySOL, mango, payer);
    });

    it(`Withdraw 10 USDC of insurance`, async function () {
      await withdrawInsuranceMangoDepositoryTest(10, authority, controller, mangoDepositorySOL, mango);
    });
  });

  describe("Quote Mint And Redeem Suite", async function () {
    quoteMintAndRedeemSuite(authority, user, payer, controller, mangoDepositorySOL);
  });

  describe("Test minting/redeeming SOL", async function () {
    it(`Mint 10 ${controller.redeemableMintSymbol} then redeem the outcome (${(slippage / slippageBase) * 100
      } % slippage)`, async function () {
        const perpPrice = await mangoDepositorySOL.getCollateralPerpPriceUI(mango);
        const amount = 10 / perpPrice;
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

    it(`Mint twice min mint trading size, then redeem them (${(slippage / slippageBase) * 100
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

  // Note - Keep a mint/redeem before rebalancing so that it creates the necessary accounts for computing
  describe.skip("mangoDepositoryRebalancingSuite SOL", function () {
    const paramsRebalancing = new MangoDepositoryRebalancingSuiteParameters(slippage);
    mangoDepositoryRebalancingSuite(user, bank, controller, mangoDepositorySOL, paramsRebalancing);
  });

  describe.skip("info SOL", async function () {
    it("info", async function () {
      await printUserInfo(user.publicKey, controller, mangoDepositorySOL);
      await printDepositoryInfo(controller, mangoDepositorySOL, mango);
    });
  });

  this.afterAll("Transfer funds back to bank", async function () {
    await transferAllTokens(USDC_DEVNET, USDC_DECIMALS, user, bank.publicKey);
    await transferAllTokens(BTC_DEVNET, BTC_DECIMALS, user, bank.publicKey);
    await transferAllSol(user, bank.publicKey);
  });
});
