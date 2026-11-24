#[cfg(feature = "invoice_rendering_templates")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoiceRenderingTemplate {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "A brief description of the template, hidden from customers"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::invoice_rendering_templates::InvoiceRenderingTemplateObject,
    #[doc = "The status of the template, one of `active` or `archived`."]
    #[json(default)]
    pub status: stripe_types_01::invoice_rendering_templates::InvoiceRenderingTemplateStatus,
    #[doc = "Version of this template; version increases by one when an update on the template changes any field that controls inv..."]
    #[json(default)]
    pub version: i64,
}

#[cfg(feature = "invoice_rendering_templates")]
impl crate::PagedItem for InvoiceRenderingTemplate {
    fn id(&self) -> &str {
        &self.id
    }
}
