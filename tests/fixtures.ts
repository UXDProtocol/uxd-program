import { createAndInitializeMango, Mango } from "@uxd-protocol/uxd-client";
import { CLUSTER } from "./constants";
import { getConnection } from "./connection";

export let mango: Mango;

export async function mochaGlobalSetup() {
    mango = await createAndInitializeMango(getConnection(), CLUSTER);
    console.log("MANGO INIT");
} 