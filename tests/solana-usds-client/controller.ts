import { Cluster } from "@blockworks-foundation/mango-client";
import { BN, Program, Provider, RpcNamespace, Wallet, workspace } from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, ConfirmOptions } from "@solana/web3.js";
import { provider } from "../provider";
import { Depository } from "./depository";
import { Mango } from "./mango";
import { createAssocTokenIx, findAssocTokenAddressSync, findPDA, UXD_DECIMALS } from "./utils";

enum ControllerPDASeed {
  State = "STATE",
  UXD = "STABLECOIN",
  Mango = "MANGO",
  Depository = "DEPOSITORY",
  Passthrough = "PASSTHROUGH",
}

export class ControllerUXD {
  public static ProgramId: PublicKey = workspace.Controller.programId;
  static rpc: RpcNamespace = (workspace.Controller as Program).rpc;
  static statePda: PublicKey = ControllerUXD.findControllerPda(ControllerPDASeed.State);
  static mintPda: PublicKey = ControllerUXD.findControllerPda(ControllerPDASeed.UXD);
  mango: Mango;

  public constructor(cluster: Cluster) {
    this.mango = new Mango(provider, cluster);
  }

  public async initialize(admin: Wallet, options: ConfirmOptions) {
    await ControllerUXD.rpc.initialize({
      accounts: {
        authority: admin.publicKey,
        state: ControllerUXD.statePda,
        uxdMint: ControllerUXD.mintPda,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [admin.payer],
      options: options,
    });
  }

  public async register(depository: Depository, admin: Wallet, options: ConfirmOptions) {
    await ControllerUXD.rpc.registerDepository({
      accounts: {
        authority: admin.publicKey,
        state: ControllerUXD.statePda,
        depository: ControllerUXD.depositoryPda(depository.collateralMint),
        collateralMint: depository.collateralMint,
        collateralPassthrough: ControllerUXD.collateralPassthroughPda(depository.collateralMint),
        mangoGroup: this.mango.group.publicKey,
        mangoAccount: ControllerUXD.mangoPda(depository.collateralMint),
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        mangoProgram: this.mango.programId,
      },
      signers: [admin.payer],
      options: options,
    });
  }

  public async mintUXD(
    provider: Provider, // necessary to create account beforehand
    collateralAmount: number,
    slippage: number,
    depository: Depository,
    user: Wallet,
    options: ConfirmOptions
  ) {
    const depositedTokenIndex = this.mango.group.getTokenIndex(depository.collateralMint);
    const mangoCacheAccount = this.mango.getMangoCache();
    const mangoRootBankAccount = this.mango.getRootBankForToken(depositedTokenIndex);
    const mangoNodeBankAccount = this.mango.getNodeBankFor(depositedTokenIndex);
    const mangoDepositedVaultAccount = this.mango.getVaultFor(depositedTokenIndex);
    const mangoPerpMarketConfig = this.mango.getPerpMarketConfigFor(depository.collateralSymbol);

    const userCollateralTokenAccount = findAssocTokenAddressSync(user, depository.collateralMint)[0];
    const userUXDTokenAccount = findAssocTokenAddressSync(user, ControllerUXD.mintPda)[0];
    let ixs = undefined;
    if (!(await provider.connection.getAccountInfo(userUXDTokenAccount))) {
      // Create the token account for user's UXD if not exists
      ixs = [createAssocTokenIx(user.publicKey, userUXDTokenAccount, ControllerUXD.mintPda)];
    }
    const collateralAmountBN = new BN(collateralAmount * 10 ** depository.decimals);
    await ControllerUXD.rpc.mintUxd(collateralAmountBN, slippage, {
      accounts: {
        user: user.publicKey,
        state: ControllerUXD.statePda,
        depository: ControllerUXD.depositoryPda(depository.collateralMint),
        collateralMint: depository.collateralMint,
        uxdMint: ControllerUXD.mintPda,
        userCollateral: userCollateralTokenAccount,
        userUxd: userUXDTokenAccount,
        collateralPassthrough: ControllerUXD.collateralPassthroughPda(depository.collateralMint),
        // mango stuff
        mangoGroup: this.mango.group.publicKey,
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
        mangoProgram: this.mango.programId,
        //
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [user.payer],
      options: options,
      instructions: ixs,
    });
  }

  public async redeemUXD(
    amountUXD: number,
    slippage: number,
    depository: Depository,
    user: Wallet,
    options: ConfirmOptions
  ) {
    const depositedTokenIndex = this.mango.group.getTokenIndex(depository.collateralMint);
    const mangoCacheAccount = this.mango.getMangoCache();
    const mangoGroupSigner = this.mango.group.signerKey;
    const mangoRootBankAccount = this.mango.getRootBankForToken(depositedTokenIndex);
    const mangoNodeBankAccount = this.mango.getNodeBankFor(depositedTokenIndex);
    const mangoDepositedVaultAccount = this.mango.getVaultFor(depositedTokenIndex);
    const mangoPerpMarketConfig = this.mango.getPerpMarketConfigFor(depository.collateralSymbol);
    const userCollateralTokenAccount = findAssocTokenAddressSync(user, depository.collateralMint)[0];
    const userUXDTokenAccount = findAssocTokenAddressSync(user, ControllerUXD.mintPda)[0];
    const redeemAmount = new BN(amountUXD * 10 ** UXD_DECIMALS);
    // const slippage = 10; // point based (1000 <=> 100%, 0.1% granularity)
    await ControllerUXD.rpc.redeemUxd(redeemAmount, slippage, {
      accounts: {
        user: user.publicKey,
        state: ControllerUXD.statePda,
        depository: ControllerUXD.depositoryPda(depository.collateralMint),
        collateralMint: depository.collateralMint,
        uxdMint: ControllerUXD.mintPda,
        userCollateral: userCollateralTokenAccount,
        userUxd: userUXDTokenAccount,
        collateralPassthrough: ControllerUXD.collateralPassthroughPda(depository.collateralMint),
        // mango stuff
        mangoGroup: this.mango.group.publicKey,
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
        mangoProgram: this.mango.programId,
        //
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [user.payer],
      options: options,
    });
  }

  public static depositoryPda(collateralMint: PublicKey): PublicKey {
    return findPDA(ControllerUXD.ProgramId, [Buffer.from(ControllerPDASeed.Depository), collateralMint.toBuffer()])[0];
  }

  // This pda is function of the depository mint
  public static collateralPassthroughPda(collateralMint: PublicKey): PublicKey {
    return findPDA(ControllerUXD.ProgramId, [Buffer.from(ControllerPDASeed.Passthrough), collateralMint.toBuffer()])[0];
  }

  // This pda is function of the depository mint
  public static mangoPda(collateralMint: PublicKey): PublicKey {
    return findPDA(ControllerUXD.ProgramId, [Buffer.from(ControllerPDASeed.Mango), collateralMint.toBuffer()])[0];
  }

  // Find the depository program PDA adresse for a given seed
  private static findControllerPda(seed: ControllerPDASeed): PublicKey {
    return findPDA(ControllerUXD.ProgramId, [Buffer.from(seed.toString())])[0];
  }

  public info() {
    console.log(`\
      [Controller debug info]
        * statePda:                                     ${ControllerUXD.statePda}
        * mintPda:                                      ${ControllerUXD.mintPda}`);
  }
}
