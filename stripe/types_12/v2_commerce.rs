#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CommerceProductCatalogImport {
    #[doc = "The time this ProductCatalogImport was created."]
    #[json(default)]
    pub created: String,
    #[doc = "The type of feed data being imported into the product catalog."]
    #[json(default)]
    pub feed_type: stripe_types_03::v2_commerce::V2CommerceProductCatalogImportFeedType,
    #[doc = "The unique identifier for this ProductCatalogImport."]
    #[json(default)]
    pub id: String,
    #[doc = "Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Additional information about the object in a structured format."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "The import strategy for handling existing catalog data."]
    #[json(default)]
    pub mode: stripe_types_02::v2_commerce::V2CommerceProductCatalogImportMode,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::v2_commerce::V2CommerceProductCatalogImportObject,
    #[doc = "The current status of this ProductCatalogImport."]
    #[json(default)]
    pub status: stripe_types_01::v2_commerce::V2CommerceProductCatalogImportStatus,
    #[doc = "Details about the current import status."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<stripe_types_11::v2_commerce::V2CommerceProductCatalogImportStatusDetails>,
}

#[cfg(feature = "v2_commerce")]
impl crate::PagedItem for V2CommerceProductCatalogImport {
    fn id(&self) -> &str {
        &self.id
    }
}
