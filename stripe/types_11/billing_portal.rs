#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdate {
    #[doc = "Determines the value to use for the billing cycle anchor on subscription updates."]
    pub billing_cycle_anchor: Option<stripe_types_01::billing_portal::CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateBillingCycleAnchor>,
    #[doc = "The types of subscription updates that are supported."]
    pub default_allowed_updates: Option<json::OwnedValue>,
    #[doc = "Whether the feature is enabled."]
    pub enabled: bool,
    #[doc = "The list of up to 10 products that support subscription updates."]
    pub products: Option<json::OwnedValue>,
    #[doc = "Determines how to handle prorations resulting from subscription updates."]
    pub proration_behavior: Option<stripe_types_05::billing_portal::CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateProrationBehavior>,
    #[doc = "Setting to control when an update should be scheduled at the end of the period instead of applying immediately."]
    pub schedule_at_period_end: Option<stripe_types_09::billing_portal::CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEnd>,
    #[doc = "The behavior when updating a subscription that is trialing."]
    pub trial_update_behavior: Option<stripe_types_00::billing_portal::CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateTrialUpdateBehavior>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdate {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_cycle_anchor", self.billing_cycle_anchor.as_ref());
        form.field("enabled", &self.enabled);
        form.field_opt("proration_behavior", self.proration_behavior.as_ref());
        form.field_opt("schedule_at_period_end", self.schedule_at_period_end.as_ref());
        form.field_opt("trial_update_behavior", self.trial_update_behavior.as_ref());
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone)]
pub struct CreateBillingPortalSessionParamsFlowData {
    #[doc = "Behavior after the flow is completed."]
    pub after_completion:
        Option<stripe_types_05::billing_portal::CreateBillingPortalSessionParamsFlowDataAfterCompletion>,
    #[doc = "Configuration when `flow_data.type=subscription_cancel`."]
    pub subscription_cancel:
        Option<stripe_types_09::billing_portal::CreateBillingPortalSessionParamsFlowDataSubscriptionCancel>,
    #[doc = "Configuration when `flow_data.type=subscription_update`."]
    pub subscription_update:
        Option<stripe_types_04::billing_portal::CreateBillingPortalSessionParamsFlowDataSubscriptionUpdate>,
    #[doc = "Configuration when `flow_data.type=subscription_update_confirm`."]
    pub subscription_update_confirm:
        Option<stripe_types_05::billing_portal::CreateBillingPortalSessionParamsFlowDataSubscriptionUpdateConfirm>,
    #[doc = "Type of flow that the customer will go through."]
    pub r#type: stripe_types_03::billing_portal::CreateBillingPortalSessionParamsFlowDataType,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalSessionParamsFlowData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("after_completion", self.after_completion.as_ref());
        form.field_opt("subscription_cancel", self.subscription_cancel.as_ref());
        form.field_opt("subscription_update", self.subscription_update.as_ref());
        form.field_opt("subscription_update_confirm", self.subscription_update_confirm.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalFlowsFlow {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub after_completion: Option<stripe_types_06::billing_portal::PortalFlowsFlowAfterCompletion>,
    #[doc = "Configuration when `flow.type=customer_update`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_update: Option<stripe_types_00::billing_portal::PortalFlowsFlowCustomerUpdate>,
    #[doc = "Configuration when `flow.type=subscription_cancel`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription_cancel: Option<stripe_types_08::billing_portal::PortalFlowsFlowSubscriptionCancel>,
    #[doc = "Configuration when `flow.type=subscription_update`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription_update: Option<stripe_types_03::billing_portal::PortalFlowsFlowSubscriptionUpdate>,
    #[doc = "Configuration when `flow.type=subscription_update_confirm`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription_update_confirm: Option<stripe_types_06::billing_portal::PortalFlowsFlowSubscriptionUpdateConfirm>,
    #[doc = "Type of flow that the customer will go through."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::billing_portal::PortalFlowsFlowType,
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalSubscriptionCancel {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<stripe_types_09::billing_portal::PortalSubscriptionCancellationReason>,
    #[doc = "Whether the feature is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "Whether to cancel subscriptions immediately or at the end of the billing period."]
    #[json(default)]
    pub mode: stripe_types_03::billing_portal::PortalSubscriptionCancelMode,
    #[doc = "Whether to create prorations when canceling subscriptions."]
    #[json(default)]
    pub proration_behavior: stripe_types_03::billing_portal::PortalSubscriptionCancelProrationBehavior,
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalSubscriptionUpdate {
    #[doc = "Determines the value to use for the billing cycle anchor on subscription updates."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<stripe_types_02::billing_portal::PortalSubscriptionUpdateBillingCycleAnchor>,
    #[doc = "The types of subscription updates that are supported for items listed in the `products` attribute."]
    #[json(default)]
    pub default_allowed_updates:
        Vec<stripe_types_03::billing_portal::PortalSubscriptionUpdateDefaultAllowedUpdatesItem>,
    #[doc = "Whether the feature is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "The list of up to 10 products that support subscription updates."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<stripe_types_05::billing_portal::PortalSubscriptionUpdateProduct>>,
    #[doc = "Determines how to handle prorations resulting from subscription updates."]
    #[json(default)]
    pub proration_behavior: stripe_types_03::billing_portal::PortalSubscriptionCancelProrationBehavior,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub schedule_at_period_end: Option<stripe_types_07::billing_portal::PortalResourceScheduleUpdateAtPeriodEnd>,
    #[doc = "Determines how handle updates to trialing subscriptions."]
    #[json(default)]
    pub trial_update_behavior: stripe_types_00::billing_portal::PortalSubscriptionUpdateTrialUpdateBehavior,
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct UpdateBillingPortalConfigurationParamsFeatures {
    #[doc = "Information about updating the customer details in the portal."]
    pub customer_update:
        Option<stripe_types_02::billing_portal::UpdateBillingPortalConfigurationParamsFeaturesCustomerUpdate>,
    #[doc = "Information about showing the billing history in the portal."]
    pub invoice_history:
        Option<stripe_types_06::billing_portal::UpdateBillingPortalConfigurationParamsFeaturesInvoiceHistory>,
    #[doc = "Information about updating payment methods in the portal."]
    pub payment_method_update:
        Option<stripe_types_05::billing_portal::UpdateBillingPortalConfigurationParamsFeaturesPaymentMethodUpdate>,
    #[doc = "Information about canceling subscriptions in the portal."]
    pub subscription_cancel:
        Option<stripe_types_09::billing_portal::UpdateBillingPortalConfigurationParamsFeaturesSubscriptionCancel>,
    #[doc = "Information about updating subscriptions in the portal."]
    pub subscription_update:
        Option<stripe_types_08::billing_portal::UpdateBillingPortalConfigurationParamsFeaturesSubscriptionUpdate>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for UpdateBillingPortalConfigurationParamsFeatures {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("customer_update", self.customer_update.as_ref());
        form.field_opt("invoice_history", self.invoice_history.as_ref());
        form.field_opt("payment_method_update", self.payment_method_update.as_ref());
        form.field_opt("subscription_cancel", self.subscription_cancel.as_ref());
        form.field_opt("subscription_update", self.subscription_update.as_ref());
    }
}
