import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository, Zo, ZoDepository } from "@uxdprotocol/uxd-client";
import { depositInsuranceMangoDepositoryTest } from "../cases/depositInsuranceMangoDepositoryTest";
import { depositInsuranceZoDepositoryTest } from "../cases/depositInsuranceZoDepositoryTest";
import { initializeZoDepositoryTest } from "../cases/initializeZoDepositoryTest";
import { registerMangoDepositoryTest } from "../cases/registerMangoDepositoryTest";
import { registerZoDepositoryTest } from "../cases/registerZoDepositoryTest";
import { mango } from "../fixtures";

export const mangoDepositorySetupSuite = function (authority: Signer, payer: Signer, controller: Controller, depository: MangoDepository, insuranceAmount: number) {
    it(`Initialize ${depository.collateralMintSymbol} MangoDepository`, async function () {
        await registerMangoDepositoryTest(authority, controller, depository, mango, payer);
    });

    it(`Deposit ${insuranceAmount} USDC of insurance`, async function () {
        await depositInsuranceMangoDepositoryTest(insuranceAmount, authority, controller, depository, mango);
    });
};

export const zoDepositorySetupSuite = function (authority: Signer, payer: Signer, controller: Controller, depository: ZoDepository, zo: Zo, insuranceAmount: number) {
    it(`Initialize ${depository.collateralMintSymbol} ZoDepository`, async function () {
        await registerZoDepositoryTest(authority, controller, depository, payer);
        await initializeZoDepositoryTest(authority, controller, depository, zo, payer);
    });

    it(`Deposit ${insuranceAmount} USDC of insurance`, async function () {
        await depositInsuranceZoDepositoryTest(insuranceAmount, authority, controller, depository, zo);
    });
};