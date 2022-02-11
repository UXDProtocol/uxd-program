import { createAndInitializeMango, Mango } from "@uxdprotocol/uxd-client";
import { CLUSTER } from "./constants";
import { getConnection } from "./connection";

export let mango: Mango;

export async function mochaGlobalSetup() {
    mango = await createAndInitializeMango(getConnection(), CLUSTER);
} 