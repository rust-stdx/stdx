#[cfg(feature = "webhook_endpoints")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedWebhookEndpoint {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::webhook_endpoints::WebhookEndpointObject,
}

#[cfg(feature = "webhook_endpoints")]
impl crate::PagedItem for DeletedWebhookEndpoint {
    fn id(&self) -> &str {
        &self.id
    }
}
