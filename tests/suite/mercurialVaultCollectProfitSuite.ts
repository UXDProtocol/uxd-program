import { Signer } from "@solana/web3.js";
import { Controller, MercurialVaultDepository } from "@uxd-protocol/uxd-client";
import { getConnection } from "../connection";
import { collectProfitOfMercurialVaultDepositoryTest } from "../cases/collectProfitOfMercurialVaultDepositoryTest";
import { MERCURIAL_USDC_DEVNET, MERCURIAL_USDC_DEVNET_DECIMALS, uxdProgramId } from "../constants";
import { transferLpTokenToDepositoryLpVault } from "../mercurial_vault_utils";

export const mercurialVaultDepositoryCollectProfitSuite = async function ({
    authority,
    payer,
    controller,
}: {
    authority: Signer;
    payer: Signer;
    controller: Controller;
}) {
    const collateralSymbol = 'USDC';
    let depository: MercurialVaultDepository;

    before('Setup: add LP token to mercurial vault depository LP token safe to simulate interests', async function () {
        depository = await MercurialVaultDepository.initialize({
            connection: getConnection(),
            collateralMint: {
                mint: MERCURIAL_USDC_DEVNET,
                name: "USDC",
                symbol: collateralSymbol,
                decimals: MERCURIAL_USDC_DEVNET_DECIMALS,
            },
            uxdProgramId,
        });

        console.log('depository.collateralMint.mint', depository.collateralMint.mint.toBase58());
        console.log('depository.collateralMint.decimals', depository.collateralMint.decimals);

        // Send LP token directly to depository LP token vault to simulate interest
        await transferLpTokenToDepositoryLpVault({
            amount: 0.001,
            depository,
            payer,
        });
    });

    describe("Collect profit of mercurial vault depository", () => {
        it(`Collect some ${collateralSymbol} should work`, () => collectProfitOfMercurialVaultDepositoryTest({
            authority,
            controller,
            depository,
            payer,
        }));
    });
};