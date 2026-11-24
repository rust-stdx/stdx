#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsAfterCompletionRedirect {
    #[doc = "The URL the customer will be redirected to after the purchase is complete."]
    pub url: String,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsAfterCompletionRedirect {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("url", &self.url);
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsBillingAddressCollection {
    Auto,
    Required,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsBillingAddressCollection {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsBillingAddressCollection::Auto => "auto",
            CreatePaymentLinkParamsBillingAddressCollection::Required => "required",
            CreatePaymentLinkParamsBillingAddressCollection::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsBillingAddressCollection {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentLinkParamsBillingAddressCollection::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => CreatePaymentLinkParamsBillingAddressCollection::Auto,
            "required" => CreatePaymentLinkParamsBillingAddressCollection::Required,
            other => CreatePaymentLinkParamsBillingAddressCollection::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsBillingAddressCollection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreementPosition {
    Auto,
    Hidden,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreementPosition {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreementPosition::Auto => "auto",
            CreatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreementPosition::Hidden => "hidden",
            CreatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreementPosition::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreementPosition {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreementPosition::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => CreatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreementPosition::Auto,
            "hidden" => CreatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreementPosition::Hidden,
            other => {
                CreatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreementPosition::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreementPosition {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsCustomFieldsItemType {
    Dropdown,
    Numeric,
    Text,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsCustomFieldsItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsCustomFieldsItemType::Dropdown => "dropdown",
            CreatePaymentLinkParamsCustomFieldsItemType::Numeric => "numeric",
            CreatePaymentLinkParamsCustomFieldsItemType::Text => "text",
            CreatePaymentLinkParamsCustomFieldsItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsCustomFieldsItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentLinkParamsCustomFieldsItemType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "dropdown" => CreatePaymentLinkParamsCustomFieldsItemType::Dropdown,
            "numeric" => CreatePaymentLinkParamsCustomFieldsItemType::Numeric,
            "text" => CreatePaymentLinkParamsCustomFieldsItemType::Text,
            other => CreatePaymentLinkParamsCustomFieldsItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsCustomFieldsItemType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsTaxIdCollectionRequired {
    IfSupported,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsTaxIdCollectionRequired {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsTaxIdCollectionRequired::IfSupported => "if_supported",
            CreatePaymentLinkParamsTaxIdCollectionRequired::Never => "never",
            CreatePaymentLinkParamsTaxIdCollectionRequired::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsTaxIdCollectionRequired {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentLinkParamsTaxIdCollectionRequired::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "if_supported" => CreatePaymentLinkParamsTaxIdCollectionRequired::IfSupported,
            "never" => CreatePaymentLinkParamsTaxIdCollectionRequired::Never,
            other => CreatePaymentLinkParamsTaxIdCollectionRequired::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsTaxIdCollectionRequired {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsTransferData {
    #[doc = "The amount that will be transferred automatically when a charge succeeds."]
    pub amount: Option<i64>,
    #[doc = "If specified, successful charges will be attributed to the destination account for tax reporting, and the funds from ..."]
    pub destination: String,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsTransferData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field("destination", &self.destination);
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
pub enum PaymentLinkObject {
    PaymentLink,
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
impl PaymentLinkObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentLinkObject::PaymentLink => "payment_link",
            PaymentLinkObject::Unknown(value) => value,
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
impl Default for PaymentLinkObject {
    fn default() -> Self {
        PaymentLinkObject::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl<'json> json::Decode<'json> for PaymentLinkObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentLinkObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "payment_link" => PaymentLinkObject::PaymentLink,
            other => PaymentLinkObject::Unknown(other.to_string()),
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
pub struct PaymentLinksResourceCustomFieldsNumeric {
    #[doc = "The value that pre-fills the field on the payment page."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "The maximum character length constraint for the customer's input."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    #[doc = "The minimum character length requirement for the customer's input."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentLinksResourceCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
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
impl PaymentLinksResourceCustomFieldsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentLinksResourceCustomFieldsType::Dropdown => "dropdown",
            PaymentLinksResourceCustomFieldsType::Numeric => "numeric",
            PaymentLinksResourceCustomFieldsType::Text => "text",
            PaymentLinksResourceCustomFieldsType::Unknown(value) => value,
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
impl Default for PaymentLinksResourceCustomFieldsType {
    fn default() -> Self {
        PaymentLinksResourceCustomFieldsType::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl<'json> json::Decode<'json> for PaymentLinksResourceCustomFieldsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentLinksResourceCustomFieldsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "dropdown" => PaymentLinksResourceCustomFieldsType::Dropdown,
            "numeric" => PaymentLinksResourceCustomFieldsType::Numeric,
            "text" => PaymentLinksResourceCustomFieldsType::Text,
            other => PaymentLinksResourceCustomFieldsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum UpdatePaymentLinkParamsPaymentIntentDataSetupFutureUsage {
    Unnamed,
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl UpdatePaymentLinkParamsPaymentIntentDataSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            UpdatePaymentLinkParamsPaymentIntentDataSetupFutureUsage::Unnamed => "",
            UpdatePaymentLinkParamsPaymentIntentDataSetupFutureUsage::OffSession => "off_session",
            UpdatePaymentLinkParamsPaymentIntentDataSetupFutureUsage::OnSession => "on_session",
            UpdatePaymentLinkParamsPaymentIntentDataSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for UpdatePaymentLinkParamsPaymentIntentDataSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(UpdatePaymentLinkParamsPaymentIntentDataSetupFutureUsage::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => UpdatePaymentLinkParamsPaymentIntentDataSetupFutureUsage::Unnamed,
            "off_session" => UpdatePaymentLinkParamsPaymentIntentDataSetupFutureUsage::OffSession,
            "on_session" => UpdatePaymentLinkParamsPaymentIntentDataSetupFutureUsage::OnSession,
            other => UpdatePaymentLinkParamsPaymentIntentDataSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsPaymentIntentDataSetupFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParamsTransferData {
    #[doc = "The amount that will be transferred automatically when a charge succeeds."]
    pub amount: Option<json::OwnedValue>,
    #[doc = "If specified, successful charges will be attributed to the destination  account for tax reporting, and the funds from..."]
    pub destination: String,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsTransferData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("destination", &self.destination);
    }
}
