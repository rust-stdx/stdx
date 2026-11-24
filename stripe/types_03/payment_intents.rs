#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ApplyCustomerBalanceParams {
    #[doc = "Amount that you intend to apply to this PaymentIntent from the customer’s cash balance."]
    pub amount: Option<i64>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ApplyCustomerBalanceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsAmountDetailsTax {
    #[doc = "The total amount of tax on the transaction represented in the smallest currency unit..."]
    pub total_tax_amount: i64,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsAmountDetailsTax {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("total_tax_amount", &self.total_tax_amount);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsCaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsCaptureMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsCaptureMethod::Automatic => "automatic",
            CreatePaymentIntentParamsCaptureMethod::AutomaticAsync => "automatic_async",
            CreatePaymentIntentParamsCaptureMethod::Manual => "manual",
            CreatePaymentIntentParamsCaptureMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsCaptureMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsCaptureMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => CreatePaymentIntentParamsCaptureMethod::Automatic,
            "automatic_async" => CreatePaymentIntentParamsCaptureMethod::AutomaticAsync,
            "manual" => CreatePaymentIntentParamsCaptureMethod::Manual,
            other => CreatePaymentIntentParamsCaptureMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsCaptureMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsHooksInputsTax {
    #[doc = "The [TaxCalculation](https://docs.stripe.com/api/tax/calculations) id"]
    pub calculation: json::OwnedValue,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsHooksInputsTax {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline {}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsMandateDataCustomerAcceptanceType {
    Offline,
    Online,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsMandateDataCustomerAcceptanceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsMandateDataCustomerAcceptanceType::Offline => "offline",
            CreatePaymentIntentParamsMandateDataCustomerAcceptanceType::Online => "online",
            CreatePaymentIntentParamsMandateDataCustomerAcceptanceType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsMandateDataCustomerAcceptanceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsMandateDataCustomerAcceptanceType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "offline" => CreatePaymentIntentParamsMandateDataCustomerAcceptanceType::Offline,
            "online" => CreatePaymentIntentParamsMandateDataCustomerAcceptanceType::Online,
            other => CreatePaymentIntentParamsMandateDataCustomerAcceptanceType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsMandateDataCustomerAcceptanceType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataAcssDebit {
    #[doc = "Customer's bank account number."]
    pub account_number: String,
    #[doc = "Institution number of the customer's bank."]
    pub institution_number: String,
    #[doc = "Transit number of the customer's bank."]
    pub transit_number: String,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataAcssDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("account_number", &self.account_number);
        form.field("institution_number", &self.institution_number);
        form.field("transit_number", &self.transit_number);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataBoleto {
    #[doc = "The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers)"]
    pub tax_id: String,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataBoleto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("tax_id", &self.tax_id);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataPayto {
    #[doc = "The account number for the bank account."]
    pub account_number: Option<String>,
    #[doc = "Bank-State-Branch number of the bank account."]
    pub bsb_number: Option<String>,
    #[doc = "The PayID alias for the bank account."]
    pub pay_id: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataPayto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_number", self.account_number.as_ref());
        form.field_opt("bsb_number", self.bsb_number.as_ref());
        form.field_opt("pay_id", self.pay_id.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataRadarOptions {
    #[doc = "A [Radar Session](https://docs.stripe.com/radar/radar-session) is a snapshot of the browser metadata and device detai..."]
    pub session: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataRadarOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("session", self.session.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataSepaDebit {
    #[doc = "IBAN of the bank account."]
    pub iban: String,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataSepaDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("iban", &self.iban);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodDataSofortCountry {
    AT,
    BE,
    DE,
    ES,
    IT,
    NL,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodDataSofortCountry {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodDataSofortCountry::AT => "AT",
            CreatePaymentIntentParamsPaymentMethodDataSofortCountry::BE => "BE",
            CreatePaymentIntentParamsPaymentMethodDataSofortCountry::DE => "DE",
            CreatePaymentIntentParamsPaymentMethodDataSofortCountry::ES => "ES",
            CreatePaymentIntentParamsPaymentMethodDataSofortCountry::IT => "IT",
            CreatePaymentIntentParamsPaymentMethodDataSofortCountry::NL => "NL",
            CreatePaymentIntentParamsPaymentMethodDataSofortCountry::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodDataSofortCountry {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodDataSofortCountry::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "AT" => CreatePaymentIntentParamsPaymentMethodDataSofortCountry::AT,
            "BE" => CreatePaymentIntentParamsPaymentMethodDataSofortCountry::BE,
            "DE" => CreatePaymentIntentParamsPaymentMethodDataSofortCountry::DE,
            "ES" => CreatePaymentIntentParamsPaymentMethodDataSofortCountry::ES,
            "IT" => CreatePaymentIntentParamsPaymentMethodDataSofortCountry::IT,
            "NL" => CreatePaymentIntentParamsPaymentMethodDataSofortCountry::NL,
            other => CreatePaymentIntentParamsPaymentMethodDataSofortCountry::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataSofortCountry {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Combined => "combined",
            CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Interval => "interval",
            CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Sporadic => "sporadic",
            CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json>
    for CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "combined" => CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Combined,
            "interval" => CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Interval,
            "sporadic" => CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Sporadic,
            other => CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => "automatic",
            CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => "microdeposits",
            CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Automatic,
            "instant" => CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Instant,
            "microdeposits" => CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits,
            other => {
                CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsBacsDebitMandateOptions {
    #[doc = "Prefix used to generate the Mandate reference."]
    pub reference_prefix: Option<json::OwnedValue>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsBacsDebitMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanInterval {
    Month,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanInterval::Month => "month",
            CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanInterval::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "month" => CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanInterval::Month,
            other => {
                CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanInterval::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsCardPresentCaptureMethod {
    Manual,
    ManualPreferred,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsCardPresentCaptureMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsCardPresentCaptureMethod::Manual => "manual",
            CreatePaymentIntentParamsPaymentMethodOptionsCardPresentCaptureMethod::ManualPreferred => {
                "manual_preferred"
            }
            CreatePaymentIntentParamsPaymentMethodOptionsCardPresentCaptureMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsCardPresentCaptureMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsCardPresentCaptureMethod::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "manual" => CreatePaymentIntentParamsPaymentMethodOptionsCardPresentCaptureMethod::Manual,
            "manual_preferred" => {
                CreatePaymentIntentParamsPaymentMethodOptionsCardPresentCaptureMethod::ManualPreferred
            }
            other => CreatePaymentIntentParamsPaymentMethodOptionsCardPresentCaptureMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCardPresentCaptureMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            CreatePaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
            CreatePaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Challenge => "challenge",
            CreatePaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "any" => CreatePaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Any,
            "automatic" => CreatePaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Automatic,
            "challenge" => CreatePaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Challenge,
            other => CreatePaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion {
    N102,
    N210,
    N220,
    N230,
    N231,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N102 => "1.0.2",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N210 => "2.1.0",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N220 => "2.2.0",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N230 => "2.3.0",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N231 => "2.3.1",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "1.0.2" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N102,
            "2.1.0" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N210,
            "2.2.0" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N220,
            "2.3.0" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N230,
            "2.3.1" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N231,
            other => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale {
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

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::CsCZ => "cs-CZ",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::DaDK => "da-DK",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::DeAT => "de-AT",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::DeDE => "de-DE",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::DeLU => "de-LU",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::ElGR => "el-GR",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::EnGB => "en-GB",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::EnUS => "en-US",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::EsES => "es-ES",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::FiFI => "fi-FI",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::FrBE => "fr-BE",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::FrFR => "fr-FR",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::FrLU => "fr-LU",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::HuHU => "hu-HU",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::ItIT => "it-IT",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::NlBE => "nl-BE",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::NlNL => "nl-NL",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::PlPL => "pl-PL",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::PtPT => "pt-PT",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::SkSK => "sk-SK",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::SvSE => "sv-SE",
            CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cs-CZ" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::CsCZ,
            "da-DK" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::DaDK,
            "de-AT" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::DeAT,
            "de-DE" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::DeDE,
            "de-LU" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::DeLU,
            "el-GR" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::ElGR,
            "en-GB" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::EnGB,
            "en-US" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::EnUS,
            "es-ES" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::EsES,
            "fi-FI" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::FiFI,
            "fr-BE" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::FrBE,
            "fr-FR" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::FrFR,
            "fr-LU" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::FrLU,
            "hu-HU" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::HuHU,
            "it-IT" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::ItIT,
            "nl-BE" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::NlBE,
            "nl-NL" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::NlNL,
            "pl-PL" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::PlPL,
            "pt-PT" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::PtPT,
            "sk-SK" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::SkSK,
            "sv-SE" => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::SvSE,
            other => CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType {
    Unnamed,
    Fixed,
    Maximum,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Unnamed => "",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Fixed => "fixed",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Maximum => "maximum",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Unnamed,
            "fixed" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Fixed,
            "maximum" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Maximum,
            other => {
                CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem {
    Ach,
    UsDomesticWire,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem::Ach => "ach",
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem::UsDomesticWire => {
                "us_domestic_wire"
            }
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach" => CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem::Ach,
            "us_domestic_wire" => {
                CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem::UsDomesticWire
            }
            other => CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsRadarOptions {
    #[doc = "The referrer URL of the current checkout session."]
    pub referrer: Option<json::OwnedValue>,
    #[doc = "A [Radar Session](https://docs.stripe.com/radar/radar-session) is a snapshot of the browser metadata and device detai..."]
    pub session: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsRadarOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("session", self.session.as_ref());
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
pub enum FundingInstructionsBankTransferFinancialAddressType {
    Aba,
    Iban,
    SortCode,
    Spei,
    Swift,
    Zengin,
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
impl FundingInstructionsBankTransferFinancialAddressType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FundingInstructionsBankTransferFinancialAddressType::Aba => "aba",
            FundingInstructionsBankTransferFinancialAddressType::Iban => "iban",
            FundingInstructionsBankTransferFinancialAddressType::SortCode => "sort_code",
            FundingInstructionsBankTransferFinancialAddressType::Spei => "spei",
            FundingInstructionsBankTransferFinancialAddressType::Swift => "swift",
            FundingInstructionsBankTransferFinancialAddressType::Zengin => "zengin",
            FundingInstructionsBankTransferFinancialAddressType::Unknown(value) => value,
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
impl Default for FundingInstructionsBankTransferFinancialAddressType {
    fn default() -> Self {
        FundingInstructionsBankTransferFinancialAddressType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for FundingInstructionsBankTransferFinancialAddressType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FundingInstructionsBankTransferFinancialAddressType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "aba" => FundingInstructionsBankTransferFinancialAddressType::Aba,
            "iban" => FundingInstructionsBankTransferFinancialAddressType::Iban,
            "sort_code" => FundingInstructionsBankTransferFinancialAddressType::SortCode,
            "spei" => FundingInstructionsBankTransferFinancialAddressType::Spei,
            "swift" => FundingInstructionsBankTransferFinancialAddressType::Swift,
            "zengin" => FundingInstructionsBankTransferFinancialAddressType::Zengin,
            other => FundingInstructionsBankTransferFinancialAddressType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, PartialEq)]
pub enum FundingInstructionsBankTransferType {
    EuBankTransfer,
    JpBankTransfer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "customers")]
impl FundingInstructionsBankTransferType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FundingInstructionsBankTransferType::EuBankTransfer => "eu_bank_transfer",
            FundingInstructionsBankTransferType::JpBankTransfer => "jp_bank_transfer",
            FundingInstructionsBankTransferType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "customers")]
impl Default for FundingInstructionsBankTransferType {
    fn default() -> Self {
        FundingInstructionsBankTransferType::Unknown(String::new())
    }
}

#[cfg(feature = "customers")]
impl<'json> json::Decode<'json> for FundingInstructionsBankTransferType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FundingInstructionsBankTransferType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "eu_bank_transfer" => FundingInstructionsBankTransferType::EuBankTransfer,
            "jp_bank_transfer" => FundingInstructionsBankTransferType::JpBankTransfer,
            other => FundingInstructionsBankTransferType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct GetPaymentIntentParams {
    pub client_secret: Option<String>,
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for GetPaymentIntentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("client_secret", self.client_secret.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ListPaymentIntentsParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ListPaymentIntentsParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
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
pub enum PaymentFlowsAmountDetailsResourceErrorCode {
    AmountDetailsAmountMismatch,
    AmountDetailsTaxShippingDiscountGreaterThanAmount,
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
impl PaymentFlowsAmountDetailsResourceErrorCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentFlowsAmountDetailsResourceErrorCode::AmountDetailsAmountMismatch => "amount_details_amount_mismatch",
            PaymentFlowsAmountDetailsResourceErrorCode::AmountDetailsTaxShippingDiscountGreaterThanAmount => {
                "amount_details_tax_shipping_discount_greater_than_amount"
            }
            PaymentFlowsAmountDetailsResourceErrorCode::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentFlowsAmountDetailsResourceErrorCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentFlowsAmountDetailsResourceErrorCode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "amount_details_amount_mismatch" => PaymentFlowsAmountDetailsResourceErrorCode::AmountDetailsAmountMismatch,
            "amount_details_tax_shipping_discount_greater_than_amount" => {
                PaymentFlowsAmountDetailsResourceErrorCode::AmountDetailsTaxShippingDiscountGreaterThanAmount
            }
            other => PaymentFlowsAmountDetailsResourceErrorCode::Unknown(other.to_string()),
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourceTax {
    #[doc = "The total amount of tax on a single line item represented in the smallest currency unit..."]
    #[json(default)]
    pub total_tax_amount: i64,
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentFlowsPrivatePaymentMethodsAlipay {}

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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub commodity_code: Option<String>,
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
pub enum PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategoriesItem
{
    Checking,
    Savings,
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
impl PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategoriesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategoriesItem::Checking => "checking",
            PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategoriesItem::Savings => "savings",
            PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategoriesItem::Unknown(value) => value,
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
impl<'json> json::Decode<'json>
    for PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategoriesItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategoriesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "checking" => PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategoriesItem::Checking,
            "savings" => PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategoriesItem::Savings,
            other => PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategoriesItem::Unknown(other.to_string()),
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
pub enum PaymentIntentAllowedPaymentMethodTypesItem {
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
    BokuPromptpay,
    Boleto,
    CapchasePay,
    Card,
    CardPresent,
    Cashapp,
    CheckScan,
    ClickToPay,
    Crypto,
    CustomerBalance,
    DemoPay,
    Duitnow,
    DummyAuthPush,
    DummyPassthroughCard,
    Edenred,
    Eps,
    Fpx,
    Gcash,
    Getbalance,
    GiftCard,
    Giropay,
    Gopay,
    Grabpay,
    IdBankTransfer,
    Ideal,
    InteracPresent,
    KakaoPay,
    Klarna,
    Knet,
    Konbini,
    KrCard,
    KrMarket,
    Kriya,
    Link,
    MbWay,
    Mobilepay,
    Momo,
    Mondu,
    Multibanco,
    NaverPay,
    Netbanking,
    NgBank,
    NgBankTransfer,
    NgCard,
    NgMarket,
    NgUssd,
    NgWallet,
    NzBankAccount,
    Octopus,
    Oxxo,
    P24,
    PaperCheck,
    PayByBank,
    Payco,
    Paynow,
    Paypal,
    Paypay,
    Payto,
    Pix,
    Promptpay,
    Qris,
    Rechnung,
    RevolutPay,
    SamsungPay,
    Satispay,
    Scalapay,
    SepaDebit,
    Sequra,
    ShopPay,
    Shopeepay,
    Sofort,
    SouthKoreaMarket,
    StripeBalance,
    Sunbit,
    Swish,
    Tamara,
    TestPay,
    TouchNGo,
    Truemoney,
    Twint,
    Upi,
    UsBankAccount,
    UsCashVoucher,
    Vipps,
    WechatPay,
    Wero,
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
impl PaymentIntentAllowedPaymentMethodTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentAllowedPaymentMethodTypesItem::AcssDebit => "acss_debit",
            PaymentIntentAllowedPaymentMethodTypesItem::Affirm => "affirm",
            PaymentIntentAllowedPaymentMethodTypesItem::AfterpayClearpay => "afterpay_clearpay",
            PaymentIntentAllowedPaymentMethodTypesItem::Alipay => "alipay",
            PaymentIntentAllowedPaymentMethodTypesItem::Alma => "alma",
            PaymentIntentAllowedPaymentMethodTypesItem::AmazonPay => "amazon_pay",
            PaymentIntentAllowedPaymentMethodTypesItem::AuBecsDebit => "au_becs_debit",
            PaymentIntentAllowedPaymentMethodTypesItem::BacsDebit => "bacs_debit",
            PaymentIntentAllowedPaymentMethodTypesItem::Bancontact => "bancontact",
            PaymentIntentAllowedPaymentMethodTypesItem::Billie => "billie",
            PaymentIntentAllowedPaymentMethodTypesItem::Bizum => "bizum",
            PaymentIntentAllowedPaymentMethodTypesItem::Blik => "blik",
            PaymentIntentAllowedPaymentMethodTypesItem::BokuPromptpay => "boku_promptpay",
            PaymentIntentAllowedPaymentMethodTypesItem::Boleto => "boleto",
            PaymentIntentAllowedPaymentMethodTypesItem::CapchasePay => "capchase_pay",
            PaymentIntentAllowedPaymentMethodTypesItem::Card => "card",
            PaymentIntentAllowedPaymentMethodTypesItem::CardPresent => "card_present",
            PaymentIntentAllowedPaymentMethodTypesItem::Cashapp => "cashapp",
            PaymentIntentAllowedPaymentMethodTypesItem::CheckScan => "check_scan",
            PaymentIntentAllowedPaymentMethodTypesItem::ClickToPay => "click_to_pay",
            PaymentIntentAllowedPaymentMethodTypesItem::Crypto => "crypto",
            PaymentIntentAllowedPaymentMethodTypesItem::CustomerBalance => "customer_balance",
            PaymentIntentAllowedPaymentMethodTypesItem::DemoPay => "demo_pay",
            PaymentIntentAllowedPaymentMethodTypesItem::Duitnow => "duitnow",
            PaymentIntentAllowedPaymentMethodTypesItem::DummyAuthPush => "dummy_auth_push",
            PaymentIntentAllowedPaymentMethodTypesItem::DummyPassthroughCard => "dummy_passthrough_card",
            PaymentIntentAllowedPaymentMethodTypesItem::Edenred => "edenred",
            PaymentIntentAllowedPaymentMethodTypesItem::Eps => "eps",
            PaymentIntentAllowedPaymentMethodTypesItem::Fpx => "fpx",
            PaymentIntentAllowedPaymentMethodTypesItem::Gcash => "gcash",
            PaymentIntentAllowedPaymentMethodTypesItem::Getbalance => "getbalance",
            PaymentIntentAllowedPaymentMethodTypesItem::GiftCard => "gift_card",
            PaymentIntentAllowedPaymentMethodTypesItem::Giropay => "giropay",
            PaymentIntentAllowedPaymentMethodTypesItem::Gopay => "gopay",
            PaymentIntentAllowedPaymentMethodTypesItem::Grabpay => "grabpay",
            PaymentIntentAllowedPaymentMethodTypesItem::IdBankTransfer => "id_bank_transfer",
            PaymentIntentAllowedPaymentMethodTypesItem::Ideal => "ideal",
            PaymentIntentAllowedPaymentMethodTypesItem::InteracPresent => "interac_present",
            PaymentIntentAllowedPaymentMethodTypesItem::KakaoPay => "kakao_pay",
            PaymentIntentAllowedPaymentMethodTypesItem::Klarna => "klarna",
            PaymentIntentAllowedPaymentMethodTypesItem::Knet => "knet",
            PaymentIntentAllowedPaymentMethodTypesItem::Konbini => "konbini",
            PaymentIntentAllowedPaymentMethodTypesItem::KrCard => "kr_card",
            PaymentIntentAllowedPaymentMethodTypesItem::KrMarket => "kr_market",
            PaymentIntentAllowedPaymentMethodTypesItem::Kriya => "kriya",
            PaymentIntentAllowedPaymentMethodTypesItem::Link => "link",
            PaymentIntentAllowedPaymentMethodTypesItem::MbWay => "mb_way",
            PaymentIntentAllowedPaymentMethodTypesItem::Mobilepay => "mobilepay",
            PaymentIntentAllowedPaymentMethodTypesItem::Momo => "momo",
            PaymentIntentAllowedPaymentMethodTypesItem::Mondu => "mondu",
            PaymentIntentAllowedPaymentMethodTypesItem::Multibanco => "multibanco",
            PaymentIntentAllowedPaymentMethodTypesItem::NaverPay => "naver_pay",
            PaymentIntentAllowedPaymentMethodTypesItem::Netbanking => "netbanking",
            PaymentIntentAllowedPaymentMethodTypesItem::NgBank => "ng_bank",
            PaymentIntentAllowedPaymentMethodTypesItem::NgBankTransfer => "ng_bank_transfer",
            PaymentIntentAllowedPaymentMethodTypesItem::NgCard => "ng_card",
            PaymentIntentAllowedPaymentMethodTypesItem::NgMarket => "ng_market",
            PaymentIntentAllowedPaymentMethodTypesItem::NgUssd => "ng_ussd",
            PaymentIntentAllowedPaymentMethodTypesItem::NgWallet => "ng_wallet",
            PaymentIntentAllowedPaymentMethodTypesItem::NzBankAccount => "nz_bank_account",
            PaymentIntentAllowedPaymentMethodTypesItem::Octopus => "octopus",
            PaymentIntentAllowedPaymentMethodTypesItem::Oxxo => "oxxo",
            PaymentIntentAllowedPaymentMethodTypesItem::P24 => "p24",
            PaymentIntentAllowedPaymentMethodTypesItem::PaperCheck => "paper_check",
            PaymentIntentAllowedPaymentMethodTypesItem::PayByBank => "pay_by_bank",
            PaymentIntentAllowedPaymentMethodTypesItem::Payco => "payco",
            PaymentIntentAllowedPaymentMethodTypesItem::Paynow => "paynow",
            PaymentIntentAllowedPaymentMethodTypesItem::Paypal => "paypal",
            PaymentIntentAllowedPaymentMethodTypesItem::Paypay => "paypay",
            PaymentIntentAllowedPaymentMethodTypesItem::Payto => "payto",
            PaymentIntentAllowedPaymentMethodTypesItem::Pix => "pix",
            PaymentIntentAllowedPaymentMethodTypesItem::Promptpay => "promptpay",
            PaymentIntentAllowedPaymentMethodTypesItem::Qris => "qris",
            PaymentIntentAllowedPaymentMethodTypesItem::Rechnung => "rechnung",
            PaymentIntentAllowedPaymentMethodTypesItem::RevolutPay => "revolut_pay",
            PaymentIntentAllowedPaymentMethodTypesItem::SamsungPay => "samsung_pay",
            PaymentIntentAllowedPaymentMethodTypesItem::Satispay => "satispay",
            PaymentIntentAllowedPaymentMethodTypesItem::Scalapay => "scalapay",
            PaymentIntentAllowedPaymentMethodTypesItem::SepaDebit => "sepa_debit",
            PaymentIntentAllowedPaymentMethodTypesItem::Sequra => "sequra",
            PaymentIntentAllowedPaymentMethodTypesItem::ShopPay => "shop_pay",
            PaymentIntentAllowedPaymentMethodTypesItem::Shopeepay => "shopeepay",
            PaymentIntentAllowedPaymentMethodTypesItem::Sofort => "sofort",
            PaymentIntentAllowedPaymentMethodTypesItem::SouthKoreaMarket => "south_korea_market",
            PaymentIntentAllowedPaymentMethodTypesItem::StripeBalance => "stripe_balance",
            PaymentIntentAllowedPaymentMethodTypesItem::Sunbit => "sunbit",
            PaymentIntentAllowedPaymentMethodTypesItem::Swish => "swish",
            PaymentIntentAllowedPaymentMethodTypesItem::Tamara => "tamara",
            PaymentIntentAllowedPaymentMethodTypesItem::TestPay => "test_pay",
            PaymentIntentAllowedPaymentMethodTypesItem::TouchNGo => "touch_n_go",
            PaymentIntentAllowedPaymentMethodTypesItem::Truemoney => "truemoney",
            PaymentIntentAllowedPaymentMethodTypesItem::Twint => "twint",
            PaymentIntentAllowedPaymentMethodTypesItem::Upi => "upi",
            PaymentIntentAllowedPaymentMethodTypesItem::UsBankAccount => "us_bank_account",
            PaymentIntentAllowedPaymentMethodTypesItem::UsCashVoucher => "us_cash_voucher",
            PaymentIntentAllowedPaymentMethodTypesItem::Vipps => "vipps",
            PaymentIntentAllowedPaymentMethodTypesItem::WechatPay => "wechat_pay",
            PaymentIntentAllowedPaymentMethodTypesItem::Wero => "wero",
            PaymentIntentAllowedPaymentMethodTypesItem::Zip => "zip",
            PaymentIntentAllowedPaymentMethodTypesItem::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentIntentAllowedPaymentMethodTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentAllowedPaymentMethodTypesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "acss_debit" => PaymentIntentAllowedPaymentMethodTypesItem::AcssDebit,
            "affirm" => PaymentIntentAllowedPaymentMethodTypesItem::Affirm,
            "afterpay_clearpay" => PaymentIntentAllowedPaymentMethodTypesItem::AfterpayClearpay,
            "alipay" => PaymentIntentAllowedPaymentMethodTypesItem::Alipay,
            "alma" => PaymentIntentAllowedPaymentMethodTypesItem::Alma,
            "amazon_pay" => PaymentIntentAllowedPaymentMethodTypesItem::AmazonPay,
            "au_becs_debit" => PaymentIntentAllowedPaymentMethodTypesItem::AuBecsDebit,
            "bacs_debit" => PaymentIntentAllowedPaymentMethodTypesItem::BacsDebit,
            "bancontact" => PaymentIntentAllowedPaymentMethodTypesItem::Bancontact,
            "billie" => PaymentIntentAllowedPaymentMethodTypesItem::Billie,
            "bizum" => PaymentIntentAllowedPaymentMethodTypesItem::Bizum,
            "blik" => PaymentIntentAllowedPaymentMethodTypesItem::Blik,
            "boku_promptpay" => PaymentIntentAllowedPaymentMethodTypesItem::BokuPromptpay,
            "boleto" => PaymentIntentAllowedPaymentMethodTypesItem::Boleto,
            "capchase_pay" => PaymentIntentAllowedPaymentMethodTypesItem::CapchasePay,
            "card" => PaymentIntentAllowedPaymentMethodTypesItem::Card,
            "card_present" => PaymentIntentAllowedPaymentMethodTypesItem::CardPresent,
            "cashapp" => PaymentIntentAllowedPaymentMethodTypesItem::Cashapp,
            "check_scan" => PaymentIntentAllowedPaymentMethodTypesItem::CheckScan,
            "click_to_pay" => PaymentIntentAllowedPaymentMethodTypesItem::ClickToPay,
            "crypto" => PaymentIntentAllowedPaymentMethodTypesItem::Crypto,
            "customer_balance" => PaymentIntentAllowedPaymentMethodTypesItem::CustomerBalance,
            "demo_pay" => PaymentIntentAllowedPaymentMethodTypesItem::DemoPay,
            "duitnow" => PaymentIntentAllowedPaymentMethodTypesItem::Duitnow,
            "dummy_auth_push" => PaymentIntentAllowedPaymentMethodTypesItem::DummyAuthPush,
            "dummy_passthrough_card" => PaymentIntentAllowedPaymentMethodTypesItem::DummyPassthroughCard,
            "edenred" => PaymentIntentAllowedPaymentMethodTypesItem::Edenred,
            "eps" => PaymentIntentAllowedPaymentMethodTypesItem::Eps,
            "fpx" => PaymentIntentAllowedPaymentMethodTypesItem::Fpx,
            "gcash" => PaymentIntentAllowedPaymentMethodTypesItem::Gcash,
            "getbalance" => PaymentIntentAllowedPaymentMethodTypesItem::Getbalance,
            "gift_card" => PaymentIntentAllowedPaymentMethodTypesItem::GiftCard,
            "giropay" => PaymentIntentAllowedPaymentMethodTypesItem::Giropay,
            "gopay" => PaymentIntentAllowedPaymentMethodTypesItem::Gopay,
            "grabpay" => PaymentIntentAllowedPaymentMethodTypesItem::Grabpay,
            "id_bank_transfer" => PaymentIntentAllowedPaymentMethodTypesItem::IdBankTransfer,
            "ideal" => PaymentIntentAllowedPaymentMethodTypesItem::Ideal,
            "interac_present" => PaymentIntentAllowedPaymentMethodTypesItem::InteracPresent,
            "kakao_pay" => PaymentIntentAllowedPaymentMethodTypesItem::KakaoPay,
            "klarna" => PaymentIntentAllowedPaymentMethodTypesItem::Klarna,
            "knet" => PaymentIntentAllowedPaymentMethodTypesItem::Knet,
            "konbini" => PaymentIntentAllowedPaymentMethodTypesItem::Konbini,
            "kr_card" => PaymentIntentAllowedPaymentMethodTypesItem::KrCard,
            "kr_market" => PaymentIntentAllowedPaymentMethodTypesItem::KrMarket,
            "kriya" => PaymentIntentAllowedPaymentMethodTypesItem::Kriya,
            "link" => PaymentIntentAllowedPaymentMethodTypesItem::Link,
            "mb_way" => PaymentIntentAllowedPaymentMethodTypesItem::MbWay,
            "mobilepay" => PaymentIntentAllowedPaymentMethodTypesItem::Mobilepay,
            "momo" => PaymentIntentAllowedPaymentMethodTypesItem::Momo,
            "mondu" => PaymentIntentAllowedPaymentMethodTypesItem::Mondu,
            "multibanco" => PaymentIntentAllowedPaymentMethodTypesItem::Multibanco,
            "naver_pay" => PaymentIntentAllowedPaymentMethodTypesItem::NaverPay,
            "netbanking" => PaymentIntentAllowedPaymentMethodTypesItem::Netbanking,
            "ng_bank" => PaymentIntentAllowedPaymentMethodTypesItem::NgBank,
            "ng_bank_transfer" => PaymentIntentAllowedPaymentMethodTypesItem::NgBankTransfer,
            "ng_card" => PaymentIntentAllowedPaymentMethodTypesItem::NgCard,
            "ng_market" => PaymentIntentAllowedPaymentMethodTypesItem::NgMarket,
            "ng_ussd" => PaymentIntentAllowedPaymentMethodTypesItem::NgUssd,
            "ng_wallet" => PaymentIntentAllowedPaymentMethodTypesItem::NgWallet,
            "nz_bank_account" => PaymentIntentAllowedPaymentMethodTypesItem::NzBankAccount,
            "octopus" => PaymentIntentAllowedPaymentMethodTypesItem::Octopus,
            "oxxo" => PaymentIntentAllowedPaymentMethodTypesItem::Oxxo,
            "p24" => PaymentIntentAllowedPaymentMethodTypesItem::P24,
            "paper_check" => PaymentIntentAllowedPaymentMethodTypesItem::PaperCheck,
            "pay_by_bank" => PaymentIntentAllowedPaymentMethodTypesItem::PayByBank,
            "payco" => PaymentIntentAllowedPaymentMethodTypesItem::Payco,
            "paynow" => PaymentIntentAllowedPaymentMethodTypesItem::Paynow,
            "paypal" => PaymentIntentAllowedPaymentMethodTypesItem::Paypal,
            "paypay" => PaymentIntentAllowedPaymentMethodTypesItem::Paypay,
            "payto" => PaymentIntentAllowedPaymentMethodTypesItem::Payto,
            "pix" => PaymentIntentAllowedPaymentMethodTypesItem::Pix,
            "promptpay" => PaymentIntentAllowedPaymentMethodTypesItem::Promptpay,
            "qris" => PaymentIntentAllowedPaymentMethodTypesItem::Qris,
            "rechnung" => PaymentIntentAllowedPaymentMethodTypesItem::Rechnung,
            "revolut_pay" => PaymentIntentAllowedPaymentMethodTypesItem::RevolutPay,
            "samsung_pay" => PaymentIntentAllowedPaymentMethodTypesItem::SamsungPay,
            "satispay" => PaymentIntentAllowedPaymentMethodTypesItem::Satispay,
            "scalapay" => PaymentIntentAllowedPaymentMethodTypesItem::Scalapay,
            "sepa_debit" => PaymentIntentAllowedPaymentMethodTypesItem::SepaDebit,
            "sequra" => PaymentIntentAllowedPaymentMethodTypesItem::Sequra,
            "shop_pay" => PaymentIntentAllowedPaymentMethodTypesItem::ShopPay,
            "shopeepay" => PaymentIntentAllowedPaymentMethodTypesItem::Shopeepay,
            "sofort" => PaymentIntentAllowedPaymentMethodTypesItem::Sofort,
            "south_korea_market" => PaymentIntentAllowedPaymentMethodTypesItem::SouthKoreaMarket,
            "stripe_balance" => PaymentIntentAllowedPaymentMethodTypesItem::StripeBalance,
            "sunbit" => PaymentIntentAllowedPaymentMethodTypesItem::Sunbit,
            "swish" => PaymentIntentAllowedPaymentMethodTypesItem::Swish,
            "tamara" => PaymentIntentAllowedPaymentMethodTypesItem::Tamara,
            "test_pay" => PaymentIntentAllowedPaymentMethodTypesItem::TestPay,
            "touch_n_go" => PaymentIntentAllowedPaymentMethodTypesItem::TouchNGo,
            "truemoney" => PaymentIntentAllowedPaymentMethodTypesItem::Truemoney,
            "twint" => PaymentIntentAllowedPaymentMethodTypesItem::Twint,
            "upi" => PaymentIntentAllowedPaymentMethodTypesItem::Upi,
            "us_bank_account" => PaymentIntentAllowedPaymentMethodTypesItem::UsBankAccount,
            "us_cash_voucher" => PaymentIntentAllowedPaymentMethodTypesItem::UsCashVoucher,
            "vipps" => PaymentIntentAllowedPaymentMethodTypesItem::Vipps,
            "wechat_pay" => PaymentIntentAllowedPaymentMethodTypesItem::WechatPay,
            "wero" => PaymentIntentAllowedPaymentMethodTypesItem::Wero,
            "zip" => PaymentIntentAllowedPaymentMethodTypesItem::Zip,
            other => PaymentIntentAllowedPaymentMethodTypesItem::Unknown(other.to_string()),
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
pub enum PaymentIntentExcludedPaymentMethodTypesItem {
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
impl PaymentIntentExcludedPaymentMethodTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentExcludedPaymentMethodTypesItem::AcssDebit => "acss_debit",
            PaymentIntentExcludedPaymentMethodTypesItem::Affirm => "affirm",
            PaymentIntentExcludedPaymentMethodTypesItem::AfterpayClearpay => "afterpay_clearpay",
            PaymentIntentExcludedPaymentMethodTypesItem::Alipay => "alipay",
            PaymentIntentExcludedPaymentMethodTypesItem::Alma => "alma",
            PaymentIntentExcludedPaymentMethodTypesItem::AmazonPay => "amazon_pay",
            PaymentIntentExcludedPaymentMethodTypesItem::AuBecsDebit => "au_becs_debit",
            PaymentIntentExcludedPaymentMethodTypesItem::BacsDebit => "bacs_debit",
            PaymentIntentExcludedPaymentMethodTypesItem::Bancontact => "bancontact",
            PaymentIntentExcludedPaymentMethodTypesItem::Billie => "billie",
            PaymentIntentExcludedPaymentMethodTypesItem::Bizum => "bizum",
            PaymentIntentExcludedPaymentMethodTypesItem::Blik => "blik",
            PaymentIntentExcludedPaymentMethodTypesItem::Boleto => "boleto",
            PaymentIntentExcludedPaymentMethodTypesItem::Card => "card",
            PaymentIntentExcludedPaymentMethodTypesItem::Cashapp => "cashapp",
            PaymentIntentExcludedPaymentMethodTypesItem::Crypto => "crypto",
            PaymentIntentExcludedPaymentMethodTypesItem::CustomerBalance => "customer_balance",
            PaymentIntentExcludedPaymentMethodTypesItem::Eps => "eps",
            PaymentIntentExcludedPaymentMethodTypesItem::Fpx => "fpx",
            PaymentIntentExcludedPaymentMethodTypesItem::Giropay => "giropay",
            PaymentIntentExcludedPaymentMethodTypesItem::Grabpay => "grabpay",
            PaymentIntentExcludedPaymentMethodTypesItem::Ideal => "ideal",
            PaymentIntentExcludedPaymentMethodTypesItem::KakaoPay => "kakao_pay",
            PaymentIntentExcludedPaymentMethodTypesItem::Klarna => "klarna",
            PaymentIntentExcludedPaymentMethodTypesItem::Konbini => "konbini",
            PaymentIntentExcludedPaymentMethodTypesItem::KrCard => "kr_card",
            PaymentIntentExcludedPaymentMethodTypesItem::MbWay => "mb_way",
            PaymentIntentExcludedPaymentMethodTypesItem::Mobilepay => "mobilepay",
            PaymentIntentExcludedPaymentMethodTypesItem::Multibanco => "multibanco",
            PaymentIntentExcludedPaymentMethodTypesItem::NaverPay => "naver_pay",
            PaymentIntentExcludedPaymentMethodTypesItem::NzBankAccount => "nz_bank_account",
            PaymentIntentExcludedPaymentMethodTypesItem::Oxxo => "oxxo",
            PaymentIntentExcludedPaymentMethodTypesItem::P24 => "p24",
            PaymentIntentExcludedPaymentMethodTypesItem::PayByBank => "pay_by_bank",
            PaymentIntentExcludedPaymentMethodTypesItem::Payco => "payco",
            PaymentIntentExcludedPaymentMethodTypesItem::Paynow => "paynow",
            PaymentIntentExcludedPaymentMethodTypesItem::Paypal => "paypal",
            PaymentIntentExcludedPaymentMethodTypesItem::Paypay => "paypay",
            PaymentIntentExcludedPaymentMethodTypesItem::Payto => "payto",
            PaymentIntentExcludedPaymentMethodTypesItem::Pix => "pix",
            PaymentIntentExcludedPaymentMethodTypesItem::Promptpay => "promptpay",
            PaymentIntentExcludedPaymentMethodTypesItem::RevolutPay => "revolut_pay",
            PaymentIntentExcludedPaymentMethodTypesItem::SamsungPay => "samsung_pay",
            PaymentIntentExcludedPaymentMethodTypesItem::Satispay => "satispay",
            PaymentIntentExcludedPaymentMethodTypesItem::Scalapay => "scalapay",
            PaymentIntentExcludedPaymentMethodTypesItem::SepaDebit => "sepa_debit",
            PaymentIntentExcludedPaymentMethodTypesItem::Sequra => "sequra",
            PaymentIntentExcludedPaymentMethodTypesItem::Sofort => "sofort",
            PaymentIntentExcludedPaymentMethodTypesItem::Sunbit => "sunbit",
            PaymentIntentExcludedPaymentMethodTypesItem::Swish => "swish",
            PaymentIntentExcludedPaymentMethodTypesItem::Twint => "twint",
            PaymentIntentExcludedPaymentMethodTypesItem::Upi => "upi",
            PaymentIntentExcludedPaymentMethodTypesItem::UsBankAccount => "us_bank_account",
            PaymentIntentExcludedPaymentMethodTypesItem::WechatPay => "wechat_pay",
            PaymentIntentExcludedPaymentMethodTypesItem::Zip => "zip",
            PaymentIntentExcludedPaymentMethodTypesItem::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentIntentExcludedPaymentMethodTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentExcludedPaymentMethodTypesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "acss_debit" => PaymentIntentExcludedPaymentMethodTypesItem::AcssDebit,
            "affirm" => PaymentIntentExcludedPaymentMethodTypesItem::Affirm,
            "afterpay_clearpay" => PaymentIntentExcludedPaymentMethodTypesItem::AfterpayClearpay,
            "alipay" => PaymentIntentExcludedPaymentMethodTypesItem::Alipay,
            "alma" => PaymentIntentExcludedPaymentMethodTypesItem::Alma,
            "amazon_pay" => PaymentIntentExcludedPaymentMethodTypesItem::AmazonPay,
            "au_becs_debit" => PaymentIntentExcludedPaymentMethodTypesItem::AuBecsDebit,
            "bacs_debit" => PaymentIntentExcludedPaymentMethodTypesItem::BacsDebit,
            "bancontact" => PaymentIntentExcludedPaymentMethodTypesItem::Bancontact,
            "billie" => PaymentIntentExcludedPaymentMethodTypesItem::Billie,
            "bizum" => PaymentIntentExcludedPaymentMethodTypesItem::Bizum,
            "blik" => PaymentIntentExcludedPaymentMethodTypesItem::Blik,
            "boleto" => PaymentIntentExcludedPaymentMethodTypesItem::Boleto,
            "card" => PaymentIntentExcludedPaymentMethodTypesItem::Card,
            "cashapp" => PaymentIntentExcludedPaymentMethodTypesItem::Cashapp,
            "crypto" => PaymentIntentExcludedPaymentMethodTypesItem::Crypto,
            "customer_balance" => PaymentIntentExcludedPaymentMethodTypesItem::CustomerBalance,
            "eps" => PaymentIntentExcludedPaymentMethodTypesItem::Eps,
            "fpx" => PaymentIntentExcludedPaymentMethodTypesItem::Fpx,
            "giropay" => PaymentIntentExcludedPaymentMethodTypesItem::Giropay,
            "grabpay" => PaymentIntentExcludedPaymentMethodTypesItem::Grabpay,
            "ideal" => PaymentIntentExcludedPaymentMethodTypesItem::Ideal,
            "kakao_pay" => PaymentIntentExcludedPaymentMethodTypesItem::KakaoPay,
            "klarna" => PaymentIntentExcludedPaymentMethodTypesItem::Klarna,
            "konbini" => PaymentIntentExcludedPaymentMethodTypesItem::Konbini,
            "kr_card" => PaymentIntentExcludedPaymentMethodTypesItem::KrCard,
            "mb_way" => PaymentIntentExcludedPaymentMethodTypesItem::MbWay,
            "mobilepay" => PaymentIntentExcludedPaymentMethodTypesItem::Mobilepay,
            "multibanco" => PaymentIntentExcludedPaymentMethodTypesItem::Multibanco,
            "naver_pay" => PaymentIntentExcludedPaymentMethodTypesItem::NaverPay,
            "nz_bank_account" => PaymentIntentExcludedPaymentMethodTypesItem::NzBankAccount,
            "oxxo" => PaymentIntentExcludedPaymentMethodTypesItem::Oxxo,
            "p24" => PaymentIntentExcludedPaymentMethodTypesItem::P24,
            "pay_by_bank" => PaymentIntentExcludedPaymentMethodTypesItem::PayByBank,
            "payco" => PaymentIntentExcludedPaymentMethodTypesItem::Payco,
            "paynow" => PaymentIntentExcludedPaymentMethodTypesItem::Paynow,
            "paypal" => PaymentIntentExcludedPaymentMethodTypesItem::Paypal,
            "paypay" => PaymentIntentExcludedPaymentMethodTypesItem::Paypay,
            "payto" => PaymentIntentExcludedPaymentMethodTypesItem::Payto,
            "pix" => PaymentIntentExcludedPaymentMethodTypesItem::Pix,
            "promptpay" => PaymentIntentExcludedPaymentMethodTypesItem::Promptpay,
            "revolut_pay" => PaymentIntentExcludedPaymentMethodTypesItem::RevolutPay,
            "samsung_pay" => PaymentIntentExcludedPaymentMethodTypesItem::SamsungPay,
            "satispay" => PaymentIntentExcludedPaymentMethodTypesItem::Satispay,
            "scalapay" => PaymentIntentExcludedPaymentMethodTypesItem::Scalapay,
            "sepa_debit" => PaymentIntentExcludedPaymentMethodTypesItem::SepaDebit,
            "sequra" => PaymentIntentExcludedPaymentMethodTypesItem::Sequra,
            "sofort" => PaymentIntentExcludedPaymentMethodTypesItem::Sofort,
            "sunbit" => PaymentIntentExcludedPaymentMethodTypesItem::Sunbit,
            "swish" => PaymentIntentExcludedPaymentMethodTypesItem::Swish,
            "twint" => PaymentIntentExcludedPaymentMethodTypesItem::Twint,
            "upi" => PaymentIntentExcludedPaymentMethodTypesItem::Upi,
            "us_bank_account" => PaymentIntentExcludedPaymentMethodTypesItem::UsBankAccount,
            "wechat_pay" => PaymentIntentExcludedPaymentMethodTypesItem::WechatPay,
            "zip" => PaymentIntentExcludedPaymentMethodTypesItem::Zip,
            other => PaymentIntentExcludedPaymentMethodTypesItem::Unknown(other.to_string()),
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentIntentNextActionDisplayMultibancoDetails {
    #[doc = "Entity number associated with this Multibanco payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
    #[doc = "The timestamp at which the Multibanco voucher expires."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[doc = "The URL for the hosted Multibanco voucher page, which allows customers to view a Multibanco voucher."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_voucher_url: Option<String>,
    #[doc = "Reference number associated with this Multibanco payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentIntentNextActionPaynowDisplayQrCode {
    #[doc = "The raw data string used to generate QR code, it should be used together with QR code library."]
    #[json(default)]
    pub data: String,
    #[doc = "The URL to the hosted PayNow instructions page, which allows customers to view the PayNow QR code."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_instructions_url: Option<String>,
    #[doc = "The image_url_png string used to render QR code"]
    #[json(default)]
    pub image_url_png: String,
    #[doc = "The image_url_svg string used to render QR code"]
    #[json(default)]
    pub image_url_svg: String,
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentIntentNextActionPixDisplayQrCode {
    #[doc = "The raw data string used to generate QR code, it should be used together with QR code library."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[doc = "The date (unix timestamp) when the PIX expires."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[doc = "The URL to the hosted pix instructions page, which allows customers to view the pix QR code."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_instructions_url: Option<String>,
    #[doc = "The image_url_png string used to render png QR code"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub image_url_png: Option<String>,
    #[doc = "The image_url_svg string used to render svg QR code"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub image_url_svg: Option<String>,
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentIntentNextActionWechatPayRedirectToAndroidApp {
    #[doc = "app_id is the APP ID registered on WeChat open platform"]
    #[json(default)]
    pub app_id: String,
    #[doc = "nonce_str is a random string"]
    #[json(default)]
    pub nonce_str: String,
    #[doc = "package is static value"]
    #[json(default)]
    pub package: String,
    #[doc = "an unique merchant ID assigned by WeChat Pay"]
    #[json(default)]
    pub partner_id: String,
    #[doc = "an unique trading ID assigned by WeChat Pay"]
    #[json(default)]
    pub prepay_id: String,
    #[doc = "A signature"]
    #[json(default)]
    pub sign: String,
    #[doc = "Specifies the current time in epoch format"]
    #[json(default)]
    pub timestamp: String,
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
pub enum PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
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
impl PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage::None => "none",
            PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage::OffSession => "off_session",
            PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage::OnSession => "on_session",
            PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage::None,
            "off_session" => PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage::OffSession,
            "on_session" => PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage::OnSession,
            other => PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage::Unknown(other.to_string()),
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentIntentPaymentMethodOptionsMandateOptionsBacsDebit {
    #[doc = "Prefix used to generate the Mandate reference."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference_prefix: Option<String>,
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
pub enum PaymentIntentPaymentMethodOptionsMandateOptionsBlikType {
    OffSession,
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
impl PaymentIntentPaymentMethodOptionsMandateOptionsBlikType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentPaymentMethodOptionsMandateOptionsBlikType::OffSession => "off_session",
            PaymentIntentPaymentMethodOptionsMandateOptionsBlikType::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentIntentPaymentMethodOptionsMandateOptionsBlikType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentPaymentMethodOptionsMandateOptionsBlikType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "off_session" => PaymentIntentPaymentMethodOptionsMandateOptionsBlikType::OffSession,
            other => PaymentIntentPaymentMethodOptionsMandateOptionsBlikType::Unknown(other.to_string()),
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
pub enum PaymentIntentProcessingType {
    Card,
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
impl PaymentIntentProcessingType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentProcessingType::Card => "card",
            PaymentIntentProcessingType::Unknown(value) => value,
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
impl Default for PaymentIntentProcessingType {
    fn default() -> Self {
        PaymentIntentProcessingType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for PaymentIntentProcessingType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentProcessingType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "card" => PaymentIntentProcessingType::Card,
            other => PaymentIntentProcessingType::Unknown(other.to_string()),
        })
    }
}
