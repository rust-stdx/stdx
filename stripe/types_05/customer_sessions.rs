#[cfg(feature = "customer_sessions")]
pub type CreateCustomerSessionParamsComponentsBuyButton =
    stripe_types_00::customer_sessions::CreateCustomerSessionParamsComponentsActiveEntitlements;

#[cfg(feature = "customer_sessions")]
pub type CreateCustomerSessionParamsComponentsCustomerPortal =
    stripe_types_00::customer_sessions::CreateCustomerSessionParamsComponentsActiveEntitlements;

#[cfg(feature = "customer_sessions")]
pub type CreateCustomerSessionParamsComponentsMobilePaymentElementFeaturesPaymentMethodAllowRedisplayFiltersItem = stripe_types_02::customer_sessions::CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFiltersItem;

#[cfg(feature = "customer_sessions")]
pub type CreateCustomerSessionParamsComponentsMobilePaymentElementFeaturesPaymentMethodSave =
    stripe_types_01::customer_sessions::CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove;

#[cfg(feature = "customer_sessions")]
pub type CreateCustomerSessionParamsComponentsMobilePaymentElementFeaturesPaymentMethodSaveAllowRedisplayOverride = stripe_types_02::customer_sessions::CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFiltersItem;

#[cfg(feature = "customer_sessions")]
pub type CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodRemove =
    stripe_types_01::customer_sessions::CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove;

#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures {
    #[doc = "A list of `allow_redisplay`..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_allow_redisplay_filters: Option<Vec<stripe_types_02::customer_sessions::CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFiltersItem>>,
    #[doc = "Controls whether the customer sheet displays the option to remove a saved payment method.'  Allowing buyers to remove..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_remove: Option<stripe_types_01::customer_sessions::CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove>,
}

#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures {
    #[doc = "A list of `allow_redisplay`..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_allow_redisplay_filters: Option<Vec<stripe_types_02::customer_sessions::CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFiltersItem>>,
    #[doc = "Controls whether or not the mobile payment element shows saved payment methods."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_redisplay: Option<stripe_types_01::customer_sessions::CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove>,
    #[doc = "Controls whether the mobile payment element displays the option to remove a saved payment method.'  Allowing buyers t..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_remove: Option<stripe_types_01::customer_sessions::CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove>,
    #[doc = "Controls whether the mobile payment element displays a checkbox offering to save a new payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_save: Option<stripe_types_01::customer_sessions::CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove>,
    #[doc = "Allows overriding the value of allow_override when saving a new payment method when payment_method_save is set to dis..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_save_allow_redisplay_override: Option<stripe_types_02::customer_sessions::CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFiltersItem>,
}

#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures {
    #[doc = "A list of `allow_redisplay`..."]
    #[json(default)]
    pub payment_method_allow_redisplay_filters: Vec<stripe_types_02::customer_sessions::CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFiltersItem>,
    #[doc = "Controls whether or not the Payment Element shows saved payment methods."]
    #[json(default)]
    pub payment_method_redisplay: stripe_types_01::customer_sessions::CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove,
    #[doc = "Determines the max number of saved payment methods for the Payment Element to display."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_redisplay_limit: Option<i64>,
    #[doc = "Controls whether the Payment Element displays the option to remove a saved payment method."]
    #[json(default)]
    pub payment_method_remove: stripe_types_01::customer_sessions::CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove,
    #[doc = "Controls whether the Payment Element displays a checkbox offering to save a new payment method."]
    #[json(default)]
    pub payment_method_save: stripe_types_01::customer_sessions::CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove,
    #[doc = "When using PaymentIntents and the customer checks the save checkbox, this field determines the `setup_future_usage`..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_save_usage: Option<stripe_types_02::customer_sessions::CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage>,
}
