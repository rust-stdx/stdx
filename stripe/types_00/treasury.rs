#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryFinancialAccountParamsFeaturesCardIssuing {
    #[doc = "Whether the FinancialAccount should have the Feature."]
    pub requested: bool,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryFinancialAccountParamsFeaturesCardIssuing {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("requested", &self.requested);
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTreasuryFinancialAccountParamsPlatformRestrictionsInboundFlows {
    Restricted,
    Unrestricted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl CreateTreasuryFinancialAccountParamsPlatformRestrictionsInboundFlows {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTreasuryFinancialAccountParamsPlatformRestrictionsInboundFlows::Restricted => "restricted",
            CreateTreasuryFinancialAccountParamsPlatformRestrictionsInboundFlows::Unrestricted => "unrestricted",
            CreateTreasuryFinancialAccountParamsPlatformRestrictionsInboundFlows::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json> for CreateTreasuryFinancialAccountParamsPlatformRestrictionsInboundFlows {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTreasuryFinancialAccountParamsPlatformRestrictionsInboundFlows::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "restricted" => CreateTreasuryFinancialAccountParamsPlatformRestrictionsInboundFlows::Restricted,
            "unrestricted" => CreateTreasuryFinancialAccountParamsPlatformRestrictionsInboundFlows::Unrestricted,
            other => CreateTreasuryFinancialAccountParamsPlatformRestrictionsInboundFlows::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryFinancialAccountParamsPlatformRestrictionsInboundFlows {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType {
    Company,
    Individual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType::Company => "company",
            CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType::Individual => "individual",
            CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json>
    for CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "company" => CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType::Company,
            "individual" => CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType::Individual,
            other => CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode
    for CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryOutboundPaymentParamsEndUserDetails {
    #[doc = "IP address of the user initiating the OutboundPayment."]
    pub ip_address: Option<String>,
    #[doc = "`True` if the OutboundPayment creation request is being made on behalf of an end user by a platform."]
    pub present: bool,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryOutboundPaymentParamsEndUserDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ip_address", self.ip_address.as_ref());
        form.field("present", &self.present);
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    Checking,
    Savings,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InboundTransfersPaymentMethodDetailsUsBankAccountAccountType::Checking => "checking",
            InboundTransfersPaymentMethodDetailsUsBankAccountAccountType::Savings => "savings",
            InboundTransfersPaymentMethodDetailsUsBankAccountAccountType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InboundTransfersPaymentMethodDetailsUsBankAccountAccountType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "checking" => InboundTransfersPaymentMethodDetailsUsBankAccountAccountType::Checking,
            "savings" => InboundTransfersPaymentMethodDetailsUsBankAccountAccountType::Savings,
            other => InboundTransfersPaymentMethodDetailsUsBankAccountAccountType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListTreasuryCreditReversalsParamsStatus {
    Canceled,
    Posted,
    Processing,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl ListTreasuryCreditReversalsParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListTreasuryCreditReversalsParamsStatus::Canceled => "canceled",
            ListTreasuryCreditReversalsParamsStatus::Posted => "posted",
            ListTreasuryCreditReversalsParamsStatus::Processing => "processing",
            ListTreasuryCreditReversalsParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json> for ListTreasuryCreditReversalsParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListTreasuryCreditReversalsParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "canceled" => ListTreasuryCreditReversalsParamsStatus::Canceled,
            "posted" => ListTreasuryCreditReversalsParamsStatus::Posted,
            "processing" => ListTreasuryCreditReversalsParamsStatus::Processing,
            other => ListTreasuryCreditReversalsParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryCreditReversalsParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListTreasuryTransactionsParamsOrderBy {
    Created,
    PostedAt,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl ListTreasuryTransactionsParamsOrderBy {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListTreasuryTransactionsParamsOrderBy::Created => "created",
            ListTreasuryTransactionsParamsOrderBy::PostedAt => "posted_at",
            ListTreasuryTransactionsParamsOrderBy::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json> for ListTreasuryTransactionsParamsOrderBy {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListTreasuryTransactionsParamsOrderBy::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "created" => ListTreasuryTransactionsParamsOrderBy::Created,
            "posted_at" => ListTreasuryTransactionsParamsOrderBy::PostedAt,
            other => ListTreasuryTransactionsParamsOrderBy::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryTransactionsParamsOrderBy {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum ReserveHoldReason {
    Charge,
    Standalone,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl ReserveHoldReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ReserveHoldReason::Charge => "charge",
            ReserveHoldReason::Standalone => "standalone",
            ReserveHoldReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for ReserveHoldReason {
    fn default() -> Self {
        ReserveHoldReason::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for ReserveHoldReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ReserveHoldReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "charge" => ReserveHoldReason::Charge,
            "standalone" => ReserveHoldReason::Standalone,
            other => ReserveHoldReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum ReserveHoldSourceType {
    BankAccount,
    Card,
    Fpx,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl ReserveHoldSourceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ReserveHoldSourceType::BankAccount => "bank_account",
            ReserveHoldSourceType::Card => "card",
            ReserveHoldSourceType::Fpx => "fpx",
            ReserveHoldSourceType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for ReserveHoldSourceType {
    fn default() -> Self {
        ReserveHoldSourceType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for ReserveHoldSourceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ReserveHoldSourceType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "bank_account" => ReserveHoldSourceType::BankAccount,
            "card" => ReserveHoldSourceType::Card,
            "fpx" => ReserveHoldSourceType::Fpx,
            other => ReserveHoldSourceType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum ReserveReleaseDestination {
    Other,
    Payments,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl ReserveReleaseDestination {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ReserveReleaseDestination::Other => "other",
            ReserveReleaseDestination::Payments => "payments",
            ReserveReleaseDestination::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for ReserveReleaseDestination {
    fn default() -> Self {
        ReserveReleaseDestination::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for ReserveReleaseDestination {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ReserveReleaseDestination::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "other" => ReserveReleaseDestination::Other,
            "payments" => ReserveReleaseDestination::Payments,
            other => ReserveReleaseDestination::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReservesReserveHoldsResourcesReleaseDetail {
    #[doc = "The amount released by the ReserveRelease from this ReserveHold."]
    #[json(default)]
    pub amount: i64,
    #[doc = "The ReserveRelease which released funds from this ReserveHold (e.g., resrel_123)."]
    #[json(default)]
    pub reserve_release: String,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReservesReserveHoldsResourcesReleaseSchedule {
    #[doc = "The time after which the ReserveHold is requested to be released."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub release_after: Option<i64>,
    #[doc = "The time at which the ReserveHold is scheduled to be released, automatically set to midnight UTC of the day after `re..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub scheduled_release: Option<i64>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum ReservesReserveReleasesResourcesSourceTransactionType {
    Dispute,
    Refund,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl ReservesReserveReleasesResourcesSourceTransactionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ReservesReserveReleasesResourcesSourceTransactionType::Dispute => "dispute",
            ReservesReserveReleasesResourcesSourceTransactionType::Refund => "refund",
            ReservesReserveReleasesResourcesSourceTransactionType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for ReservesReserveReleasesResourcesSourceTransactionType {
    fn default() -> Self {
        ReservesReserveReleasesResourcesSourceTransactionType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for ReservesReserveReleasesResourcesSourceTransactionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ReservesReserveReleasesResourcesSourceTransactionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "dispute" => ReservesReserveReleasesResourcesSourceTransactionType::Dispute,
            "refund" => ReservesReserveReleasesResourcesSourceTransactionType::Refund,
            other => ReservesReserveReleasesResourcesSourceTransactionType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryDebitReversalStatus {
    Failed,
    Processing,
    Succeeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryDebitReversalStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryDebitReversalStatus::Failed => "failed",
            TreasuryDebitReversalStatus::Processing => "processing",
            TreasuryDebitReversalStatus::Succeeded => "succeeded",
            TreasuryDebitReversalStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryDebitReversalStatus {
    fn default() -> Self {
        TreasuryDebitReversalStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryDebitReversalStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryDebitReversalStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "failed" => TreasuryDebitReversalStatus::Failed,
            "processing" => TreasuryDebitReversalStatus::Processing,
            "succeeded" => TreasuryDebitReversalStatus::Succeeded,
            other => TreasuryDebitReversalStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryFinancialAccountFeaturesObject {
    TreasuryFinancialAccountFeatures,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl TreasuryFinancialAccountFeaturesObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryFinancialAccountFeaturesObject::TreasuryFinancialAccountFeatures => {
                "treasury.financial_account_features"
            }
            TreasuryFinancialAccountFeaturesObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl Default for TreasuryFinancialAccountFeaturesObject {
    fn default() -> Self {
        TreasuryFinancialAccountFeaturesObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryFinancialAccountFeaturesObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryFinancialAccountFeaturesObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "treasury.financial_account_features" => {
                TreasuryFinancialAccountFeaturesObject::TreasuryFinancialAccountFeatures
            }
            other => TreasuryFinancialAccountFeaturesObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::Active => "active",
            TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::Pending => "pending",
            TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::Restricted => "restricted",
            TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl Default for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    fn default() -> Self {
        TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::Active,
            "pending" => TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::Pending,
            "restricted" => TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::Restricted,
            other => TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworksItem {
    Ach,
    Rtp,
    UsDomesticWire,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworksItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworksItem::Ach => "ach",
            TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworksItem::Rtp => "rtp",
            TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworksItem::UsDomesticWire => {
                "us_domestic_wire"
            }
            TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworksItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworksItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworksItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach" => TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworksItem::Ach,
            "rtp" => TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworksItem::Rtp,
            "us_domestic_wire" => {
                TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworksItem::UsDomesticWire
            }
            other => TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworksItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    ContactStripe,
    ProvideInformation,
    RemoveRestriction,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution::ContactStripe => "contact_stripe",
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution::ProvideInformation => {
                "provide_information"
            }
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution::RemoveRestriction => {
                "remove_restriction"
            }
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "contact_stripe" => TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution::ContactStripe,
            "provide_information" => {
                TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution::ProvideInformation
            }
            "remove_restriction" => {
                TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution::RemoveRestriction
            }
            other => TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryInboundTransferObject {
    TreasuryInboundTransfer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryInboundTransferObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryInboundTransferObject::TreasuryInboundTransfer => "treasury.inbound_transfer",
            TreasuryInboundTransferObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryInboundTransferObject {
    fn default() -> Self {
        TreasuryInboundTransferObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryInboundTransferObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryInboundTransferObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "treasury.inbound_transfer" => TreasuryInboundTransferObject::TreasuryInboundTransfer,
            other => TreasuryInboundTransferObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryReceivedCreditStatus {
    Failed,
    Succeeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryReceivedCreditStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryReceivedCreditStatus::Failed => "failed",
            TreasuryReceivedCreditStatus::Succeeded => "succeeded",
            TreasuryReceivedCreditStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryReceivedCreditStatus {
    fn default() -> Self {
        TreasuryReceivedCreditStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryReceivedCreditStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryReceivedCreditStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "failed" => TreasuryReceivedCreditStatus::Failed,
            "succeeded" => TreasuryReceivedCreditStatus::Succeeded,
            other => TreasuryReceivedCreditStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    AlreadyReversed,
    DeadlinePassed,
    NetworkRestricted,
    Other,
    SourceFlowRestricted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::AlreadyReversed => "already_reversed",
            TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::DeadlinePassed => "deadline_passed",
            TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::NetworkRestricted => "network_restricted",
            TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::Other => "other",
            TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::SourceFlowRestricted => {
                "source_flow_restricted"
            }
            TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "already_reversed" => TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::AlreadyReversed,
            "deadline_passed" => TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::DeadlinePassed,
            "network_restricted" => TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::NetworkRestricted,
            "other" => TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::Other,
            "source_flow_restricted" => {
                TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::SourceFlowRestricted
            }
            other => TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    CreditReversal,
    Other,
    OutboundPayment,
    OutboundTransfer,
    Payout,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryReceivedCreditsResourceSourceFlowsDetailsType::CreditReversal => "credit_reversal",
            TreasuryReceivedCreditsResourceSourceFlowsDetailsType::Other => "other",
            TreasuryReceivedCreditsResourceSourceFlowsDetailsType::OutboundPayment => "outbound_payment",
            TreasuryReceivedCreditsResourceSourceFlowsDetailsType::OutboundTransfer => "outbound_transfer",
            TreasuryReceivedCreditsResourceSourceFlowsDetailsType::Payout => "payout",
            TreasuryReceivedCreditsResourceSourceFlowsDetailsType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn default() -> Self {
        TreasuryReceivedCreditsResourceSourceFlowsDetailsType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryReceivedCreditsResourceSourceFlowsDetailsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "credit_reversal" => TreasuryReceivedCreditsResourceSourceFlowsDetailsType::CreditReversal,
            "other" => TreasuryReceivedCreditsResourceSourceFlowsDetailsType::Other,
            "outbound_payment" => TreasuryReceivedCreditsResourceSourceFlowsDetailsType::OutboundPayment,
            "outbound_transfer" => TreasuryReceivedCreditsResourceSourceFlowsDetailsType::OutboundTransfer,
            "payout" => TreasuryReceivedCreditsResourceSourceFlowsDetailsType::Payout,
            other => TreasuryReceivedCreditsResourceSourceFlowsDetailsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryTransactionEntryType {
    CreditReversal,
    CreditReversalPosting,
    DebitReversal,
    InboundTransfer,
    InboundTransferReturn,
    IssuingAuthorizationHold,
    IssuingAuthorizationRelease,
    Other,
    OutboundPayment,
    OutboundPaymentCancellation,
    OutboundPaymentFailure,
    OutboundPaymentPosting,
    OutboundPaymentReturn,
    OutboundTransfer,
    OutboundTransferCancellation,
    OutboundTransferFailure,
    OutboundTransferPosting,
    OutboundTransferReturn,
    ReceivedCredit,
    ReceivedDebit,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryTransactionEntryType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryTransactionEntryType::CreditReversal => "credit_reversal",
            TreasuryTransactionEntryType::CreditReversalPosting => "credit_reversal_posting",
            TreasuryTransactionEntryType::DebitReversal => "debit_reversal",
            TreasuryTransactionEntryType::InboundTransfer => "inbound_transfer",
            TreasuryTransactionEntryType::InboundTransferReturn => "inbound_transfer_return",
            TreasuryTransactionEntryType::IssuingAuthorizationHold => "issuing_authorization_hold",
            TreasuryTransactionEntryType::IssuingAuthorizationRelease => "issuing_authorization_release",
            TreasuryTransactionEntryType::Other => "other",
            TreasuryTransactionEntryType::OutboundPayment => "outbound_payment",
            TreasuryTransactionEntryType::OutboundPaymentCancellation => "outbound_payment_cancellation",
            TreasuryTransactionEntryType::OutboundPaymentFailure => "outbound_payment_failure",
            TreasuryTransactionEntryType::OutboundPaymentPosting => "outbound_payment_posting",
            TreasuryTransactionEntryType::OutboundPaymentReturn => "outbound_payment_return",
            TreasuryTransactionEntryType::OutboundTransfer => "outbound_transfer",
            TreasuryTransactionEntryType::OutboundTransferCancellation => "outbound_transfer_cancellation",
            TreasuryTransactionEntryType::OutboundTransferFailure => "outbound_transfer_failure",
            TreasuryTransactionEntryType::OutboundTransferPosting => "outbound_transfer_posting",
            TreasuryTransactionEntryType::OutboundTransferReturn => "outbound_transfer_return",
            TreasuryTransactionEntryType::ReceivedCredit => "received_credit",
            TreasuryTransactionEntryType::ReceivedDebit => "received_debit",
            TreasuryTransactionEntryType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryTransactionEntryType {
    fn default() -> Self {
        TreasuryTransactionEntryType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryTransactionEntryType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryTransactionEntryType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "credit_reversal" => TreasuryTransactionEntryType::CreditReversal,
            "credit_reversal_posting" => TreasuryTransactionEntryType::CreditReversalPosting,
            "debit_reversal" => TreasuryTransactionEntryType::DebitReversal,
            "inbound_transfer" => TreasuryTransactionEntryType::InboundTransfer,
            "inbound_transfer_return" => TreasuryTransactionEntryType::InboundTransferReturn,
            "issuing_authorization_hold" => TreasuryTransactionEntryType::IssuingAuthorizationHold,
            "issuing_authorization_release" => TreasuryTransactionEntryType::IssuingAuthorizationRelease,
            "other" => TreasuryTransactionEntryType::Other,
            "outbound_payment" => TreasuryTransactionEntryType::OutboundPayment,
            "outbound_payment_cancellation" => TreasuryTransactionEntryType::OutboundPaymentCancellation,
            "outbound_payment_failure" => TreasuryTransactionEntryType::OutboundPaymentFailure,
            "outbound_payment_posting" => TreasuryTransactionEntryType::OutboundPaymentPosting,
            "outbound_payment_return" => TreasuryTransactionEntryType::OutboundPaymentReturn,
            "outbound_transfer" => TreasuryTransactionEntryType::OutboundTransfer,
            "outbound_transfer_cancellation" => TreasuryTransactionEntryType::OutboundTransferCancellation,
            "outbound_transfer_failure" => TreasuryTransactionEntryType::OutboundTransferFailure,
            "outbound_transfer_posting" => TreasuryTransactionEntryType::OutboundTransferPosting,
            "outbound_transfer_return" => TreasuryTransactionEntryType::OutboundTransferReturn,
            "received_credit" => TreasuryTransactionEntryType::ReceivedCredit,
            "received_debit" => TreasuryTransactionEntryType::ReceivedDebit,
            other => TreasuryTransactionEntryType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryTransactionObject {
    TreasuryTransaction,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryTransactionObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryTransactionObject::TreasuryTransaction => "treasury.transaction",
            TreasuryTransactionObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryTransactionObject {
    fn default() -> Self {
        TreasuryTransactionObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryTransactionObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryTransactionObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "treasury.transaction" => TreasuryTransactionObject::TreasuryTransaction,
            other => TreasuryTransactionObject::Unknown(other.to_string()),
        })
    }
}
