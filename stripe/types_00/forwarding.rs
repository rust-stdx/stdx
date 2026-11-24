#[cfg(feature = "forwarding")]
#[derive(Debug, Clone, Default)]
pub struct CreateForwardingRequestParamsRequestHeadersItem {
    #[doc = "The header name."]
    pub name: String,
    #[doc = "The header value."]
    pub value: String,
}

#[cfg(feature = "forwarding")]
impl crate::params::FormEncode for CreateForwardingRequestParamsRequestHeadersItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("name", &self.name);
        form.field("value", &self.value);
    }
}
