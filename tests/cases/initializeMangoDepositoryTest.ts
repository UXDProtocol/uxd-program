import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository, Mango } from "@uxdprotocol/uxd-client";
import { registerMangoDepository } from "../api";
import { provider } from "../provider";

export const initializeMangoDepositoryTest = async (authority: Signer, controller: Controller, depository: MangoDepository, mango: Mango) => {
    console.groupCollapsed("🧭 initializeMangoDepositoryTest");
    // WHEN
    if (await provider.connection.getAccountInfo(depository.mangoAccountPda)) {
        console.log("🚧 Already registered.");
    } else {
        const txId = await registerMangoDepository(authority, controller, depository, mango);
        console.log(`🔗 'https://explorer.solana.com/address/${txId}?cluster=devnet'`);
    }

    // THEN
    console.log(`🧾 Initialized`, depository.collateralMintSymbol, "Depository");
    depository.info();
    console.groupEnd();
}