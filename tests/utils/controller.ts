import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { Depository } from "./depository";
import { MANGO_PROGRAM_ID } from "./mango";
import { connection, createAssocTokenIx, TXN_OPTS, utils, UXD_DECIMALS } from "./utils";

enum ControllerPDASeed {
  State = "STATE",
  UXD = "STABLECOIN",
  Mango = "MANGO",
  Depository = "DEPOSITORY",
  Passthrough = "PASSTHROUGH",
}

export class ControllerUXD {
  public static ProgramId: PublicKey = anchor.workspace.Controller.programId;
  public static rpc: anchor.RpcNamespace = (anchor.workspace.Controller as Program).rpc;

  // Pda
  public static statePda: PublicKey = ControllerUXD.findControllerPda(ControllerPDASeed.State);
  public static mintPda: PublicKey = ControllerUXD.findControllerPda(ControllerPDASeed.UXD);

  public static async mintUXD(collateralAmount: number, slippage: number, depository: Depository, user: anchor.Wallet) {
    const depositedTokenIndex = utils.mango.group.getTokenIndex(depository.collateralMint);
    const mangoCacheAccount = utils.mango.getMangoCache();
    const mangoRootBankAccount = utils.mango.getRootBankForToken(depositedTokenIndex);
    const mangoNodeBankAccount = utils.mango.getNodeBankFor(depositedTokenIndex);
    const mangoDepositedVaultAccount = utils.mango.getVaultFor(depositedTokenIndex);
    const mangoPerpMarketConfig = utils.mango.getPerpMarketConfigFor(depository.collateralSymbol);

    const userCollateralTokenAccount = utils.findAssocTokenAddressSync(user, depository.collateralMint)[0];
    const userUXDTokenAccount = utils.findAssocTokenAddressSync(user, ControllerUXD.mintPda)[0];
    let ixs = undefined;
    if (!(await connection.getAccountInfo(userUXDTokenAccount))) {
      // Create the token account for user's UXD if not exists
      ixs = [createAssocTokenIx(user.publicKey, userUXDTokenAccount, ControllerUXD.mintPda)];
    }
    const collateralAmountBN = new anchor.BN(collateralAmount * 10 ** depository.decimals);
    await ControllerUXD.rpc.mintUxd(collateralAmountBN, slippage, {
      accounts: {
        user: user.publicKey,
        state: ControllerUXD.statePda,
        depository: ControllerUXD.depositoryPda(depository.collateralMint),
        coinMint: depository.collateralMint,
        coinPassthrough: ControllerUXD.coinPassthroughPda(depository.collateralMint),
        userCoin: userCollateralTokenAccount,
        userUxd: userUXDTokenAccount,
        uxdMint: ControllerUXD.mintPda,
        // mango stuff
        mangoGroup: utils.mango.group.publicKey,
        mangoAccount: ControllerUXD.mangoPda(depository.collateralMint),
        mangoCache: mangoCacheAccount,
        // -- for the deposit
        mangoRootBank: mangoRootBankAccount,
        mangoNodeBank: mangoNodeBankAccount,
        mangoVault: mangoDepositedVaultAccount,
        // -- for the position perp opening
        mangoPerpMarket: mangoPerpMarketConfig.publicKey,
        mangoBids: mangoPerpMarketConfig.bidsKey,
        mangoAsks: mangoPerpMarketConfig.asksKey,
        mangoEventQueue: mangoPerpMarketConfig.eventsKey,
        //
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        mangoProgram: MANGO_PROGRAM_ID,
        //
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [user.payer],
      options: TXN_OPTS,
      instructions: ixs,
    });
  }

  public static async redeemUXD(amountUXD: number, slippage: number, depository: Depository, user: anchor.Wallet) {
    const depositedTokenIndex = utils.mango.group.getTokenIndex(depository.collateralMint);
    const mangoCacheAccount = utils.mango.getMangoCache();
    const mangoGroupSigner = utils.mango.group.signerKey;
    const mangoRootBankAccount = utils.mango.getRootBankForToken(depositedTokenIndex);
    const mangoNodeBankAccount = utils.mango.getNodeBankFor(depositedTokenIndex);
    const mangoDepositedVaultAccount = utils.mango.getVaultFor(depositedTokenIndex);
    const mangoPerpMarketConfig = utils.mango.getPerpMarketConfigFor(depository.collateralSymbol);
    const userCollateralTokenAccount = utils.findAssocTokenAddressSync(user, depository.collateralMint)[0];
    const userUXDTokenAccount = utils.findAssocTokenAddressSync(user, ControllerUXD.mintPda)[0];
    const redeemAmount = new anchor.BN(amountUXD * 10 ** UXD_DECIMALS);
    // const slippage = 10; // point based (1000 <=> 100%, 0.1% granularity)
    await ControllerUXD.rpc.redeemUxd(redeemAmount, slippage, {
      accounts: {
        user: user.publicKey,
        state: ControllerUXD.statePda,
        depository: ControllerUXD.depositoryPda(depository.collateralMint),
        coinMint: depository.collateralMint,
        coinPassthrough: ControllerUXD.coinPassthroughPda(depository.collateralMint),
        userCoin: userCollateralTokenAccount,
        userUxd: userUXDTokenAccount,
        uxdMint: ControllerUXD.mintPda,
        // mango stuff
        mangoGroup: utils.mango.group.publicKey,
        mangoAccount: ControllerUXD.mangoPda(depository.collateralMint),
        mangoCache: mangoCacheAccount,
        mangoSigner: mangoGroupSigner,
        // -- for the withdraw
        mangoRootBank: mangoRootBankAccount,
        mangoNodeBank: mangoNodeBankAccount,
        mangoVault: mangoDepositedVaultAccount,
        // -- for the perp position closing
        mangoPerpMarket: mangoPerpMarketConfig.publicKey,
        mangoBids: mangoPerpMarketConfig.bidsKey,
        mangoAsks: mangoPerpMarketConfig.asksKey,
        mangoEventQueue: mangoPerpMarketConfig.eventsKey,
        //
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        mangoProgram: MANGO_PROGRAM_ID,
        //
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [user.payer],
      options: TXN_OPTS,
    });
  }

  public static depositoryPda(collateralMint: PublicKey): PublicKey {
    return utils.findProgramAddressSync(ControllerUXD.ProgramId, [
      Buffer.from(ControllerPDASeed.Depository),
      collateralMint.toBuffer(),
    ])[0];
  }

  // This pda is function of the depository mint
  public static coinPassthroughPda(collateralMint: PublicKey): PublicKey {
    return utils.findProgramAddressSync(ControllerUXD.ProgramId, [
      Buffer.from(ControllerPDASeed.Passthrough),
      collateralMint.toBuffer(),
    ])[0];
  }

  // This pda is function of the depository mint
  public static mangoPda(collateralMint: PublicKey): PublicKey {
    return utils.findProgramAddressSync(ControllerUXD.ProgramId, [
      Buffer.from(ControllerPDASeed.Mango),
      collateralMint.toBuffer(),
    ])[0];
  }

  // Find the depository program PDA adresse for a given seed
  private static findControllerPda(seed: ControllerPDASeed): PublicKey {
    return utils.findProgramAddressSync(ControllerUXD.ProgramId, [Buffer.from(seed.toString())])[0];
  }

  public info() {
    console.log(`\
      [Controller debug info]
        * statePda:                                     ${ControllerUXD.statePda}
        * mintPda:                                      ${ControllerUXD.mintPda}`);
  }
}
