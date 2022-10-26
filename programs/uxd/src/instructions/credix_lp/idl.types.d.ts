import { AnchorTypes } from "@saberhq/anchor-contrib";
import { Credix } from "./credix";
export declare type CredixTypes = AnchorTypes<Credix, {
    programState: ProgramState;
    deal: Deal;
    globalMarketState: GlobalMarketState;
    borrowerInfo: BorrowerInfo;
    credixPass: CredixPass;
    dealTranches: Tranches;
    investorTranche: InvestorTranche;
    tranchePass: TranchePass;
    repaymentSchedule: RepaymentSchedule;
    marketAdmins: MarketAdmins;
}, {
    Fraction: Fraction;
    DealTranche: Tranche;
    RepaymentPeriod: RepaymentPeriod;
    RepaymentPeriodInput: RepaymentPeriodInput;
    TrancheConfig: TrancheConfig;
}>;
export declare type CredixProgram = CredixTypes["Program"];
export declare type CredixAccounts = CredixTypes["Accounts"];
export declare type ProgramState = CredixAccounts["programState"];
export declare type Deal = CredixAccounts["deal"];
export declare type CredixPass = CredixAccounts["credixPass"];
export declare type GlobalMarketState = CredixAccounts["globalMarketState"];
export declare type BorrowerInfo = CredixAccounts["borrowerInfo"];
export declare type Tranches = CredixAccounts["dealTranches"];
export declare type InvestorTranche = CredixAccounts["investorTranche"];
export declare type TranchePass = CredixAccounts["tranchePass"];
export declare type RepaymentSchedule = CredixAccounts["repaymentSchedule"];
export declare type MarketAdmins = CredixAccounts["marketAdmins"];
export declare type Fraction = CredixTypes["Defined"]["Fraction"];
export declare type Tranche = CredixTypes["Defined"]["DealTranche"];
export declare type RepaymentPeriod = CredixTypes["Defined"]["RepaymentPeriod"];
export declare type RepaymentPeriodInput = CredixTypes["Defined"]["RepaymentPeriodInput"];
export declare type TrancheConfig = CredixTypes["Defined"]["TrancheConfig"];
export declare type CredixEvents = CredixTypes["Events"];
export declare type BurnTrancheTokensEvent = CredixEvents["BurnTrancheTokensEvent"];
export declare type CreateCredixPassEvent = CredixEvents["CreateCredixPassEvent"];
export declare type CreateTranchePassEvent = CredixEvents["CreateTranchePassEvent"];
export declare type DealCreationEvent = CredixEvents["DealCreationEvent"];
export declare type DealActivationEvent = CredixEvents["DealActivationEvent"];
export declare type DealRepaymentEvent = CredixEvents["DealRepaymentEvent"];
export declare type DealWithdrawEvent = CredixEvents["DealWithdrawEvent"];
export declare type DepositEvent = CredixEvents["DepositEvent"];
export declare type DepositTrancheEvent = CredixEvents["DepositTrancheEvent"];
export declare type FreezeGlobalMarketStateEvent = CredixEvents["FreezeGlobalMarketStateEvent"];
export declare type OpenDealEvent = CredixEvents["OpenDealEvent"];
export declare type SetRepaymentScheduleEvent = CredixEvents["SetRepaymentScheduleEvent"];
export declare type SetTranchesEvent = CredixEvents["SetTranchesEvent"];
export declare type ThawGlobalMarketStateEvent = CredixEvents["ThawGlobalMarketStateEvent"];
export declare type UpdateCredixPassEvent = CredixEvents["UpdateCredixPassEvent"];
export declare type UpdateTranchePassEvent = CredixEvents["UpdateTranchePassEvent"];
export declare type WithdrawEvent = CredixEvents["WithdrawEvent"];
export declare type WithdrawTrancheEvent = CredixEvents["WithdrawTrancheEvent"];
export declare enum AccountNames {
    Deal = "Deal",
    CredixPass = "credixPass",
    GlobalMarketState = "globalMarketState",
    BorrowerInfo = "borrowerInfo",
    Tranches = "dealTranches",
    InvestorTranche = "investorTranche",
    TranchePass = "tranchePass",
    RepaymentSchedule = "repaymentSchedule",
    ProgramState = "programState",
    MarketAdmins = "marketAdmins"
}
//# sourceMappingURL=idl.types.d.ts.map