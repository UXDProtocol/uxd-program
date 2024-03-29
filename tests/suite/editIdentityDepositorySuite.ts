import { Signer } from '@solana/web3.js';
import {
  Controller,
  IdentityDepository,
  IdentityDepositoryAccount,
  nativeToUi,
} from '@uxd-protocol/uxd-client';
import { getConnection, TXN_OPTS } from '../connection';
import { editIdentityDepositoryTest } from '../cases/editIdentityDepositoryTest';
import { createIdentityDepositoryDevnet } from '../utils';

export const editIdentityDepositorySuite = async function ({
  authority,
  controller,
}: {
  authority: Signer;
  controller: Controller;
}) {
  let depository: IdentityDepository;

  let beforeDepository: IdentityDepositoryAccount;

  describe('Edit mint/redeem', () => {
    // Snapshot the initial depository values
    before(async () => {
      depository = createIdentityDepositoryDevnet();

      beforeDepository = await depository.getOnchainAccount(
        getConnection(),
        TXN_OPTS
      );
    });

    it('Edit redeemableAmountUnderManagementCap alone should work', async function () {
      const redeemableAmountUnderManagementCap = 50;

      console.log(
        '[🧾 redeemableAmountUnderManagementCap',
        redeemableAmountUnderManagementCap,
        ']'
      );

      await editIdentityDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          redeemableAmountUnderManagementCap,
        },
      });
    });

    it('Edit mintingDisabled alone should work', async function () {
      const mintingDisabled = true;

      console.log('[🧾 mintingDisabled', mintingDisabled, ']');

      await editIdentityDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          mintingDisabled,
        },
      });
    });

    // Restore initial depository values there
    it('Edit redeemableAmountUnderManagementCap should work', async function () {
      const { redeemableAmountUnderManagementCap, mintingDisabled } =
        beforeDepository;

      const uiRedeemableAmountUnderManagementCap = nativeToUi(
        redeemableAmountUnderManagementCap,
        controller.redeemableMintDecimals
      );

      console.log(
        '[🧾 redeemableAmountUnderManagementCap',
        uiRedeemableAmountUnderManagementCap,
        ']'
      );
      console.log('[🧾 mintingDisabled', mintingDisabled, ']');

      await editIdentityDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          mintingDisabled,
          redeemableAmountUnderManagementCap:
            uiRedeemableAmountUnderManagementCap,
        },
      });
    });
  });
};
