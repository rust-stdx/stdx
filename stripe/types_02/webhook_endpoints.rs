#[cfg(feature = "webhook_endpoints")]
#[derive(Debug, Clone, Default)]
pub struct GetWebhookEndpointParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "webhook_endpoints")]
impl crate::params::FormEncode for GetWebhookEndpointParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
