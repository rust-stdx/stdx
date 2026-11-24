#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListCommerceProductCatalogImportsParamsStatus {
    AwaitingUpload,
    Failed,
    Processing,
    Succeeded,
    SucceededWithErrors,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_commerce")]
impl ListCommerceProductCatalogImportsParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListCommerceProductCatalogImportsParamsStatus::AwaitingUpload => "awaiting_upload",
            ListCommerceProductCatalogImportsParamsStatus::Failed => "failed",
            ListCommerceProductCatalogImportsParamsStatus::Processing => "processing",
            ListCommerceProductCatalogImportsParamsStatus::Succeeded => "succeeded",
            ListCommerceProductCatalogImportsParamsStatus::SucceededWithErrors => "succeeded_with_errors",
            ListCommerceProductCatalogImportsParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_commerce")]
impl<'json> json::Decode<'json> for ListCommerceProductCatalogImportsParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListCommerceProductCatalogImportsParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "awaiting_upload" => ListCommerceProductCatalogImportsParamsStatus::AwaitingUpload,
            "failed" => ListCommerceProductCatalogImportsParamsStatus::Failed,
            "processing" => ListCommerceProductCatalogImportsParamsStatus::Processing,
            "succeeded" => ListCommerceProductCatalogImportsParamsStatus::Succeeded,
            "succeeded_with_errors" => ListCommerceProductCatalogImportsParamsStatus::SucceededWithErrors,
            other => ListCommerceProductCatalogImportsParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_commerce")]
impl crate::params::FormEncode for ListCommerceProductCatalogImportsParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CommerceProductCatalogImportStatus {
    AwaitingUpload,
    Failed,
    Processing,
    Succeeded,
    SucceededWithErrors,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_commerce")]
impl V2CommerceProductCatalogImportStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CommerceProductCatalogImportStatus::AwaitingUpload => "awaiting_upload",
            V2CommerceProductCatalogImportStatus::Failed => "failed",
            V2CommerceProductCatalogImportStatus::Processing => "processing",
            V2CommerceProductCatalogImportStatus::Succeeded => "succeeded",
            V2CommerceProductCatalogImportStatus::SucceededWithErrors => "succeeded_with_errors",
            V2CommerceProductCatalogImportStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_commerce")]
impl Default for V2CommerceProductCatalogImportStatus {
    fn default() -> Self {
        V2CommerceProductCatalogImportStatus::Unknown(String::new())
    }
}

#[cfg(feature = "v2_commerce")]
impl<'json> json::Decode<'json> for V2CommerceProductCatalogImportStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CommerceProductCatalogImportStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "awaiting_upload" => V2CommerceProductCatalogImportStatus::AwaitingUpload,
            "failed" => V2CommerceProductCatalogImportStatus::Failed,
            "processing" => V2CommerceProductCatalogImportStatus::Processing,
            "succeeded" => V2CommerceProductCatalogImportStatus::Succeeded,
            "succeeded_with_errors" => V2CommerceProductCatalogImportStatus::SucceededWithErrors,
            other => V2CommerceProductCatalogImportStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CommerceProductCatalogImportStatusDetailsAwaitingUploadUploadUrl {
    #[doc = "The timestamp when the upload URL expires."]
    #[json(default)]
    pub expires_at: String,
    #[doc = "The pre-signed URL for uploading the catalog file."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CommerceProductCatalogImportStatusDetailsSucceeded {
    #[doc = "The total number of records processed."]
    #[json(default)]
    pub success_count: String,
}
