#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsAfterCompletionHostedConfirmation {
    #[doc = "A custom message to display to the customer after the purchase is complete."]
    pub custom_message: Option<String>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsAfterCompletionHostedConfirmation {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("custom_message", self.custom_message.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsCustomFieldsItemNumeric {
    #[doc = "The value that pre-fills the field on the payment page."]
    pub default_value: Option<String>,
    #[doc = "The maximum character length constraint for the customer's input."]
    pub maximum_length: Option<i64>,
    #[doc = "The minimum character length requirement for the customer's input."]
    pub minimum_length: Option<i64>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsCustomFieldsItemNumeric {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("default_value", self.default_value.as_ref());
        form.field_opt("maximum_length", self.maximum_length.as_ref());
        form.field_opt("minimum_length", self.minimum_length.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsCustomerCreation {
    Always,
    IfRequired,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsCustomerCreation {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsCustomerCreation::Always => "always",
            CreatePaymentLinkParamsCustomerCreation::IfRequired => "if_required",
            CreatePaymentLinkParamsCustomerCreation::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsCustomerCreation {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentLinkParamsCustomerCreation::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CreatePaymentLinkParamsCustomerCreation::Always,
            "if_required" => CreatePaymentLinkParamsCustomerCreation::IfRequired,
            other => CreatePaymentLinkParamsCustomerCreation::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsCustomerCreation {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsLineItemsItemPriceDataProductDataTaxDetails {
    #[doc = "A tax location ID."]
    pub performance_location: Option<String>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<json::OwnedValue>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsLineItemsItemPriceDataProductDataTaxDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("performance_location", self.performance_location.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsLineItemsItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsLineItemsItemPriceDataTaxBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsLineItemsItemPriceDataTaxBehavior::Exclusive => "exclusive",
            CreatePaymentLinkParamsLineItemsItemPriceDataTaxBehavior::Inclusive => "inclusive",
            CreatePaymentLinkParamsLineItemsItemPriceDataTaxBehavior::Unspecified => "unspecified",
            CreatePaymentLinkParamsLineItemsItemPriceDataTaxBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsLineItemsItemPriceDataTaxBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentLinkParamsLineItemsItemPriceDataTaxBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclusive" => CreatePaymentLinkParamsLineItemsItemPriceDataTaxBehavior::Exclusive,
            "inclusive" => CreatePaymentLinkParamsLineItemsItemPriceDataTaxBehavior::Inclusive,
            "unspecified" => CreatePaymentLinkParamsLineItemsItemPriceDataTaxBehavior::Unspecified,
            other => CreatePaymentLinkParamsLineItemsItemPriceDataTaxBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsLineItemsItemPriceDataTaxBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsPaymentIntentDataCaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsPaymentIntentDataCaptureMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsPaymentIntentDataCaptureMethod::Automatic => "automatic",
            CreatePaymentLinkParamsPaymentIntentDataCaptureMethod::AutomaticAsync => "automatic_async",
            CreatePaymentLinkParamsPaymentIntentDataCaptureMethod::Manual => "manual",
            CreatePaymentLinkParamsPaymentIntentDataCaptureMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsPaymentIntentDataCaptureMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentLinkParamsPaymentIntentDataCaptureMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => CreatePaymentLinkParamsPaymentIntentDataCaptureMethod::Automatic,
            "automatic_async" => CreatePaymentLinkParamsPaymentIntentDataCaptureMethod::AutomaticAsync,
            "manual" => CreatePaymentLinkParamsPaymentIntentDataCaptureMethod::Manual,
            other => CreatePaymentLinkParamsPaymentIntentDataCaptureMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsPaymentIntentDataCaptureMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
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
pub enum PaymentLinkBillingAddressCollection {
    Auto,
    Required,
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
impl PaymentLinkBillingAddressCollection {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentLinkBillingAddressCollection::Auto => "auto",
            PaymentLinkBillingAddressCollection::Required => "required",
            PaymentLinkBillingAddressCollection::Unknown(value) => value,
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
impl Default for PaymentLinkBillingAddressCollection {
    fn default() -> Self {
        PaymentLinkBillingAddressCollection::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl<'json> json::Decode<'json> for PaymentLinkBillingAddressCollection {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentLinkBillingAddressCollection::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => PaymentLinkBillingAddressCollection::Auto,
            "required" => PaymentLinkBillingAddressCollection::Required,
            other => PaymentLinkBillingAddressCollection::Unknown(other.to_string()),
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
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentLinksResourceAfterCompletionType {
    HostedConfirmation,
    Redirect,
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
impl PaymentLinksResourceAfterCompletionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentLinksResourceAfterCompletionType::HostedConfirmation => "hosted_confirmation",
            PaymentLinksResourceAfterCompletionType::Redirect => "redirect",
            PaymentLinksResourceAfterCompletionType::Unknown(value) => value,
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
impl Default for PaymentLinksResourceAfterCompletionType {
    fn default() -> Self {
        PaymentLinksResourceAfterCompletionType::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl<'json> json::Decode<'json> for PaymentLinksResourceAfterCompletionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentLinksResourceAfterCompletionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "hosted_confirmation" => PaymentLinksResourceAfterCompletionType::HostedConfirmation,
            "redirect" => PaymentLinksResourceAfterCompletionType::Redirect,
            other => PaymentLinksResourceAfterCompletionType::Unknown(other.to_string()),
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
pub struct PaymentLinksResourceCompletionBehaviorConfirmationPage {
    #[doc = "The custom message that is displayed to the customer after the purchase is complete."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentLinksResourceConsentCollectionPromotions {
    Auto,
    None,
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
impl PaymentLinksResourceConsentCollectionPromotions {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentLinksResourceConsentCollectionPromotions::Auto => "auto",
            PaymentLinksResourceConsentCollectionPromotions::None => "none",
            PaymentLinksResourceConsentCollectionPromotions::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentLinksResourceConsentCollectionPromotions {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentLinksResourceConsentCollectionPromotions::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => PaymentLinksResourceConsentCollectionPromotions::Auto,
            "none" => PaymentLinksResourceConsentCollectionPromotions::None,
            other => PaymentLinksResourceConsentCollectionPromotions::Unknown(other.to_string()),
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
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentLinksResourceConsentCollectionTermsOfService {
    None,
    Required,
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
impl PaymentLinksResourceConsentCollectionTermsOfService {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentLinksResourceConsentCollectionTermsOfService::None => "none",
            PaymentLinksResourceConsentCollectionTermsOfService::Required => "required",
            PaymentLinksResourceConsentCollectionTermsOfService::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentLinksResourceConsentCollectionTermsOfService {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentLinksResourceConsentCollectionTermsOfService::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => PaymentLinksResourceConsentCollectionTermsOfService::None,
            "required" => PaymentLinksResourceConsentCollectionTermsOfService::Required,
            other => PaymentLinksResourceConsentCollectionTermsOfService::Unknown(other.to_string()),
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
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentLinksResourceCustomFieldsLabelType {
    Custom,
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
impl PaymentLinksResourceCustomFieldsLabelType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentLinksResourceCustomFieldsLabelType::Custom => "custom",
            PaymentLinksResourceCustomFieldsLabelType::Unknown(value) => value,
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
impl Default for PaymentLinksResourceCustomFieldsLabelType {
    fn default() -> Self {
        PaymentLinksResourceCustomFieldsLabelType::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl<'json> json::Decode<'json> for PaymentLinksResourceCustomFieldsLabelType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentLinksResourceCustomFieldsLabelType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "custom" => PaymentLinksResourceCustomFieldsLabelType::Custom,
            other => PaymentLinksResourceCustomFieldsLabelType::Unknown(other.to_string()),
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
pub struct PaymentLinksResourceCustomFieldsText {
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentLinksResourceOptionalItemAdjustableQuantity {
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
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentLinksResourcePhoneNumberCollection {
    #[doc = "If `true`, a phone number will be collected during checkout."]
    #[json(default)]
    pub enabled: bool,
}
