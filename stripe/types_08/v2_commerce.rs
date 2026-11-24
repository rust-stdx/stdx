#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CommerceProductCatalogImportStatusDetailsSucceededWithErrors {
    #[doc = "The total number of records that failed to process."]
    #[json(default)]
    pub error_count: String,
    #[doc = "A file containing details about all errors that occurred."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub error_file:
        Option<stripe_types_06::v2_commerce::V2CommerceProductCatalogImportStatusDetailsSucceededWithErrorsErrorFile>,
    #[doc = "A sample of errors that occurred during processing."]
    #[json(default)]
    pub samples:
        Vec<stripe_types_03::v2_commerce::V2CommerceProductCatalogImportStatusDetailsSucceededWithErrorsSamplesItem>,
    #[doc = "The total number of records processed."]
    #[json(default)]
    pub success_count: String,
}
