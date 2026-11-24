#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateCustomerSessionParamsComponentsMobilePaymentElementFeatures {
    #[doc = "A list of `allow_redisplay`..."]
    pub payment_method_allow_redisplay_filters: Option<Vec<stripe_types_05::customer_sessions::CreateCustomerSessionParamsComponentsMobilePaymentElementFeaturesPaymentMethodAllowRedisplayFiltersItem>>,
    #[doc = "Controls whether or not the mobile payment element shows saved payment methods."]
    pub payment_method_redisplay: Option<stripe_types_06::customer_sessions::CreateCustomerSessionParamsComponentsMobilePaymentElementFeaturesPaymentMethodRedisplay>,
    #[doc = "Controls whether the mobile payment element displays the option to remove a saved payment method.'  Allowing buyers t..."]
    pub payment_method_remove: Option<stripe_types_06::customer_sessions::CreateCustomerSessionParamsComponentsMobilePaymentElementFeaturesPaymentMethodRemove>,
    #[doc = "Controls whether the mobile payment element displays a checkbox offering to save a new payment method."]
    pub payment_method_save: Option<stripe_types_05::customer_sessions::CreateCustomerSessionParamsComponentsMobilePaymentElementFeaturesPaymentMethodSave>,
    #[doc = "Allows overriding the value of allow_override when saving a new payment method when payment_method_save is set to dis..."]
    pub payment_method_save_allow_redisplay_override: Option<stripe_types_05::customer_sessions::CreateCustomerSessionParamsComponentsMobilePaymentElementFeaturesPaymentMethodSaveAllowRedisplayOverride>,
}

#[cfg(feature = "customer_sessions")]
impl crate::params::FormEncode for CreateCustomerSessionParamsComponentsMobilePaymentElementFeatures {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.payment_method_allow_redisplay_filters.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("payment_method_allow_redisplay_filters", index, item);
            }
        }
        form.field_opt("payment_method_redisplay", self.payment_method_redisplay.as_ref());
        form.field_opt("payment_method_remove", self.payment_method_remove.as_ref());
        form.field_opt("payment_method_save", self.payment_method_save.as_ref());
        form.field_opt(
            "payment_method_save_allow_redisplay_override",
            self.payment_method_save_allow_redisplay_override.as_ref(),
        );
    }
}
