#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalFlowsFlowSubscriptionCancel {
    #[doc = "Specify a retention strategy to be used in the cancellation flow."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<stripe_types_06::billing_portal::PortalFlowsRetention>,
    #[doc = "The ID of the subscription to be canceled."]
    #[json(default)]
    pub subscription: String,
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct UpdateBillingPortalConfigurationParamsFeaturesSubscriptionUpdate {
    #[doc = "Determines the value to use for the billing cycle anchor on subscription updates."]
    pub billing_cycle_anchor: Option<stripe_types_06::billing_portal::UpdateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateBillingCycleAnchor>,
    #[doc = "The types of subscription updates that are supported."]
    pub default_allowed_updates: Option<json::OwnedValue>,
    #[doc = "Whether the feature is enabled."]
    pub enabled: Option<bool>,
    #[doc = "The list of up to 10 products that support subscription updates."]
    pub products: Option<json::OwnedValue>,
    #[doc = "Determines how to handle prorations resulting from subscription updates."]
    pub proration_behavior: Option<stripe_types_06::billing_portal::UpdateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateProrationBehavior>,
    #[doc = "Setting to control when an update should be scheduled at the end of the period instead of applying immediately."]
    pub schedule_at_period_end: Option<stripe_types_01::billing_portal::UpdateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEnd>,
    #[doc = "The behavior when updating a subscription that is trialing."]
    pub trial_update_behavior: Option<stripe_types_06::billing_portal::UpdateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateTrialUpdateBehavior>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for UpdateBillingPortalConfigurationParamsFeaturesSubscriptionUpdate {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_cycle_anchor", self.billing_cycle_anchor.as_ref());
        form.field_opt("enabled", self.enabled.as_ref());
        form.field_opt("proration_behavior", self.proration_behavior.as_ref());
        form.field_opt("schedule_at_period_end", self.schedule_at_period_end.as_ref());
        form.field_opt("trial_update_behavior", self.trial_update_behavior.as_ref());
    }
}
