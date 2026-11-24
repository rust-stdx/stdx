#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone)]
pub struct CreateCustomerSessionParams {
    #[doc = "Configuration for each component."]
    pub components: stripe_types_12::customer_sessions::CreateCustomerSessionParamsComponents,
    #[doc = "The ID of an existing customer for which to create the Customer Session."]
    pub customer: Option<String>,
    #[doc = "The ID of an existing Account for which to create the Customer Session."]
    pub customer_account: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "customer_sessions")]
impl crate::params::FormEncode for CreateCustomerSessionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("components", &self.components);
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
