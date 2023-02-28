import { Signer } from '@solana/web3.js';
import { Controller, nativeToUi } from '@uxd-protocol/uxd-client';
import { expect } from 'chai';
import { editController } from '../api';
import { getConnection, TXN_OPTS } from '../connection';
import { CLUSTER } from '../constants';

export const editControllerTest = async function ({
  authority,
  controller,
  uiFields,
}: {
  authority: Signer;
  controller: Controller;
  uiFields: {
    redeemableGlobalSupplyCap?: number;
  };
}) {
  const connection = getConnection();
  const options = TXN_OPTS;

  console.group('🧭 editControllerTest');
  try {
    // GIVEN
    const controllerOnChainAccount = await controller.getOnchainAccount(
      connection,
      options
    );

    const redeemableGlobalSupplyCap_pre =
      controllerOnChainAccount.redeemableGlobalSupplyCap;

    // WHEN
    const txId = await editController({
      authority,
      controller,
      uiFields,
    });
    console.log(
      `🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`
    );

    // THEN
    const controllerOnChainAccount_post = await controller.getOnchainAccount(
      connection,
      options
    );

    const redeemableCirculatingSupply = nativeToUi(
      controllerOnChainAccount_post.redeemableCirculatingSupply,
      controller.redeemableMintDecimals
    );

    const redeemableGlobalSupplyCap_post =
      controllerOnChainAccount_post.redeemableGlobalSupplyCap;

    if (typeof uiFields.redeemableGlobalSupplyCap !== 'undefined') {
      const redeemableGlobalSupplyCap_postUi = nativeToUi(
        redeemableGlobalSupplyCap_post,
        controller.redeemableMintDecimals
      );
      expect(
        redeemableGlobalSupplyCap_postUi.toFixed(
          controller.redeemableMintDecimals
        )
      ).equals(
        uiFields.redeemableGlobalSupplyCap.toFixed(
          controller.redeemableMintDecimals
        ),
        'Redeemable Global Supply Cap must bet set'
      );
      console.log(
        `🧾 Previous global supply cap was`,
        redeemableGlobalSupplyCap_pre,
        'now is',
        redeemableGlobalSupplyCap_post,
        '(circulating supply',
        redeemableCirculatingSupply,
        ')'
      );
    } else {
      expect(
        redeemableGlobalSupplyCap_pre.cmp(redeemableGlobalSupplyCap_post)
      ).equals(0, 'Redeemable Global Supply Cap must not have changed');
    }

    controller.info();
    console.groupEnd();
  } catch (error) {
    console.error('❌', error);
    console.groupEnd();
    throw error;
  }
};
