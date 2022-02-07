import { getProvider } from "@project-serum/anchor";
import { createAndInitializeMango, Mango } from "@uxdprotocol/uxd-client";
import { CLUSTER } from "./constants";

export let mango: Mango;

export async function mochaGlobalSetup() {
    mango = await createAndInitializeMango(getProvider().connection, CLUSTER);
}