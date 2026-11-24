#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutAcssDebitMandateOptionsDefaultForItem {
    Invoice,
    Subscription,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutAcssDebitMandateOptionsDefaultForItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutAcssDebitMandateOptionsDefaultForItem::Invoice => "invoice",
            CheckoutAcssDebitMandateOptionsDefaultForItem::Subscription => "subscription",
            CheckoutAcssDebitMandateOptionsDefaultForItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutAcssDebitMandateOptionsDefaultForItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutAcssDebitMandateOptionsDefaultForItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "invoice" => CheckoutAcssDebitMandateOptionsDefaultForItem::Invoice,
            "subscription" => CheckoutAcssDebitMandateOptionsDefaultForItem::Subscription,
            other => CheckoutAcssDebitMandateOptionsDefaultForItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutAcssDebitMandateOptionsPaymentSchedule {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutAcssDebitMandateOptionsPaymentSchedule::Combined => "combined",
            CheckoutAcssDebitMandateOptionsPaymentSchedule::Interval => "interval",
            CheckoutAcssDebitMandateOptionsPaymentSchedule::Sporadic => "sporadic",
            CheckoutAcssDebitMandateOptionsPaymentSchedule::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutAcssDebitMandateOptionsPaymentSchedule::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "combined" => CheckoutAcssDebitMandateOptionsPaymentSchedule::Combined,
            "interval" => CheckoutAcssDebitMandateOptionsPaymentSchedule::Interval,
            "sporadic" => CheckoutAcssDebitMandateOptionsPaymentSchedule::Sporadic,
            other => CheckoutAcssDebitMandateOptionsPaymentSchedule::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutFinancialConnectionsPaymentMethodOptionsFiltersAccountSubcategoriesItem {
    Checking,
    Savings,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutFinancialConnectionsPaymentMethodOptionsFiltersAccountSubcategoriesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutFinancialConnectionsPaymentMethodOptionsFiltersAccountSubcategoriesItem::Checking => "checking",
            CheckoutFinancialConnectionsPaymentMethodOptionsFiltersAccountSubcategoriesItem::Savings => "savings",
            CheckoutFinancialConnectionsPaymentMethodOptionsFiltersAccountSubcategoriesItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutFinancialConnectionsPaymentMethodOptionsFiltersAccountSubcategoriesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CheckoutFinancialConnectionsPaymentMethodOptionsFiltersAccountSubcategoriesItem::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "checking" => CheckoutFinancialConnectionsPaymentMethodOptionsFiltersAccountSubcategoriesItem::Checking,
            "savings" => CheckoutFinancialConnectionsPaymentMethodOptionsFiltersAccountSubcategoriesItem::Savings,
            other => CheckoutFinancialConnectionsPaymentMethodOptionsFiltersAccountSubcategoriesItem::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutPaymentMethodOptionsMandateOptionsBacsDebit {
    #[doc = "Prefix used to generate the Mandate reference."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference_prefix: Option<String>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutSessionPaymentStatus {
    NoPaymentRequired,
    Paid,
    Unpaid,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutSessionPaymentStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutSessionPaymentStatus::NoPaymentRequired => "no_payment_required",
            CheckoutSessionPaymentStatus::Paid => "paid",
            CheckoutSessionPaymentStatus::Unpaid => "unpaid",
            CheckoutSessionPaymentStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl Default for CheckoutSessionPaymentStatus {
    fn default() -> Self {
        CheckoutSessionPaymentStatus::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutSessionPaymentStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutSessionPaymentStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "no_payment_required" => CheckoutSessionPaymentStatus::NoPaymentRequired,
            "paid" => CheckoutSessionPaymentStatus::Paid,
            "unpaid" => CheckoutSessionPaymentStatus::Unpaid,
            other => CheckoutSessionPaymentStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsAdaptivePricing {
    #[doc = "If set to `true`, Adaptive Pricing is available on eligible sessions..."]
    pub enabled: Option<bool>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsAdaptivePricing {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("enabled", self.enabled.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsAutomaticTaxLiabilityType {
    Account,
    SelfValue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsAutomaticTaxLiabilityType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsAutomaticTaxLiabilityType::Account => "account",
            CreateCheckoutSessionParamsAutomaticTaxLiabilityType::SelfValue => "self",
            CreateCheckoutSessionParamsAutomaticTaxLiabilityType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsAutomaticTaxLiabilityType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsAutomaticTaxLiabilityType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account" => CreateCheckoutSessionParamsAutomaticTaxLiabilityType::Account,
            "self" => CreateCheckoutSessionParamsAutomaticTaxLiabilityType::SelfValue,
            other => CreateCheckoutSessionParamsAutomaticTaxLiabilityType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsAutomaticTaxLiabilityType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsBillingAddressCollection {
    Auto,
    Required,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsBillingAddressCollection {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsBillingAddressCollection::Auto => "auto",
            CreateCheckoutSessionParamsBillingAddressCollection::Required => "required",
            CreateCheckoutSessionParamsBillingAddressCollection::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsBillingAddressCollection {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsBillingAddressCollection::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => CreateCheckoutSessionParamsBillingAddressCollection::Auto,
            "required" => CreateCheckoutSessionParamsBillingAddressCollection::Required,
            other => CreateCheckoutSessionParamsBillingAddressCollection::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsBillingAddressCollection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsConsentCollectionPromotions {
    Auto,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsConsentCollectionPromotions {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsConsentCollectionPromotions::Auto => "auto",
            CreateCheckoutSessionParamsConsentCollectionPromotions::None => "none",
            CreateCheckoutSessionParamsConsentCollectionPromotions::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsConsentCollectionPromotions {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsConsentCollectionPromotions::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => CreateCheckoutSessionParamsConsentCollectionPromotions::Auto,
            "none" => CreateCheckoutSessionParamsConsentCollectionPromotions::None,
            other => CreateCheckoutSessionParamsConsentCollectionPromotions::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsConsentCollectionPromotions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsCustomFieldsItemNumeric {
    #[doc = "The value that pre-fills the field on the payment page."]
    pub default_value: Option<String>,
    #[doc = "The maximum character length constraint for the customer's input."]
    pub maximum_length: Option<i64>,
    #[doc = "The minimum character length requirement for the customer's input."]
    pub minimum_length: Option<i64>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsCustomFieldsItemNumeric {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("default_value", self.default_value.as_ref());
        form.field_opt("maximum_length", self.maximum_length.as_ref());
        form.field_opt("minimum_length", self.minimum_length.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitVerificationMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => "automatic",
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => {
                "microdeposits"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitVerificationMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitVerificationMethod::Automatic,
            "instant" => CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitVerificationMethod::Instant,
            "microdeposits" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits
            }
            other => {
                CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitVerificationMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsBlikMandateOptions {
    #[doc = "Date when the mandate expires and no further payments will be charged."]
    pub expires_at: Option<i64>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsBlikMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("expires_at", self.expires_at.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsBlikSetupFutureUsage {
    Unnamed,
    None,
    OffSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsBlikSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsBlikSetupFutureUsage::Unnamed => "",
            CreateCheckoutSessionParamsPaymentMethodOptionsBlikSetupFutureUsage::None => "none",
            CreateCheckoutSessionParamsPaymentMethodOptionsBlikSetupFutureUsage::OffSession => "off_session",
            CreateCheckoutSessionParamsPaymentMethodOptionsBlikSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsBlikSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsPaymentMethodOptionsBlikSetupFutureUsage::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateCheckoutSessionParamsPaymentMethodOptionsBlikSetupFutureUsage::Unnamed,
            "none" => CreateCheckoutSessionParamsPaymentMethodOptionsBlikSetupFutureUsage::None,
            "off_session" => CreateCheckoutSessionParamsPaymentMethodOptionsBlikSetupFutureUsage::OffSession,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsBlikSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsBlikSetupFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestExtendedAuthorization {
    IfAvailable,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestExtendedAuthorization {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestExtendedAuthorization::IfAvailable => {
                "if_available"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestExtendedAuthorization::Never => "never",
            CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestExtendedAuthorization::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestExtendedAuthorization {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestExtendedAuthorization::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "if_available" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestExtendedAuthorization::IfAvailable
            }
            "never" => CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestExtendedAuthorization::Never,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestExtendedAuthorization::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestExtendedAuthorization {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem {
    AmericanExpress,
    DiscoverGlobalNetwork,
    Mastercard,
    Visa,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::AmericanExpress => {
                "american_express"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::DiscoverGlobalNetwork => {
                "discover_global_network"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::Mastercard => {
                "mastercard"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::Visa => "visa",
            CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "american_express" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::AmericanExpress
            }
            "discover_global_network" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::DiscoverGlobalNetwork
            }
            "mastercard" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::Mastercard
            }
            "visa" => CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::Visa,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale {
    CsCZ,
    DaDK,
    DeAT,
    DeDE,
    DeLU,
    ElGR,
    EnGB,
    EnUS,
    EsES,
    FiFI,
    FrBE,
    FrFR,
    FrLU,
    HuHU,
    ItIT,
    NlBE,
    NlNL,
    PlPL,
    PtPT,
    SkSK,
    SvSE,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::CsCZ => "cs-CZ",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::DaDK => "da-DK",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::DeAT => "de-AT",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::DeDE => "de-DE",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::DeLU => "de-LU",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::ElGR => "el-GR",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::EnGB => "en-GB",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::EnUS => "en-US",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::EsES => "es-ES",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::FiFI => "fi-FI",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::FrBE => "fr-BE",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::FrFR => "fr-FR",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::FrLU => "fr-LU",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::HuHU => "hu-HU",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::ItIT => "it-IT",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::NlBE => "nl-BE",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::NlNL => "nl-NL",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::PlPL => "pl-PL",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::PtPT => "pt-PT",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::SkSK => "sk-SK",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::SvSE => "sv-SE",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cs-CZ" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::CsCZ,
            "da-DK" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::DaDK,
            "de-AT" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::DeAT,
            "de-DE" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::DeDE,
            "de-LU" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::DeLU,
            "el-GR" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::ElGR,
            "en-GB" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::EnGB,
            "en-US" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::EnUS,
            "es-ES" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::EsES,
            "fi-FI" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::FiFI,
            "fr-BE" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::FrBE,
            "fr-FR" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::FrFR,
            "fr-LU" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::FrLU,
            "hu-HU" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::HuHU,
            "it-IT" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::ItIT,
            "nl-BE" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::NlBE,
            "nl-NL" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::NlNL,
            "pl-PL" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::PlPL,
            "pt-PT" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::PtPT,
            "sk-SK" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::SkSK,
            "sv-SE" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::SvSE,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsAmountType {
    Fixed,
    Maximum,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsAmountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsAmountType::Fixed => "fixed",
            CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsAmountType::Maximum => "maximum",
            CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsAmountType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsAmountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsAmountType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "fixed" => CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsAmountType::Fixed,
            "maximum" => CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsAmountType::Maximum,
            other => {
                CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsAmountType::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsAmountType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsSwish {
    #[doc = "The order reference that will be displayed to customers in the Swish application."]
    pub reference: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsSwish {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("reference", self.reference.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsUpiSetupFutureUsage {
    Unnamed,
    None,
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsUpiSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsUpiSetupFutureUsage::Unnamed => "",
            CreateCheckoutSessionParamsPaymentMethodOptionsUpiSetupFutureUsage::None => "none",
            CreateCheckoutSessionParamsPaymentMethodOptionsUpiSetupFutureUsage::OffSession => "off_session",
            CreateCheckoutSessionParamsPaymentMethodOptionsUpiSetupFutureUsage::OnSession => "on_session",
            CreateCheckoutSessionParamsPaymentMethodOptionsUpiSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsUpiSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsPaymentMethodOptionsUpiSetupFutureUsage::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateCheckoutSessionParamsPaymentMethodOptionsUpiSetupFutureUsage::Unnamed,
            "none" => CreateCheckoutSessionParamsPaymentMethodOptionsUpiSetupFutureUsage::None,
            "off_session" => CreateCheckoutSessionParamsPaymentMethodOptionsUpiSetupFutureUsage::OffSession,
            "on_session" => CreateCheckoutSessionParamsPaymentMethodOptionsUpiSetupFutureUsage::OnSession,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsUpiSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsUpiSetupFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountVerificationMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountVerificationMethod::Automatic => "automatic",
            CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountVerificationMethod::Instant => "instant",
            CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountVerificationMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountVerificationMethod::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountVerificationMethod::Automatic,
            "instant" => CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountVerificationMethod::Instant,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountVerificationMethod::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsSubscriptionDataBillingCycleAnchorConfig {
    #[doc = "The day of the month the anchor should be."]
    pub day_of_month: i64,
    #[doc = "The hour of the day the anchor should be."]
    pub hour: Option<i64>,
    #[doc = "The minute of the hour the anchor should be."]
    pub minute: Option<i64>,
    #[doc = "The month to start full cycle periods."]
    pub month: Option<i64>,
    #[doc = "The second of the minute the anchor should be."]
    pub second: Option<i64>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsSubscriptionDataBillingCycleAnchorConfig {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("day_of_month", &self.day_of_month);
        form.field_opt("hour", self.hour.as_ref());
        form.field_opt("minute", self.minute.as_ref());
        form.field_opt("month", self.month.as_ref());
        form.field_opt("second", self.second.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsSubscriptionDataBillingModeFlexibleProrationDiscounts {
    Included,
    Itemized,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsSubscriptionDataBillingModeFlexibleProrationDiscounts {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsSubscriptionDataBillingModeFlexibleProrationDiscounts::Included => "included",
            CreateCheckoutSessionParamsSubscriptionDataBillingModeFlexibleProrationDiscounts::Itemized => "itemized",
            CreateCheckoutSessionParamsSubscriptionDataBillingModeFlexibleProrationDiscounts::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsSubscriptionDataBillingModeFlexibleProrationDiscounts {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsSubscriptionDataBillingModeFlexibleProrationDiscounts::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "included" => CreateCheckoutSessionParamsSubscriptionDataBillingModeFlexibleProrationDiscounts::Included,
            "itemized" => CreateCheckoutSessionParamsSubscriptionDataBillingModeFlexibleProrationDiscounts::Itemized,
            other => CreateCheckoutSessionParamsSubscriptionDataBillingModeFlexibleProrationDiscounts::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsSubscriptionDataBillingModeFlexibleProrationDiscounts {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct ListCheckoutSessionsParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for ListCheckoutSessionsParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct ListCheckoutSessionsParamsCustomerDetails {
    #[doc = "Customer's email address."]
    pub email: String,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for ListCheckoutSessionsParamsCustomerDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("email", &self.email);
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionAfterExpirationRecovery {
    #[doc = "Enables user redeemable promotion codes on the recovered Checkout Sessions."]
    #[json(default)]
    pub allow_promotion_codes: bool,
    #[doc = "If `true`, a recovery url will be generated to recover this Checkout Session if it expires before a transaction is co..."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "The timestamp at which the recovery URL will expire."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[doc = "URL that creates a new Checkout Session when clicked that is a copy of this expired Checkout Session"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    Pill,
    Rectangular,
    Rounded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesCheckoutSessionBrandingSettingsBorderStyle::Pill => "pill",
            PaymentPagesCheckoutSessionBrandingSettingsBorderStyle::Rectangular => "rectangular",
            PaymentPagesCheckoutSessionBrandingSettingsBorderStyle::Rounded => "rounded",
            PaymentPagesCheckoutSessionBrandingSettingsBorderStyle::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl Default for PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    fn default() -> Self {
        PaymentPagesCheckoutSessionBrandingSettingsBorderStyle::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentPagesCheckoutSessionBrandingSettingsBorderStyle::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "pill" => PaymentPagesCheckoutSessionBrandingSettingsBorderStyle::Pill,
            "rectangular" => PaymentPagesCheckoutSessionBrandingSettingsBorderStyle::Rectangular,
            "rounded" => PaymentPagesCheckoutSessionBrandingSettingsBorderStyle::Rounded,
            other => PaymentPagesCheckoutSessionBrandingSettingsBorderStyle::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentPagesCheckoutSessionBrandingSettingsIconType {
    File,
    Url,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesCheckoutSessionBrandingSettingsIconType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesCheckoutSessionBrandingSettingsIconType::File => "file",
            PaymentPagesCheckoutSessionBrandingSettingsIconType::Url => "url",
            PaymentPagesCheckoutSessionBrandingSettingsIconType::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl Default for PaymentPagesCheckoutSessionBrandingSettingsIconType {
    fn default() -> Self {
        PaymentPagesCheckoutSessionBrandingSettingsIconType::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionBrandingSettingsIconType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentPagesCheckoutSessionBrandingSettingsIconType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "file" => PaymentPagesCheckoutSessionBrandingSettingsIconType::File,
            "url" => PaymentPagesCheckoutSessionBrandingSettingsIconType::Url,
            other => PaymentPagesCheckoutSessionBrandingSettingsIconType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionManagedPayments {
    #[doc = "Set to `true` to enable [Managed Payments](https://docs.stripe.com/payments/managed-payments), Stripe's merchant of r..."]
    #[json(default)]
    pub enabled: bool,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionOptionalItemAdjustableQuantity {
    #[doc = "Set to true if the quantity can be adjusted to any non-negative integer."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "The maximum quantity of this item the customer can purchase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[doc = "The minimum quantity of this item the customer must purchase, if they choose to purchase it."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsFundingTypesBlockedItem {
    Credit,
    Debit,
    Prepaid,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsFundingTypesBlockedItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsFundingTypesBlockedItem::Credit => "credit",
            PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsFundingTypesBlockedItem::Debit => "debit",
            PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsFundingTypesBlockedItem::Prepaid => {
                "prepaid"
            }
            PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsFundingTypesBlockedItem::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json>
    for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsFundingTypesBlockedItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsFundingTypesBlockedItem::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "credit" => PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsFundingTypesBlockedItem::Credit,
            "debit" => PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsFundingTypesBlockedItem::Debit,
            "prepaid" => {
                PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsFundingTypesBlockedItem::Prepaid
            }
            other => PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsFundingTypesBlockedItem::Unknown(
                other.to_string(),
            ),
        })
    }
}
