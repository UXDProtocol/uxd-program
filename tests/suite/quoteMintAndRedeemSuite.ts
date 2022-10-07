import { BN } from "@project-serum/anchor";
import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { uiToNative } from "@uxd-protocol/uxd-client";
import { nativeToUi } from "@uxd-protocol/uxd-client";
import { Controller, findATAAddrSync, MangoDepository, PnLPolarity } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { editMangoDepositoryTest } from "../cases/editMangoDepositoryTest";
import { mintWithMangoDepositoryTest } from "../cases/mintWithMangoDepositoryTest";
import { quoteMintWithMangoDepositoryAccountingTest } from "../cases/quoteMintWithMangoDepositoryAccountingTest";
import { quoteMintWithMangoDepositoryTest } from "../cases/quoteMintWithMangoDepositoryTest";
import { quoteRedeemFromMangoDepositoryAccountingTest } from "../cases/quoteRedeemFromMangoDepositoryAccountingTest";
import { quoteRedeemFromMangoDepositoryTest } from "../cases/quoteRedeemFromMangoDepositoryTest";
import { redeemFromMangoDepositoryTest } from "../cases/redeemFromMangoDepositoryTest";
import { setMangoDepositoryQuoteMintAndRedeemFeeTest } from "../cases/setMangoDepositoryQuoteMintAndRedeemFeeTest";
import { setMangoDepositoryQuoteMintAndRedeemSoftCapTest } from "../cases/setMangoDepositoryQuoteMintAndRedeemSoftCapTest";
import { getConnection, TXN_OPTS } from "../connection";
import { slippageBase } from "../constants";
import { mango } from "../fixtures";
import { getBalance, transferAllTokens, transferSol, transferTokens } from "../utils";

export const quoteMintAndRedeemSuite = function (
  authority: Signer,
  user: Signer,
  payer: Signer,
  controller: Controller,
  depository: MangoDepository
) {
  let initialRedeemableDepositorySupplyCap: BN;

  before(`Transfer 50${depository.quoteMintSymbol} from payer to user`, async function () {
    await transferTokens(50, depository.quoteMint, depository.quoteMintDecimals, payer, user.publicKey);

    const onChainDepository = await depository.getOnchainAccount(getConnection(), TXN_OPTS);

    initialRedeemableDepositorySupplyCap = onChainDepository.redeemableDepositorySupplyCap;
  });

  // to prepare enough SOL for minting below
  it(`Transfer 50 USD worth of ${depository.collateralMintSymbol} from payer to user`, async function () {
    const perpPrice = await depository.getCollateralPerpPriceUI(mango);
    const amount = 50 / perpPrice;
    console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
    // For Wsol we send sol, the API handle the wrapping before each minting
    if (depository.collateralMint.equals(NATIVE_MINT)) {
      await transferSol(amount, payer, user.publicKey);
    } else {
      await transferTokens(amount, depository.collateralMint, depository.collateralMintDecimals, payer, user.publicKey);
    }
  });

  // to prepare enough redeemable mint in user's wallet for quote redeem, if the polarity is positive
  // would redeem the remaining before the end of the test suite
  it(`Mint 50 ${controller.redeemableMintSymbol} (${(20 / slippageBase) * 100} % slippage)`, async function () {
    const perpPrice = await depository.getCollateralPerpPriceUI(mango);
    const amount = 50 / perpPrice;
    console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
    await mintWithMangoDepositoryTest(amount, 20, user, controller, depository, mango, payer);
  });

  it(`Change the quote mint and redeem soft cap to 1_000_000`, async function () {
    await setMangoDepositoryQuoteMintAndRedeemSoftCapTest(1_000_000, authority, controller, depository);
  });

  it(`Change the depository quote mint and redeem fee to 2`, async function () {
    await editMangoDepositoryTest(authority, controller, depository, {
      quoteMintAndRedeemFee: 2,
    });
  });

  it(`Change the quote mint and redeem fees to 0`, async function () {
    await setMangoDepositoryQuoteMintAndRedeemFeeTest(0, authority, controller, depository);
  });

  it(`Quote mint or redeem 5$ (without fees)`, async function () {
    const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
    const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

    if (Math.abs(unrealizedPnl) < 5) {
      console.log("🔵  skipping mint/redeem, unrealized pnl too small");
      return;
    }
    switch (polarity) {
      case `Positive`: {
        await quoteRedeemFromMangoDepositoryTest(5, user, controller, depository, mango, payer);
        break;
      }
      case `Negative`: {
        await quoteMintWithMangoDepositoryTest(5, user, controller, depository, mango, payer);
        break;
      }
    }
  });

  it(`Accounting test for quote mint or redeem 5$ (without fees)`, async function () {
    const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
    const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

    if (Math.abs(unrealizedPnl) < 5) {
      console.log("🔵  skipping quote mint/redeem, unrealized pnl too small");
      return;
    }
    switch (polarity) {
      case `Positive`: {
        await quoteRedeemFromMangoDepositoryAccountingTest(5, user, controller, depository, mango, payer);
        break;
      }
      case `Negative`: {
        await quoteMintWithMangoDepositoryAccountingTest(5, user, controller, depository, mango, payer);
        break;
      }
    }
  });

  it(`Change the quote mint and redeem fees to 5 bps`, async function () {
    await setMangoDepositoryQuoteMintAndRedeemFeeTest(5, authority, controller, depository);
  });

  it(`Quote mint or redeem 5$ (with fees)`, async function () {
    const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
    const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

    if (Math.abs(unrealizedPnl) < 5) {
      console.log("🔵  skipping mint/redeem, unrealized pnl too small");
      return;
    }
    switch (polarity) {
      case `Positive`: {
        await quoteRedeemFromMangoDepositoryTest(5, user, controller, depository, mango, payer);
        break;
      }
      case `Negative`: {
        await quoteMintWithMangoDepositoryTest(5, user, controller, depository, mango, payer);
        break;
      }
    }
  });

  it(`Accounting test for quote mint or redeem 5$ (with fees)`, async function () {
    const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
    const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

    if (Math.abs(unrealizedPnl) < 5) {
      console.log("🔵  skipping quote mint/redeem, unrealized pnl too small");
      return;
    }
    switch (polarity) {
      case `Positive`: {
        await quoteRedeemFromMangoDepositoryAccountingTest(5, user, controller, depository, mango, payer);
        break;
      }
      case `Negative`: {
        await quoteMintWithMangoDepositoryAccountingTest(5, user, controller, depository, mango, payer);
        break;
      }
    }
  });

  it(`Quote mint or redeem with the wrong polarity (should fail)`, async function () {
    const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
    const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

    if (Math.abs(unrealizedPnl) < 5) {
      console.log("🔵  skipping mint/redeem, unrealized pnl too small");
      return;
    }
    try {
      switch (polarity) {
        case `Negative`: {
          await quoteRedeemFromMangoDepositoryTest(5, user, controller, depository, mango, payer);
          break;
        }
        case `Positive`: {
          await quoteMintWithMangoDepositoryTest(5, user, controller, depository, mango, payer);
          break;
        }
      }
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, "Should have failed - Did the wrong instruction given polarity");
  });

  it(`Quote mint or redeem more than is available to mint (should fail)`, async function () {
    const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
    const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

    const amountToMintOrRedeem = Math.abs(unrealizedPnl) * 1.5;

    try {
      switch (polarity) {
        case `Positive`: {
          await quoteRedeemFromMangoDepositoryTest(amountToMintOrRedeem, user, controller, depository, mango, payer);
          break;
        }
        case `Negative`: {
          await quoteMintWithMangoDepositoryTest(amountToMintOrRedeem, user, controller, depository, mango, payer);
          break;
        }
      }
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, "Should have failed - Minting or redeeming more than available");
  });

  it(`Quote mint or redeem 0 (should fail)`, async function () {
    const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
    const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

    try {
      switch (polarity) {
        case `Positive`: {
          await quoteRedeemFromMangoDepositoryTest(0, user, controller, depository, mango, payer);
          break;
        }
        case `Negative`: {
          await quoteMintWithMangoDepositoryTest(0, user, controller, depository, mango, payer);
          break;
        }
      }
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, "Should have failed - Tried minting or redeeming 0");
  });

  it(`Change the quote mint and redeem soft cap to 50`, async function () {
    await setMangoDepositoryQuoteMintAndRedeemSoftCapTest(10, authority, controller, depository);
  });

  it(`Quote mint or redeem 5$ (with fees)`, async function () {
    const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
    const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

    if (Math.abs(unrealizedPnl) < 5) {
      console.log("🔵  skipping mint/redeem, unrealized pnl too small");
      return;
    }
    switch (polarity) {
      case `Positive`: {
        await quoteRedeemFromMangoDepositoryTest(5, user, controller, depository, mango, payer);
        break;
      }
      case `Negative`: {
        await quoteMintWithMangoDepositoryTest(5, user, controller, depository, mango, payer);
        break;
      }
    }
  });

  it(`Quote redeem 100$ (should fail)`, async function () {
    try {
      await quoteRedeemFromMangoDepositoryTest(100, user, controller, depository, mango, payer);
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, "Should have failed - No collateral deposited yet");
  });

  it(`Quote mint 100$ (should fail)`, async function () {
    try {
      await quoteMintWithMangoDepositoryTest(100, user, controller, depository, mango, payer);
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, "Should have failed - No collateral deposited yet");
  });

  it('Set redeemable depository supply cap to 0,0005 more than actual minted amount', async function () {
    const onChainDepository = await depository.getOnchainAccount(getConnection(), TXN_OPTS);

    await editMangoDepositoryTest(authority, controller, depository, {
      redeemableDepositorySupplyCap: onChainDepository.mintedRedeemableAmount + uiToNative(0.0005, controller.redeemableMintDecimals),
    });
  });

  it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralMintSymbol} (should fail)`, async function () {
    const collateralAmount = 0.001;

    console.log("[🧾 collateralAmount", collateralAmount, depository.collateralMintSymbol, "]");

    try {
      await quoteMintWithMangoDepositoryTest(
        collateralAmount,
        user,
        controller,
        depository,
        mango,
        payer
      );
    } catch {
      expect(true, "Failing as planned");
    }

    expect(false, `Should have failed - amount of redeemable overflow the redeemable depository supply cap`);
  });

  it(`Reset redeemable depository supply cap back to its original value`, async function () {
    const redeemableDepositorySupplyCap = nativeToUi(initialRedeemableDepositorySupplyCap, controller.redeemableMintDecimals);

    await editMangoDepositoryTest(authority, controller, depository, {
      redeemableDepositorySupplyCap,
    });
  });

  it(`Redeem remaining ${controller.redeemableMintSymbol} (${(20 / slippageBase) * 100} % slippage)`, async function () {
    const userRedeemableATA: PublicKey = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
    const remainingRedeemableAmount = await getBalance(userRedeemableATA);
    await redeemFromMangoDepositoryTest(remainingRedeemableAmount, 20, user, controller, depository, mango, payer);
  });

  it(`Return remaining balances from user to the payer`, async function () {
    await transferAllTokens(depository.quoteMint, depository.quoteMintDecimals, user, payer.publicKey);
    await transferAllTokens(depository.collateralMint, depository.collateralMintDecimals, user, payer.publicKey);
  });
};
