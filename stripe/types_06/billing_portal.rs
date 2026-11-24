#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalFlowsFlowAfterCompletion {
    #[doc = "Configuration when `after_completion.type=hosted_confirmation`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation: Option<stripe_types_04::billing_portal::PortalFlowsAfterCompletionHostedConfirmation>,
    #[doc = "Configuration when `after_completion.type=redirect`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub redirect: Option<stripe_types_00::billing_portal::PortalFlowsAfterCompletionRedirect>,
    #[doc = "The specified type of behavior after the flow is completed."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::billing_portal::PortalFlowsFlowAfterCompletionType,
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalFlowsFlowSubscriptionUpdateConfirm {
    #[doc = "The coupon or promotion code to apply to this subscription update."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<stripe_types_00::billing_portal::PortalFlowsSubscriptionUpdateConfirmDiscount>>,
    #[doc = "The [subscription item](https://docs.stripe.com/api/subscription_items) to be updated through this flow."]
    #[json(default)]
    pub items: Vec<stripe_types_04::billing_portal::PortalFlowsSubscriptionUpdateConfirmItem>,
    #[doc = "The ID of the subscription to be updated."]
    #[json(default)]
    pub subscription: String,
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalFlowsRetention {
    #[doc = "Configuration when `retention.type=coupon_offer`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub coupon_offer: Option<stripe_types_03::billing_portal::PortalFlowsCouponOffer>,
    #[doc = "Type of retention strategy that will be used."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::billing_portal::PortalFlowsRetentionType,
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalResourceScheduleUpdateAtPeriodEndCondition {
    #[doc = "The type of condition."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::billing_portal::PortalResourceScheduleUpdateAtPeriodEndConditionType,
}

#[cfg(feature = "billing_portal")]
pub type UpdateBillingPortalConfigurationParamsFeaturesInvoiceHistory =
    stripe_types_04::billing_portal::CreateBillingPortalConfigurationParamsFeaturesInvoiceHistory;

#[cfg(feature = "billing_portal")]
pub type UpdateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateBillingCycleAnchor =
    stripe_types_01::billing_portal::CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateBillingCycleAnchor;

#[cfg(feature = "billing_portal")]
pub type UpdateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateProrationBehavior =
    stripe_types_03::billing_portal::CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior;

#[cfg(feature = "billing_portal")]
pub type UpdateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateTrialUpdateBehavior = stripe_types_00::billing_portal::CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateTrialUpdateBehavior;

#[cfg(feature = "billing_portal")]
pub type UpdateBillingPortalConfigurationParamsLoginPage =
    stripe_types_04::billing_portal::CreateBillingPortalConfigurationParamsFeaturesInvoiceHistory;
