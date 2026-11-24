#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCommerceProductCatalogImportParamsMode {
    Replace,
    Upsert,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_commerce")]
impl CreateCommerceProductCatalogImportParamsMode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCommerceProductCatalogImportParamsMode::Replace => "replace",
            CreateCommerceProductCatalogImportParamsMode::Upsert => "upsert",
            CreateCommerceProductCatalogImportParamsMode::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_commerce")]
impl json::Encode for CreateCommerceProductCatalogImportParamsMode {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_commerce")]
impl<'json> json::Decode<'json> for CreateCommerceProductCatalogImportParamsMode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCommerceProductCatalogImportParamsMode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "replace" => CreateCommerceProductCatalogImportParamsMode::Replace,
            "upsert" => CreateCommerceProductCatalogImportParamsMode::Upsert,
            other => CreateCommerceProductCatalogImportParamsMode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CommerceProductCatalogImportStatusDetailsFailedType {
    CannotProceed,
    TransientFailure,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_commerce")]
impl V2CommerceProductCatalogImportStatusDetailsFailedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CommerceProductCatalogImportStatusDetailsFailedType::CannotProceed => "cannot_proceed",
            V2CommerceProductCatalogImportStatusDetailsFailedType::TransientFailure => "transient_failure",
            V2CommerceProductCatalogImportStatusDetailsFailedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_commerce")]
impl Default for V2CommerceProductCatalogImportStatusDetailsFailedType {
    fn default() -> Self {
        V2CommerceProductCatalogImportStatusDetailsFailedType::Unknown(String::new())
    }
}

#[cfg(feature = "v2_commerce")]
impl<'json> json::Decode<'json> for V2CommerceProductCatalogImportStatusDetailsFailedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CommerceProductCatalogImportStatusDetailsFailedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cannot_proceed" => V2CommerceProductCatalogImportStatusDetailsFailedType::CannotProceed,
            "transient_failure" => V2CommerceProductCatalogImportStatusDetailsFailedType::TransientFailure,
            other => V2CommerceProductCatalogImportStatusDetailsFailedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CommerceProductCatalogImportStatusDetailsProcessing {
    #[doc = "The number of records that failed to process so far."]
    #[json(default)]
    pub error_count: String,
    #[doc = "The number of records processed so far."]
    #[json(default)]
    pub success_count: String,
}
