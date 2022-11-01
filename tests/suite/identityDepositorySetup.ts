import { Signer } from "@solana/web3.js";
import { Controller, IdentityDepository } from "@uxd-protocol/uxd-client";
import { initializeIdentityDepository } from "../api";

export const identityDepositorySetupSuite = function (authority: Signer, payer: Signer, controller: Controller, depository: IdentityDepository) {
    it(`Initialize IdentityDepository`, async function () {
        await initializeIdentityDepository(authority, payer, controller, depository);
    });
};