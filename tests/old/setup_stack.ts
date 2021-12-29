import { depositoryBTC, depositoryWSOL } from "../constants";
import { registerMangoDepository } from "../api";
import { authority } from "../identities";
import { provider } from "../provider";
import { controllerUXD } from "../constants";
import { initializeController, getControllerAccount } from "../api";
import { createAndInitializeMango, Mango } from "@uxdprotocol/uxd-client";

export let mango: Mango;
// ----------------------------------------------------------------------------

describe(" ======= [Suite 0 : Initialize mango (1 op)] ======= ", async () => {
    mango = await createAndInitializeMango(provider, `devnet`);
});

