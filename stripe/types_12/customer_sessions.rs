#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateCustomerSessionParamsComponents {
    #[doc = "Configuration for active entitlements."]
    pub active_entitlements:
        Option<stripe_types_00::customer_sessions::CreateCustomerSessionParamsComponentsActiveEntitlements>,
    #[doc = "Configuration for buy button."]
    pub buy_button: Option<stripe_types_05::customer_sessions::CreateCustomerSessionParamsComponentsBuyButton>,
    #[doc = "Configuration for customer portal."]
    pub customer_portal:
        Option<stripe_types_05::customer_sessions::CreateCustomerSessionParamsComponentsCustomerPortal>,
    #[doc = "Configuration for the customer sheet."]
    pub customer_sheet: Option<stripe_types_08::customer_sessions::CreateCustomerSessionParamsComponentsCustomerSheet>,
    #[doc = "Configuration for the mobile payment element."]
    pub mobile_payment_element:
        Option<stripe_types_10::customer_sessions::CreateCustomerSessionParamsComponentsMobilePaymentElement>,
    #[doc = "Configuration for the Payment Element."]
    pub payment_element:
        Option<stripe_types_11::customer_sessions::CreateCustomerSessionParamsComponentsPaymentElement>,
    #[doc = "Configuration for the pricing table."]
    pub pricing_table: Option<stripe_types_06::customer_sessions::CreateCustomerSessionParamsComponentsPricingTable>,
}

#[cfg(feature = "customer_sessions")]
impl crate::params::FormEncode for CreateCustomerSessionParamsComponents {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active_entitlements", self.active_entitlements.as_ref());
        form.field_opt("buy_button", self.buy_button.as_ref());
        form.field_opt("customer_portal", self.customer_portal.as_ref());
        form.field_opt("customer_sheet", self.customer_sheet.as_ref());
        form.field_opt("mobile_payment_element", self.mobile_payment_element.as_ref());
        form.field_opt("payment_element", self.payment_element.as_ref());
        form.field_opt("pricing_table", self.pricing_table.as_ref());
    }
}
