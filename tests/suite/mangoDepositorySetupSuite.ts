import { BN } from "@project-serum/anchor";
import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository, UXD_DECIMALS } from "@uxd-protocol/uxd-client";
import { depositInsuranceMangoDepositoryTest } from "../cases/depositInsuranceMangoDepositoryTest";
import { registerMangoDepositoryTest } from "../cases/registerMangoDepositoryTest";
import { mango } from "../fixtures";

export const mangoDepositorySetupSuite = function (authority: Signer, payer: Signer, controller: Controller, depository: MangoDepository, redeemableAmountUnderManagementCap: number, insuranceAmount: number) {
    it(`Initialize ${depository.collateralMintSymbol} MangoDepository`, async function () {
        await registerMangoDepositoryTest(authority, controller, depository, mango, redeemableAmountUnderManagementCap, payer);
    });

    it(`Deposit ${insuranceAmount} USDC of insurance`, async function () {
        await depositInsuranceMangoDepositoryTest(insuranceAmount, authority, controller, depository, mango);
    });
};