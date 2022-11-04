import { Signer, PublicKey } from "@solana/web3.js";
import { IdentityDepository } from "@uxd-protocol/uxd-client";
import { Controller, MangoDepository, findATAAddrSync, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { reinjectMangoToIdentityDepository } from "../api";
import { CLUSTER } from "../constants";
import { getBalance } from "../utils";

export const mangoReimburseTest = async function (
  user: Signer,
  payer: Signer,
  controller: Controller,
  depository: IdentityDepository,
  mangoDepository: MangoDepository
): Promise<void> {
  console.group("🧭 mangoReimburseTest");
  try {
    // GIVEN
    const [authorityTokenAccount] = findATAAddrSync(authority.publicKey, tokenMint);
    const [depositoryTokenAccount] = findATAAddrSync(depository.pda, tokenMint);

    const [authorityTokenAccountBalance_pre, depositoryTokenAccountBalance_pre] = await Promise.all([
      getBalance(authorityTokenAccount),
      getBalance(depositoryTokenAccount),
    ]);

    // WHEN
    const txId = await reinjectMangoToIdentityDepository(user, payer, controller, depository, mangoDepository);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const [authorityTokenAccountBalance_post, depositoryTokenAccountBalance_post] = await Promise.all([
      getBalance(authorityTokenAccount),
      getBalance(depositoryTokenAccount),
    ]);

    const receivedTokenAmount = authorityTokenAccountBalance_post - authorityTokenAccountBalance_pre;
    const uiReceivedTokenAmount = nativeToUi(receivedTokenAmount, tokenDecimals);

    console.log(`🧾 Received`, uiReceivedTokenAmount.toLocaleString(), tokenMintSymbol);
    expect(receivedTokenAmount.toString()).equals(
      expectedReceivedTokenAmount.toString(),
      "The amount of received tokens should be bigger than 0"
    );
    expect((depositoryTokenAccountBalance_post - depositoryTokenAccountBalance_pre).toString()).equals(
      0,
      "Depository ATA balance should not change"
    );

    console.groupEnd();
  } catch (error) {
    console.error("❌", error);
    console.groupEnd();
    throw error;
  }
};
