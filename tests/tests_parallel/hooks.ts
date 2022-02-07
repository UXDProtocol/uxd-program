import { getProvider } from "@project-serum/anchor";
import { Mango, createAndInitializeMango } from "@uxdprotocol/uxd-client";
import { CLUSTER } from "../constants";


// Available root hooks and their behavior:

// beforeAll:
// In serial mode(Mocha’s default ), before all tests begin, once only
// In parallel mode, run before all tests begin, for each file
// beforeEach:
// In both modes, run before each test
// afterAll:
// In serial mode, run after all tests end, once only
// In parallel mode, run after all tests end, for each file
// afterEach:
// In both modes, run after every test


export const mochaHooks = () => {
    // root hooks object
    return {
        beforeAll: [
            function () {
                if (process.env.TEST_SOL == "true") {

                }
                if (process.env.TEST_BTC) {

                }
                if (process.env.TEST_ETH) {

                }
                if (process.env.TEST_BTC) {

                }
            }
        ],
        afterEach() {
            beforeEach("\n", () => { console.log("=============================================\n\n") });
        }
    };
};