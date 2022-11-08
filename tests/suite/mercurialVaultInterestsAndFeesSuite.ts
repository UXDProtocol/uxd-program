import { Keypair, Signer } from "@solana/web3.js";
import { Controller, MercurialVaultDepository } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { transferTokens } from "../utils";
import { getConnection } from "../connection";
import { collectInterestsAndFeesFromMercurialVaultDepositoryTest } from "../cases/collectInterestsAndFeesFromMercurialVaultDepositoryTest";

export const mercurialVaultDepositoryMintInterestsAndFeesSuite = async function (controllerAuthority: Signer, interestsAndFeesRedeemAuthority: Signer, payer: Signer, controller: Controller, depository: MercurialVaultDepository) {
    before('Setup: add LP token to mercurial vault depository LP token safe to simulate interests', async function () {
        console.log('depository.collateralMint.mint', depository.collateralMint.mint.toBase58());
        console.log('depository.collateralMint.decimals', depository.collateralMint.decimals);

        const onChainDepository = await depository.getOnchainAccount(getConnection());

        await transferTokens(0.1, depository.mercurialVaultLpMint.mint, depository.mercurialVaultLpMint.decimals, payer, onChainDepository.lpTokenVault);
    });

    describe("Collect interests and fees from mercurial vault", () => {
        it(`Collect some ${depository.collateralMint.symbol} should work`, () => collectInterestsAndFeesFromMercurialVaultDepositoryTest(interestsAndFeesRedeemAuthority, controller, depository, payer));
    });

    describe("Wrong authority", () => {
        it(`Collect some ${depository.collateralMint.symbol} should fail`, async () => {
            let err = false;

            try {
                await collectInterestsAndFeesFromMercurialVaultDepositoryTest(new Keypair(), controller, depository, payer);
            } catch {
                err = true;
            }

            expect(err).equals(true, 'Should have failed due to wrong interests and fees redeem authority');
        });
    });
};