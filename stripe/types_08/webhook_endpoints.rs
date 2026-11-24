#[cfg(feature = "webhook_endpoints")]
#[derive(Debug, Clone, Default)]
pub struct UpdateWebhookEndpointParams {
    #[doc = "An optional description of what the webhook is used for."]
    pub description: Option<json::OwnedValue>,
    #[doc = "Disable the webhook endpoint if set to true."]
    pub disabled: Option<bool>,
    #[doc = "The list of events to enable for this endpoint."]
    pub enabled_events: Option<Vec<stripe_types_06::webhook_endpoints::UpdateWebhookEndpointParamsEnabledEventsItem>>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The URL of the webhook endpoint."]
    pub url: Option<String>,
}

#[cfg(feature = "webhook_endpoints")]
impl crate::params::FormEncode for UpdateWebhookEndpointParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("disabled", self.disabled.as_ref());
        if let Some(values) = self.enabled_events.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("enabled_events", index, item);
            }
        }
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("url", self.url.as_ref());
    }
}
