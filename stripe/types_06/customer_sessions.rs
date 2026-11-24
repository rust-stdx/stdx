#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateCustomerSessionParamsComponentsCustomerSheetFeatures {
    #[doc = "A list of `allow_redisplay`..."]
    pub payment_method_allow_redisplay_filters: Option<Vec<stripe_types_02::customer_sessions::CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFiltersItem>>,
    #[doc = "Controls whether the customer sheet displays the option to remove a saved payment method.'  Allowing buyers to remove..."]
    pub payment_method_remove: Option<stripe_types_01::customer_sessions::CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove>,
}

#[cfg(feature = "customer_sessions")]
impl crate::params::FormEncode for CreateCustomerSessionParamsComponentsCustomerSheetFeatures {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.payment_method_allow_redisplay_filters.as_ref() {
            form.field_array("payment_method_allow_redisplay_filters", values.as_slice());
        }
        form.field_opt("payment_method_remove", self.payment_method_remove.as_ref());
    }
}

#[cfg(feature = "customer_sessions")]
pub type CreateCustomerSessionParamsComponentsMobilePaymentElementFeaturesPaymentMethodRedisplay =
    stripe_types_01::customer_sessions::CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove;

#[cfg(feature = "customer_sessions")]
pub type CreateCustomerSessionParamsComponentsMobilePaymentElementFeaturesPaymentMethodRemove =
    stripe_types_01::customer_sessions::CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove;

#[cfg(feature = "customer_sessions")]
pub type CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFiltersItem = stripe_types_02::customer_sessions::CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFiltersItem;

#[cfg(feature = "customer_sessions")]
pub type CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodRedisplay =
    stripe_types_01::customer_sessions::CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove;

#[cfg(feature = "customer_sessions")]
pub type CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodSave =
    stripe_types_01::customer_sessions::CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove;

#[cfg(feature = "customer_sessions")]
pub type CreateCustomerSessionParamsComponentsPricingTable =
    stripe_types_00::customer_sessions::CreateCustomerSessionParamsComponentsActiveEntitlements;
