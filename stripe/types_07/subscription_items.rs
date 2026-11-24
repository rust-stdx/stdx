#[cfg(feature = "subscription_items")]
#[derive(Debug, Clone)]
pub struct UpdateSubscriptionItemParamsPriceDataRecurring {
    #[doc = "Specifies billing frequency."]
    pub interval: stripe_types_05::subscription_items::UpdateSubscriptionItemParamsPriceDataRecurringInterval,
    #[doc = "The number of intervals between subscription billings."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "subscription_items")]
impl crate::params::FormEncode for UpdateSubscriptionItemParamsPriceDataRecurring {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
    }
}
