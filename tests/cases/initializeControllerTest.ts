import { Signer } from "@solana/web3.js";
import { Controller } from "@uxdprotocol/uxd-client";
import { initializeController, getControllerAccount } from "../api";
import { provider } from "../provider";

export const initializeControllerTest = async (authority: Signer, controller: Controller) => {
    console.groupCollapsed("⏱ initializeControllerTest");
    // WHEN
    if (await provider.connection.getAccountInfo(controller.pda)) {
        console.log("🚧 Already initialized.");
    } else {
        const txId = await initializeController(authority, controller);
        console.log(`🔗 'https://explorer.solana.com/address/${txId}?cluster=devnet'`);
    }

    // THEN
    const controllerAccount = await getControllerAccount(controller);
    console.log(`🧾 Initialized`, controller.redeemableMintSymbol, "Controller");
    controller.info();
    console.groupEnd();
}