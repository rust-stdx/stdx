#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCommerceProductCatalogImportParams {
    #[doc = "The type of catalog data to import."]
    pub feed_type: stripe_types_03::v2_commerce::CreateCommerceProductCatalogImportParamsFeedType,
    #[doc = "Additional information about the import in a structured format."]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "The strategy for handling existing catalog data during import."]
    pub mode: stripe_types_04::v2_commerce::CreateCommerceProductCatalogImportParamsMode,
}

#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CommerceProductCatalogImportStatusDetailsAwaitingUpload {
    #[doc = "The pre-signed URL information for uploading the catalog file."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub upload_url:
        Option<stripe_types_01::v2_commerce::V2CommerceProductCatalogImportStatusDetailsAwaitingUploadUploadUrl>,
}

#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CommerceProductCatalogImportStatusDetailsFailed {
    #[doc = "The error code for this product catalog processing failure."]
    #[json(default)]
    pub code: stripe_types_00::v2_commerce::V2CommerceProductCatalogImportStatusDetailsFailedCode,
    #[doc = "A message explaining why the import failed."]
    #[json(default)]
    pub failure_message: String,
    #[doc = "The error type for this product catalog processing failure."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::v2_commerce::V2CommerceProductCatalogImportStatusDetailsFailedType,
}

#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CommerceProductCatalogImportStatusDetailsSucceededWithErrorsErrorFile {
    #[doc = "The MIME type of the error file."]
    #[json(default)]
    pub content_type: String,
    #[doc = "The pre-signed URL information for downloading the error file."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub download_url:
        Option<stripe_types_01::v2_commerce::V2CommerceProductCatalogImportStatusDetailsAwaitingUploadUploadUrl>,
    #[doc = "The size of the error file in bytes."]
    #[json(default)]
    pub size: String,
}
