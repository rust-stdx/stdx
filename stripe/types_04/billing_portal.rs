#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BillingPortalSessionObject {
    BillingPortalSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl BillingPortalSessionObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingPortalSessionObject::BillingPortalSession => "billing_portal.session",
            BillingPortalSessionObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl Default for BillingPortalSessionObject {
    fn default() -> Self {
        BillingPortalSessionObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl<'json> json::Decode<'json> for BillingPortalSessionObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingPortalSessionObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing_portal.session" => BillingPortalSessionObject::BillingPortalSession,
            other => BillingPortalSessionObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalConfigurationParamsFeaturesInvoiceHistory {
    #[doc = "Whether the feature is enabled."]
    pub enabled: bool,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalConfigurationParamsFeaturesInvoiceHistory {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalSessionParamsFlowDataAfterCompletionHostedConfirmation {
    #[doc = "A custom message to display to the customer after the flow is completed."]
    pub custom_message: Option<String>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalSessionParamsFlowDataAfterCompletionHostedConfirmation {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("custom_message", self.custom_message.as_ref());
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalSessionParamsFlowDataSubscriptionUpdate {
    #[doc = "The ID of the subscription to be updated."]
    pub subscription: String,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalSessionParamsFlowDataSubscriptionUpdate {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("subscription", &self.subscription);
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct GetBillingPortalConfigurationParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for GetBillingPortalConfigurationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct ListBillingPortalConfigurationsParams {
    pub active: Option<bool>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub is_default: Option<bool>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for ListBillingPortalConfigurationsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("is_default", self.is_default.as_ref());
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum PortalCustomerUpdateAllowedUpdatesItem {
    Address,
    Email,
    Name,
    Phone,
    Shipping,
    TaxId,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl PortalCustomerUpdateAllowedUpdatesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PortalCustomerUpdateAllowedUpdatesItem::Address => "address",
            PortalCustomerUpdateAllowedUpdatesItem::Email => "email",
            PortalCustomerUpdateAllowedUpdatesItem::Name => "name",
            PortalCustomerUpdateAllowedUpdatesItem::Phone => "phone",
            PortalCustomerUpdateAllowedUpdatesItem::Shipping => "shipping",
            PortalCustomerUpdateAllowedUpdatesItem::TaxId => "tax_id",
            PortalCustomerUpdateAllowedUpdatesItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl<'json> json::Decode<'json> for PortalCustomerUpdateAllowedUpdatesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PortalCustomerUpdateAllowedUpdatesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "address" => PortalCustomerUpdateAllowedUpdatesItem::Address,
            "email" => PortalCustomerUpdateAllowedUpdatesItem::Email,
            "name" => PortalCustomerUpdateAllowedUpdatesItem::Name,
            "phone" => PortalCustomerUpdateAllowedUpdatesItem::Phone,
            "shipping" => PortalCustomerUpdateAllowedUpdatesItem::Shipping,
            "tax_id" => PortalCustomerUpdateAllowedUpdatesItem::TaxId,
            other => PortalCustomerUpdateAllowedUpdatesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalFlowsAfterCompletionHostedConfirmation {
    #[doc = "A custom message to display to the customer after the flow is completed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalFlowsSubscriptionUpdateConfirmItem {
    #[doc = "The ID of the [subscription item](https://docs.stripe.com/api/subscriptions/object#subscription_object-items-data-id)..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The price the customer should subscribe to through this flow."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[doc = "[Quantity](https://docs.stripe.com/subscriptions/quantities) for this item that the customer should subscribe to thro..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}
