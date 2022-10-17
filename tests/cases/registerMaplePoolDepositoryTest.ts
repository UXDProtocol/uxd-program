import { Signer } from "@solana/web3.js";
import { Controller, MaplePoolDepository } from "@uxd-protocol/uxd-client";
import { registerMaplePoolDepository } from "../api";
import { CLUSTER } from "../constants";
import { getConnection } from "../connection";

export const registerMaplePoolDepositoryTest = async function (
  authority: Signer,
  controller: Controller,
  depository: MaplePoolDepository,
  uiAccountingSupplyRedeemableSoftCap: number,
  accountingBpsStampFeeMint: number,
  accountingBpsStampFeeRedeem: number,
  payer?: Signer
) {
  console.group("🧭 initializeMaplePoolDepositoryTest");
  try {
    // WHEN
    if (await getConnection().getAccountInfo(depository.pda)) {
      console.log("🚧 Already registered.");
    } else {
      const txId = await registerMaplePoolDepository(
        authority,
        payer ?? authority,
        controller,
        depository,
        uiAccountingSupplyRedeemableSoftCap,
        accountingBpsStampFeeMint,
        accountingBpsStampFeeRedeem
      );
      console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);
    }

    // THEN
    console.log(`🧾 Initialized`, "Maple Pool Depository");
    depository.info();
    console.groupEnd();
  } catch (error) {
    console.error("❌", error);
    console.groupEnd();
    throw error;
  }
};
