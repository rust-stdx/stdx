#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateCustomerSessionParamsComponentsPaymentElement {
    #[doc = "Whether the Payment Element is enabled."]
    pub enabled: bool,
    #[doc = "This hash defines whether the Payment Element supports certain features."]
    pub features:
        Option<stripe_types_07::customer_sessions::CreateCustomerSessionParamsComponentsPaymentElementFeatures>,
}

#[cfg(feature = "customer_sessions")]
impl crate::params::FormEncode for CreateCustomerSessionParamsComponentsPaymentElement {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSessionResourceComponents {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub active_entitlements:
        Option<stripe_types_03::customer_sessions::CustomerSessionResourceComponentsResourceActiveEntitlements>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub buy_button: Option<stripe_types_02::customer_sessions::CustomerSessionResourceComponentsResourceBuyButton>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_portal:
        Option<stripe_types_00::customer_sessions::CustomerSessionResourceComponentsResourceCustomerPortal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_sheet:
        Option<stripe_types_08::customer_sessions::CustomerSessionResourceComponentsResourceCustomerSheet>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mobile_payment_element:
        Option<stripe_types_08::customer_sessions::CustomerSessionResourceComponentsResourceMobilePaymentElement>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_element:
        Option<stripe_types_08::customer_sessions::CustomerSessionResourceComponentsResourcePaymentElement>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pricing_table:
        Option<stripe_types_01::customer_sessions::CustomerSessionResourceComponentsResourcePricingTable>,
}
