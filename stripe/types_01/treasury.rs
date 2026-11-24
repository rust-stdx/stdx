#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryDebitReversalParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The ReceivedDebit to reverse."]
    pub received_debit: String,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryDebitReversalParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("received_debit", &self.received_debit);
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryInboundTransferParams {
    #[doc = "Amount (in cents) to be transferred."]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The FinancialAccount to send funds to."]
    pub financial_account: String,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The origin payment method to be debited for the InboundTransfer."]
    pub origin_payment_method: String,
    #[doc = "The complete description that appears on your customers' statements."]
    pub statement_descriptor: Option<String>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryInboundTransferParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("currency", &self.currency);
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("financial_account", &self.financial_account);
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("origin_payment_method", &self.origin_payment_method);
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct GetTreasuryCreditReversalParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for GetTreasuryCreditReversalParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListTreasuryInboundTransfersParamsStatus {
    Canceled,
    Failed,
    Processing,
    Succeeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl ListTreasuryInboundTransfersParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListTreasuryInboundTransfersParamsStatus::Canceled => "canceled",
            ListTreasuryInboundTransfersParamsStatus::Failed => "failed",
            ListTreasuryInboundTransfersParamsStatus::Processing => "processing",
            ListTreasuryInboundTransfersParamsStatus::Succeeded => "succeeded",
            ListTreasuryInboundTransfersParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json> for ListTreasuryInboundTransfersParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListTreasuryInboundTransfersParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "canceled" => ListTreasuryInboundTransfersParamsStatus::Canceled,
            "failed" => ListTreasuryInboundTransfersParamsStatus::Failed,
            "processing" => ListTreasuryInboundTransfersParamsStatus::Processing,
            "succeeded" => ListTreasuryInboundTransfersParamsStatus::Succeeded,
            other => ListTreasuryInboundTransfersParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryInboundTransfersParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListTreasuryTransactionEntriesParamsOrderBy {
    Created,
    EffectiveAt,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl ListTreasuryTransactionEntriesParamsOrderBy {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListTreasuryTransactionEntriesParamsOrderBy::Created => "created",
            ListTreasuryTransactionEntriesParamsOrderBy::EffectiveAt => "effective_at",
            ListTreasuryTransactionEntriesParamsOrderBy::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json> for ListTreasuryTransactionEntriesParamsOrderBy {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListTreasuryTransactionEntriesParamsOrderBy::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "created" => ListTreasuryTransactionEntriesParamsOrderBy::Created,
            "effective_at" => ListTreasuryTransactionEntriesParamsOrderBy::EffectiveAt,
            other => ListTreasuryTransactionEntriesParamsOrderBy::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryTransactionEntriesParamsOrderBy {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum ReserveHoldCreatedBy {
    Application,
    Stripe,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl ReserveHoldCreatedBy {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ReserveHoldCreatedBy::Application => "application",
            ReserveHoldCreatedBy::Stripe => "stripe",
            ReserveHoldCreatedBy::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for ReserveHoldCreatedBy {
    fn default() -> Self {
        ReserveHoldCreatedBy::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for ReserveHoldCreatedBy {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ReserveHoldCreatedBy::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "application" => ReserveHoldCreatedBy::Application,
            "stripe" => ReserveHoldCreatedBy::Stripe,
            other => ReserveHoldCreatedBy::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum ReserveReleaseObject {
    ReserveRelease,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl ReserveReleaseObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ReserveReleaseObject::ReserveRelease => "reserve.release",
            ReserveReleaseObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for ReserveReleaseObject {
    fn default() -> Self {
        ReserveReleaseObject::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for ReserveReleaseObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ReserveReleaseObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "reserve.release" => ReserveReleaseObject::ReserveRelease,
            other => ReserveReleaseObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReservesReservePlansResourcesFixedRelease {
    #[doc = "The time after which all reserved funds are requested for release."]
    #[json(default)]
    pub release_after: i64,
    #[doc = "The time at which reserved funds are scheduled for release, automatically set to midnight UTC of the day after `relea..."]
    #[json(default)]
    pub scheduled_release: i64,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryCreditReversalNetwork {
    Ach,
    Stripe,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryCreditReversalNetwork {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryCreditReversalNetwork::Ach => "ach",
            TreasuryCreditReversalNetwork::Stripe => "stripe",
            TreasuryCreditReversalNetwork::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryCreditReversalNetwork {
    fn default() -> Self {
        TreasuryCreditReversalNetwork::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryCreditReversalNetwork {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryCreditReversalNetwork::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach" => TreasuryCreditReversalNetwork::Ach,
            "stripe" => TreasuryCreditReversalNetwork::Stripe,
            other => TreasuryCreditReversalNetwork::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryFinancialAccountsResourceClosedStatusDetailsReasonsItem {
    AccountRejected,
    ClosedByPlatform,
    Other,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl TreasuryFinancialAccountsResourceClosedStatusDetailsReasonsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryFinancialAccountsResourceClosedStatusDetailsReasonsItem::AccountRejected => "account_rejected",
            TreasuryFinancialAccountsResourceClosedStatusDetailsReasonsItem::ClosedByPlatform => "closed_by_platform",
            TreasuryFinancialAccountsResourceClosedStatusDetailsReasonsItem::Other => "other",
            TreasuryFinancialAccountsResourceClosedStatusDetailsReasonsItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryFinancialAccountsResourceClosedStatusDetailsReasonsItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryFinancialAccountsResourceClosedStatusDetailsReasonsItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_rejected" => TreasuryFinancialAccountsResourceClosedStatusDetailsReasonsItem::AccountRejected,
            "closed_by_platform" => TreasuryFinancialAccountsResourceClosedStatusDetailsReasonsItem::ClosedByPlatform,
            "other" => TreasuryFinancialAccountsResourceClosedStatusDetailsReasonsItem::Other,
            other => TreasuryFinancialAccountsResourceClosedStatusDetailsReasonsItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    InboundFlows,
    OutboundFlows,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction::InboundFlows => "inbound_flows",
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction::OutboundFlows => "outbound_flows",
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "inbound_flows" => TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction::InboundFlows,
            "outbound_flows" => TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction::OutboundFlows,
            other => {
                TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryOutboundPaymentObject {
    TreasuryOutboundPayment,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryOutboundPaymentObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryOutboundPaymentObject::TreasuryOutboundPayment => "treasury.outbound_payment",
            TreasuryOutboundPaymentObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryOutboundPaymentObject {
    fn default() -> Self {
        TreasuryOutboundPaymentObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryOutboundPaymentObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryOutboundPaymentObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "treasury.outbound_payment" => TreasuryOutboundPaymentObject::TreasuryOutboundPayment,
            other => TreasuryOutboundPaymentObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundPaymentsResourceAchTrackingDetails {
    #[doc = "ACH trace ID of the OutboundPayment for payments sent over the `ach` network."]
    #[json(default)]
    pub trace_id: String,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryOutboundPaymentsResourceReturnedStatusCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    Declined,
    IncorrectAccountHolderName,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    Other,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryOutboundPaymentsResourceReturnedStatusCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryOutboundPaymentsResourceReturnedStatusCode::AccountClosed => "account_closed",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::AccountFrozen => "account_frozen",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::BankAccountRestricted => "bank_account_restricted",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::BankOwnershipChanged => "bank_ownership_changed",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::Declined => "declined",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::IncorrectAccountHolderName => {
                "incorrect_account_holder_name"
            }
            TreasuryOutboundPaymentsResourceReturnedStatusCode::InvalidAccountNumber => "invalid_account_number",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::InvalidCurrency => "invalid_currency",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::NoAccount => "no_account",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::Other => "other",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryOutboundPaymentsResourceReturnedStatusCode {
    fn default() -> Self {
        TreasuryOutboundPaymentsResourceReturnedStatusCode::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryOutboundPaymentsResourceReturnedStatusCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryOutboundPaymentsResourceReturnedStatusCode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_closed" => TreasuryOutboundPaymentsResourceReturnedStatusCode::AccountClosed,
            "account_frozen" => TreasuryOutboundPaymentsResourceReturnedStatusCode::AccountFrozen,
            "bank_account_restricted" => TreasuryOutboundPaymentsResourceReturnedStatusCode::BankAccountRestricted,
            "bank_ownership_changed" => TreasuryOutboundPaymentsResourceReturnedStatusCode::BankOwnershipChanged,
            "declined" => TreasuryOutboundPaymentsResourceReturnedStatusCode::Declined,
            "incorrect_account_holder_name" => {
                TreasuryOutboundPaymentsResourceReturnedStatusCode::IncorrectAccountHolderName
            }
            "invalid_account_number" => TreasuryOutboundPaymentsResourceReturnedStatusCode::InvalidAccountNumber,
            "invalid_currency" => TreasuryOutboundPaymentsResourceReturnedStatusCode::InvalidCurrency,
            "no_account" => TreasuryOutboundPaymentsResourceReturnedStatusCode::NoAccount,
            "other" => TreasuryOutboundPaymentsResourceReturnedStatusCode::Other,
            other => TreasuryOutboundPaymentsResourceReturnedStatusCode::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundTransfersResourceStatusTransitions {
    #[doc = "Timestamp describing when an OutboundTransfer changed status to `canceled`"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    #[doc = "Timestamp describing when an OutboundTransfer changed status to `failed`"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<i64>,
    #[doc = "Timestamp describing when an OutboundTransfer changed status to `posted`"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<i64>,
    #[doc = "Timestamp describing when an OutboundTransfer changed status to `returned`"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<i64>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryReceivedDebitFailureCode {
    AccountClosed,
    AccountFrozen,
    InsufficientFunds,
    InternationalTransaction,
    Other,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryReceivedDebitFailureCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryReceivedDebitFailureCode::AccountClosed => "account_closed",
            TreasuryReceivedDebitFailureCode::AccountFrozen => "account_frozen",
            TreasuryReceivedDebitFailureCode::InsufficientFunds => "insufficient_funds",
            TreasuryReceivedDebitFailureCode::InternationalTransaction => "international_transaction",
            TreasuryReceivedDebitFailureCode::Other => "other",
            TreasuryReceivedDebitFailureCode::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryReceivedDebitFailureCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryReceivedDebitFailureCode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_closed" => TreasuryReceivedDebitFailureCode::AccountClosed,
            "account_frozen" => TreasuryReceivedDebitFailureCode::AccountFrozen,
            "insufficient_funds" => TreasuryReceivedDebitFailureCode::InsufficientFunds,
            "international_transaction" => TreasuryReceivedDebitFailureCode::InternationalTransaction,
            "other" => TreasuryReceivedDebitFailureCode::Other,
            other => TreasuryReceivedDebitFailureCode::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    Balance,
    FinancialAccount,
    IssuingCard,
    Stripe,
    UsBankAccount,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::Balance => "balance",
            TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::FinancialAccount => "financial_account",
            TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::IssuingCard => "issuing_card",
            TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::Stripe => "stripe",
            TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::UsBankAccount => "us_bank_account",
            TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    fn default() -> Self {
        TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json>
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balance" => {
                TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::Balance
            }
            "financial_account" => {
                TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::FinancialAccount
            }
            "issuing_card" => {
                TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::IssuingCard
            }
            "stripe" => TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::Stripe,
            "us_bank_account" => {
                TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::UsBankAccount
            }
            other => TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount {
    #[doc = "Bank name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[doc = "The last four digits of the bank account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "The routing number for the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryTransactionFlowType {
    CreditReversal,
    DebitReversal,
    InboundTransfer,
    IssuingAuthorization,
    Other,
    OutboundPayment,
    OutboundTransfer,
    ReceivedCredit,
    ReceivedDebit,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryTransactionFlowType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryTransactionFlowType::CreditReversal => "credit_reversal",
            TreasuryTransactionFlowType::DebitReversal => "debit_reversal",
            TreasuryTransactionFlowType::InboundTransfer => "inbound_transfer",
            TreasuryTransactionFlowType::IssuingAuthorization => "issuing_authorization",
            TreasuryTransactionFlowType::Other => "other",
            TreasuryTransactionFlowType::OutboundPayment => "outbound_payment",
            TreasuryTransactionFlowType::OutboundTransfer => "outbound_transfer",
            TreasuryTransactionFlowType::ReceivedCredit => "received_credit",
            TreasuryTransactionFlowType::ReceivedDebit => "received_debit",
            TreasuryTransactionFlowType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryTransactionFlowType {
    fn default() -> Self {
        TreasuryTransactionFlowType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryTransactionFlowType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryTransactionFlowType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "credit_reversal" => TreasuryTransactionFlowType::CreditReversal,
            "debit_reversal" => TreasuryTransactionFlowType::DebitReversal,
            "inbound_transfer" => TreasuryTransactionFlowType::InboundTransfer,
            "issuing_authorization" => TreasuryTransactionFlowType::IssuingAuthorization,
            "other" => TreasuryTransactionFlowType::Other,
            "outbound_payment" => TreasuryTransactionFlowType::OutboundPayment,
            "outbound_transfer" => TreasuryTransactionFlowType::OutboundTransfer,
            "received_credit" => TreasuryTransactionFlowType::ReceivedCredit,
            "received_debit" => TreasuryTransactionFlowType::ReceivedDebit,
            other => TreasuryTransactionFlowType::Unknown(other.to_string()),
        })
    }
}
