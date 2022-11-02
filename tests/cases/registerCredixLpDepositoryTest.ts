import { Signer } from "@solana/web3.js";
import { Controller, CredixLpDepository } from "@uxd-protocol/uxd-client";
import { registerCredixLpDepository } from "../api";
import { CLUSTER } from "../constants";
import { getConnection } from "../connection";

export const registerCredixLpDepositoryTest = async function (
  authority: Signer,
  controller: Controller,
  depository: CredixLpDepository,
  uiAccountingSupplyRedeemableSoftCap: number,
  accountingBpsStampFeeMint: number,
  accountingBpsStampFeeRedeem: number,
  payer?: Signer
) {
  console.group("🧭 initializeCredixLpDepositoryTest");
  try {
    // WHEN
    if (await getConnection().getAccountInfo(depository.pda)) {
      console.log("🚧 Already registered.");
    } else {
      const txId = await registerCredixLpDepository(
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
