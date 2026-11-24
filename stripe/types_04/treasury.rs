#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryCreditReversalParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The ReceivedCredit to reverse."]
    pub received_credit: String,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryCreditReversalParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("received_credit", &self.received_credit);
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountType::Checking => {
                "checking"
            }
            CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountType::Savings => {
                "savings"
            }
            CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountType::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json>
    for CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "checking" => {
                CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountType::Checking
            }
            "savings" => {
                CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountType::Savings
            }
            other => CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode
    for CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountType
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTreasuryOutboundTransferParamsDestinationPaymentMethodDataType {
    FinancialAccount,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl CreateTreasuryOutboundTransferParamsDestinationPaymentMethodDataType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTreasuryOutboundTransferParamsDestinationPaymentMethodDataType::FinancialAccount => {
                "financial_account"
            }
            CreateTreasuryOutboundTransferParamsDestinationPaymentMethodDataType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json> for CreateTreasuryOutboundTransferParamsDestinationPaymentMethodDataType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTreasuryOutboundTransferParamsDestinationPaymentMethodDataType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "financial_account" => {
                CreateTreasuryOutboundTransferParamsDestinationPaymentMethodDataType::FinancialAccount
            }
            other => CreateTreasuryOutboundTransferParamsDestinationPaymentMethodDataType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryOutboundTransferParamsDestinationPaymentMethodDataType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListTreasuryReceivedCreditsParamsStatus {
    Failed,
    Succeeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl ListTreasuryReceivedCreditsParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListTreasuryReceivedCreditsParamsStatus::Failed => "failed",
            ListTreasuryReceivedCreditsParamsStatus::Succeeded => "succeeded",
            ListTreasuryReceivedCreditsParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json> for ListTreasuryReceivedCreditsParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListTreasuryReceivedCreditsParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "failed" => ListTreasuryReceivedCreditsParamsStatus::Failed,
            "succeeded" => ListTreasuryReceivedCreditsParamsStatus::Succeeded,
            other => ListTreasuryReceivedCreditsParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryReceivedCreditsParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListTreasuryTransactionsParamsStatus {
    Open,
    Posted,
    Void,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl ListTreasuryTransactionsParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListTreasuryTransactionsParamsStatus::Open => "open",
            ListTreasuryTransactionsParamsStatus::Posted => "posted",
            ListTreasuryTransactionsParamsStatus::Void => "void",
            ListTreasuryTransactionsParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json> for ListTreasuryTransactionsParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListTreasuryTransactionsParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "open" => ListTreasuryTransactionsParamsStatus::Open,
            "posted" => ListTreasuryTransactionsParamsStatus::Posted,
            "void" => ListTreasuryTransactionsParamsStatus::Void,
            other => ListTreasuryTransactionsParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryTransactionsParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::Ach => "ach",
            OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::UsDomesticWire => "us_domestic_wire",
            OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn default() -> Self {
        OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach" => OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::Ach,
            "us_domestic_wire" => OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::UsDomesticWire,
            other => OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum ReserveHoldDestination {
    Other,
    RiskReserved,
    SettlementReserved,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl ReserveHoldDestination {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ReserveHoldDestination::Other => "other",
            ReserveHoldDestination::RiskReserved => "risk_reserved",
            ReserveHoldDestination::SettlementReserved => "settlement_reserved",
            ReserveHoldDestination::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for ReserveHoldDestination {
    fn default() -> Self {
        ReserveHoldDestination::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for ReserveHoldDestination {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ReserveHoldDestination::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "other" => ReserveHoldDestination::Other,
            "risk_reserved" => ReserveHoldDestination::RiskReserved,
            "settlement_reserved" => ReserveHoldDestination::SettlementReserved,
            other => ReserveHoldDestination::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum ReserveReleaseReason {
    HoldExpired,
    HoldReleasedEarly,
    HoldReversed,
    PlanDisabled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl ReserveReleaseReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ReserveReleaseReason::HoldExpired => "hold_expired",
            ReserveReleaseReason::HoldReleasedEarly => "hold_released_early",
            ReserveReleaseReason::HoldReversed => "hold_reversed",
            ReserveReleaseReason::PlanDisabled => "plan_disabled",
            ReserveReleaseReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for ReserveReleaseReason {
    fn default() -> Self {
        ReserveReleaseReason::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for ReserveReleaseReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ReserveReleaseReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "hold_expired" => ReserveReleaseReason::HoldExpired,
            "hold_released_early" => ReserveReleaseReason::HoldReleasedEarly,
            "hold_reversed" => ReserveReleaseReason::HoldReversed,
            "plan_disabled" => ReserveReleaseReason::PlanDisabled,
            other => ReserveReleaseReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReservesReservePlansResourcesManualRelease {}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReservesReservePlansResourcesRollingRelease {
    #[doc = "The number of days to reserve funds before releasing."]
    #[json(default)]
    pub days_after_charge: i64,
    #[doc = "The time at which the ReservePlan expires."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<i64>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryCreditReversalObject {
    TreasuryCreditReversal,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryCreditReversalObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryCreditReversalObject::TreasuryCreditReversal => "treasury.credit_reversal",
            TreasuryCreditReversalObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryCreditReversalObject {
    fn default() -> Self {
        TreasuryCreditReversalObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryCreditReversalObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryCreditReversalObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "treasury.credit_reversal" => TreasuryCreditReversalObject::TreasuryCreditReversal,
            other => TreasuryCreditReversalObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryCreditReversalStatus {
    Canceled,
    Posted,
    Processing,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryCreditReversalStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryCreditReversalStatus::Canceled => "canceled",
            TreasuryCreditReversalStatus::Posted => "posted",
            TreasuryCreditReversalStatus::Processing => "processing",
            TreasuryCreditReversalStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryCreditReversalStatus {
    fn default() -> Self {
        TreasuryCreditReversalStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryCreditReversalStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryCreditReversalStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "canceled" => TreasuryCreditReversalStatus::Canceled,
            "posted" => TreasuryCreditReversalStatus::Posted,
            "processing" => TreasuryCreditReversalStatus::Processing,
            other => TreasuryCreditReversalStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryDebitReversalObject {
    TreasuryDebitReversal,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryDebitReversalObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryDebitReversalObject::TreasuryDebitReversal => "treasury.debit_reversal",
            TreasuryDebitReversalObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryDebitReversalObject {
    fn default() -> Self {
        TreasuryDebitReversalObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryDebitReversalObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryDebitReversalObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "treasury.debit_reversal" => TreasuryDebitReversalObject::TreasuryDebitReversal,
            other => TreasuryDebitReversalObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryFinancialAccountsResourceFinancialAddressType {
    Aba,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl TreasuryFinancialAccountsResourceFinancialAddressType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryFinancialAccountsResourceFinancialAddressType::Aba => "aba",
            TreasuryFinancialAccountsResourceFinancialAddressType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl Default for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn default() -> Self {
        TreasuryFinancialAccountsResourceFinancialAddressType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryFinancialAccountsResourceFinancialAddressType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "aba" => TreasuryFinancialAccountsResourceFinancialAddressType::Aba,
            other => TreasuryFinancialAccountsResourceFinancialAddressType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions {
    #[doc = "Timestamp describing when an InboundTransfer changed status to `canceled`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    #[doc = "Timestamp describing when an InboundTransfer changed status to `failed`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<i64>,
    #[doc = "Timestamp describing when an InboundTransfer changed status to `succeeded`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub succeeded_at: Option<i64>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails {
    #[doc = "IP address of the user initiating the OutboundPayment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "`true` if the OutboundPayment creation request is being made on behalf of an end user by a platform."]
    #[json(default)]
    pub present: bool,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetails {
    #[doc = "CHIPS System Sequence Number (SSN) of the OutboundTransfer for transfers sent over the `us_domestic_wire` network."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub chips: Option<String>,
    #[doc = "IMAD of the OutboundTransfer for transfers sent over the `us_domestic_wire` network."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub imad: Option<String>,
    #[doc = "OMAD of the OutboundTransfer for transfers sent over the `us_domestic_wire` network."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub omad: Option<String>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryReceivedCreditFailureCode {
    AccountClosed,
    AccountFrozen,
    InternationalTransaction,
    Other,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryReceivedCreditFailureCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryReceivedCreditFailureCode::AccountClosed => "account_closed",
            TreasuryReceivedCreditFailureCode::AccountFrozen => "account_frozen",
            TreasuryReceivedCreditFailureCode::InternationalTransaction => "international_transaction",
            TreasuryReceivedCreditFailureCode::Other => "other",
            TreasuryReceivedCreditFailureCode::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryReceivedCreditFailureCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryReceivedCreditFailureCode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_closed" => TreasuryReceivedCreditFailureCode::AccountClosed,
            "account_frozen" => TreasuryReceivedCreditFailureCode::AccountFrozen,
            "international_transaction" => TreasuryReceivedCreditFailureCode::InternationalTransaction,
            "other" => TreasuryReceivedCreditFailureCode::Other,
            other => TreasuryReceivedCreditFailureCode::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryReceivedDebitNetwork {
    Ach,
    Card,
    Stripe,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryReceivedDebitNetwork {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryReceivedDebitNetwork::Ach => "ach",
            TreasuryReceivedDebitNetwork::Card => "card",
            TreasuryReceivedDebitNetwork::Stripe => "stripe",
            TreasuryReceivedDebitNetwork::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryReceivedDebitNetwork {
    fn default() -> Self {
        TreasuryReceivedDebitNetwork::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryReceivedDebitNetwork {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryReceivedDebitNetwork::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach" => TreasuryReceivedDebitNetwork::Ach,
            "card" => TreasuryReceivedDebitNetwork::Card,
            "stripe" => TreasuryReceivedDebitNetwork::Stripe,
            other => TreasuryReceivedDebitNetwork::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryReceivedDebitsResourceLinkedFlows {
    #[doc = "The DebitReversal created as a result of this ReceivedDebit being reversed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub debit_reversal: Option<String>,
    #[doc = "Set if the ReceivedDebit is associated with an InboundTransfer's return of funds."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub inbound_transfer: Option<String>,
    #[doc = "Set if the ReceivedDebit was created due to an Issuing Authorization..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuing_authorization: Option<String>,
    #[doc = "Set if the ReceivedDebit is also viewable as an [Issuing Dispute](https://docs.stripe.com/api#issuing_disputes) object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuing_transaction: Option<String>,
    #[doc = "Set if the ReceivedDebit was created due to a [Payout](https://docs.stripe.com/api#payouts) object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payout: Option<String>,
    #[doc = "Set if the ReceivedDebit was created due to a [Topup](https://docs.stripe.com/api#topups) object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub topup: Option<String>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryReceivedDebitsResourceStatusTransitions {
    #[doc = "Timestamp describing when the DebitReversal changed status to `completed`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
}
