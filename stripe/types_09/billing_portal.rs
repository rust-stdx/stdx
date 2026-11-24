#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone)]
pub enum BillingPortalConfigurationApplicationExpanded {
    /// Expanded `application`.
    Application(Box<stripe_types_05::common::Application>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl<'json> json::Decode<'json> for BillingPortalConfigurationApplicationExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "application" => BillingPortalConfigurationApplicationExpanded::Application(Box::new(
                json::from_slice_owned(&json::to_vec(&value))?,
            )),
            _ => BillingPortalConfigurationApplicationExpanded::Other(value),
        })
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEnd {
    #[doc = "List of conditions."]
    pub conditions: Option<Vec<stripe_types_05::billing_portal::CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsItem>>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEnd {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.conditions.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("conditions", index, item);
            }
        }
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalSessionParamsFlowDataSubscriptionCancel {
    #[doc = "Specify a retention strategy to be used in the cancellation flow."]
    pub retention:
        Option<stripe_types_05::billing_portal::CreateBillingPortalSessionParamsFlowDataSubscriptionCancelRetention>,
    #[doc = "The ID of the subscription to be canceled."]
    pub subscription: String,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalSessionParamsFlowDataSubscriptionCancel {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("retention", self.retention.as_ref());
        form.field("subscription", &self.subscription);
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalSubscriptionCancellationReason {
    #[doc = "Whether the feature is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "The IDs of custom feedback options configured for this cancellation reason."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub feedback_options: Option<Vec<crate::Expandable<stripe_types_06::billing::BillingFeedbackOption>>>,
    #[doc = "Which cancellation reasons will be given as options to the customer."]
    #[json(default)]
    pub options: Vec<stripe_types_03::billing_portal::PortalSubscriptionCancellationReasonOptionsItem>,
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct UpdateBillingPortalConfigurationParamsFeaturesSubscriptionCancel {
    #[doc = "Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer"]
    pub cancellation_reason: Option<stripe_types_03::billing_portal::UpdateBillingPortalConfigurationParamsFeaturesSubscriptionCancelCancellationReason>,
    #[doc = "Whether the feature is enabled."]
    pub enabled: Option<bool>,
    #[doc = "Whether to cancel subscriptions immediately or at the end of the billing period."]
    pub mode: Option<stripe_types_05::billing_portal::UpdateBillingPortalConfigurationParamsFeaturesSubscriptionCancelMode>,
    #[doc = "Whether to create prorations when canceling subscriptions."]
    pub proration_behavior: Option<stripe_types_05::billing_portal::UpdateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for UpdateBillingPortalConfigurationParamsFeaturesSubscriptionCancel {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cancellation_reason", self.cancellation_reason.as_ref());
        form.field_opt("enabled", self.enabled.as_ref());
        form.field_opt("mode", self.mode.as_ref());
        form.field_opt("proration_behavior", self.proration_behavior.as_ref());
    }
}
