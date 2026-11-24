#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsReceiptItem {
    pub description: Option<String>,
    pub quantity: Option<String>,
    pub total: Option<i64>,
    pub unit_cost: Option<i64>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsReceiptItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("description", self.description.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
        form.field_opt("total", self.total.as_ref());
        form.field_opt("unit_cost", self.unit_cost.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataAcssDebit {
    #[doc = "Customer's bank account number."]
    pub account_number: String,
    #[doc = "Institution number of the customer's bank."]
    pub institution_number: String,
    #[doc = "Transit number of the customer's bank."]
    pub transit_number: String,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataAcssDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("account_number", &self.account_number);
        form.field("institution_number", &self.institution_number);
        form.field("transit_number", &self.transit_number);
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataAuBecsDebit {
    #[doc = "The account number for the bank account."]
    pub account_number: String,
    #[doc = "Bank-State-Branch number of the bank account."]
    pub bsb_number: String,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataAuBecsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("account_number", &self.account_number);
        form.field("bsb_number", &self.bsb_number);
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataKlarnaDob {
    #[doc = "The day of birth, between 1 and 31."]
    pub day: i64,
    #[doc = "The month of birth, between 1 and 12."]
    pub month: i64,
    #[doc = "The four-digit year of birth."]
    pub year: i64,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataKlarnaDob {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("day", &self.day);
        form.field("month", &self.month);
        form.field("year", &self.year);
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataRadarOptions {
    #[doc = "A [Radar Session](https://docs.stripe.com/radar/radar-session) is a snapshot of the browser metadata and device detai..."]
    pub session: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataRadarOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("session", self.session.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperConfirmationTokenParamsSetupFutureUsage {
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperConfirmationTokenParamsSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperConfirmationTokenParamsSetupFutureUsage::OffSession => "off_session",
            CreateTestHelperConfirmationTokenParamsSetupFutureUsage::OnSession => "on_session",
            CreateTestHelperConfirmationTokenParamsSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperConfirmationTokenParamsSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperConfirmationTokenParamsSetupFutureUsage::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "off_session" => CreateTestHelperConfirmationTokenParamsSetupFutureUsage::OffSession,
            "on_session" => CreateTestHelperConfirmationTokenParamsSetupFutureUsage::OnSession,
            other => CreateTestHelperConfirmationTokenParamsSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsSetupFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperIssuingAuthorizationParamsFleetCardholderPromptData {
    #[doc = "Driver ID."]
    pub driver_id: Option<String>,
    #[doc = "Odometer reading."]
    pub odometer: Option<i64>,
    #[doc = "An alphanumeric ID."]
    pub unspecified_id: Option<String>,
    #[doc = "User ID."]
    pub user_id: Option<String>,
    #[doc = "Vehicle number."]
    pub vehicle_number: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsFleetCardholderPromptData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("driver_id", self.driver_id.as_ref());
        form.field_opt("odometer", self.odometer.as_ref());
        form.field_opt("unspecified_id", self.unspecified_id.as_ref());
        form.field_opt("user_id", self.user_id.as_ref());
        form.field_opt("vehicle_number", self.vehicle_number.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressLine1Check {
    Match,
    Mismatch,
    NotProvided,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressLine1Check {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressLine1Check::Match => "match",
            CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressLine1Check::Mismatch => "mismatch",
            CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressLine1Check::NotProvided => "not_provided",
            CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressLine1Check::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressLine1Check {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressLine1Check::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "match" => CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressLine1Check::Match,
            "mismatch" => CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressLine1Check::Mismatch,
            "not_provided" => CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressLine1Check::NotProvided,
            other => {
                CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressLine1Check::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressLine1Check {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionType {
    LowValueTransaction,
    TransactionRiskAnalysis,
    UnknownValue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionType::LowValueTransaction => "low_value_transaction",
            CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionType::TransactionRiskAnalysis => "transaction_risk_analysis",
            CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionType::UnknownValue => "unknown",
            CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json>
    for CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "low_value_transaction" => CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionType::LowValueTransaction,
            "transaction_risk_analysis" => CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionType::TransactionRiskAnalysis,
            "unknown" => CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionType::UnknownValue,
            other => CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode
    for CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionType
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperIssuingAuthorizationParamsWallet {
    ApplePay,
    GooglePay,
    SamsungPay,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperIssuingAuthorizationParamsWallet {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperIssuingAuthorizationParamsWallet::ApplePay => "apple_pay",
            CreateTestHelperIssuingAuthorizationParamsWallet::GooglePay => "google_pay",
            CreateTestHelperIssuingAuthorizationParamsWallet::SamsungPay => "samsung_pay",
            CreateTestHelperIssuingAuthorizationParamsWallet::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperIssuingAuthorizationParamsWallet {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperIssuingAuthorizationParamsWallet::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "apple_pay" => CreateTestHelperIssuingAuthorizationParamsWallet::ApplePay,
            "google_pay" => CreateTestHelperIssuingAuthorizationParamsWallet::GooglePay,
            "samsung_pay" => CreateTestHelperIssuingAuthorizationParamsWallet::SamsungPay,
            other => CreateTestHelperIssuingAuthorizationParamsWallet::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsWallet {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct ListTestHelperTestClocksParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for ListTestHelperTestClocksParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum PresentPaymentMethodParamsType {
    Card,
    CardPresent,
    InteracPresent,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl PresentPaymentMethodParamsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PresentPaymentMethodParamsType::Card => "card",
            PresentPaymentMethodParamsType::CardPresent => "card_present",
            PresentPaymentMethodParamsType::InteracPresent => "interac_present",
            PresentPaymentMethodParamsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for PresentPaymentMethodParamsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PresentPaymentMethodParamsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "card" => PresentPaymentMethodParamsType::Card,
            "card_present" => PresentPaymentMethodParamsType::CardPresent,
            "interac_present" => PresentPaymentMethodParamsType::InteracPresent,
            other => PresentPaymentMethodParamsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for PresentPaymentMethodParamsType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem {
    GeographicLocation,
    Inappropriate,
    NetworkName,
    NonBinaryImage,
    NonFiatCurrency,
    Other,
    OtherEntity,
    PromotionalMaterial,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::GeographicLocation => {
                "geographic_location"
            }
            RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::Inappropriate => {
                "inappropriate"
            }
            RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::NetworkName => {
                "network_name"
            }
            RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::NonBinaryImage => {
                "non_binary_image"
            }
            RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::NonFiatCurrency => {
                "non_fiat_currency"
            }
            RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::Other => "other",
            RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::OtherEntity => {
                "other_entity"
            }
            RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::PromotionalMaterial => {
                "promotional_material"
            }
            RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "geographic_location" => {
                RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::GeographicLocation
            }
            "inappropriate" => {
                RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::Inappropriate
            }
            "network_name" => {
                RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::NetworkName
            }
            "non_binary_image" => {
                RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::NonBinaryImage
            }
            "non_fiat_currency" => {
                RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::NonFiatCurrency
            }
            "other" => RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::Other,
            "other_entity" => {
                RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::OtherEntity
            }
            "promotional_material" => {
                RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::PromotionalMaterial
            }
            other => RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode {
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

#[cfg(feature = "test_helpers")]
impl ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::AccountClosed => "account_closed",
            ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::AccountFrozen => "account_frozen",
            ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::BankAccountRestricted => {
                "bank_account_restricted"
            }
            ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::BankOwnershipChanged => {
                "bank_ownership_changed"
            }
            ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::Declined => "declined",
            ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::IncorrectAccountHolderName => {
                "incorrect_account_holder_name"
            }
            ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::InvalidAccountNumber => {
                "invalid_account_number"
            }
            ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::InvalidCurrency => "invalid_currency",
            ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::NoAccount => "no_account",
            ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::Other => "other",
            ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_closed" => ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::AccountClosed,
            "account_frozen" => ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::AccountFrozen,
            "bank_account_restricted" => {
                ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::BankAccountRestricted
            }
            "bank_ownership_changed" => {
                ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::BankOwnershipChanged
            }
            "declined" => ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::Declined,
            "incorrect_account_holder_name" => {
                ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::IncorrectAccountHolderName
            }
            "invalid_account_number" => {
                ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::InvalidAccountNumber
            }
            "invalid_currency" => ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::InvalidCurrency,
            "no_account" => ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::NoAccount,
            "other" => ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::Other,
            other => ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
