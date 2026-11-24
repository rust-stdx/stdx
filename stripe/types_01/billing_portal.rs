#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BillingPortalConfigurationObject {
    BillingPortalConfiguration,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl BillingPortalConfigurationObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingPortalConfigurationObject::BillingPortalConfiguration => "billing_portal.configuration",
            BillingPortalConfigurationObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl Default for BillingPortalConfigurationObject {
    fn default() -> Self {
        BillingPortalConfigurationObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl<'json> json::Decode<'json> for BillingPortalConfigurationObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingPortalConfigurationObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing_portal.configuration" => BillingPortalConfigurationObject::BillingPortalConfiguration,
            other => BillingPortalConfigurationObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateBillingCycleAnchor {
    Now,
    Unchanged,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing_portal")]
impl CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateBillingCycleAnchor {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateBillingCycleAnchor::Now => "now",
            CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateBillingCycleAnchor::Unchanged => {
                "unchanged"
            }
            CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateBillingCycleAnchor::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing_portal")]
impl<'json> json::Decode<'json> for CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateBillingCycleAnchor {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateBillingCycleAnchor::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "now" => CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateBillingCycleAnchor::Now,
            "unchanged" => {
                CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateBillingCycleAnchor::Unchanged
            }
            other => CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateBillingCycleAnchor::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateBillingCycleAnchor {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalSessionParamsFlowDataSubscriptionUpdateConfirmItemsItem {
    #[doc = "The ID of the [subscription item](https://docs.stripe.com/api/subscriptions/object#subscription_object-items-data-id)..."]
    pub id: String,
    #[doc = "The price the customer should subscribe to through this flow."]
    pub price: Option<String>,
    #[doc = "[Quantity](https://docs.stripe.com/subscriptions/quantities) for this item that the customer should subscribe to thro..."]
    pub quantity: Option<i64>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalSessionParamsFlowDataSubscriptionUpdateConfirmItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("id", &self.id);
        form.field_opt("price", self.price.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum PortalFlowsFlowAfterCompletionType {
    HostedConfirmation,
    PortalHomepage,
    Redirect,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl PortalFlowsFlowAfterCompletionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PortalFlowsFlowAfterCompletionType::HostedConfirmation => "hosted_confirmation",
            PortalFlowsFlowAfterCompletionType::PortalHomepage => "portal_homepage",
            PortalFlowsFlowAfterCompletionType::Redirect => "redirect",
            PortalFlowsFlowAfterCompletionType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl Default for PortalFlowsFlowAfterCompletionType {
    fn default() -> Self {
        PortalFlowsFlowAfterCompletionType::Unknown(String::new())
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl<'json> json::Decode<'json> for PortalFlowsFlowAfterCompletionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PortalFlowsFlowAfterCompletionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "hosted_confirmation" => PortalFlowsFlowAfterCompletionType::HostedConfirmation,
            "portal_homepage" => PortalFlowsFlowAfterCompletionType::PortalHomepage,
            "redirect" => PortalFlowsFlowAfterCompletionType::Redirect,
            other => PortalFlowsFlowAfterCompletionType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum PortalFlowsRetentionType {
    CouponOffer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl PortalFlowsRetentionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PortalFlowsRetentionType::CouponOffer => "coupon_offer",
            PortalFlowsRetentionType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl Default for PortalFlowsRetentionType {
    fn default() -> Self {
        PortalFlowsRetentionType::Unknown(String::new())
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl<'json> json::Decode<'json> for PortalFlowsRetentionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PortalFlowsRetentionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "coupon_offer" => PortalFlowsRetentionType::CouponOffer,
            other => PortalFlowsRetentionType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalPaymentMethodUpdate {
    #[doc = "Whether the feature is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "The [Payment Method Configuration](/api/payment_method_configurations) to use for this portal session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<String>,
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct UpdateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEnd {
    #[doc = "List of conditions."]
    pub conditions: Option<json::OwnedValue>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for UpdateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEnd {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}
