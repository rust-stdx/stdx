#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataBillingDetailsAddress {
    #[doc = "City, district, suburb, town, or village."]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: Option<String>,
    #[doc = "Address line 1, such as the street, PO Box, or company name."]
    pub line1: Option<String>,
    #[doc = "Address line 2, such as the apartment, suite, unit, or building."]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    pub postal_code: Option<String>,
    #[doc = "State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    pub state: Option<String>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode
    for CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataBillingDetailsAddress
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("line1", self.line1.as_ref());
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum InboundTransfersType {
    UsBankAccount,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl InboundTransfersType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InboundTransfersType::UsBankAccount => "us_bank_account",
            InboundTransfersType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for InboundTransfersType {
    fn default() -> Self {
        InboundTransfersType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for InboundTransfersType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InboundTransfersType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "us_bank_account" => InboundTransfersType::UsBankAccount,
            other => InboundTransfersType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct ListTreasuryFinancialAccountsParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryFinancialAccountsParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListTreasuryFinancialAccountsParamsStatus {
    Closed,
    Open,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl ListTreasuryFinancialAccountsParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListTreasuryFinancialAccountsParamsStatus::Closed => "closed",
            ListTreasuryFinancialAccountsParamsStatus::Open => "open",
            ListTreasuryFinancialAccountsParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json> for ListTreasuryFinancialAccountsParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListTreasuryFinancialAccountsParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "closed" => ListTreasuryFinancialAccountsParamsStatus::Closed,
            "open" => ListTreasuryFinancialAccountsParamsStatus::Open,
            other => ListTreasuryFinancialAccountsParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryFinancialAccountsParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    Stripe,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork::Stripe => "stripe",
            OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn default() -> Self {
        OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "stripe" => OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork::Stripe,
            other => OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum OutboundPaymentsPaymentMethodDetailsType {
    FinancialAccount,
    UsBankAccount,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl OutboundPaymentsPaymentMethodDetailsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            OutboundPaymentsPaymentMethodDetailsType::FinancialAccount => "financial_account",
            OutboundPaymentsPaymentMethodDetailsType::UsBankAccount => "us_bank_account",
            OutboundPaymentsPaymentMethodDetailsType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for OutboundPaymentsPaymentMethodDetailsType {
    fn default() -> Self {
        OutboundPaymentsPaymentMethodDetailsType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for OutboundPaymentsPaymentMethodDetailsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(OutboundPaymentsPaymentMethodDetailsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "financial_account" => OutboundPaymentsPaymentMethodDetailsType::FinancialAccount,
            "us_bank_account" => OutboundPaymentsPaymentMethodDetailsType::UsBankAccount,
            other => OutboundPaymentsPaymentMethodDetailsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum ReserveHoldObject {
    ReserveHold,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl ReserveHoldObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ReserveHoldObject::ReserveHold => "reserve.hold",
            ReserveHoldObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for ReserveHoldObject {
    fn default() -> Self {
        ReserveHoldObject::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for ReserveHoldObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ReserveHoldObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "reserve.hold" => ReserveHoldObject::ReserveHold,
            other => ReserveHoldObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum ReservePlanObject {
    ReservePlan,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl ReservePlanObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ReservePlanObject::ReservePlan => "reserve.plan",
            ReservePlanObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for ReservePlanObject {
    fn default() -> Self {
        ReservePlanObject::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for ReservePlanObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ReservePlanObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "reserve.plan" => ReservePlanObject::ReservePlan,
            other => ReservePlanObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum ReservePlanStatus {
    Active,
    Disabled,
    Expired,
    Other,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl ReservePlanStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ReservePlanStatus::Active => "active",
            ReservePlanStatus::Disabled => "disabled",
            ReservePlanStatus::Expired => "expired",
            ReservePlanStatus::Other => "other",
            ReservePlanStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for ReservePlanStatus {
    fn default() -> Self {
        ReservePlanStatus::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for ReservePlanStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ReservePlanStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => ReservePlanStatus::Active,
            "disabled" => ReservePlanStatus::Disabled,
            "expired" => ReservePlanStatus::Expired,
            "other" => ReservePlanStatus::Other,
            other => ReservePlanStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum ReservePlanType {
    FixedRelease,
    ManualRelease,
    Other,
    RollingRelease,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl ReservePlanType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ReservePlanType::FixedRelease => "fixed_release",
            ReservePlanType::ManualRelease => "manual_release",
            ReservePlanType::Other => "other",
            ReservePlanType::RollingRelease => "rolling_release",
            ReservePlanType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for ReservePlanType {
    fn default() -> Self {
        ReservePlanType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for ReservePlanType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ReservePlanType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "fixed_release" => ReservePlanType::FixedRelease,
            "manual_release" => ReservePlanType::ManualRelease,
            "other" => ReservePlanType::Other,
            "rolling_release" => ReservePlanType::RollingRelease,
            other => ReservePlanType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryFinancialAccountActiveFeaturesItem {
    CardIssuing,
    DepositInsurance,
    FinancialAddressesAba,
    FinancialAddressesAbaForwarding,
    InboundTransfersAch,
    IntraStripeFlows,
    OutboundPaymentsAch,
    OutboundPaymentsUsDomesticWire,
    OutboundTransfersAch,
    OutboundTransfersUsDomesticWire,
    RemoteDepositCapture,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl TreasuryFinancialAccountActiveFeaturesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryFinancialAccountActiveFeaturesItem::CardIssuing => "card_issuing",
            TreasuryFinancialAccountActiveFeaturesItem::DepositInsurance => "deposit_insurance",
            TreasuryFinancialAccountActiveFeaturesItem::FinancialAddressesAba => "financial_addresses.aba",
            TreasuryFinancialAccountActiveFeaturesItem::FinancialAddressesAbaForwarding => {
                "financial_addresses.aba.forwarding"
            }
            TreasuryFinancialAccountActiveFeaturesItem::InboundTransfersAch => "inbound_transfers.ach",
            TreasuryFinancialAccountActiveFeaturesItem::IntraStripeFlows => "intra_stripe_flows",
            TreasuryFinancialAccountActiveFeaturesItem::OutboundPaymentsAch => "outbound_payments.ach",
            TreasuryFinancialAccountActiveFeaturesItem::OutboundPaymentsUsDomesticWire => {
                "outbound_payments.us_domestic_wire"
            }
            TreasuryFinancialAccountActiveFeaturesItem::OutboundTransfersAch => "outbound_transfers.ach",
            TreasuryFinancialAccountActiveFeaturesItem::OutboundTransfersUsDomesticWire => {
                "outbound_transfers.us_domestic_wire"
            }
            TreasuryFinancialAccountActiveFeaturesItem::RemoteDepositCapture => "remote_deposit_capture",
            TreasuryFinancialAccountActiveFeaturesItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryFinancialAccountActiveFeaturesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryFinancialAccountActiveFeaturesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "card_issuing" => TreasuryFinancialAccountActiveFeaturesItem::CardIssuing,
            "deposit_insurance" => TreasuryFinancialAccountActiveFeaturesItem::DepositInsurance,
            "financial_addresses.aba" => TreasuryFinancialAccountActiveFeaturesItem::FinancialAddressesAba,
            "financial_addresses.aba.forwarding" => {
                TreasuryFinancialAccountActiveFeaturesItem::FinancialAddressesAbaForwarding
            }
            "inbound_transfers.ach" => TreasuryFinancialAccountActiveFeaturesItem::InboundTransfersAch,
            "intra_stripe_flows" => TreasuryFinancialAccountActiveFeaturesItem::IntraStripeFlows,
            "outbound_payments.ach" => TreasuryFinancialAccountActiveFeaturesItem::OutboundPaymentsAch,
            "outbound_payments.us_domestic_wire" => {
                TreasuryFinancialAccountActiveFeaturesItem::OutboundPaymentsUsDomesticWire
            }
            "outbound_transfers.ach" => TreasuryFinancialAccountActiveFeaturesItem::OutboundTransfersAch,
            "outbound_transfers.us_domestic_wire" => {
                TreasuryFinancialAccountActiveFeaturesItem::OutboundTransfersUsDomesticWire
            }
            "remote_deposit_capture" => TreasuryFinancialAccountActiveFeaturesItem::RemoteDepositCapture,
            other => TreasuryFinancialAccountActiveFeaturesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccountsResourceBalance {
    #[doc = "Funds the user can spend right now."]
    #[json(default)]
    pub cash: std::collections::HashMap<String, i64>,
    #[doc = "Funds not spendable yet, but will become available at a later time."]
    #[json(default)]
    pub inbound_pending: std::collections::HashMap<String, i64>,
    #[doc = "Funds in the account, but not spendable because they are being held for pending outbound flows."]
    #[json(default)]
    pub outbound_pending: std::collections::HashMap<String, i64>,
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    Activating,
    CapabilityNotRequested,
    FinancialAccountClosed,
    RejectedOther,
    RejectedUnsupportedBusiness,
    RequirementsPastDue,
    RequirementsPendingVerification,
    RestrictedByPlatform,
    RestrictedOther,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::Activating => "activating",
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::CapabilityNotRequested => {
                "capability_not_requested"
            }
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::FinancialAccountClosed => {
                "financial_account_closed"
            }
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RejectedOther => "rejected_other",
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RejectedUnsupportedBusiness => {
                "rejected_unsupported_business"
            }
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RequirementsPastDue => {
                "requirements_past_due"
            }
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RequirementsPendingVerification => {
                "requirements_pending_verification"
            }
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RestrictedByPlatform => {
                "restricted_by_platform"
            }
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RestrictedOther => "restricted_other",
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl Default for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    fn default() -> Self {
        TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "activating" => TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::Activating,
            "capability_not_requested" => {
                TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::CapabilityNotRequested
            }
            "financial_account_closed" => {
                TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::FinancialAccountClosed
            }
            "rejected_other" => TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RejectedOther,
            "rejected_unsupported_business" => {
                TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RejectedUnsupportedBusiness
            }
            "requirements_past_due" => {
                TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RequirementsPastDue
            }
            "requirements_pending_verification" => {
                TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RequirementsPendingVerification
            }
            "restricted_by_platform" => {
                TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RestrictedByPlatform
            }
            "restricted_other" => TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RestrictedOther,
            other => TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryInboundTransfersResourceFailureDetailsCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    DebitNotAuthorized,
    IncorrectAccountHolderAddress,
    IncorrectAccountHolderName,
    IncorrectAccountHolderTaxId,
    InsufficientFunds,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    Other,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryInboundTransfersResourceFailureDetailsCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryInboundTransfersResourceFailureDetailsCode::AccountClosed => "account_closed",
            TreasuryInboundTransfersResourceFailureDetailsCode::AccountFrozen => "account_frozen",
            TreasuryInboundTransfersResourceFailureDetailsCode::BankAccountRestricted => "bank_account_restricted",
            TreasuryInboundTransfersResourceFailureDetailsCode::BankOwnershipChanged => "bank_ownership_changed",
            TreasuryInboundTransfersResourceFailureDetailsCode::DebitNotAuthorized => "debit_not_authorized",
            TreasuryInboundTransfersResourceFailureDetailsCode::IncorrectAccountHolderAddress => {
                "incorrect_account_holder_address"
            }
            TreasuryInboundTransfersResourceFailureDetailsCode::IncorrectAccountHolderName => {
                "incorrect_account_holder_name"
            }
            TreasuryInboundTransfersResourceFailureDetailsCode::IncorrectAccountHolderTaxId => {
                "incorrect_account_holder_tax_id"
            }
            TreasuryInboundTransfersResourceFailureDetailsCode::InsufficientFunds => "insufficient_funds",
            TreasuryInboundTransfersResourceFailureDetailsCode::InvalidAccountNumber => "invalid_account_number",
            TreasuryInboundTransfersResourceFailureDetailsCode::InvalidCurrency => "invalid_currency",
            TreasuryInboundTransfersResourceFailureDetailsCode::NoAccount => "no_account",
            TreasuryInboundTransfersResourceFailureDetailsCode::Other => "other",
            TreasuryInboundTransfersResourceFailureDetailsCode::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryInboundTransfersResourceFailureDetailsCode {
    fn default() -> Self {
        TreasuryInboundTransfersResourceFailureDetailsCode::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryInboundTransfersResourceFailureDetailsCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryInboundTransfersResourceFailureDetailsCode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_closed" => TreasuryInboundTransfersResourceFailureDetailsCode::AccountClosed,
            "account_frozen" => TreasuryInboundTransfersResourceFailureDetailsCode::AccountFrozen,
            "bank_account_restricted" => TreasuryInboundTransfersResourceFailureDetailsCode::BankAccountRestricted,
            "bank_ownership_changed" => TreasuryInboundTransfersResourceFailureDetailsCode::BankOwnershipChanged,
            "debit_not_authorized" => TreasuryInboundTransfersResourceFailureDetailsCode::DebitNotAuthorized,
            "incorrect_account_holder_address" => {
                TreasuryInboundTransfersResourceFailureDetailsCode::IncorrectAccountHolderAddress
            }
            "incorrect_account_holder_name" => {
                TreasuryInboundTransfersResourceFailureDetailsCode::IncorrectAccountHolderName
            }
            "incorrect_account_holder_tax_id" => {
                TreasuryInboundTransfersResourceFailureDetailsCode::IncorrectAccountHolderTaxId
            }
            "insufficient_funds" => TreasuryInboundTransfersResourceFailureDetailsCode::InsufficientFunds,
            "invalid_account_number" => TreasuryInboundTransfersResourceFailureDetailsCode::InvalidAccountNumber,
            "invalid_currency" => TreasuryInboundTransfersResourceFailureDetailsCode::InvalidCurrency,
            "no_account" => TreasuryInboundTransfersResourceFailureDetailsCode::NoAccount,
            "other" => TreasuryInboundTransfersResourceFailureDetailsCode::Other,
            other => TreasuryInboundTransfersResourceFailureDetailsCode::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundPaymentsResourceUsDomesticWireTrackingDetails {
    #[doc = "CHIPS System Sequence Number (SSN) of the OutboundPayment for payments sent over the `us_domestic_wire` network."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub chips: Option<String>,
    #[doc = "IMAD of the OutboundPayment for payments sent over the `us_domestic_wire` network."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub imad: Option<String>,
    #[doc = "OMAD of the OutboundPayment for payments sent over the `us_domestic_wire` network."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub omad: Option<String>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryReceivedDebitsResourceDebitReversalLinkedFlows {
    #[doc = "Set if there is an Issuing dispute associated with the DebitReversal."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuing_dispute: Option<String>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    Payments,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance::Payments => {
                "payments"
            }
            TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance::Unknown(
                value,
            ) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json>
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "payments" => {
                TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance::Payments
            }
            other => {
                TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance::Unknown(
                    other.to_string(),
                )
            }
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryTransactionEntryObject {
    TreasuryTransactionEntry,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryTransactionEntryObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryTransactionEntryObject::TreasuryTransactionEntry => "treasury.transaction_entry",
            TreasuryTransactionEntryObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryTransactionEntryObject {
    fn default() -> Self {
        TreasuryTransactionEntryObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryTransactionEntryObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryTransactionEntryObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "treasury.transaction_entry" => TreasuryTransactionEntryObject::TreasuryTransactionEntry,
            other => TreasuryTransactionEntryObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions {
    #[doc = "Timestamp describing when the Transaction changed status to `posted`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<i64>,
    #[doc = "Timestamp describing when the Transaction changed status to `void`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub void_at: Option<i64>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryTransactionsResourceBalanceImpact {
    #[doc = "The change made to funds the user can spend right now."]
    #[json(default)]
    pub cash: i64,
    #[doc = "The change made to funds that are not spendable yet, but will become available at a later time."]
    #[json(default)]
    pub inbound_pending: i64,
    #[doc = "The change made to funds in the account, but not spendable because they are being held for pending outbound flows."]
    #[json(default)]
    pub outbound_pending: i64,
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateTreasuryFinancialAccountParamsForwardingSettingsType {
    FinancialAccount,
    PaymentMethod,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl UpdateTreasuryFinancialAccountParamsForwardingSettingsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            UpdateTreasuryFinancialAccountParamsForwardingSettingsType::FinancialAccount => "financial_account",
            UpdateTreasuryFinancialAccountParamsForwardingSettingsType::PaymentMethod => "payment_method",
            UpdateTreasuryFinancialAccountParamsForwardingSettingsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json> for UpdateTreasuryFinancialAccountParamsForwardingSettingsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(UpdateTreasuryFinancialAccountParamsForwardingSettingsType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "financial_account" => UpdateTreasuryFinancialAccountParamsForwardingSettingsType::FinancialAccount,
            "payment_method" => UpdateTreasuryFinancialAccountParamsForwardingSettingsType::PaymentMethod,
            other => UpdateTreasuryFinancialAccountParamsForwardingSettingsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for UpdateTreasuryFinancialAccountParamsForwardingSettingsType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
