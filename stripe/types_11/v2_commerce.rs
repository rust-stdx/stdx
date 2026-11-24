#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CommerceProductCatalogImportStatusDetails {
    #[doc = "Details when the import is awaiting file upload."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub awaiting_upload:
        Option<stripe_types_06::v2_commerce::V2CommerceProductCatalogImportStatusDetailsAwaitingUpload>,
    #[doc = "Details when the import didn't complete."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<stripe_types_06::v2_commerce::V2CommerceProductCatalogImportStatusDetailsFailed>,
    #[doc = "Details when the import is processing."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub processing: Option<stripe_types_04::v2_commerce::V2CommerceProductCatalogImportStatusDetailsProcessing>,
    #[doc = "Details when the import has succeeded."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<stripe_types_01::v2_commerce::V2CommerceProductCatalogImportStatusDetailsSucceeded>,
    #[doc = "Details when the import completed but some records failed to process."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub succeeded_with_errors:
        Option<stripe_types_08::v2_commerce::V2CommerceProductCatalogImportStatusDetailsSucceededWithErrors>,
}
