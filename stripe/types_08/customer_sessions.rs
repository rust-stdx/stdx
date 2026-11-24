#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateCustomerSessionParamsComponentsCustomerSheet {
    #[doc = "Whether the customer sheet is enabled."]
    pub enabled: bool,
    #[doc = "This hash defines whether the customer sheet supports certain features."]
    pub features:
        Option<stripe_types_06::customer_sessions::CreateCustomerSessionParamsComponentsCustomerSheetFeatures>,
}

#[cfg(feature = "customer_sessions")]
impl crate::params::FormEncode for CreateCustomerSessionParamsComponentsCustomerSheet {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSessionResourceComponentsResourceCustomerSheet {
    #[doc = "Whether the customer sheet is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "This hash defines whether the customer sheet supports certain features."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<
        stripe_types_05::customer_sessions::CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures,
    >,
}

#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSessionResourceComponentsResourceMobilePaymentElement {
    #[doc = "Whether the mobile payment element is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "This hash defines whether the mobile payment element supports certain features."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<stripe_types_05::customer_sessions::CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures>,
}

#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSessionResourceComponentsResourcePaymentElement {
    #[doc = "Whether the Payment Element is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "This hash defines whether the Payment Element supports certain features."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<
        stripe_types_05::customer_sessions::CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures,
    >,
}
