#[cfg(feature = "webhook_endpoints")]
#[derive(Debug, Clone, Default)]
pub struct CreateWebhookEndpointParams {
    #[doc = "Events sent to this endpoint will be generated with this Stripe Version instead of your account's default Stripe Vers..."]
    pub api_version: Option<stripe_types_01::webhook_endpoints::CreateWebhookEndpointParamsApiVersion>,
    #[doc = "Whether this endpoint should receive events from connected accounts (`true`), or from your account (`false`)."]
    pub connect: Option<bool>,
    #[doc = "An optional description of what the webhook is used for."]
    pub description: Option<json::OwnedValue>,
    #[doc = "The list of events to enable for this endpoint."]
    pub enabled_events: Vec<stripe_types_04::webhook_endpoints::CreateWebhookEndpointParamsEnabledEventsItem>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The URL of the webhook endpoint."]
    pub url: String,
}

#[cfg(feature = "webhook_endpoints")]
impl crate::params::FormEncode for CreateWebhookEndpointParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("api_version", self.api_version.as_ref());
        form.field_opt("connect", self.connect.as_ref());
        form.field_array("enabled_events", self.enabled_events.as_slice());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("url", &self.url);
    }
}

#[cfg(feature = "webhook_endpoints")]
pub type UpdateWebhookEndpointParamsEnabledEventsItem =
    stripe_types_04::webhook_endpoints::CreateWebhookEndpointParamsEnabledEventsItem;

#[cfg(feature = "webhook_endpoints")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct WebhookEndpoint {
    #[doc = "The API version that events are rendered as for this webhook endpoint."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[doc = "The ID of the associated Connect application."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "An optional description of what the webhook is used for."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The list of events to enable for this endpoint."]
    #[json(default)]
    pub enabled_events: Vec<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::webhook_endpoints::WebhookEndpointObject,
    #[doc = "The endpoint's secret, used to generate [webhook signatures](https://docs.stripe.com/webhooks/signatures)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[doc = "The status of the webhook."]
    #[json(default)]
    pub status: String,
    #[doc = "The URL of the webhook endpoint."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "webhook_endpoints")]
impl crate::PagedItem for WebhookEndpoint {
    fn id(&self) -> &str {
        &self.id
    }
}
