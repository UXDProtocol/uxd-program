import { Controller, MangoDepository, SOL_DECIMALS, USDC_DECIMALS, UXD_DECIMALS } from "@uxdprotocol/uxd-client";
import { authority, USDC, user, WSOL, uxdProgram } from "./constants";
import { mangoDepositoryMintRedeemSuite } from "./suite/mangoDepositoryMintRedeemSuite";

const depositoryWSOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgram.programId);
const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgram.programId);

describe("SOL Mint/Redeem tests", () => {
    mangoDepositoryMintRedeemSuite(authority, user, controllerUXD, depositoryWSOL);
});