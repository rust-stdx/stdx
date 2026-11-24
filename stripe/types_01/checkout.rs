#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutAcssDebitMandateOptionsTransactionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutAcssDebitMandateOptionsTransactionType::Business => "business",
            CheckoutAcssDebitMandateOptionsTransactionType::Personal => "personal",
            CheckoutAcssDebitMandateOptionsTransactionType::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutAcssDebitMandateOptionsTransactionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutAcssDebitMandateOptionsTransactionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "business" => CheckoutAcssDebitMandateOptionsTransactionType::Business,
            "personal" => CheckoutAcssDebitMandateOptionsTransactionType::Personal,
            other => CheckoutAcssDebitMandateOptionsTransactionType::Unknown(other.to_string()),
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
pub enum CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::Automatic => "automatic",
            CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::Instant => "instant",
            CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::Microdeposits => "microdeposits",
            CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::Automatic,
            "instant" => CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::Instant,
            "microdeposits" => CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::Microdeposits,
            other => CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::Unknown(other.to_string()),
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
pub enum CheckoutAffirmPaymentMethodOptionsCaptureMethod {
    Manual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutAffirmPaymentMethodOptionsCaptureMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutAffirmPaymentMethodOptionsCaptureMethod::Manual => "manual",
            CheckoutAffirmPaymentMethodOptionsCaptureMethod::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutAffirmPaymentMethodOptionsCaptureMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutAffirmPaymentMethodOptionsCaptureMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "manual" => CheckoutAffirmPaymentMethodOptionsCaptureMethod::Manual,
            other => CheckoutAffirmPaymentMethodOptionsCaptureMethod::Unknown(other.to_string()),
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
pub enum CheckoutLinkWalletOptionsDisplay {
    Auto,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutLinkWalletOptionsDisplay {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutLinkWalletOptionsDisplay::Auto => "auto",
            CheckoutLinkWalletOptionsDisplay::Never => "never",
            CheckoutLinkWalletOptionsDisplay::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutLinkWalletOptionsDisplay {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutLinkWalletOptionsDisplay::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => CheckoutLinkWalletOptionsDisplay::Auto,
            "never" => CheckoutLinkWalletOptionsDisplay::Never,
            other => CheckoutLinkWalletOptionsDisplay::Unknown(other.to_string()),
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
pub struct CheckoutPaymentMethodOptionsMandateOptionsSepaDebit {
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
pub enum CheckoutSessionCustomerCreation {
    Always,
    IfRequired,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutSessionCustomerCreation {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutSessionCustomerCreation::Always => "always",
            CheckoutSessionCustomerCreation::IfRequired => "if_required",
            CheckoutSessionCustomerCreation::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutSessionCustomerCreation {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutSessionCustomerCreation::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CheckoutSessionCustomerCreation::Always,
            "if_required" => CheckoutSessionCustomerCreation::IfRequired,
            other => CheckoutSessionCustomerCreation::Unknown(other.to_string()),
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
pub enum CheckoutSessionSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
    Subscribe,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutSessionSubmitType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutSessionSubmitType::Auto => "auto",
            CheckoutSessionSubmitType::Book => "book",
            CheckoutSessionSubmitType::Donate => "donate",
            CheckoutSessionSubmitType::Pay => "pay",
            CheckoutSessionSubmitType::Subscribe => "subscribe",
            CheckoutSessionSubmitType::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutSessionSubmitType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutSessionSubmitType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => CheckoutSessionSubmitType::Auto,
            "book" => CheckoutSessionSubmitType::Book,
            "donate" => CheckoutSessionSubmitType::Donate,
            "pay" => CheckoutSessionSubmitType::Pay,
            "subscribe" => CheckoutSessionSubmitType::Subscribe,
            other => CheckoutSessionSubmitType::Unknown(other.to_string()),
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
pub enum CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    Automatic,
    Instant,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod::Automatic => "automatic",
            CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod::Instant => "instant",
            CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod::Automatic,
            "instant" => CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod::Instant,
            other => CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod::Unknown(other.to_string()),
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
pub enum CheckoutWechatPayPaymentMethodOptionsClient {
    Android,
    Ios,
    Web,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutWechatPayPaymentMethodOptionsClient {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutWechatPayPaymentMethodOptionsClient::Android => "android",
            CheckoutWechatPayPaymentMethodOptionsClient::Ios => "ios",
            CheckoutWechatPayPaymentMethodOptionsClient::Web => "web",
            CheckoutWechatPayPaymentMethodOptionsClient::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutWechatPayPaymentMethodOptionsClient {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutWechatPayPaymentMethodOptionsClient::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "android" => CheckoutWechatPayPaymentMethodOptionsClient::Android,
            "ios" => CheckoutWechatPayPaymentMethodOptionsClient::Ios,
            "web" => CheckoutWechatPayPaymentMethodOptionsClient::Web,
            other => CheckoutWechatPayPaymentMethodOptionsClient::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsConsentCollectionPaymentMethodReuseAgreementPosition {
    Auto,
    Hidden,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsConsentCollectionPaymentMethodReuseAgreementPosition {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsConsentCollectionPaymentMethodReuseAgreementPosition::Auto => "auto",
            CreateCheckoutSessionParamsConsentCollectionPaymentMethodReuseAgreementPosition::Hidden => "hidden",
            CreateCheckoutSessionParamsConsentCollectionPaymentMethodReuseAgreementPosition::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsConsentCollectionPaymentMethodReuseAgreementPosition {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsConsentCollectionPaymentMethodReuseAgreementPosition::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => CreateCheckoutSessionParamsConsentCollectionPaymentMethodReuseAgreementPosition::Auto,
            "hidden" => CreateCheckoutSessionParamsConsentCollectionPaymentMethodReuseAgreementPosition::Hidden,
            other => CreateCheckoutSessionParamsConsentCollectionPaymentMethodReuseAgreementPosition::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsConsentCollectionPaymentMethodReuseAgreementPosition {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsCustomFieldsItemLabelType {
    Custom,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsCustomFieldsItemLabelType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsCustomFieldsItemLabelType::Custom => "custom",
            CreateCheckoutSessionParamsCustomFieldsItemLabelType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsCustomFieldsItemLabelType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsCustomFieldsItemLabelType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "custom" => CreateCheckoutSessionParamsCustomFieldsItemLabelType::Custom,
            other => CreateCheckoutSessionParamsCustomFieldsItemLabelType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsCustomFieldsItemLabelType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsDiscountsItem {
    #[doc = "The ID of the coupon to apply to this Session."]
    pub coupon: Option<String>,
    #[doc = "The ID of a promotion code to apply to this Session."]
    pub promotion_code: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsDiscountsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("coupon", self.coupon.as_ref());
        form.field_opt("promotion_code", self.promotion_code.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsLineItemsItemPriceDataProductDataTaxDetails {
    #[doc = "A tax location ID."]
    pub performance_location: Option<String>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<json::OwnedValue>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsLineItemsItemPriceDataProductDataTaxDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("performance_location", self.performance_location.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval::Day => "day",
            CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval::Month => "month",
            CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval::Week => "week",
            CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval::Year => "year",
            CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval::Day,
            "month" => CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval::Month,
            "week" => CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval::Week,
            "year" => CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval::Year,
            other => CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentIntentDataSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentIntentDataSetupFutureUsage::OffSession => "off_session",
            CreateCheckoutSessionParamsPaymentIntentDataSetupFutureUsage::OnSession => "on_session",
            CreateCheckoutSessionParamsPaymentIntentDataSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentIntentDataSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsPaymentIntentDataSetupFutureUsage::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "off_session" => CreateCheckoutSessionParamsPaymentIntentDataSetupFutureUsage::OffSession,
            "on_session" => CreateCheckoutSessionParamsPaymentIntentDataSetupFutureUsage::OnSession,
            other => CreateCheckoutSessionParamsPaymentIntentDataSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentIntentDataSetupFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentIntentDataTransferData {
    #[doc = "The amount that will be transferred automatically when a charge succeeds."]
    pub amount: Option<i64>,
    #[doc = "If specified, successful charges will be attributed to the destination account for tax reporting, and the funds from ..."]
    pub destination: String,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentIntentDataTransferData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field("destination", &self.destination);
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodDataAllowRedisplay {
    Always,
    Limited,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodDataAllowRedisplay {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodDataAllowRedisplay::Always => "always",
            CreateCheckoutSessionParamsPaymentMethodDataAllowRedisplay::Limited => "limited",
            CreateCheckoutSessionParamsPaymentMethodDataAllowRedisplay::Unspecified => "unspecified",
            CreateCheckoutSessionParamsPaymentMethodDataAllowRedisplay::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodDataAllowRedisplay {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsPaymentMethodDataAllowRedisplay::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CreateCheckoutSessionParamsPaymentMethodDataAllowRedisplay::Always,
            "limited" => CreateCheckoutSessionParamsPaymentMethodDataAllowRedisplay::Limited,
            "unspecified" => CreateCheckoutSessionParamsPaymentMethodDataAllowRedisplay::Unspecified,
            other => CreateCheckoutSessionParamsPaymentMethodDataAllowRedisplay::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodDataAllowRedisplay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod {
    Manual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod::Manual => "manual",
            CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "manual" => CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod::Manual,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
    #[doc = "The desired country code of the bank account information."]
    pub country: String,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode
    for CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("country", &self.country);
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem {
    Aba,
    Iban,
    Sepa,
    SortCode,
    Spei,
    Swift,
    Zengin,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Aba => "aba",
            CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Iban => "iban",
            CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Sepa => "sepa",
            CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::SortCode => "sort_code",
            CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Spei => "spei",
            CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Swift => "swift",
            CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Zengin => "zengin",
            CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json>
    for CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "aba" => CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Aba,
            "iban" => CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Iban,
            "sepa" => CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Sepa,
            "sort_code" => CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::SortCode,
            "spei" => CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Spei,
            "swift" => CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Swift,
            "zengin" => CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Zengin,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode
    for CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule {
    Halfyearly,
    Monthly,
    Quarterly,
    Weekly,
    Yearly,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Halfyearly => "halfyearly",
            CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Monthly => "monthly",
            CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Quarterly => "quarterly",
            CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Weekly => "weekly",
            CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Yearly => "yearly",
            CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "halfyearly" => CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Halfyearly,
            "monthly" => CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Monthly,
            "quarterly" => CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Quarterly,
            "weekly" => CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Weekly,
            "yearly" => CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Yearly,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Balances => "balances",
            CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Ownership => "ownership",
            CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::PaymentMethod => "payment_method",
            CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Transactions => "transactions",
            CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json>
    for CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balances" => CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Balances,
            "ownership" => CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Ownership,
            "payment_method" => CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::PaymentMethod,
            "transactions" => CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Transactions,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode
    for CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPhoneNumberCollection {
    #[doc = "Set to `true` to enable phone number collection."]
    pub enabled: bool,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPhoneNumberCollection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsSavedPaymentMethodOptionsPaymentMethodRemove {
    Disabled,
    Enabled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsSavedPaymentMethodOptionsPaymentMethodRemove {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsSavedPaymentMethodOptionsPaymentMethodRemove::Disabled => "disabled",
            CreateCheckoutSessionParamsSavedPaymentMethodOptionsPaymentMethodRemove::Enabled => "enabled",
            CreateCheckoutSessionParamsSavedPaymentMethodOptionsPaymentMethodRemove::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsSavedPaymentMethodOptionsPaymentMethodRemove {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsSavedPaymentMethodOptionsPaymentMethodRemove::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "disabled" => CreateCheckoutSessionParamsSavedPaymentMethodOptionsPaymentMethodRemove::Disabled,
            "enabled" => CreateCheckoutSessionParamsSavedPaymentMethodOptionsPaymentMethodRemove::Enabled,
            other => {
                CreateCheckoutSessionParamsSavedPaymentMethodOptionsPaymentMethodRemove::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsSavedPaymentMethodOptionsPaymentMethodRemove {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsSetupIntentData {
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The Stripe account for which the setup is intended."]
    pub on_behalf_of: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsSetupIntentData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("on_behalf_of", self.on_behalf_of.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataType {
    FixedAmount,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataType::FixedAmount => "fixed_amount",
            CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "fixed_amount" => CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataType::FixedAmount,
            other => CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct ListCheckoutSessionLineItemsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for ListCheckoutSessionLineItemsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListCheckoutSessionsParamsStatus {
    Complete,
    Expired,
    Open,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl ListCheckoutSessionsParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListCheckoutSessionsParamsStatus::Complete => "complete",
            ListCheckoutSessionsParamsStatus::Expired => "expired",
            ListCheckoutSessionsParamsStatus::Open => "open",
            ListCheckoutSessionsParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for ListCheckoutSessionsParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListCheckoutSessionsParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "complete" => ListCheckoutSessionsParamsStatus::Complete,
            "expired" => ListCheckoutSessionsParamsStatus::Expired,
            "open" => ListCheckoutSessionsParamsStatus::Open,
            other => ListCheckoutSessionsParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for ListCheckoutSessionsParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentPagesCheckoutSessionConsentTermsOfService {
    Accepted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesCheckoutSessionConsentTermsOfService {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesCheckoutSessionConsentTermsOfService::Accepted => "accepted",
            PaymentPagesCheckoutSessionConsentTermsOfService::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionConsentTermsOfService {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentPagesCheckoutSessionConsentTermsOfService::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "accepted" => PaymentPagesCheckoutSessionConsentTermsOfService::Accepted,
            other => PaymentPagesCheckoutSessionConsentTermsOfService::Unknown(other.to_string()),
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
pub enum PaymentPagesCheckoutSessionCustomFieldsLabelType {
    Custom,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesCheckoutSessionCustomFieldsLabelType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesCheckoutSessionCustomFieldsLabelType::Custom => "custom",
            PaymentPagesCheckoutSessionCustomFieldsLabelType::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl Default for PaymentPagesCheckoutSessionCustomFieldsLabelType {
    fn default() -> Self {
        PaymentPagesCheckoutSessionCustomFieldsLabelType::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionCustomFieldsLabelType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentPagesCheckoutSessionCustomFieldsLabelType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "custom" => PaymentPagesCheckoutSessionCustomFieldsLabelType::Custom,
            other => PaymentPagesCheckoutSessionCustomFieldsLabelType::Unknown(other.to_string()),
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
pub enum PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::Exempt => "exempt",
            PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::None => "none",
            PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::Reverse => "reverse",
            PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exempt" => PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::Exempt,
            "none" => PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::None,
            "reverse" => PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::Reverse,
            other => PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::Unknown(other.to_string()),
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
pub enum PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove {
    Disabled,
    Enabled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove::Disabled => "disabled",
            PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove::Enabled => "enabled",
            PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "disabled" => PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove::Disabled,
            "enabled" => PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove::Enabled,
            other => {
                PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove::Unknown(other.to_string())
            }
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
pub enum PaymentPagesCheckoutSessionTaxIdCollectionRequired {
    IfSupported,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesCheckoutSessionTaxIdCollectionRequired {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesCheckoutSessionTaxIdCollectionRequired::IfSupported => "if_supported",
            PaymentPagesCheckoutSessionTaxIdCollectionRequired::Never => "never",
            PaymentPagesCheckoutSessionTaxIdCollectionRequired::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl Default for PaymentPagesCheckoutSessionTaxIdCollectionRequired {
    fn default() -> Self {
        PaymentPagesCheckoutSessionTaxIdCollectionRequired::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionTaxIdCollectionRequired {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentPagesCheckoutSessionTaxIdCollectionRequired::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "if_supported" => PaymentPagesCheckoutSessionTaxIdCollectionRequired::IfSupported,
            "never" => PaymentPagesCheckoutSessionTaxIdCollectionRequired::Never,
            other => PaymentPagesCheckoutSessionTaxIdCollectionRequired::Unknown(other.to_string()),
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
pub enum PaymentPagesCheckoutSessionTaxIdType {
    AdNrt,
    AeTrn,
    AlTin,
    AmTin,
    AoTin,
    ArCuit,
    AuAbn,
    AuArn,
    AwTin,
    AzTin,
    BaTin,
    BbTin,
    BdBin,
    BfIfu,
    BgUic,
    BhVat,
    BjIfu,
    BoTin,
    BrCnpj,
    BrCpf,
    BsTin,
    ByTin,
    CaBn,
    CaGstHst,
    CaPstBc,
    CaPstMb,
    CaPstSk,
    CaQst,
    CdNif,
    ChUid,
    ChVat,
    ClTin,
    CmNiu,
    CnTin,
    CoNit,
    CrTin,
    CvNif,
    DeStn,
    DoRcn,
    EcRuc,
    EgTin,
    EsCif,
    EtTin,
    EuOssVat,
    EuVat,
    FoVat,
    GbVat,
    GeVat,
    GiTin,
    GnNif,
    HkBr,
    HrOib,
    HuTin,
    IcNif,
    IdNpwp,
    IlVat,
    InGst,
    IsVat,
    ItCf,
    JpCn,
    JpRn,
    JpTrn,
    KePin,
    KgTin,
    KhTin,
    KrBrn,
    KzBin,
    LaTin,
    LiUid,
    LiVat,
    LkVat,
    MaVat,
    MdVat,
    MePib,
    MkVat,
    MrNif,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NgTin,
    NoVat,
    NoVoec,
    NpPan,
    NzGst,
    OmVat,
    PeRuc,
    PhTin,
    PlNip,
    PyRuc,
    RoTin,
    RsPib,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    SnNinea,
    SrFin,
    SvNit,
    ThVat,
    TjTin,
    TrTin,
    TwVat,
    TzVat,
    UaVat,
    UgTin,
    UnknownValue,
    UsEin,
    UyRuc,
    UzTin,
    UzVat,
    VeRif,
    VnTin,
    ZaVat,
    ZmTin,
    ZwTin,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesCheckoutSessionTaxIdType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesCheckoutSessionTaxIdType::AdNrt => "ad_nrt",
            PaymentPagesCheckoutSessionTaxIdType::AeTrn => "ae_trn",
            PaymentPagesCheckoutSessionTaxIdType::AlTin => "al_tin",
            PaymentPagesCheckoutSessionTaxIdType::AmTin => "am_tin",
            PaymentPagesCheckoutSessionTaxIdType::AoTin => "ao_tin",
            PaymentPagesCheckoutSessionTaxIdType::ArCuit => "ar_cuit",
            PaymentPagesCheckoutSessionTaxIdType::AuAbn => "au_abn",
            PaymentPagesCheckoutSessionTaxIdType::AuArn => "au_arn",
            PaymentPagesCheckoutSessionTaxIdType::AwTin => "aw_tin",
            PaymentPagesCheckoutSessionTaxIdType::AzTin => "az_tin",
            PaymentPagesCheckoutSessionTaxIdType::BaTin => "ba_tin",
            PaymentPagesCheckoutSessionTaxIdType::BbTin => "bb_tin",
            PaymentPagesCheckoutSessionTaxIdType::BdBin => "bd_bin",
            PaymentPagesCheckoutSessionTaxIdType::BfIfu => "bf_ifu",
            PaymentPagesCheckoutSessionTaxIdType::BgUic => "bg_uic",
            PaymentPagesCheckoutSessionTaxIdType::BhVat => "bh_vat",
            PaymentPagesCheckoutSessionTaxIdType::BjIfu => "bj_ifu",
            PaymentPagesCheckoutSessionTaxIdType::BoTin => "bo_tin",
            PaymentPagesCheckoutSessionTaxIdType::BrCnpj => "br_cnpj",
            PaymentPagesCheckoutSessionTaxIdType::BrCpf => "br_cpf",
            PaymentPagesCheckoutSessionTaxIdType::BsTin => "bs_tin",
            PaymentPagesCheckoutSessionTaxIdType::ByTin => "by_tin",
            PaymentPagesCheckoutSessionTaxIdType::CaBn => "ca_bn",
            PaymentPagesCheckoutSessionTaxIdType::CaGstHst => "ca_gst_hst",
            PaymentPagesCheckoutSessionTaxIdType::CaPstBc => "ca_pst_bc",
            PaymentPagesCheckoutSessionTaxIdType::CaPstMb => "ca_pst_mb",
            PaymentPagesCheckoutSessionTaxIdType::CaPstSk => "ca_pst_sk",
            PaymentPagesCheckoutSessionTaxIdType::CaQst => "ca_qst",
            PaymentPagesCheckoutSessionTaxIdType::CdNif => "cd_nif",
            PaymentPagesCheckoutSessionTaxIdType::ChUid => "ch_uid",
            PaymentPagesCheckoutSessionTaxIdType::ChVat => "ch_vat",
            PaymentPagesCheckoutSessionTaxIdType::ClTin => "cl_tin",
            PaymentPagesCheckoutSessionTaxIdType::CmNiu => "cm_niu",
            PaymentPagesCheckoutSessionTaxIdType::CnTin => "cn_tin",
            PaymentPagesCheckoutSessionTaxIdType::CoNit => "co_nit",
            PaymentPagesCheckoutSessionTaxIdType::CrTin => "cr_tin",
            PaymentPagesCheckoutSessionTaxIdType::CvNif => "cv_nif",
            PaymentPagesCheckoutSessionTaxIdType::DeStn => "de_stn",
            PaymentPagesCheckoutSessionTaxIdType::DoRcn => "do_rcn",
            PaymentPagesCheckoutSessionTaxIdType::EcRuc => "ec_ruc",
            PaymentPagesCheckoutSessionTaxIdType::EgTin => "eg_tin",
            PaymentPagesCheckoutSessionTaxIdType::EsCif => "es_cif",
            PaymentPagesCheckoutSessionTaxIdType::EtTin => "et_tin",
            PaymentPagesCheckoutSessionTaxIdType::EuOssVat => "eu_oss_vat",
            PaymentPagesCheckoutSessionTaxIdType::EuVat => "eu_vat",
            PaymentPagesCheckoutSessionTaxIdType::FoVat => "fo_vat",
            PaymentPagesCheckoutSessionTaxIdType::GbVat => "gb_vat",
            PaymentPagesCheckoutSessionTaxIdType::GeVat => "ge_vat",
            PaymentPagesCheckoutSessionTaxIdType::GiTin => "gi_tin",
            PaymentPagesCheckoutSessionTaxIdType::GnNif => "gn_nif",
            PaymentPagesCheckoutSessionTaxIdType::HkBr => "hk_br",
            PaymentPagesCheckoutSessionTaxIdType::HrOib => "hr_oib",
            PaymentPagesCheckoutSessionTaxIdType::HuTin => "hu_tin",
            PaymentPagesCheckoutSessionTaxIdType::IcNif => "ic_nif",
            PaymentPagesCheckoutSessionTaxIdType::IdNpwp => "id_npwp",
            PaymentPagesCheckoutSessionTaxIdType::IlVat => "il_vat",
            PaymentPagesCheckoutSessionTaxIdType::InGst => "in_gst",
            PaymentPagesCheckoutSessionTaxIdType::IsVat => "is_vat",
            PaymentPagesCheckoutSessionTaxIdType::ItCf => "it_cf",
            PaymentPagesCheckoutSessionTaxIdType::JpCn => "jp_cn",
            PaymentPagesCheckoutSessionTaxIdType::JpRn => "jp_rn",
            PaymentPagesCheckoutSessionTaxIdType::JpTrn => "jp_trn",
            PaymentPagesCheckoutSessionTaxIdType::KePin => "ke_pin",
            PaymentPagesCheckoutSessionTaxIdType::KgTin => "kg_tin",
            PaymentPagesCheckoutSessionTaxIdType::KhTin => "kh_tin",
            PaymentPagesCheckoutSessionTaxIdType::KrBrn => "kr_brn",
            PaymentPagesCheckoutSessionTaxIdType::KzBin => "kz_bin",
            PaymentPagesCheckoutSessionTaxIdType::LaTin => "la_tin",
            PaymentPagesCheckoutSessionTaxIdType::LiUid => "li_uid",
            PaymentPagesCheckoutSessionTaxIdType::LiVat => "li_vat",
            PaymentPagesCheckoutSessionTaxIdType::LkVat => "lk_vat",
            PaymentPagesCheckoutSessionTaxIdType::MaVat => "ma_vat",
            PaymentPagesCheckoutSessionTaxIdType::MdVat => "md_vat",
            PaymentPagesCheckoutSessionTaxIdType::MePib => "me_pib",
            PaymentPagesCheckoutSessionTaxIdType::MkVat => "mk_vat",
            PaymentPagesCheckoutSessionTaxIdType::MrNif => "mr_nif",
            PaymentPagesCheckoutSessionTaxIdType::MxRfc => "mx_rfc",
            PaymentPagesCheckoutSessionTaxIdType::MyFrp => "my_frp",
            PaymentPagesCheckoutSessionTaxIdType::MyItn => "my_itn",
            PaymentPagesCheckoutSessionTaxIdType::MySst => "my_sst",
            PaymentPagesCheckoutSessionTaxIdType::NgTin => "ng_tin",
            PaymentPagesCheckoutSessionTaxIdType::NoVat => "no_vat",
            PaymentPagesCheckoutSessionTaxIdType::NoVoec => "no_voec",
            PaymentPagesCheckoutSessionTaxIdType::NpPan => "np_pan",
            PaymentPagesCheckoutSessionTaxIdType::NzGst => "nz_gst",
            PaymentPagesCheckoutSessionTaxIdType::OmVat => "om_vat",
            PaymentPagesCheckoutSessionTaxIdType::PeRuc => "pe_ruc",
            PaymentPagesCheckoutSessionTaxIdType::PhTin => "ph_tin",
            PaymentPagesCheckoutSessionTaxIdType::PlNip => "pl_nip",
            PaymentPagesCheckoutSessionTaxIdType::PyRuc => "py_ruc",
            PaymentPagesCheckoutSessionTaxIdType::RoTin => "ro_tin",
            PaymentPagesCheckoutSessionTaxIdType::RsPib => "rs_pib",
            PaymentPagesCheckoutSessionTaxIdType::RuInn => "ru_inn",
            PaymentPagesCheckoutSessionTaxIdType::RuKpp => "ru_kpp",
            PaymentPagesCheckoutSessionTaxIdType::SaVat => "sa_vat",
            PaymentPagesCheckoutSessionTaxIdType::SgGst => "sg_gst",
            PaymentPagesCheckoutSessionTaxIdType::SgUen => "sg_uen",
            PaymentPagesCheckoutSessionTaxIdType::SiTin => "si_tin",
            PaymentPagesCheckoutSessionTaxIdType::SnNinea => "sn_ninea",
            PaymentPagesCheckoutSessionTaxIdType::SrFin => "sr_fin",
            PaymentPagesCheckoutSessionTaxIdType::SvNit => "sv_nit",
            PaymentPagesCheckoutSessionTaxIdType::ThVat => "th_vat",
            PaymentPagesCheckoutSessionTaxIdType::TjTin => "tj_tin",
            PaymentPagesCheckoutSessionTaxIdType::TrTin => "tr_tin",
            PaymentPagesCheckoutSessionTaxIdType::TwVat => "tw_vat",
            PaymentPagesCheckoutSessionTaxIdType::TzVat => "tz_vat",
            PaymentPagesCheckoutSessionTaxIdType::UaVat => "ua_vat",
            PaymentPagesCheckoutSessionTaxIdType::UgTin => "ug_tin",
            PaymentPagesCheckoutSessionTaxIdType::UnknownValue => "unknown",
            PaymentPagesCheckoutSessionTaxIdType::UsEin => "us_ein",
            PaymentPagesCheckoutSessionTaxIdType::UyRuc => "uy_ruc",
            PaymentPagesCheckoutSessionTaxIdType::UzTin => "uz_tin",
            PaymentPagesCheckoutSessionTaxIdType::UzVat => "uz_vat",
            PaymentPagesCheckoutSessionTaxIdType::VeRif => "ve_rif",
            PaymentPagesCheckoutSessionTaxIdType::VnTin => "vn_tin",
            PaymentPagesCheckoutSessionTaxIdType::ZaVat => "za_vat",
            PaymentPagesCheckoutSessionTaxIdType::ZmTin => "zm_tin",
            PaymentPagesCheckoutSessionTaxIdType::ZwTin => "zw_tin",
            PaymentPagesCheckoutSessionTaxIdType::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl Default for PaymentPagesCheckoutSessionTaxIdType {
    fn default() -> Self {
        PaymentPagesCheckoutSessionTaxIdType::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionTaxIdType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentPagesCheckoutSessionTaxIdType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ad_nrt" => PaymentPagesCheckoutSessionTaxIdType::AdNrt,
            "ae_trn" => PaymentPagesCheckoutSessionTaxIdType::AeTrn,
            "al_tin" => PaymentPagesCheckoutSessionTaxIdType::AlTin,
            "am_tin" => PaymentPagesCheckoutSessionTaxIdType::AmTin,
            "ao_tin" => PaymentPagesCheckoutSessionTaxIdType::AoTin,
            "ar_cuit" => PaymentPagesCheckoutSessionTaxIdType::ArCuit,
            "au_abn" => PaymentPagesCheckoutSessionTaxIdType::AuAbn,
            "au_arn" => PaymentPagesCheckoutSessionTaxIdType::AuArn,
            "aw_tin" => PaymentPagesCheckoutSessionTaxIdType::AwTin,
            "az_tin" => PaymentPagesCheckoutSessionTaxIdType::AzTin,
            "ba_tin" => PaymentPagesCheckoutSessionTaxIdType::BaTin,
            "bb_tin" => PaymentPagesCheckoutSessionTaxIdType::BbTin,
            "bd_bin" => PaymentPagesCheckoutSessionTaxIdType::BdBin,
            "bf_ifu" => PaymentPagesCheckoutSessionTaxIdType::BfIfu,
            "bg_uic" => PaymentPagesCheckoutSessionTaxIdType::BgUic,
            "bh_vat" => PaymentPagesCheckoutSessionTaxIdType::BhVat,
            "bj_ifu" => PaymentPagesCheckoutSessionTaxIdType::BjIfu,
            "bo_tin" => PaymentPagesCheckoutSessionTaxIdType::BoTin,
            "br_cnpj" => PaymentPagesCheckoutSessionTaxIdType::BrCnpj,
            "br_cpf" => PaymentPagesCheckoutSessionTaxIdType::BrCpf,
            "bs_tin" => PaymentPagesCheckoutSessionTaxIdType::BsTin,
            "by_tin" => PaymentPagesCheckoutSessionTaxIdType::ByTin,
            "ca_bn" => PaymentPagesCheckoutSessionTaxIdType::CaBn,
            "ca_gst_hst" => PaymentPagesCheckoutSessionTaxIdType::CaGstHst,
            "ca_pst_bc" => PaymentPagesCheckoutSessionTaxIdType::CaPstBc,
            "ca_pst_mb" => PaymentPagesCheckoutSessionTaxIdType::CaPstMb,
            "ca_pst_sk" => PaymentPagesCheckoutSessionTaxIdType::CaPstSk,
            "ca_qst" => PaymentPagesCheckoutSessionTaxIdType::CaQst,
            "cd_nif" => PaymentPagesCheckoutSessionTaxIdType::CdNif,
            "ch_uid" => PaymentPagesCheckoutSessionTaxIdType::ChUid,
            "ch_vat" => PaymentPagesCheckoutSessionTaxIdType::ChVat,
            "cl_tin" => PaymentPagesCheckoutSessionTaxIdType::ClTin,
            "cm_niu" => PaymentPagesCheckoutSessionTaxIdType::CmNiu,
            "cn_tin" => PaymentPagesCheckoutSessionTaxIdType::CnTin,
            "co_nit" => PaymentPagesCheckoutSessionTaxIdType::CoNit,
            "cr_tin" => PaymentPagesCheckoutSessionTaxIdType::CrTin,
            "cv_nif" => PaymentPagesCheckoutSessionTaxIdType::CvNif,
            "de_stn" => PaymentPagesCheckoutSessionTaxIdType::DeStn,
            "do_rcn" => PaymentPagesCheckoutSessionTaxIdType::DoRcn,
            "ec_ruc" => PaymentPagesCheckoutSessionTaxIdType::EcRuc,
            "eg_tin" => PaymentPagesCheckoutSessionTaxIdType::EgTin,
            "es_cif" => PaymentPagesCheckoutSessionTaxIdType::EsCif,
            "et_tin" => PaymentPagesCheckoutSessionTaxIdType::EtTin,
            "eu_oss_vat" => PaymentPagesCheckoutSessionTaxIdType::EuOssVat,
            "eu_vat" => PaymentPagesCheckoutSessionTaxIdType::EuVat,
            "fo_vat" => PaymentPagesCheckoutSessionTaxIdType::FoVat,
            "gb_vat" => PaymentPagesCheckoutSessionTaxIdType::GbVat,
            "ge_vat" => PaymentPagesCheckoutSessionTaxIdType::GeVat,
            "gi_tin" => PaymentPagesCheckoutSessionTaxIdType::GiTin,
            "gn_nif" => PaymentPagesCheckoutSessionTaxIdType::GnNif,
            "hk_br" => PaymentPagesCheckoutSessionTaxIdType::HkBr,
            "hr_oib" => PaymentPagesCheckoutSessionTaxIdType::HrOib,
            "hu_tin" => PaymentPagesCheckoutSessionTaxIdType::HuTin,
            "ic_nif" => PaymentPagesCheckoutSessionTaxIdType::IcNif,
            "id_npwp" => PaymentPagesCheckoutSessionTaxIdType::IdNpwp,
            "il_vat" => PaymentPagesCheckoutSessionTaxIdType::IlVat,
            "in_gst" => PaymentPagesCheckoutSessionTaxIdType::InGst,
            "is_vat" => PaymentPagesCheckoutSessionTaxIdType::IsVat,
            "it_cf" => PaymentPagesCheckoutSessionTaxIdType::ItCf,
            "jp_cn" => PaymentPagesCheckoutSessionTaxIdType::JpCn,
            "jp_rn" => PaymentPagesCheckoutSessionTaxIdType::JpRn,
            "jp_trn" => PaymentPagesCheckoutSessionTaxIdType::JpTrn,
            "ke_pin" => PaymentPagesCheckoutSessionTaxIdType::KePin,
            "kg_tin" => PaymentPagesCheckoutSessionTaxIdType::KgTin,
            "kh_tin" => PaymentPagesCheckoutSessionTaxIdType::KhTin,
            "kr_brn" => PaymentPagesCheckoutSessionTaxIdType::KrBrn,
            "kz_bin" => PaymentPagesCheckoutSessionTaxIdType::KzBin,
            "la_tin" => PaymentPagesCheckoutSessionTaxIdType::LaTin,
            "li_uid" => PaymentPagesCheckoutSessionTaxIdType::LiUid,
            "li_vat" => PaymentPagesCheckoutSessionTaxIdType::LiVat,
            "lk_vat" => PaymentPagesCheckoutSessionTaxIdType::LkVat,
            "ma_vat" => PaymentPagesCheckoutSessionTaxIdType::MaVat,
            "md_vat" => PaymentPagesCheckoutSessionTaxIdType::MdVat,
            "me_pib" => PaymentPagesCheckoutSessionTaxIdType::MePib,
            "mk_vat" => PaymentPagesCheckoutSessionTaxIdType::MkVat,
            "mr_nif" => PaymentPagesCheckoutSessionTaxIdType::MrNif,
            "mx_rfc" => PaymentPagesCheckoutSessionTaxIdType::MxRfc,
            "my_frp" => PaymentPagesCheckoutSessionTaxIdType::MyFrp,
            "my_itn" => PaymentPagesCheckoutSessionTaxIdType::MyItn,
            "my_sst" => PaymentPagesCheckoutSessionTaxIdType::MySst,
            "ng_tin" => PaymentPagesCheckoutSessionTaxIdType::NgTin,
            "no_vat" => PaymentPagesCheckoutSessionTaxIdType::NoVat,
            "no_voec" => PaymentPagesCheckoutSessionTaxIdType::NoVoec,
            "np_pan" => PaymentPagesCheckoutSessionTaxIdType::NpPan,
            "nz_gst" => PaymentPagesCheckoutSessionTaxIdType::NzGst,
            "om_vat" => PaymentPagesCheckoutSessionTaxIdType::OmVat,
            "pe_ruc" => PaymentPagesCheckoutSessionTaxIdType::PeRuc,
            "ph_tin" => PaymentPagesCheckoutSessionTaxIdType::PhTin,
            "pl_nip" => PaymentPagesCheckoutSessionTaxIdType::PlNip,
            "py_ruc" => PaymentPagesCheckoutSessionTaxIdType::PyRuc,
            "ro_tin" => PaymentPagesCheckoutSessionTaxIdType::RoTin,
            "rs_pib" => PaymentPagesCheckoutSessionTaxIdType::RsPib,
            "ru_inn" => PaymentPagesCheckoutSessionTaxIdType::RuInn,
            "ru_kpp" => PaymentPagesCheckoutSessionTaxIdType::RuKpp,
            "sa_vat" => PaymentPagesCheckoutSessionTaxIdType::SaVat,
            "sg_gst" => PaymentPagesCheckoutSessionTaxIdType::SgGst,
            "sg_uen" => PaymentPagesCheckoutSessionTaxIdType::SgUen,
            "si_tin" => PaymentPagesCheckoutSessionTaxIdType::SiTin,
            "sn_ninea" => PaymentPagesCheckoutSessionTaxIdType::SnNinea,
            "sr_fin" => PaymentPagesCheckoutSessionTaxIdType::SrFin,
            "sv_nit" => PaymentPagesCheckoutSessionTaxIdType::SvNit,
            "th_vat" => PaymentPagesCheckoutSessionTaxIdType::ThVat,
            "tj_tin" => PaymentPagesCheckoutSessionTaxIdType::TjTin,
            "tr_tin" => PaymentPagesCheckoutSessionTaxIdType::TrTin,
            "tw_vat" => PaymentPagesCheckoutSessionTaxIdType::TwVat,
            "tz_vat" => PaymentPagesCheckoutSessionTaxIdType::TzVat,
            "ua_vat" => PaymentPagesCheckoutSessionTaxIdType::UaVat,
            "ug_tin" => PaymentPagesCheckoutSessionTaxIdType::UgTin,
            "unknown" => PaymentPagesCheckoutSessionTaxIdType::UnknownValue,
            "us_ein" => PaymentPagesCheckoutSessionTaxIdType::UsEin,
            "uy_ruc" => PaymentPagesCheckoutSessionTaxIdType::UyRuc,
            "uz_tin" => PaymentPagesCheckoutSessionTaxIdType::UzTin,
            "uz_vat" => PaymentPagesCheckoutSessionTaxIdType::UzVat,
            "ve_rif" => PaymentPagesCheckoutSessionTaxIdType::VeRif,
            "vn_tin" => PaymentPagesCheckoutSessionTaxIdType::VnTin,
            "za_vat" => PaymentPagesCheckoutSessionTaxIdType::ZaVat,
            "zm_tin" => PaymentPagesCheckoutSessionTaxIdType::ZmTin,
            "zw_tin" => PaymentPagesCheckoutSessionTaxIdType::ZwTin,
            other => PaymentPagesCheckoutSessionTaxIdType::Unknown(other.to_string()),
        })
    }
}
