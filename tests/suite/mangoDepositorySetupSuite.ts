import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxdprotocol/uxd-client";
import { depositInsuranceMangoDepositoryTest } from "../cases/depositInsuranceMangoDepositoryTest";
import { initializeMangoDepositoryTest } from "../cases/initializeMangoDepositoryTest";
import { mango } from "../fixtures";

export const mangoDepositorySetupSuite = (authority: Signer, controller: Controller, depository: MangoDepository, insuranceAmount: number) => {

    it(`Initialize ${depository.collateralMintSymbol} Depository`, async () => {
        await initializeMangoDepositoryTest(authority, controller, depository, mango);
    });

    it(`Deposit ${insuranceAmount} USDC of insurance`, async () => {
        await depositInsuranceMangoDepositoryTest(insuranceAmount, authority, controller, depository, mango);
    });
};
