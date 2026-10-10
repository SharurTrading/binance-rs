// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated venue enumerations; regenerate with scripts/codegen/generate.py.

/// Native `CloudMiningPaymentType` values documented for this product.
///
/// Documented at:
/// - <https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-cloud-mining-payment-and-refund-history>
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CloudMiningPaymentType {
    /// Venue code `248`.
    Payment,
    /// Venue code `249`.
    Refund,
    /// Future native integer code, retained exactly.
    Unknown(i64),
}
impl CloudMiningPaymentType {
    /// The exact native integer code.
    #[must_use]
    pub fn value(&self) -> i64 {
        match self {
            Self::Payment => 248,
            Self::Refund => 249,
            Self::Unknown(value) => *value,
        }
    }
}
impl serde::Serialize for CloudMiningPaymentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(self.value())
    }
}
impl<'de> serde::Deserialize<'de> for CloudMiningPaymentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value {
            248 => Self::Payment,
            249 => Self::Refund,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `CloudMiningStatus` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-cloud-mining-payment-and-refund-history>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CloudMiningStatus {
    /// Venue `S`.
    S,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl CloudMiningStatus {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::S => "S",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CloudMiningStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CloudMiningStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "S" => Self::S,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `DepositStatus` values documented for this product.
///
/// Documented at:
/// - <https://developers.binance.com/legacy-docs/wallet/capital/deposite-history>
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DepositStatus {
    /// Venue code `0`.
    Pending,
    /// Venue code `1`.
    Success,
    /// Venue code `2`.
    Rejected,
    /// Venue code `6`.
    CreditedNotWithdrawable,
    /// Venue code `7`.
    WrongDeposit,
    /// Venue code `8`.
    WaitingUserConfirmation,
    /// Future native integer code, retained exactly.
    Unknown(i64),
}
impl DepositStatus {
    /// The exact native integer code.
    #[must_use]
    pub fn value(&self) -> i64 {
        match self {
            Self::Pending => 0,
            Self::Success => 1,
            Self::Rejected => 2,
            Self::CreditedNotWithdrawable => 6,
            Self::WrongDeposit => 7,
            Self::WaitingUserConfirmation => 8,
            Self::Unknown(value) => *value,
        }
    }
}
impl serde::Serialize for DepositStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(self.value())
    }
}
impl<'de> serde::Deserialize<'de> for DepositStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Pending,
            1 => Self::Success,
            2 => Self::Rejected,
            6 => Self::CreditedNotWithdrawable,
            7 => Self::WrongDeposit,
            8 => Self::WaitingUserConfirmation,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `DepositTravelRuleStatus` values documented for this product.
///
/// Documented at:
/// - <https://developers.binance.com/legacy-docs/wallet/capital/deposite-history>
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DepositTravelRuleStatus {
    /// Venue code `0`.
    Ready,
    /// Venue code `1`.
    InformationRequired,
    /// Future native integer code, retained exactly.
    Unknown(i64),
}
impl DepositTravelRuleStatus {
    /// The exact native integer code.
    #[must_use]
    pub fn value(&self) -> i64 {
        match self {
            Self::Ready => 0,
            Self::InformationRequired => 1,
            Self::Unknown(value) => *value,
        }
    }
}
impl serde::Serialize for DepositTravelRuleStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(self.value())
    }
}
impl<'de> serde::Deserialize<'de> for DepositTravelRuleStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Ready,
            1 => Self::InformationRequired,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `SystemStatus` values documented for this product.
///
/// Documented at:
/// - <https://developers.binance.com/legacy-docs/wallet/others/system-status>
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SystemStatus {
    /// Venue code `0`.
    Normal,
    /// Venue code `1`.
    Maintenance,
    /// Future native integer code, retained exactly.
    Unknown(i64),
}
impl SystemStatus {
    /// The exact native integer code.
    #[must_use]
    pub fn value(&self) -> i64 {
        match self {
            Self::Normal => 0,
            Self::Maintenance => 1,
            Self::Unknown(value) => *value,
        }
    }
}
impl serde::Serialize for SystemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(self.value())
    }
}
impl<'de> serde::Deserialize<'de> for SystemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Normal,
            1 => Self::Maintenance,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `TransferDirection` values documented for this product.
///
/// Documented at:
/// - <https://developers.binance.com/legacy-docs/wallet/capital/deposite-history>
/// - <https://developers.binance.com/legacy-docs/wallet/capital/withdraw-history>
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TransferDirection {
    /// Venue code `0`.
    External,
    /// Venue code `1`.
    Internal,
    /// Future native integer code, retained exactly.
    Unknown(i64),
}
impl TransferDirection {
    /// The exact native integer code.
    #[must_use]
    pub fn value(&self) -> i64 {
        match self {
            Self::External => 0,
            Self::Internal => 1,
            Self::Unknown(value) => *value,
        }
    }
}
impl serde::Serialize for TransferDirection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(self.value())
    }
}
impl<'de> serde::Deserialize<'de> for TransferDirection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::External,
            1 => Self::Internal,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `TravelRuleStatus` values documented for this product.
///
/// Documented at:
/// - <https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#deposit-history-travel-rule>
/// - <https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#withdraw-history-v1>
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TravelRuleStatus {
    /// Venue code `0`.
    Completed,
    /// Venue code `1`.
    Pending,
    /// Venue code `2`.
    Failed,
    /// Future native integer code, retained exactly.
    Unknown(i64),
}
impl TravelRuleStatus {
    /// The exact native integer code.
    #[must_use]
    pub fn value(&self) -> i64 {
        match self {
            Self::Completed => 0,
            Self::Pending => 1,
            Self::Failed => 2,
            Self::Unknown(value) => *value,
        }
    }
}
impl serde::Serialize for TravelRuleStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(self.value())
    }
}
impl<'de> serde::Deserialize<'de> for TravelRuleStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Completed,
            1 => Self::Pending,
            2 => Self::Failed,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `TravelRuleVerificationStatus` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#deposit-history-travel-rule>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TravelRuleVerificationStatus {
    /// Venue `PASSED`.
    Passed,
    /// Venue `PENDING`.
    Pending,
    /// Venue `REJECTED`.
    Rejected,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl TravelRuleVerificationStatus {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Passed => "PASSED",
            Self::Pending => "PENDING",
            Self::Rejected => "REJECTED",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for TravelRuleVerificationStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TravelRuleVerificationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "PASSED" => Self::Passed,
            "PENDING" => Self::Pending,
            "REJECTED" => Self::Rejected,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `UniversalTransferType` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#user-universal-transfer>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum UniversalTransferType {
    /// Venue `MAIN_UMFUTURE`.
    MainUmfuture,
    /// Venue `MAIN_CMFUTURE`.
    MainCmfuture,
    /// Venue `MAIN_MARGIN`.
    MainMargin,
    /// Venue `UMFUTURE_MAIN`.
    UmfutureMain,
    /// Venue `UMFUTURE_MARGIN`.
    UmfutureMargin,
    /// Venue `CMFUTURE_MAIN`.
    CmfutureMain,
    /// Venue `CMFUTURE_MARGIN`.
    CmfutureMargin,
    /// Venue `MARGIN_MAIN`.
    MarginMain,
    /// Venue `MARGIN_UMFUTURE`.
    MarginUmfuture,
    /// Venue `MARGIN_CMFUTURE`.
    MarginCmfuture,
    /// Venue `ISOLATEDMARGIN_MARGIN`.
    IsolatedmarginMargin,
    /// Venue `MARGIN_ISOLATEDMARGIN`.
    MarginIsolatedmargin,
    /// Venue `ISOLATEDMARGIN_ISOLATEDMARGIN`.
    IsolatedmarginIsolatedmargin,
    /// Venue `MAIN_FUNDING`.
    MainFunding,
    /// Venue `FUNDING_MAIN`.
    FundingMain,
    /// Venue `FUNDING_UMFUTURE`.
    FundingUmfuture,
    /// Venue `UMFUTURE_FUNDING`.
    UmfutureFunding,
    /// Venue `MARGIN_FUNDING`.
    MarginFunding,
    /// Venue `FUNDING_MARGIN`.
    FundingMargin,
    /// Venue `FUNDING_CMFUTURE`.
    FundingCmfuture,
    /// Venue `CMFUTURE_FUNDING`.
    CmfutureFunding,
    /// Venue `MAIN_OPTION`.
    MainOption,
    /// Venue `OPTION_MAIN`.
    OptionMain,
    /// Venue `UMFUTURE_OPTION`.
    UmfutureOption,
    /// Venue `OPTION_UMFUTURE`.
    OptionUmfuture,
    /// Venue `MARGIN_OPTION`.
    MarginOption,
    /// Venue `OPTION_MARGIN`.
    OptionMargin,
    /// Venue `FUNDING_OPTION`.
    FundingOption,
    /// Venue `OPTION_FUNDING`.
    OptionFunding,
    /// Venue `MAIN_PORTFOLIO_MARGIN`.
    MainPortfolioMargin,
    /// Venue `PORTFOLIO_MARGIN_MAIN`.
    PortfolioMarginMain,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl UniversalTransferType {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::MainUmfuture => "MAIN_UMFUTURE",
            Self::MainCmfuture => "MAIN_CMFUTURE",
            Self::MainMargin => "MAIN_MARGIN",
            Self::UmfutureMain => "UMFUTURE_MAIN",
            Self::UmfutureMargin => "UMFUTURE_MARGIN",
            Self::CmfutureMain => "CMFUTURE_MAIN",
            Self::CmfutureMargin => "CMFUTURE_MARGIN",
            Self::MarginMain => "MARGIN_MAIN",
            Self::MarginUmfuture => "MARGIN_UMFUTURE",
            Self::MarginCmfuture => "MARGIN_CMFUTURE",
            Self::IsolatedmarginMargin => "ISOLATEDMARGIN_MARGIN",
            Self::MarginIsolatedmargin => "MARGIN_ISOLATEDMARGIN",
            Self::IsolatedmarginIsolatedmargin => "ISOLATEDMARGIN_ISOLATEDMARGIN",
            Self::MainFunding => "MAIN_FUNDING",
            Self::FundingMain => "FUNDING_MAIN",
            Self::FundingUmfuture => "FUNDING_UMFUTURE",
            Self::UmfutureFunding => "UMFUTURE_FUNDING",
            Self::MarginFunding => "MARGIN_FUNDING",
            Self::FundingMargin => "FUNDING_MARGIN",
            Self::FundingCmfuture => "FUNDING_CMFUTURE",
            Self::CmfutureFunding => "CMFUTURE_FUNDING",
            Self::MainOption => "MAIN_OPTION",
            Self::OptionMain => "OPTION_MAIN",
            Self::UmfutureOption => "UMFUTURE_OPTION",
            Self::OptionUmfuture => "OPTION_UMFUTURE",
            Self::MarginOption => "MARGIN_OPTION",
            Self::OptionMargin => "OPTION_MARGIN",
            Self::FundingOption => "FUNDING_OPTION",
            Self::OptionFunding => "OPTION_FUNDING",
            Self::MainPortfolioMargin => "MAIN_PORTFOLIO_MARGIN",
            Self::PortfolioMarginMain => "PORTFOLIO_MARGIN_MAIN",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for UniversalTransferType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UniversalTransferType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "MAIN_UMFUTURE" => Self::MainUmfuture,
            "MAIN_CMFUTURE" => Self::MainCmfuture,
            "MAIN_MARGIN" => Self::MainMargin,
            "UMFUTURE_MAIN" => Self::UmfutureMain,
            "UMFUTURE_MARGIN" => Self::UmfutureMargin,
            "CMFUTURE_MAIN" => Self::CmfutureMain,
            "CMFUTURE_MARGIN" => Self::CmfutureMargin,
            "MARGIN_MAIN" => Self::MarginMain,
            "MARGIN_UMFUTURE" => Self::MarginUmfuture,
            "MARGIN_CMFUTURE" => Self::MarginCmfuture,
            "ISOLATEDMARGIN_MARGIN" => Self::IsolatedmarginMargin,
            "MARGIN_ISOLATEDMARGIN" => Self::MarginIsolatedmargin,
            "ISOLATEDMARGIN_ISOLATEDMARGIN" => Self::IsolatedmarginIsolatedmargin,
            "MAIN_FUNDING" => Self::MainFunding,
            "FUNDING_MAIN" => Self::FundingMain,
            "FUNDING_UMFUTURE" => Self::FundingUmfuture,
            "UMFUTURE_FUNDING" => Self::UmfutureFunding,
            "MARGIN_FUNDING" => Self::MarginFunding,
            "FUNDING_MARGIN" => Self::FundingMargin,
            "FUNDING_CMFUTURE" => Self::FundingCmfuture,
            "CMFUTURE_FUNDING" => Self::CmfutureFunding,
            "MAIN_OPTION" => Self::MainOption,
            "OPTION_MAIN" => Self::OptionMain,
            "UMFUTURE_OPTION" => Self::UmfutureOption,
            "OPTION_UMFUTURE" => Self::OptionUmfuture,
            "MARGIN_OPTION" => Self::MarginOption,
            "OPTION_MARGIN" => Self::OptionMargin,
            "FUNDING_OPTION" => Self::FundingOption,
            "OPTION_FUNDING" => Self::OptionFunding,
            "MAIN_PORTFOLIO_MARGIN" => Self::MainPortfolioMargin,
            "PORTFOLIO_MARGIN_MAIN" => Self::PortfolioMarginMain,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `WalletType` values documented for this product.
///
/// Documented at:
/// - <https://developers.binance.com/legacy-docs/wallet/capital/withdraw-history>
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum WalletType {
    /// Venue code `0`.
    Spot,
    /// Venue code `1`.
    Funding,
    /// Future native integer code, retained exactly.
    Unknown(i64),
}
impl WalletType {
    /// The exact native integer code.
    #[must_use]
    pub fn value(&self) -> i64 {
        match self {
            Self::Spot => 0,
            Self::Funding => 1,
            Self::Unknown(value) => *value,
        }
    }
}
impl serde::Serialize for WalletType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(self.value())
    }
}
impl<'de> serde::Deserialize<'de> for WalletType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::Spot,
            1 => Self::Funding,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `WithdrawStatus` values documented for this product.
///
/// Documented at:
/// - <https://developers.binance.com/legacy-docs/wallet/capital/withdraw-history>
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum WithdrawStatus {
    /// Venue code `0`.
    EmailSent,
    /// Venue code `2`.
    AwaitingApproval,
    /// Venue code `3`.
    Rejected,
    /// Venue code `4`.
    Processing,
    /// Venue code `6`.
    Completed,
    /// Future native integer code, retained exactly.
    Unknown(i64),
}
impl WithdrawStatus {
    /// The exact native integer code.
    #[must_use]
    pub fn value(&self) -> i64 {
        match self {
            Self::EmailSent => 0,
            Self::AwaitingApproval => 2,
            Self::Rejected => 3,
            Self::Processing => 4,
            Self::Completed => 6,
            Self::Unknown(value) => *value,
        }
    }
}
impl serde::Serialize for WithdrawStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(self.value())
    }
}
impl<'de> serde::Deserialize<'de> for WithdrawStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value {
            0 => Self::EmailSent,
            2 => Self::AwaitingApproval,
            3 => Self::Rejected,
            4 => Self::Processing,
            6 => Self::Completed,
            _ => Self::Unknown(value),
        })
    }
}
