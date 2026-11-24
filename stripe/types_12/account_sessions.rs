#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone)]
pub struct CreateAccountSessionParams {
    #[doc = "The identifier of the account to create an Account Session for."]
    pub account: String,
    #[doc = "Each key of the dictionary represents an embedded component, and each embedded component maps to its configuration (e.g."]
    pub components: stripe_types_11::account_sessions::CreateAccountSessionParamsComponents,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("account", &self.account);
        form.field("components", &self.components);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
