import { Signer } from "@solana/web3.js";
import { Controller } from "@uxdprotocol/uxd-client";
import { initializeController, getControllerAccount } from "../api";
import { CLUSTER } from "../constants";
import { getProvider } from "../provider";

export const initializeControllerTest = async (authority: Signer, controller: Controller) => {
    console.group("⏱ initializeControllerTest");
    try {
        // WHEN
        if (await getProvider().connection.getAccountInfo(controller.pda)) {
            console.log("🚧 Already initialized.");
        } else {
            const txId = await initializeController(authority, controller);
            console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);
        }

        // THEN
        // const controllerAccount = await getControllerAccount(controller);
        console.log(`🧾 Initialized`, controller.redeemableMintSymbol, "Controller");
        controller.info();
        console.groupEnd();
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}