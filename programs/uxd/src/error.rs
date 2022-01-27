use crate::UxdResult;
use crate::MAX_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP;
use crate::MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP;
use anchor_lang::prelude::error;
use anchor_lang::prelude::ProgramError;
use mango::error::MangoError;
use num_enum::IntoPrimitive;
use thiserror::Error;

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum SourceFileId {
    InstructionInitializeController = 0,
    InstructionSetRedeemableGlobalSupplyCap = 1,
    InstructionSetMangoDepositoriesRedeemableSoftCap = 2,
    InstructionRegisterMangoDepository = 3,
    InstructionMangoDexMintWithMangoDepository = 4,
    InstructionMangoDexRedeemFromMangoDepository = 5,
    InstructionMangoDexDepositInsuranceToMangoDepository = 6,
    InstructionMangoDexWithdrawInsuranceFromMangoDepository = 7,
    MangoProgramAnchorMango = 8,
    MangoProgramDeposit = 9,
    MangoProgramInitMangoAccount = 10,
    MangoProgramPlacePerpOrder = 11,
    MangoProgramWithdraw = 12,
    MangoUtilsLimitUtils = 13,
    MangoUtilsOrderDelta = 14,
    MangoUtilsOrder = 15,
    MangoUtilsPerpAccountUtils = 16,
    MangoUtilsPerpInfo = 17,
    StateController = 18,
    StateMangoDepository = 19,
    Error = 20,
    Lib = 21,
}

impl std::fmt::Display for SourceFileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceFileId::InstructionInitializeController => {
                write!(f, "src/instructions/initialize_controller.rs")
            }
            SourceFileId::InstructionSetRedeemableGlobalSupplyCap => {
                write!(f, "src/instructions/set_redeemable_global_supply_cap.rs")
            }
            SourceFileId::InstructionSetMangoDepositoriesRedeemableSoftCap => {
                write!(
                    f,
                    "src/instructions/set_mango_depositories_redeemable_soft_cap.rs"
                )
            }
            SourceFileId::InstructionRegisterMangoDepository => {
                write!(f, "src/instructions/register_mango_depository.rs")
            }
            SourceFileId::InstructionMangoDexMintWithMangoDepository => {
                write!(
                    f,
                    "src/instructions/mango_dex/mint_with_mango_depository.rs"
                )
            }
            SourceFileId::InstructionMangoDexRedeemFromMangoDepository => {
                write!(
                    f,
                    "src/instructions/mango_dex/redeem_from_mango_depository.rs"
                )
            }
            SourceFileId::InstructionMangoDexDepositInsuranceToMangoDepository => {
                write!(
                    f,
                    "src/instructions/mango_dex/deposit_insurance_to_mango_depository.rs"
                )
            }
            SourceFileId::InstructionMangoDexWithdrawInsuranceFromMangoDepository => {
                write!(
                    f,
                    "src/instructions/mango_dex/withdraw_insurance_from_mango_depository.rs"
                )
            }
            SourceFileId::MangoProgramAnchorMango => {
                write!(f, "src/mango_program/anchor_mango.rs")
            }
            SourceFileId::MangoProgramDeposit => {
                write!(f, "src/mango_program/deposit.rs")
            }
            SourceFileId::MangoProgramInitMangoAccount => {
                write!(f, "src/mango_program/init_mango_account.rs")
            }
            SourceFileId::MangoProgramPlacePerpOrder => {
                write!(f, "src/mango_program/place_perp_order.rs")
            }
            SourceFileId::MangoProgramWithdraw => {
                write!(f, "src/mango_program/withdraw.rs")
            }
            SourceFileId::MangoUtilsLimitUtils => {
                write!(f, "src/mango_utils/limit_utils.rs")
            }
            SourceFileId::MangoUtilsOrderDelta => {
                write!(f, "src/mango_utils/order_delta.rs")
            }
            SourceFileId::MangoUtilsOrder => {
                write!(f, "src/mango_utils/order.rs")
            }
            SourceFileId::MangoUtilsPerpAccountUtils => {
                write!(f, "src/mango_utils/perp_account_utils.rs")
            }
            SourceFileId::MangoUtilsPerpInfo => {
                write!(f, "src/mango_utils/perp_info.rs")
            }
            SourceFileId::StateController => {
                write!(f, "src/state/controller.rs")
            }
            SourceFileId::StateMangoDepository => {
                write!(f, "src/state/mango_depository.rs")
            }
            SourceFileId::Error => {
                write!(f, "src/error.rs")
            }
            SourceFileId::Lib => {
                write!(f, "src/lib.rs")
            }
        }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum UxdError {
    #[error(transparent)]
    ProgramError(#[from] ProgramError),
    #[error("{uxd_error_code}; {source_file_id}:{line}")]
    UxdErrorCode {
        uxd_error_code: UxdErrorCode,
        line: u32,
        source_file_id: SourceFileId,
    },
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum UxdErrorCode {
    #[error("The redeemable mint decimals must be between 0 and 9 (inclusive).")]
    InvalidRedeemableMintDecimals = 0,
    #[error("Redeemable global supply above {}.", MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP)]
    InvalidRedeemableGlobalSupplyCap,
    #[error("The associated mango root bank index cannot be found for the deposited coin..")]
    RootBankIndexNotFound,
    #[error("The slippage value is invalid. Must be in the [0...1000] range points.")]
    InvalidSlippage,
    #[error("Could not fill the order given order book state and provided slippage.")]
    EffectiveOrderPriceBeyondLimitPrice,
    #[error("Collateral amount must be > 0 in order to mint.")]
    InvalidCollateralAmount,
    #[error("The balance of the collateral ATA is not enough to fulfill the mint operation.")]
    InsufficientCollateralAmount,
    #[error("The redeemable amount for redeem must be superior to 0.")]
    InvalidRedeemableAmount,
    #[error("The balance of the redeemable ATA is not enough to fulfill the redeem operation.")]
    InsufficientRedeemableAmount,
    #[error("The perp position could not be fully filled with the provided slippage.")]
    PerpOrderPartiallyFilled,
    #[error("Minting amount would go past the Redeemable Global Supply Cap.")]
    RedeemableGlobalSupplyCapReached,
    #[error("Operation not allowed due to being over the Redeemable soft Cap.")]
    MangoDepositoriesSoftCapOverflow,
    #[error("Cannot register more mango depositories, the limit has been reached.")]
    MaxNumberOfMangoDepositoriesRegisteredReached,
    #[error("The amount to withdraw from the Insurance Fund must be superior to zero..")]
    InvalidInsuranceAmount,
    #[error("The Insurance ATA from authority doesn't have enough balance.")]
    InsufficientAuthorityInsuranceAmount,
    #[error("The rebalanced amount must be superior to zero..")]
    InvalidRebalancedAmount,
    #[error("Insufficient order book depth for order.")]
    InsufficientOrderBookDepth,
    #[error("The executed order size does not match the expected one.")]
    InvalidExecutedOrderSize,
    #[error("Could not find the perp market index for the given collateral.")]
    MangoPerpMarketIndexNotFound,
    #[error(
        "Mango depositories redeemable soft cap above {}.",
        MAX_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP
    )]
    InvalidMangoDepositoriesRedeemableSoftCap,
    #[error("Quote_lot_delta can't be 0.")]
    InvalidQuoteLotDelta,
    #[error("The perp order wasn't executed in the right direction.")]
    InvalidOrderDirection,
    #[error("Math error.")]
    MathError,
    #[error("The order couldn't be executed with the provided slippage.")]
    SlippageReached,
    #[error("MangoErrorCode::Default Check the source code for more info")]
    Default = u32::MAX,
}

#[error(offset = 200)]
pub enum UxdIdlErrorCode {
    #[msg("Only the Program initializer authority can access this instructions.")]
    InvalidAuthority,
    #[msg("The Depository's controller doesn't match the provided Controller.")]
    InvalidController,
    #[msg("The Depository provided is not registered with the Controller.")]
    InvalidDepository,
    #[msg("The provided collateral mint does not match the depository's collateral mint.")]
    InvalidCollateralMint,
    #[msg("The provided insurance mint does not match the depository's insurance mint.")]
    InvalidInsuranceMint,
    #[msg("The authority's Insurance ATA's mint does not match the Depository's one.")]
    InvalidAuthorityInsuranceATAMint,
    #[msg("The Collateral Passthrough Account isn't the Depository one.")]
    InvalidCollateralPassthroughAccount,
    #[msg("The Insurance Passthrough Account isn't the Depository one.")]
    InvalidInsurancePassthroughAccount,
    #[msg("The Mango Account isn't the Depository one.")]
    InvalidMangoAccount,
    #[msg("The Insurance Passthrough ATA's mint does not match the Depository's one.")]
    InvalidInsurancePassthroughATAMint,
    #[msg("The Redeemable Mint provided does not match the Controller's one.")]
    InvalidRedeemableMint,
    #[msg("The Collateral Passthrough ATA's mint does not match the Depository's one.")]
    InvalidCollateralPassthroughATAMint,
    // #[error("The user's Redeemable ATA's mint does not match the Controller's one.")]
    // InvalidUserRedeemableATAMint,
    // #[error("The user's Collateral ATA's mint does not match the Depository's one.")]
    // InvalidUserCollateralATAMint,
}

impl From<UxdError> for ProgramError {
    fn from(e: UxdError) -> ProgramError {
        match e {
            UxdError::ProgramError(pe) => pe,
            UxdError::UxdErrorCode {
                uxd_error_code,
                line: _,
                source_file_id: _,
            } => ProgramError::Custom(uxd_error_code.into()),
        }
    }
}

impl From<MangoError> for UxdError {
    fn from(me: MangoError) -> Self {
        let pe: ProgramError = me.into();
        pe.into()
    }
}

#[inline]
pub fn check_assert(
    cond: bool,
    uxd_error_code: UxdErrorCode,
    line: u32,
    source_file_id: SourceFileId,
) -> UxdResult<()> {
    if cond {
        Ok(())
    } else {
        Err(UxdError::UxdErrorCode {
            uxd_error_code,
            line,
            source_file_id,
        })
    }
}

#[macro_export]
macro_rules! declare_check_assert_macros {
    ($source_file_id:expr) => {
        #[allow(unused_macros)]
        macro_rules! check {
            ($cond:expr, $err:expr) => {
                check_assert($cond, $err, line!(), $source_file_id)
            };
        }

        #[allow(unused_macros)]
        macro_rules! check_eq {
            ($x:expr, $y:expr, $err:expr) => {
                check_assert($x == $y, $err, line!(), $source_file_id)
            };
        }

        #[allow(unused_macros)]
        macro_rules! throw {
            () => {
                UxdError::UxdErrorCode {
                    uxd_error_code: UxdErrorCode::Default,
                    line: line!(),
                    source_file_id: $source_file_id,
                }
            };
        }

        #[allow(unused_macros)]
        macro_rules! throw_err {
            ($err:expr) => {
                UxdError::UxdErrorCode {
                    uxd_error_code: $err,
                    line: line!(),
                    source_file_id: $source_file_id,
                }
            };
        }

        #[allow(unused_macros)]
        macro_rules! math_err {
            () => {
                UxdError::UxdErrorCode {
                    uxd_error_code: UxdErrorCode::MathError,
                    line: line!(),
                    source_file_id: $source_file_id,
                }
            };
        }
    };
}
