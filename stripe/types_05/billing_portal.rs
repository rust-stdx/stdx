#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancel {
    #[doc = "Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer"]
    pub cancellation_reason: Option<stripe_types_03::billing_portal::CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelCancellationReason>,
    #[doc = "Whether the feature is enabled."]
    pub enabled: bool,
    #[doc = "Whether to cancel subscriptions immediately or at the end of the billing period."]
    pub mode: Option<stripe_types_03::billing_portal::CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelMode>,
    #[doc = "Whether to create prorations when canceling subscriptions."]
    pub proration_behavior: Option<stripe_types_03::billing_portal::CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancel {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cancellation_reason", self.cancellation_reason.as_ref());
        form.field("enabled", &self.enabled);
        form.field_opt("mode", self.mode.as_ref());
        form.field_opt("proration_behavior", self.proration_behavior.as_ref());
    }
}

#[cfg(feature = "billing_portal")]
pub type CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateProrationBehavior =
    stripe_types_03::billing_portal::CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior;

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone)]
pub struct CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsItem {
    #[doc = "The type of condition."]
    pub r#type: stripe_types_02::billing_portal::CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsItemType,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode
    for CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsItem
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "billing_portal")]
pub type CreateBillingPortalConfigurationParamsLoginPage =
    stripe_types_04::billing_portal::CreateBillingPortalConfigurationParamsFeaturesInvoiceHistory;

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone)]
pub struct CreateBillingPortalSessionParamsFlowDataAfterCompletion {
    #[doc = "Configuration when `after_completion.type=hosted_confirmation`."]
    pub hosted_confirmation: Option<
        stripe_types_04::billing_portal::CreateBillingPortalSessionParamsFlowDataAfterCompletionHostedConfirmation,
    >,
    #[doc = "Configuration when `after_completion.type=redirect`."]
    pub redirect:
        Option<stripe_types_02::billing_portal::CreateBillingPortalSessionParamsFlowDataAfterCompletionRedirect>,
    #[doc = "The specified behavior after the flow is completed."]
    pub r#type: stripe_types_00::billing_portal::CreateBillingPortalSessionParamsFlowDataAfterCompletionType,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalSessionParamsFlowDataAfterCompletion {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("hosted_confirmation", self.hosted_confirmation.as_ref());
        form.field_opt("redirect", self.redirect.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone)]
pub struct CreateBillingPortalSessionParamsFlowDataSubscriptionCancelRetention {
    #[doc = "Configuration when `retention.type=coupon_offer`."]
    pub coupon_offer:
        stripe_types_02::billing_portal::CreateBillingPortalSessionParamsFlowDataSubscriptionCancelRetentionCouponOffer,
    #[doc = "Type of retention strategy to use with the customer."]
    pub r#type:
        stripe_types_00::billing_portal::CreateBillingPortalSessionParamsFlowDataSubscriptionCancelRetentionType,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalSessionParamsFlowDataSubscriptionCancelRetention {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("coupon_offer", &self.coupon_offer);
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalSessionParamsFlowDataSubscriptionUpdateConfirm {
    #[doc = "The coupon or promotion code to apply to this subscription update."]
    pub discounts: Option<Vec<stripe_types_00::billing_portal::CreateBillingPortalSessionParamsFlowDataSubscriptionUpdateConfirmDiscountsItem>>,
    #[doc = "The [subscription item](https://docs.stripe.com/api/subscription_items) to be updated through this flow."]
    pub items: Vec<stripe_types_01::billing_portal::CreateBillingPortalSessionParamsFlowDataSubscriptionUpdateConfirmItemsItem>,
    #[doc = "The ID of the subscription to be updated."]
    pub subscription: String,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalSessionParamsFlowDataSubscriptionUpdateConfirm {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.discounts.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("discounts", index, item);
            }
        }
        for (index, item) in self.items.iter().enumerate() {
            form.field_indexed("items", index, item);
        }
        form.field("subscription", &self.subscription);
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalCustomerUpdate {
    #[doc = "The types of customer updates that are supported."]
    #[json(default)]
    pub allowed_updates: Vec<stripe_types_04::billing_portal::PortalCustomerUpdateAllowedUpdatesItem>,
    #[doc = "Whether the feature is enabled."]
    #[json(default)]
    pub enabled: bool,
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalSubscriptionUpdateProduct {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<stripe_types_03::billing_portal::PortalSubscriptionUpdateProductAdjustableQuantity>,
    #[doc = "The list of price IDs which, when subscribed to, a subscription can be updated."]
    #[json(default)]
    pub prices: Vec<String>,
    #[doc = "The product ID."]
    #[json(default)]
    pub product: String,
}

#[cfg(feature = "billing_portal")]
pub type UpdateBillingPortalConfigurationParamsFeaturesPaymentMethodUpdate =
    stripe_types_03::billing_portal::CreateBillingPortalConfigurationParamsFeaturesPaymentMethodUpdate;

#[cfg(feature = "billing_portal")]
pub type UpdateBillingPortalConfigurationParamsFeaturesSubscriptionCancelMode =
    stripe_types_03::billing_portal::CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelMode;

#[cfg(feature = "billing_portal")]
pub type UpdateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior =
    stripe_types_03::billing_portal::CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancelProrationBehavior;
