#[cfg(feature = "forwarding")]
#[derive(Debug, Clone, Default)]
pub struct CreateForwardingRequestParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The PaymentMethod to insert into the forwarded request."]
    pub payment_method: String,
    #[doc = "The field kinds to be replaced in the forwarded request."]
    pub replacements: Vec<stripe_types_04::forwarding::CreateForwardingRequestParamsReplacementsItem>,
    #[doc = "The request body and headers to be sent to the destination endpoint."]
    pub request: Option<stripe_types_05::forwarding::CreateForwardingRequestParamsRequest>,
    #[doc = "The destination URL for the forwarded request."]
    pub url: String,
}

#[cfg(feature = "forwarding")]
impl crate::params::FormEncode for CreateForwardingRequestParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("payment_method", &self.payment_method);
        form.field_array("replacements", self.replacements.as_slice());
        form.field_opt("request", self.request.as_ref());
        form.field("url", &self.url);
    }
}
