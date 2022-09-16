import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, MercurialVaultDepository } from "@uxd-protocol/uxd-client";
import { registerMercurialVaultDepositoryTest } from "../cases/registerMercurialVaultDepositoryTest";
import { getConnection } from "../connection";
import { uxdProgramId } from "../constants";

export const mercurialVaultDepositorySetupSuite = function (authority: Signer, payer: Signer, controller: Controller, collateralMintInfo: {
    mint: PublicKey;
    name: string;
    symbol: string;
    decimals: number;
}, mintingFeeInBps: number, redeemingFeeInBps: number) {
    it(`Initialize ${collateralMintInfo.symbol} MercurialVaultDepository`, async function () {
        const mercurialVaultDepository = await MercurialVaultDepository.initialize({
            connection: getConnection(),
            collateralMint: collateralMintInfo,
            uxdProgramId,
            cluster: 'devnet',
        });

        await registerMercurialVaultDepositoryTest(authority, controller, mercurialVaultDepository, mintingFeeInBps, redeemingFeeInBps, payer);
    });
};