#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalConfigurationParamsFeaturesPaymentMethodUpdate {
    #[doc = "Whether the feature is enabled."]
    pub enabled: bool,
    #[doc = "The [Payment Method Configuration](/api/payment_method_configurations) to use for this portal session."]
    pub payment_method_configuration: Option<json::OwnedValue>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalConfigurationParamsFeaturesPaymentMethodUpdate {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelCancellationReason {
    #[doc = "Whether the feature is enabled."]
    pub enabled: bool,
    #[doc = "The IDs of custom feedback options to use for this cancellation reason."]
    pub feedback_options: Option<json::OwnedValue>,
    #[doc = "Which cancellation reasons will be given as options to the customer."]
    pub options: json::OwnedValue,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelCancellationReason {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelMode {
    AtPeriodEnd,
    Immediately,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing_portal")]
impl CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelMode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelMode::AtPeriodEnd => "at_period_end",
            CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelMode::Immediately => "immediately",
            CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelMode::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing_portal")]
impl<'json> json::Decode<'json> for CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelMode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelMode::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "at_period_end" => CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelMode::AtPeriodEnd,
            "immediately" => CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelMode::Immediately,
            other => CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelMode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelMode {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing_portal")]
impl CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior::AlwaysInvoice => {
                "always_invoice"
            }
            CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior::CreateProrations => {
                "create_prorations"
            }
            CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior::None => "none",
            CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing_portal")]
impl<'json> json::Decode<'json> for CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always_invoice" => {
                CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior::AlwaysInvoice
            }
            "create_prorations" => {
                CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior::CreateProrations
            }
            "none" => CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior::None,
            other => CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateBillingPortalSessionParamsFlowDataType {
    CustomerUpdate,
    PaymentMethodUpdate,
    SubscriptionCancel,
    SubscriptionUpdate,
    SubscriptionUpdateConfirm,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing_portal")]
impl CreateBillingPortalSessionParamsFlowDataType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateBillingPortalSessionParamsFlowDataType::CustomerUpdate => "customer_update",
            CreateBillingPortalSessionParamsFlowDataType::PaymentMethodUpdate => "payment_method_update",
            CreateBillingPortalSessionParamsFlowDataType::SubscriptionCancel => "subscription_cancel",
            CreateBillingPortalSessionParamsFlowDataType::SubscriptionUpdate => "subscription_update",
            CreateBillingPortalSessionParamsFlowDataType::SubscriptionUpdateConfirm => "subscription_update_confirm",
            CreateBillingPortalSessionParamsFlowDataType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing_portal")]
impl<'json> json::Decode<'json> for CreateBillingPortalSessionParamsFlowDataType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateBillingPortalSessionParamsFlowDataType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer_update" => CreateBillingPortalSessionParamsFlowDataType::CustomerUpdate,
            "payment_method_update" => CreateBillingPortalSessionParamsFlowDataType::PaymentMethodUpdate,
            "subscription_cancel" => CreateBillingPortalSessionParamsFlowDataType::SubscriptionCancel,
            "subscription_update" => CreateBillingPortalSessionParamsFlowDataType::SubscriptionUpdate,
            "subscription_update_confirm" => CreateBillingPortalSessionParamsFlowDataType::SubscriptionUpdateConfirm,
            other => CreateBillingPortalSessionParamsFlowDataType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalSessionParamsFlowDataType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalFlowsCouponOffer {
    #[doc = "The ID of the coupon to be offered."]
    #[json(default)]
    pub coupon: String,
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalFlowsFlowSubscriptionUpdate {
    #[doc = "The ID of the subscription to be updated."]
    #[json(default)]
    pub subscription: String,
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum PortalFlowsFlowType {
    CustomerUpdate,
    PaymentMethodUpdate,
    SubscriptionCancel,
    SubscriptionUpdate,
    SubscriptionUpdateConfirm,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl PortalFlowsFlowType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PortalFlowsFlowType::CustomerUpdate => "customer_update",
            PortalFlowsFlowType::PaymentMethodUpdate => "payment_method_update",
            PortalFlowsFlowType::SubscriptionCancel => "subscription_cancel",
            PortalFlowsFlowType::SubscriptionUpdate => "subscription_update",
            PortalFlowsFlowType::SubscriptionUpdateConfirm => "subscription_update_confirm",
            PortalFlowsFlowType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl Default for PortalFlowsFlowType {
    fn default() -> Self {
        PortalFlowsFlowType::Unknown(String::new())
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl<'json> json::Decode<'json> for PortalFlowsFlowType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PortalFlowsFlowType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer_update" => PortalFlowsFlowType::CustomerUpdate,
            "payment_method_update" => PortalFlowsFlowType::PaymentMethodUpdate,
            "subscription_cancel" => PortalFlowsFlowType::SubscriptionCancel,
            "subscription_update" => PortalFlowsFlowType::SubscriptionUpdate,
            "subscription_update_confirm" => PortalFlowsFlowType::SubscriptionUpdateConfirm,
            other => PortalFlowsFlowType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalInvoiceList {
    #[doc = "Whether the feature is enabled."]
    #[json(default)]
    pub enabled: bool,
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum PortalSubscriptionCancelMode {
    AtPeriodEnd,
    Immediately,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl PortalSubscriptionCancelMode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PortalSubscriptionCancelMode::AtPeriodEnd => "at_period_end",
            PortalSubscriptionCancelMode::Immediately => "immediately",
            PortalSubscriptionCancelMode::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl Default for PortalSubscriptionCancelMode {
    fn default() -> Self {
        PortalSubscriptionCancelMode::Unknown(String::new())
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl<'json> json::Decode<'json> for PortalSubscriptionCancelMode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PortalSubscriptionCancelMode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "at_period_end" => PortalSubscriptionCancelMode::AtPeriodEnd,
            "immediately" => PortalSubscriptionCancelMode::Immediately,
            other => PortalSubscriptionCancelMode::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum PortalSubscriptionCancelProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl PortalSubscriptionCancelProrationBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PortalSubscriptionCancelProrationBehavior::AlwaysInvoice => "always_invoice",
            PortalSubscriptionCancelProrationBehavior::CreateProrations => "create_prorations",
            PortalSubscriptionCancelProrationBehavior::None => "none",
            PortalSubscriptionCancelProrationBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl Default for PortalSubscriptionCancelProrationBehavior {
    fn default() -> Self {
        PortalSubscriptionCancelProrationBehavior::Unknown(String::new())
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl<'json> json::Decode<'json> for PortalSubscriptionCancelProrationBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PortalSubscriptionCancelProrationBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always_invoice" => PortalSubscriptionCancelProrationBehavior::AlwaysInvoice,
            "create_prorations" => PortalSubscriptionCancelProrationBehavior::CreateProrations,
            "none" => PortalSubscriptionCancelProrationBehavior::None,
            other => PortalSubscriptionCancelProrationBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum PortalSubscriptionCancellationReasonOptionsItem {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl PortalSubscriptionCancellationReasonOptionsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PortalSubscriptionCancellationReasonOptionsItem::CustomerService => "customer_service",
            PortalSubscriptionCancellationReasonOptionsItem::LowQuality => "low_quality",
            PortalSubscriptionCancellationReasonOptionsItem::MissingFeatures => "missing_features",
            PortalSubscriptionCancellationReasonOptionsItem::Other => "other",
            PortalSubscriptionCancellationReasonOptionsItem::SwitchedService => "switched_service",
            PortalSubscriptionCancellationReasonOptionsItem::TooComplex => "too_complex",
            PortalSubscriptionCancellationReasonOptionsItem::TooExpensive => "too_expensive",
            PortalSubscriptionCancellationReasonOptionsItem::Unused => "unused",
            PortalSubscriptionCancellationReasonOptionsItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl<'json> json::Decode<'json> for PortalSubscriptionCancellationReasonOptionsItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PortalSubscriptionCancellationReasonOptionsItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer_service" => PortalSubscriptionCancellationReasonOptionsItem::CustomerService,
            "low_quality" => PortalSubscriptionCancellationReasonOptionsItem::LowQuality,
            "missing_features" => PortalSubscriptionCancellationReasonOptionsItem::MissingFeatures,
            "other" => PortalSubscriptionCancellationReasonOptionsItem::Other,
            "switched_service" => PortalSubscriptionCancellationReasonOptionsItem::SwitchedService,
            "too_complex" => PortalSubscriptionCancellationReasonOptionsItem::TooComplex,
            "too_expensive" => PortalSubscriptionCancellationReasonOptionsItem::TooExpensive,
            "unused" => PortalSubscriptionCancellationReasonOptionsItem::Unused,
            other => PortalSubscriptionCancellationReasonOptionsItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum PortalSubscriptionUpdateDefaultAllowedUpdatesItem {
    Price,
    PromotionCode,
    Quantity,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl PortalSubscriptionUpdateDefaultAllowedUpdatesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PortalSubscriptionUpdateDefaultAllowedUpdatesItem::Price => "price",
            PortalSubscriptionUpdateDefaultAllowedUpdatesItem::PromotionCode => "promotion_code",
            PortalSubscriptionUpdateDefaultAllowedUpdatesItem::Quantity => "quantity",
            PortalSubscriptionUpdateDefaultAllowedUpdatesItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl<'json> json::Decode<'json> for PortalSubscriptionUpdateDefaultAllowedUpdatesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PortalSubscriptionUpdateDefaultAllowedUpdatesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "price" => PortalSubscriptionUpdateDefaultAllowedUpdatesItem::Price,
            "promotion_code" => PortalSubscriptionUpdateDefaultAllowedUpdatesItem::PromotionCode,
            "quantity" => PortalSubscriptionUpdateDefaultAllowedUpdatesItem::Quantity,
            other => PortalSubscriptionUpdateDefaultAllowedUpdatesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalSubscriptionUpdateProductAdjustableQuantity {
    #[doc = "If true, the quantity can be adjusted to any non-negative integer."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "The maximum quantity that can be set for the product."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[doc = "The minimum quantity that can be set for the product."]
    #[json(default)]
    pub minimum: i64,
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct UpdateBillingPortalConfigurationParamsBusinessProfile {
    #[doc = "The messaging shown to customers in the portal."]
    pub headline: Option<json::OwnedValue>,
    #[doc = "A link to the business’s publicly available privacy policy."]
    pub privacy_policy_url: Option<json::OwnedValue>,
    #[doc = "A link to the business’s publicly available terms of service."]
    pub terms_of_service_url: Option<json::OwnedValue>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for UpdateBillingPortalConfigurationParamsBusinessProfile {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct UpdateBillingPortalConfigurationParamsFeaturesSubscriptionCancelCancellationReason {
    #[doc = "Whether the feature is enabled."]
    pub enabled: bool,
    #[doc = "The IDs of custom feedback options to use for this cancellation reason."]
    pub feedback_options: Option<json::OwnedValue>,
    #[doc = "Which cancellation reasons will be given as options to the customer."]
    pub options: Option<json::OwnedValue>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for UpdateBillingPortalConfigurationParamsFeaturesSubscriptionCancelCancellationReason {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
    }
}
