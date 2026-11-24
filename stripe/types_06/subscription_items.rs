#[cfg(feature = "subscription_items")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionItemParamsPriceDataRecurring {
    #[doc = "Specifies billing frequency."]
    pub interval: stripe_types_03::subscription_items::CreateSubscriptionItemParamsPriceDataRecurringInterval,
    #[doc = "The number of intervals between subscription billings."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "subscription_items")]
impl crate::params::FormEncode for CreateSubscriptionItemParamsPriceDataRecurring {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
    }
}

#[cfg(feature = "subscription_items")]
pub type DeleteSubscriptionItemParamsPaymentBehavior =
    stripe_types_03::subscription_items::CreateSubscriptionItemParamsPaymentBehavior;

#[cfg(feature = "subscription_items")]
pub type UpdateSubscriptionItemParamsBillingThresholds =
    stripe_types_00::subscription_items::CreateSubscriptionItemParamsBillingThresholds;

#[cfg(feature = "subscription_items")]
pub type UpdateSubscriptionItemParamsCurrentTrial =
    stripe_types_04::subscription_items::CreateSubscriptionItemParamsCurrentTrial;
