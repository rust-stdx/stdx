#[cfg(feature = "subscription_items")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionItemParams {
    #[doc = "Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period."]
    pub billing_thresholds: Option<stripe_types_06::subscription_items::UpdateSubscriptionItemParamsBillingThresholds>,
    #[doc = "The trial offer to apply to this subscription item."]
    pub current_trial: Option<stripe_types_06::subscription_items::UpdateSubscriptionItemParamsCurrentTrial>,
    #[doc = "The coupons to redeem into discounts for the subscription item."]
    pub discounts: Option<json::OwnedValue>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Indicates if a customer is on or off-session while an invoice payment is attempted."]
    pub off_session: Option<bool>,
    #[doc = "Controls how Stripe handles payment when a subscription update requires payment and `collection_method=charge_automat..."]
    pub payment_behavior: Option<stripe_types_05::subscription_items::UpdateSubscriptionItemParamsPaymentBehavior>,
    #[doc = "The identifier of the new plan for this subscription item."]
    pub plan: Option<String>,
    #[doc = "The ID of the price object."]
    pub price: Option<String>,
    #[doc = "Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline."]
    pub price_data: Option<stripe_types_10::subscription_items::UpdateSubscriptionItemParamsPriceData>,
    #[doc = "Determines how to handle [prorations](https://docs.stripe.com/billing/subscriptions/prorations) when the billing cycl..."]
    pub proration_behavior: Option<stripe_types_05::subscription_items::UpdateSubscriptionItemParamsProrationBehavior>,
    #[doc = "If set, the proration will be calculated as though the subscription was updated at the given time."]
    pub proration_date: Option<i64>,
    #[doc = "The quantity you'd like to apply to the subscription item you're creating."]
    pub quantity: Option<i64>,
    #[doc = "A list of [Tax Rate](https://docs.stripe.com/api/tax_rates) ids."]
    pub tax_rates: Option<json::OwnedValue>,
}

#[cfg(feature = "subscription_items")]
impl crate::params::FormEncode for UpdateSubscriptionItemParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_thresholds", self.billing_thresholds.as_ref());
        form.field_opt("current_trial", self.current_trial.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("off_session", self.off_session.as_ref());
        form.field_opt("payment_behavior", self.payment_behavior.as_ref());
        form.field_opt("plan", self.plan.as_ref());
        form.field_opt("price", self.price.as_ref());
        form.field_opt("price_data", self.price_data.as_ref());
        form.field_opt("proration_behavior", self.proration_behavior.as_ref());
        form.field_opt("proration_date", self.proration_date.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
    }
}
