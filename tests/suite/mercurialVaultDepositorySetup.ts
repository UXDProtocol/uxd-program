import { Token } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, MercurialVaultDepository } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { registerMercurialVaultDepositoryTest } from "../cases/registerMercurialVaultDepositoryTest";
import { getConnection } from "../connection";
import { uxdProgramId } from "../constants";

export const mercurialVaultDepositorySetupSuite = function (authority: Signer, payer: Signer, controller: Controller, depository: MercurialVaultDepository, mintingFeeInBps: number, redeemingFeeInBps: number) {
    it(`Initialize ${depository.collateralMint.symbol} MercurialVaultDepository`, async function () {
        await registerMercurialVaultDepositoryTest(authority, controller, depository, mintingFeeInBps, redeemingFeeInBps, payer);
    });
};