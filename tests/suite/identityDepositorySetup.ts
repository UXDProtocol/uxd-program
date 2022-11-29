import { Signer } from "@solana/web3.js";
import { Controller, IdentityDepository } from "@uxd-protocol/uxd-client";
import { initializeIdentityDepositoryTest } from "../cases/InitializeIdentityDepositoryTest";

export const identityDepositorySetupSuite = function ({
  authority,
  payer,
  controller,
  depository,
}: {
  authority: Signer;
  payer: Signer;
  controller: Controller;
  depository: IdentityDepository;
}) {
  it("Initialize IdentityDepository", () => initializeIdentityDepositoryTest({
    authority,
    controller,
    depository,
    payer,
  }));
};
