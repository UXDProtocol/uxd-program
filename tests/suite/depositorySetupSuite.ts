import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxd-protocol/uxd-client";
import { depositInsuranceMangoDepositoryTest } from "../cases/depositInsuranceMangoDepositoryTest";
import { registerMangoDepositoryTest } from "../cases/registerMangoDepositoryTest";
import { mango } from "../fixtures";

export const mangoDepositorySetupSuite = function (authority: Signer, payer: Signer, controller: Controller, depository: MangoDepository, insuranceAmount: number) {
    it(`Initialize ${depository.collateralMintSymbol} MangoDepository`, async function () {
        await registerMangoDepositoryTest(authority, controller, depository, mango, payer);
    });

    it(`Deposit ${insuranceAmount} USDC of insurance`, async function () {
        await depositInsuranceMangoDepositoryTest(insuranceAmount, authority, controller, depository, mango);
    });
};