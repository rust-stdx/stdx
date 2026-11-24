#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsAfterCompletionType {
    HostedConfirmation,
    Redirect,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsAfterCompletionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsAfterCompletionType::HostedConfirmation => "hosted_confirmation",
            CreatePaymentLinkParamsAfterCompletionType::Redirect => "redirect",
            CreatePaymentLinkParamsAfterCompletionType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsAfterCompletionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentLinkParamsAfterCompletionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "hosted_confirmation" => CreatePaymentLinkParamsAfterCompletionType::HostedConfirmation,
            "redirect" => CreatePaymentLinkParamsAfterCompletionType::Redirect,
            other => CreatePaymentLinkParamsAfterCompletionType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsAfterCompletionType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsAutomaticTaxLiabilityType {
    Account,
    SelfValue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsAutomaticTaxLiabilityType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsAutomaticTaxLiabilityType::Account => "account",
            CreatePaymentLinkParamsAutomaticTaxLiabilityType::SelfValue => "self",
            CreatePaymentLinkParamsAutomaticTaxLiabilityType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsAutomaticTaxLiabilityType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentLinkParamsAutomaticTaxLiabilityType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account" => CreatePaymentLinkParamsAutomaticTaxLiabilityType::Account,
            "self" => CreatePaymentLinkParamsAutomaticTaxLiabilityType::SelfValue,
            other => CreatePaymentLinkParamsAutomaticTaxLiabilityType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsAutomaticTaxLiabilityType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsConsentCollectionPromotions {
    Auto,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsConsentCollectionPromotions {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsConsentCollectionPromotions::Auto => "auto",
            CreatePaymentLinkParamsConsentCollectionPromotions::None => "none",
            CreatePaymentLinkParamsConsentCollectionPromotions::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsConsentCollectionPromotions {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentLinkParamsConsentCollectionPromotions::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => CreatePaymentLinkParamsConsentCollectionPromotions::Auto,
            "none" => CreatePaymentLinkParamsConsentCollectionPromotions::None,
            other => CreatePaymentLinkParamsConsentCollectionPromotions::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsConsentCollectionPromotions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsConsentCollectionTermsOfService {
    None,
    Required,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsConsentCollectionTermsOfService {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsConsentCollectionTermsOfService::None => "none",
            CreatePaymentLinkParamsConsentCollectionTermsOfService::Required => "required",
            CreatePaymentLinkParamsConsentCollectionTermsOfService::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsConsentCollectionTermsOfService {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentLinkParamsConsentCollectionTermsOfService::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => CreatePaymentLinkParamsConsentCollectionTermsOfService::None,
            "required" => CreatePaymentLinkParamsConsentCollectionTermsOfService::Required,
            other => CreatePaymentLinkParamsConsentCollectionTermsOfService::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsConsentCollectionTermsOfService {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsNameCollectionBusiness {
    #[doc = "Enable business name collection on the payment link."]
    pub enabled: bool,
    #[doc = "Whether the customer is required to provide their business name before checking out."]
    pub optional: Option<bool>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsNameCollectionBusiness {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("optional", self.optional.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsPaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsPaymentIntentDataSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsPaymentIntentDataSetupFutureUsage::OffSession => "off_session",
            CreatePaymentLinkParamsPaymentIntentDataSetupFutureUsage::OnSession => "on_session",
            CreatePaymentLinkParamsPaymentIntentDataSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsPaymentIntentDataSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentLinkParamsPaymentIntentDataSetupFutureUsage::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "off_session" => CreatePaymentLinkParamsPaymentIntentDataSetupFutureUsage::OffSession,
            "on_session" => CreatePaymentLinkParamsPaymentIntentDataSetupFutureUsage::OnSession,
            other => CreatePaymentLinkParamsPaymentIntentDataSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsPaymentIntentDataSetupFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsRestrictionsCompletedSessions {
    #[doc = "The maximum number of checkout sessions that can be completed for the `completed_sessions` restriction to be met."]
    pub limit: i64,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsRestrictionsCompletedSessions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("limit", &self.limit);
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
    Subscribe,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsSubmitType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsSubmitType::Auto => "auto",
            CreatePaymentLinkParamsSubmitType::Book => "book",
            CreatePaymentLinkParamsSubmitType::Donate => "donate",
            CreatePaymentLinkParamsSubmitType::Pay => "pay",
            CreatePaymentLinkParamsSubmitType::Subscribe => "subscribe",
            CreatePaymentLinkParamsSubmitType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsSubmitType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentLinkParamsSubmitType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => CreatePaymentLinkParamsSubmitType::Auto,
            "book" => CreatePaymentLinkParamsSubmitType::Book,
            "donate" => CreatePaymentLinkParamsSubmitType::Donate,
            "pay" => CreatePaymentLinkParamsSubmitType::Pay,
            "subscribe" => CreatePaymentLinkParamsSubmitType::Subscribe,
            other => CreatePaymentLinkParamsSubmitType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsSubmitType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct GetPaymentLinkParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for GetPaymentLinkParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct ListPaymentLinkLineItemsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for ListPaymentLinkLineItemsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentLinkPaymentMethodTypesItem {
    Affirm,
    AfterpayClearpay,
    Alipay,
    Alma,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Billie,
    Bizum,
    Blik,
    Boleto,
    Card,
    Cashapp,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    MbWay,
    Mobilepay,
    Multibanco,
    Oxxo,
    P24,
    PayByBank,
    Paynow,
    Paypal,
    Paypay,
    Payto,
    Pix,
    Promptpay,
    Satispay,
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
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl PaymentLinkPaymentMethodTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentLinkPaymentMethodTypesItem::Affirm => "affirm",
            PaymentLinkPaymentMethodTypesItem::AfterpayClearpay => "afterpay_clearpay",
            PaymentLinkPaymentMethodTypesItem::Alipay => "alipay",
            PaymentLinkPaymentMethodTypesItem::Alma => "alma",
            PaymentLinkPaymentMethodTypesItem::AuBecsDebit => "au_becs_debit",
            PaymentLinkPaymentMethodTypesItem::BacsDebit => "bacs_debit",
            PaymentLinkPaymentMethodTypesItem::Bancontact => "bancontact",
            PaymentLinkPaymentMethodTypesItem::Billie => "billie",
            PaymentLinkPaymentMethodTypesItem::Bizum => "bizum",
            PaymentLinkPaymentMethodTypesItem::Blik => "blik",
            PaymentLinkPaymentMethodTypesItem::Boleto => "boleto",
            PaymentLinkPaymentMethodTypesItem::Card => "card",
            PaymentLinkPaymentMethodTypesItem::Cashapp => "cashapp",
            PaymentLinkPaymentMethodTypesItem::Eps => "eps",
            PaymentLinkPaymentMethodTypesItem::Fpx => "fpx",
            PaymentLinkPaymentMethodTypesItem::Giropay => "giropay",
            PaymentLinkPaymentMethodTypesItem::Grabpay => "grabpay",
            PaymentLinkPaymentMethodTypesItem::Ideal => "ideal",
            PaymentLinkPaymentMethodTypesItem::Klarna => "klarna",
            PaymentLinkPaymentMethodTypesItem::Konbini => "konbini",
            PaymentLinkPaymentMethodTypesItem::Link => "link",
            PaymentLinkPaymentMethodTypesItem::MbWay => "mb_way",
            PaymentLinkPaymentMethodTypesItem::Mobilepay => "mobilepay",
            PaymentLinkPaymentMethodTypesItem::Multibanco => "multibanco",
            PaymentLinkPaymentMethodTypesItem::Oxxo => "oxxo",
            PaymentLinkPaymentMethodTypesItem::P24 => "p24",
            PaymentLinkPaymentMethodTypesItem::PayByBank => "pay_by_bank",
            PaymentLinkPaymentMethodTypesItem::Paynow => "paynow",
            PaymentLinkPaymentMethodTypesItem::Paypal => "paypal",
            PaymentLinkPaymentMethodTypesItem::Paypay => "paypay",
            PaymentLinkPaymentMethodTypesItem::Payto => "payto",
            PaymentLinkPaymentMethodTypesItem::Pix => "pix",
            PaymentLinkPaymentMethodTypesItem::Promptpay => "promptpay",
            PaymentLinkPaymentMethodTypesItem::Satispay => "satispay",
            PaymentLinkPaymentMethodTypesItem::SepaDebit => "sepa_debit",
            PaymentLinkPaymentMethodTypesItem::Sequra => "sequra",
            PaymentLinkPaymentMethodTypesItem::Sofort => "sofort",
            PaymentLinkPaymentMethodTypesItem::Sunbit => "sunbit",
            PaymentLinkPaymentMethodTypesItem::Swish => "swish",
            PaymentLinkPaymentMethodTypesItem::Twint => "twint",
            PaymentLinkPaymentMethodTypesItem::Upi => "upi",
            PaymentLinkPaymentMethodTypesItem::UsBankAccount => "us_bank_account",
            PaymentLinkPaymentMethodTypesItem::WechatPay => "wechat_pay",
            PaymentLinkPaymentMethodTypesItem::Zip => "zip",
            PaymentLinkPaymentMethodTypesItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl<'json> json::Decode<'json> for PaymentLinkPaymentMethodTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentLinkPaymentMethodTypesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "affirm" => PaymentLinkPaymentMethodTypesItem::Affirm,
            "afterpay_clearpay" => PaymentLinkPaymentMethodTypesItem::AfterpayClearpay,
            "alipay" => PaymentLinkPaymentMethodTypesItem::Alipay,
            "alma" => PaymentLinkPaymentMethodTypesItem::Alma,
            "au_becs_debit" => PaymentLinkPaymentMethodTypesItem::AuBecsDebit,
            "bacs_debit" => PaymentLinkPaymentMethodTypesItem::BacsDebit,
            "bancontact" => PaymentLinkPaymentMethodTypesItem::Bancontact,
            "billie" => PaymentLinkPaymentMethodTypesItem::Billie,
            "bizum" => PaymentLinkPaymentMethodTypesItem::Bizum,
            "blik" => PaymentLinkPaymentMethodTypesItem::Blik,
            "boleto" => PaymentLinkPaymentMethodTypesItem::Boleto,
            "card" => PaymentLinkPaymentMethodTypesItem::Card,
            "cashapp" => PaymentLinkPaymentMethodTypesItem::Cashapp,
            "eps" => PaymentLinkPaymentMethodTypesItem::Eps,
            "fpx" => PaymentLinkPaymentMethodTypesItem::Fpx,
            "giropay" => PaymentLinkPaymentMethodTypesItem::Giropay,
            "grabpay" => PaymentLinkPaymentMethodTypesItem::Grabpay,
            "ideal" => PaymentLinkPaymentMethodTypesItem::Ideal,
            "klarna" => PaymentLinkPaymentMethodTypesItem::Klarna,
            "konbini" => PaymentLinkPaymentMethodTypesItem::Konbini,
            "link" => PaymentLinkPaymentMethodTypesItem::Link,
            "mb_way" => PaymentLinkPaymentMethodTypesItem::MbWay,
            "mobilepay" => PaymentLinkPaymentMethodTypesItem::Mobilepay,
            "multibanco" => PaymentLinkPaymentMethodTypesItem::Multibanco,
            "oxxo" => PaymentLinkPaymentMethodTypesItem::Oxxo,
            "p24" => PaymentLinkPaymentMethodTypesItem::P24,
            "pay_by_bank" => PaymentLinkPaymentMethodTypesItem::PayByBank,
            "paynow" => PaymentLinkPaymentMethodTypesItem::Paynow,
            "paypal" => PaymentLinkPaymentMethodTypesItem::Paypal,
            "paypay" => PaymentLinkPaymentMethodTypesItem::Paypay,
            "payto" => PaymentLinkPaymentMethodTypesItem::Payto,
            "pix" => PaymentLinkPaymentMethodTypesItem::Pix,
            "promptpay" => PaymentLinkPaymentMethodTypesItem::Promptpay,
            "satispay" => PaymentLinkPaymentMethodTypesItem::Satispay,
            "sepa_debit" => PaymentLinkPaymentMethodTypesItem::SepaDebit,
            "sequra" => PaymentLinkPaymentMethodTypesItem::Sequra,
            "sofort" => PaymentLinkPaymentMethodTypesItem::Sofort,
            "sunbit" => PaymentLinkPaymentMethodTypesItem::Sunbit,
            "swish" => PaymentLinkPaymentMethodTypesItem::Swish,
            "twint" => PaymentLinkPaymentMethodTypesItem::Twint,
            "upi" => PaymentLinkPaymentMethodTypesItem::Upi,
            "us_bank_account" => PaymentLinkPaymentMethodTypesItem::UsBankAccount,
            "wechat_pay" => PaymentLinkPaymentMethodTypesItem::WechatPay,
            "zip" => PaymentLinkPaymentMethodTypesItem::Zip,
            other => PaymentLinkPaymentMethodTypesItem::Unknown(other.to_string()),
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
pub struct PaymentLinksResourceCompletionBehaviorRedirect {
    #[doc = "The URL the customer will be redirected to after the purchase is complete."]
    #[json(default)]
    pub url: String,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentLinksResourcePaymentIntentDataCaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl PaymentLinksResourcePaymentIntentDataCaptureMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentLinksResourcePaymentIntentDataCaptureMethod::Automatic => "automatic",
            PaymentLinksResourcePaymentIntentDataCaptureMethod::AutomaticAsync => "automatic_async",
            PaymentLinksResourcePaymentIntentDataCaptureMethod::Manual => "manual",
            PaymentLinksResourcePaymentIntentDataCaptureMethod::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl<'json> json::Decode<'json> for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentLinksResourcePaymentIntentDataCaptureMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => PaymentLinksResourcePaymentIntentDataCaptureMethod::Automatic,
            "automatic_async" => PaymentLinksResourcePaymentIntentDataCaptureMethod::AutomaticAsync,
            "manual" => PaymentLinksResourcePaymentIntentDataCaptureMethod::Manual,
            other => PaymentLinksResourcePaymentIntentDataCaptureMethod::Unknown(other.to_string()),
        })
    }
}
