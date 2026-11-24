#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateCustomerSessionParamsComponentsPaymentElementFeatures {
    #[doc = "A list of `allow_redisplay`..."]
    pub payment_method_allow_redisplay_filters: Option<Vec<stripe_types_06::customer_sessions::CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFiltersItem>>,
    #[doc = "Controls whether or not the Payment Element shows saved payment methods."]
    pub payment_method_redisplay: Option<stripe_types_06::customer_sessions::CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodRedisplay>,
    #[doc = "Determines the max number of saved payment methods for the Payment Element to display."]
    pub payment_method_redisplay_limit: Option<i64>,
    #[doc = "Controls whether the Payment Element displays the option to remove a saved payment method."]
    pub payment_method_remove: Option<stripe_types_05::customer_sessions::CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodRemove>,
    #[doc = "Controls whether the Payment Element displays a checkbox offering to save a new payment method."]
    pub payment_method_save: Option<stripe_types_06::customer_sessions::CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodSave>,
    #[doc = "When using PaymentIntents and the customer checks the save checkbox, this field determines the `setup_future_usage`..."]
    pub payment_method_save_usage: Option<stripe_types_04::customer_sessions::CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodSaveUsage>,
}

#[cfg(feature = "customer_sessions")]
impl crate::params::FormEncode for CreateCustomerSessionParamsComponentsPaymentElementFeatures {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.payment_method_allow_redisplay_filters.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("payment_method_allow_redisplay_filters", index, item);
            }
        }
        form.field_opt("payment_method_redisplay", self.payment_method_redisplay.as_ref());
        form.field_opt("payment_method_redisplay_limit", self.payment_method_redisplay_limit.as_ref());
        form.field_opt("payment_method_remove", self.payment_method_remove.as_ref());
        form.field_opt("payment_method_save", self.payment_method_save.as_ref());
        form.field_opt("payment_method_save_usage", self.payment_method_save_usage.as_ref());
    }
}
