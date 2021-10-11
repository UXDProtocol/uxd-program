import { PublicKey, Keypair } from "@solana/web3.js";
import { ControllerUXD } from "./utils/controller";
import { getBalance, TestToken, testUtils, wallet } from "./utils/testutils";
import { Depository } from "./utils/depository";

// Constants
export const BTC_DECIMAL = 6;
export const SOL_DECIMAL = 9;
export const UXD_DECIMAL = 6;

// HELPERS
export async function createTokenEnv(decimals: number, price: bigint) {
  let pythPrice = await testUtils.pyth.createPriceAccount();
  let pythProduct = await testUtils.pyth.createProductAccount();

  await testUtils.pyth.updatePriceAccount(pythPrice, {
    exponent: -9,
    aggregatePriceInfo: {
      price: price * 1000000000n,
    },
  });
  await testUtils.pyth.updateProductAccount(pythProduct, {
    priceAccount: pythPrice.publicKey,
    attributes: {
      quote_currency: "USD",
    },
  });

  return {
    token: await testUtils.createToken(decimals),
    pythPrice,
    pythProduct,
  } as TokenEnv;
}
export interface TokenEnv {
  token: TestToken;
  pythPrice: Keypair;
  pythProduct: Keypair;
}

export async function createTestUser(assets: Array<TokenEnv>): Promise<TestUser> {
  const userWallet = wallet.payer; //await testUtils.createWallet(1 * LAMPORTS_PER_SOL); I WISH TO use that.... but idk it doesn't sign
  // I think it would be neat to have 2 wallets to encure things are tighs, to not have only one GOD wallet that's also the user

  const createUserTokens = async (asset: TokenEnv) => {
    const tokenAccount = await asset.token.getOrCreateAssociatedAccountInfo(userWallet.publicKey);
    return tokenAccount.address;
  };

  let tokenAccounts: Record<string, PublicKey> = {};
  for (const asset of assets) {
    tokenAccounts[asset.token.publicKey.toBase58()] = await createUserTokens(asset);
  }

  return {
    wallet: userWallet,
    tokenAccounts,
  };
}
export interface TestUser {
  wallet: Keypair;
  tokenAccounts: Record<string, PublicKey>;
}

export async function printSystemBalance(depository: Depository) {
  const SYM = depository.collateralName;
  const passthroughPda = ControllerUXD.coinPassthroughPda(depository.collateralMint);
  console.log(`\
        * [depository ${depository.collateralName}]:
        *     ${SYM}:                                        ${await getBalance(depository.depositPda)}
        * [controller]
        *     associated ${SYM} passthrough:                 ${await getBalance(passthroughPda)}`);
}
