#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline {}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsMandateDataCustomerAcceptanceType {
    Offline,
    Online,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsMandateDataCustomerAcceptanceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsMandateDataCustomerAcceptanceType::Offline => "offline",
            CreateSetupIntentParamsMandateDataCustomerAcceptanceType::Online => "online",
            CreateSetupIntentParamsMandateDataCustomerAcceptanceType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsMandateDataCustomerAcceptanceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsMandateDataCustomerAcceptanceType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "offline" => CreateSetupIntentParamsMandateDataCustomerAcceptanceType::Offline,
            "online" => CreateSetupIntentParamsMandateDataCustomerAcceptanceType::Online,
            other => CreateSetupIntentParamsMandateDataCustomerAcceptanceType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsMandateDataCustomerAcceptanceType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodDataBillingDetailsAddress {
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

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataBillingDetailsAddress {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("line1", self.line1.as_ref());
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodDataBoleto {
    #[doc = "The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers)"]
    pub tax_id: String,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataBoleto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("tax_id", &self.tax_id);
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsAcssDebitCurrency {
    Cad,
    Usd,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsAcssDebitCurrency {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsAcssDebitCurrency::Cad => "cad",
            CreateSetupIntentParamsPaymentMethodOptionsAcssDebitCurrency::Usd => "usd",
            CreateSetupIntentParamsPaymentMethodOptionsAcssDebitCurrency::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsAcssDebitCurrency {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodOptionsAcssDebitCurrency::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cad" => CreateSetupIntentParamsPaymentMethodOptionsAcssDebitCurrency::Cad,
            "usd" => CreateSetupIntentParamsPaymentMethodOptionsAcssDebitCurrency::Usd,
            other => CreateSetupIntentParamsPaymentMethodOptionsAcssDebitCurrency::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsAcssDebitCurrency {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem {
    Invoice,
    Subscription,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem::Invoice => "invoice",
            CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem::Subscription => {
                "subscription"
            }
            CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "invoice" => CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem::Invoice,
            "subscription" => {
                CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem::Subscription
            }
            other => CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => "automatic",
            CreateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            CreateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => "microdeposits",
            CreateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => CreateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Automatic,
            "instant" => CreateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Instant,
            "microdeposits" => CreateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits,
            other => CreateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Girocard,
    Interac,
    Jcb,
    Link,
    Mastercard,
    Unionpay,
    UnknownValue,
    Visa,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsCardNetwork {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Amex => "amex",
            CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::CartesBancaires => "cartes_bancaires",
            CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Diners => "diners",
            CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Discover => "discover",
            CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::EftposAu => "eftpos_au",
            CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Girocard => "girocard",
            CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Interac => "interac",
            CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Jcb => "jcb",
            CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Link => "link",
            CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Mastercard => "mastercard",
            CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Unionpay => "unionpay",
            CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::UnknownValue => "unknown",
            CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Visa => "visa",
            CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsCardNetwork {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "amex" => CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Amex,
            "cartes_bancaires" => CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::CartesBancaires,
            "diners" => CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Diners,
            "discover" => CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Discover,
            "eftpos_au" => CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::EftposAu,
            "girocard" => CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Girocard,
            "interac" => CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Interac,
            "jcb" => CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Jcb,
            "link" => CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Link,
            "mastercard" => CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Mastercard,
            "unionpay" => CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Unionpay,
            "unknown" => CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::UnknownValue,
            "visa" => CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Visa,
            other => CreateSetupIntentParamsPaymentMethodOptionsCardNetwork::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsCardNetwork {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval {
    Day,
    Month,
    Week,
    Year,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Day => "day",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Month => "month",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Week => "week",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Year => "year",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Day,
            "month" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Month,
            "week" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Week,
            "year" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Year,
            other => {
                CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsLink {
    #[doc = "Deprecated This is a legacy parameter that no longer has any function."]
    pub persistent_token: Option<String>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsLink {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("persistent_token", self.persistent_token.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose {
    Unnamed,
    DependantSupport,
    Government,
    Loan,
    Mortgage,
    Other,
    Pension,
    Personal,
    Retail,
    Salary,
    Tax,
    Utility,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unnamed => "",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::DependantSupport => {
                "dependant_support"
            }
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Government => "government",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Loan => "loan",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Mortgage => "mortgage",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Other => "other",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Pension => "pension",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Personal => "personal",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Retail => "retail",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Salary => "salary",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Tax => "tax",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Utility => "utility",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unnamed,
            "dependant_support" => {
                CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::DependantSupport
            }
            "government" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Government,
            "loan" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Loan,
            "mortgage" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Mortgage,
            "other" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Other,
            "pension" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Pension,
            "personal" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Personal,
            "retail" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Retail,
            "salary" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Salary,
            "tax" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Tax,
            "utility" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Utility,
            other => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    Unnamed,
    Paper,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::Unnamed => "",
            CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::Paper => "paper",
            CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json>
    for CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::Unnamed,
            "paper" => CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::Paper,
            other => CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode
    for CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsUsage {
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsUsage::OffSession => "off_session",
            CreateSetupIntentParamsUsage::OnSession => "on_session",
            CreateSetupIntentParamsUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsUsage::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "off_session" => CreateSetupIntentParamsUsage::OffSession,
            "on_session" => CreateSetupIntentParamsUsage::OnSession,
            other => CreateSetupIntentParamsUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum SetupIntentExcludedPaymentMethodTypesItem {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    Alma,
    AmazonPay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Billie,
    Bizum,
    Blik,
    Boleto,
    Card,
    Cashapp,
    Crypto,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    KakaoPay,
    Klarna,
    Konbini,
    KrCard,
    MbWay,
    Mobilepay,
    Multibanco,
    NaverPay,
    NzBankAccount,
    Oxxo,
    P24,
    PayByBank,
    Payco,
    Paynow,
    Paypal,
    Paypay,
    Payto,
    Pix,
    Promptpay,
    RevolutPay,
    SamsungPay,
    Satispay,
    Scalapay,
    SepaDebit,
    Sequra,
    Sofort,
    Sunbit,
    Swish,
    Twint,
    Upi,
    UsBankAccount,
    WechatPay,
    Zip,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl SetupIntentExcludedPaymentMethodTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentExcludedPaymentMethodTypesItem::AcssDebit => "acss_debit",
            SetupIntentExcludedPaymentMethodTypesItem::Affirm => "affirm",
            SetupIntentExcludedPaymentMethodTypesItem::AfterpayClearpay => "afterpay_clearpay",
            SetupIntentExcludedPaymentMethodTypesItem::Alipay => "alipay",
            SetupIntentExcludedPaymentMethodTypesItem::Alma => "alma",
            SetupIntentExcludedPaymentMethodTypesItem::AmazonPay => "amazon_pay",
            SetupIntentExcludedPaymentMethodTypesItem::AuBecsDebit => "au_becs_debit",
            SetupIntentExcludedPaymentMethodTypesItem::BacsDebit => "bacs_debit",
            SetupIntentExcludedPaymentMethodTypesItem::Bancontact => "bancontact",
            SetupIntentExcludedPaymentMethodTypesItem::Billie => "billie",
            SetupIntentExcludedPaymentMethodTypesItem::Bizum => "bizum",
            SetupIntentExcludedPaymentMethodTypesItem::Blik => "blik",
            SetupIntentExcludedPaymentMethodTypesItem::Boleto => "boleto",
            SetupIntentExcludedPaymentMethodTypesItem::Card => "card",
            SetupIntentExcludedPaymentMethodTypesItem::Cashapp => "cashapp",
            SetupIntentExcludedPaymentMethodTypesItem::Crypto => "crypto",
            SetupIntentExcludedPaymentMethodTypesItem::CustomerBalance => "customer_balance",
            SetupIntentExcludedPaymentMethodTypesItem::Eps => "eps",
            SetupIntentExcludedPaymentMethodTypesItem::Fpx => "fpx",
            SetupIntentExcludedPaymentMethodTypesItem::Giropay => "giropay",
            SetupIntentExcludedPaymentMethodTypesItem::Grabpay => "grabpay",
            SetupIntentExcludedPaymentMethodTypesItem::Ideal => "ideal",
            SetupIntentExcludedPaymentMethodTypesItem::KakaoPay => "kakao_pay",
            SetupIntentExcludedPaymentMethodTypesItem::Klarna => "klarna",
            SetupIntentExcludedPaymentMethodTypesItem::Konbini => "konbini",
            SetupIntentExcludedPaymentMethodTypesItem::KrCard => "kr_card",
            SetupIntentExcludedPaymentMethodTypesItem::MbWay => "mb_way",
            SetupIntentExcludedPaymentMethodTypesItem::Mobilepay => "mobilepay",
            SetupIntentExcludedPaymentMethodTypesItem::Multibanco => "multibanco",
            SetupIntentExcludedPaymentMethodTypesItem::NaverPay => "naver_pay",
            SetupIntentExcludedPaymentMethodTypesItem::NzBankAccount => "nz_bank_account",
            SetupIntentExcludedPaymentMethodTypesItem::Oxxo => "oxxo",
            SetupIntentExcludedPaymentMethodTypesItem::P24 => "p24",
            SetupIntentExcludedPaymentMethodTypesItem::PayByBank => "pay_by_bank",
            SetupIntentExcludedPaymentMethodTypesItem::Payco => "payco",
            SetupIntentExcludedPaymentMethodTypesItem::Paynow => "paynow",
            SetupIntentExcludedPaymentMethodTypesItem::Paypal => "paypal",
            SetupIntentExcludedPaymentMethodTypesItem::Paypay => "paypay",
            SetupIntentExcludedPaymentMethodTypesItem::Payto => "payto",
            SetupIntentExcludedPaymentMethodTypesItem::Pix => "pix",
            SetupIntentExcludedPaymentMethodTypesItem::Promptpay => "promptpay",
            SetupIntentExcludedPaymentMethodTypesItem::RevolutPay => "revolut_pay",
            SetupIntentExcludedPaymentMethodTypesItem::SamsungPay => "samsung_pay",
            SetupIntentExcludedPaymentMethodTypesItem::Satispay => "satispay",
            SetupIntentExcludedPaymentMethodTypesItem::Scalapay => "scalapay",
            SetupIntentExcludedPaymentMethodTypesItem::SepaDebit => "sepa_debit",
            SetupIntentExcludedPaymentMethodTypesItem::Sequra => "sequra",
            SetupIntentExcludedPaymentMethodTypesItem::Sofort => "sofort",
            SetupIntentExcludedPaymentMethodTypesItem::Sunbit => "sunbit",
            SetupIntentExcludedPaymentMethodTypesItem::Swish => "swish",
            SetupIntentExcludedPaymentMethodTypesItem::Twint => "twint",
            SetupIntentExcludedPaymentMethodTypesItem::Upi => "upi",
            SetupIntentExcludedPaymentMethodTypesItem::UsBankAccount => "us_bank_account",
            SetupIntentExcludedPaymentMethodTypesItem::WechatPay => "wechat_pay",
            SetupIntentExcludedPaymentMethodTypesItem::Zip => "zip",
            SetupIntentExcludedPaymentMethodTypesItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl<'json> json::Decode<'json> for SetupIntentExcludedPaymentMethodTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentExcludedPaymentMethodTypesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "acss_debit" => SetupIntentExcludedPaymentMethodTypesItem::AcssDebit,
            "affirm" => SetupIntentExcludedPaymentMethodTypesItem::Affirm,
            "afterpay_clearpay" => SetupIntentExcludedPaymentMethodTypesItem::AfterpayClearpay,
            "alipay" => SetupIntentExcludedPaymentMethodTypesItem::Alipay,
            "alma" => SetupIntentExcludedPaymentMethodTypesItem::Alma,
            "amazon_pay" => SetupIntentExcludedPaymentMethodTypesItem::AmazonPay,
            "au_becs_debit" => SetupIntentExcludedPaymentMethodTypesItem::AuBecsDebit,
            "bacs_debit" => SetupIntentExcludedPaymentMethodTypesItem::BacsDebit,
            "bancontact" => SetupIntentExcludedPaymentMethodTypesItem::Bancontact,
            "billie" => SetupIntentExcludedPaymentMethodTypesItem::Billie,
            "bizum" => SetupIntentExcludedPaymentMethodTypesItem::Bizum,
            "blik" => SetupIntentExcludedPaymentMethodTypesItem::Blik,
            "boleto" => SetupIntentExcludedPaymentMethodTypesItem::Boleto,
            "card" => SetupIntentExcludedPaymentMethodTypesItem::Card,
            "cashapp" => SetupIntentExcludedPaymentMethodTypesItem::Cashapp,
            "crypto" => SetupIntentExcludedPaymentMethodTypesItem::Crypto,
            "customer_balance" => SetupIntentExcludedPaymentMethodTypesItem::CustomerBalance,
            "eps" => SetupIntentExcludedPaymentMethodTypesItem::Eps,
            "fpx" => SetupIntentExcludedPaymentMethodTypesItem::Fpx,
            "giropay" => SetupIntentExcludedPaymentMethodTypesItem::Giropay,
            "grabpay" => SetupIntentExcludedPaymentMethodTypesItem::Grabpay,
            "ideal" => SetupIntentExcludedPaymentMethodTypesItem::Ideal,
            "kakao_pay" => SetupIntentExcludedPaymentMethodTypesItem::KakaoPay,
            "klarna" => SetupIntentExcludedPaymentMethodTypesItem::Klarna,
            "konbini" => SetupIntentExcludedPaymentMethodTypesItem::Konbini,
            "kr_card" => SetupIntentExcludedPaymentMethodTypesItem::KrCard,
            "mb_way" => SetupIntentExcludedPaymentMethodTypesItem::MbWay,
            "mobilepay" => SetupIntentExcludedPaymentMethodTypesItem::Mobilepay,
            "multibanco" => SetupIntentExcludedPaymentMethodTypesItem::Multibanco,
            "naver_pay" => SetupIntentExcludedPaymentMethodTypesItem::NaverPay,
            "nz_bank_account" => SetupIntentExcludedPaymentMethodTypesItem::NzBankAccount,
            "oxxo" => SetupIntentExcludedPaymentMethodTypesItem::Oxxo,
            "p24" => SetupIntentExcludedPaymentMethodTypesItem::P24,
            "pay_by_bank" => SetupIntentExcludedPaymentMethodTypesItem::PayByBank,
            "payco" => SetupIntentExcludedPaymentMethodTypesItem::Payco,
            "paynow" => SetupIntentExcludedPaymentMethodTypesItem::Paynow,
            "paypal" => SetupIntentExcludedPaymentMethodTypesItem::Paypal,
            "paypay" => SetupIntentExcludedPaymentMethodTypesItem::Paypay,
            "payto" => SetupIntentExcludedPaymentMethodTypesItem::Payto,
            "pix" => SetupIntentExcludedPaymentMethodTypesItem::Pix,
            "promptpay" => SetupIntentExcludedPaymentMethodTypesItem::Promptpay,
            "revolut_pay" => SetupIntentExcludedPaymentMethodTypesItem::RevolutPay,
            "samsung_pay" => SetupIntentExcludedPaymentMethodTypesItem::SamsungPay,
            "satispay" => SetupIntentExcludedPaymentMethodTypesItem::Satispay,
            "scalapay" => SetupIntentExcludedPaymentMethodTypesItem::Scalapay,
            "sepa_debit" => SetupIntentExcludedPaymentMethodTypesItem::SepaDebit,
            "sequra" => SetupIntentExcludedPaymentMethodTypesItem::Sequra,
            "sofort" => SetupIntentExcludedPaymentMethodTypesItem::Sofort,
            "sunbit" => SetupIntentExcludedPaymentMethodTypesItem::Sunbit,
            "swish" => SetupIntentExcludedPaymentMethodTypesItem::Swish,
            "twint" => SetupIntentExcludedPaymentMethodTypesItem::Twint,
            "upi" => SetupIntentExcludedPaymentMethodTypesItem::Upi,
            "us_bank_account" => SetupIntentExcludedPaymentMethodTypesItem::UsBankAccount,
            "wechat_pay" => SetupIntentExcludedPaymentMethodTypesItem::WechatPay,
            "zip" => SetupIntentExcludedPaymentMethodTypesItem::Zip,
            other => SetupIntentExcludedPaymentMethodTypesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum SetupIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Girocard,
    Interac,
    Jcb,
    Link,
    Mastercard,
    Unionpay,
    UnknownValue,
    Visa,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl SetupIntentPaymentMethodOptionsCardNetwork {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentPaymentMethodOptionsCardNetwork::Amex => "amex",
            SetupIntentPaymentMethodOptionsCardNetwork::CartesBancaires => "cartes_bancaires",
            SetupIntentPaymentMethodOptionsCardNetwork::Diners => "diners",
            SetupIntentPaymentMethodOptionsCardNetwork::Discover => "discover",
            SetupIntentPaymentMethodOptionsCardNetwork::EftposAu => "eftpos_au",
            SetupIntentPaymentMethodOptionsCardNetwork::Girocard => "girocard",
            SetupIntentPaymentMethodOptionsCardNetwork::Interac => "interac",
            SetupIntentPaymentMethodOptionsCardNetwork::Jcb => "jcb",
            SetupIntentPaymentMethodOptionsCardNetwork::Link => "link",
            SetupIntentPaymentMethodOptionsCardNetwork::Mastercard => "mastercard",
            SetupIntentPaymentMethodOptionsCardNetwork::Unionpay => "unionpay",
            SetupIntentPaymentMethodOptionsCardNetwork::UnknownValue => "unknown",
            SetupIntentPaymentMethodOptionsCardNetwork::Visa => "visa",
            SetupIntentPaymentMethodOptionsCardNetwork::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl<'json> json::Decode<'json> for SetupIntentPaymentMethodOptionsCardNetwork {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentPaymentMethodOptionsCardNetwork::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "amex" => SetupIntentPaymentMethodOptionsCardNetwork::Amex,
            "cartes_bancaires" => SetupIntentPaymentMethodOptionsCardNetwork::CartesBancaires,
            "diners" => SetupIntentPaymentMethodOptionsCardNetwork::Diners,
            "discover" => SetupIntentPaymentMethodOptionsCardNetwork::Discover,
            "eftpos_au" => SetupIntentPaymentMethodOptionsCardNetwork::EftposAu,
            "girocard" => SetupIntentPaymentMethodOptionsCardNetwork::Girocard,
            "interac" => SetupIntentPaymentMethodOptionsCardNetwork::Interac,
            "jcb" => SetupIntentPaymentMethodOptionsCardNetwork::Jcb,
            "link" => SetupIntentPaymentMethodOptionsCardNetwork::Link,
            "mastercard" => SetupIntentPaymentMethodOptionsCardNetwork::Mastercard,
            "unionpay" => SetupIntentPaymentMethodOptionsCardNetwork::Unionpay,
            "unknown" => SetupIntentPaymentMethodOptionsCardNetwork::UnknownValue,
            "visa" => SetupIntentPaymentMethodOptionsCardNetwork::Visa,
            other => SetupIntentPaymentMethodOptionsCardNetwork::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultForItem {
    Invoice,
    Subscription,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultForItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultForItem::Invoice => "invoice",
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultForItem::Subscription => "subscription",
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultForItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl<'json> json::Decode<'json> for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultForItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultForItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "invoice" => SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultForItem::Invoice,
            "subscription" => SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultForItem::Subscription,
            other => SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultForItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule {
    Adhoc,
    Annual,
    Daily,
    Fortnightly,
    Monthly,
    Quarterly,
    SemiAnnual,
    Weekly,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Adhoc => "adhoc",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Annual => "annual",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Daily => "daily",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Fortnightly => "fortnightly",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Monthly => "monthly",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Quarterly => "quarterly",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::SemiAnnual => "semi_annual",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Weekly => "weekly",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl<'json> json::Decode<'json> for SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "adhoc" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Adhoc,
            "annual" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Annual,
            "daily" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Daily,
            "fortnightly" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Fortnightly,
            "monthly" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Monthly,
            "quarterly" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Quarterly,
            "semi_annual" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::SemiAnnual,
            "weekly" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Weekly,
            other => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose {
    DependantSupport,
    Government,
    Loan,
    Mortgage,
    Other,
    Pension,
    Personal,
    Retail,
    Salary,
    Tax,
    Utility,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::DependantSupport => "dependant_support",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Government => "government",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Loan => "loan",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Mortgage => "mortgage",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Other => "other",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Pension => "pension",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Personal => "personal",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Retail => "retail",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Salary => "salary",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Tax => "tax",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Utility => "utility",
            SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl<'json> json::Decode<'json> for SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "dependant_support" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::DependantSupport,
            "government" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Government,
            "loan" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Loan,
            "mortgage" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Mortgage,
            "other" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Other,
            "pension" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Pension,
            "personal" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Personal,
            "retail" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Retail,
            "salary" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Salary,
            "tax" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Tax,
            "utility" => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Utility,
            other => SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Unknown(other.to_string()),
        })
    }
}
