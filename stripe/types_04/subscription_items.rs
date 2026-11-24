#[cfg(feature = "subscription_items")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionItemParamsCurrentTrial {
    #[doc = "The ID of the trial offer to apply to the subscription item."]
    pub trial_offer: String,
}

#[cfg(feature = "subscription_items")]
impl crate::params::FormEncode for CreateSubscriptionItemParamsCurrentTrial {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("trial_offer", &self.trial_offer);
    }
}
