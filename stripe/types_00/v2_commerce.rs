#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListCommerceProductCatalogImportsParamsFeedType {
    Inventory,
    Pricing,
    Product,
    Promotion,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_commerce")]
impl ListCommerceProductCatalogImportsParamsFeedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListCommerceProductCatalogImportsParamsFeedType::Inventory => "inventory",
            ListCommerceProductCatalogImportsParamsFeedType::Pricing => "pricing",
            ListCommerceProductCatalogImportsParamsFeedType::Product => "product",
            ListCommerceProductCatalogImportsParamsFeedType::Promotion => "promotion",
            ListCommerceProductCatalogImportsParamsFeedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_commerce")]
impl<'json> json::Decode<'json> for ListCommerceProductCatalogImportsParamsFeedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListCommerceProductCatalogImportsParamsFeedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "inventory" => ListCommerceProductCatalogImportsParamsFeedType::Inventory,
            "pricing" => ListCommerceProductCatalogImportsParamsFeedType::Pricing,
            "product" => ListCommerceProductCatalogImportsParamsFeedType::Product,
            "promotion" => ListCommerceProductCatalogImportsParamsFeedType::Promotion,
            other => ListCommerceProductCatalogImportsParamsFeedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_commerce")]
impl crate::params::FormEncode for ListCommerceProductCatalogImportsParamsFeedType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CommerceProductCatalogImportStatusDetailsFailedCode {
    FileNotFound,
    InternalError,
    InvalidFile,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_commerce")]
impl V2CommerceProductCatalogImportStatusDetailsFailedCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CommerceProductCatalogImportStatusDetailsFailedCode::FileNotFound => "file_not_found",
            V2CommerceProductCatalogImportStatusDetailsFailedCode::InternalError => "internal_error",
            V2CommerceProductCatalogImportStatusDetailsFailedCode::InvalidFile => "invalid_file",
            V2CommerceProductCatalogImportStatusDetailsFailedCode::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_commerce")]
impl Default for V2CommerceProductCatalogImportStatusDetailsFailedCode {
    fn default() -> Self {
        V2CommerceProductCatalogImportStatusDetailsFailedCode::Unknown(String::new())
    }
}

#[cfg(feature = "v2_commerce")]
impl<'json> json::Decode<'json> for V2CommerceProductCatalogImportStatusDetailsFailedCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CommerceProductCatalogImportStatusDetailsFailedCode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "file_not_found" => V2CommerceProductCatalogImportStatusDetailsFailedCode::FileNotFound,
            "internal_error" => V2CommerceProductCatalogImportStatusDetailsFailedCode::InternalError,
            "invalid_file" => V2CommerceProductCatalogImportStatusDetailsFailedCode::InvalidFile,
            other => V2CommerceProductCatalogImportStatusDetailsFailedCode::Unknown(other.to_string()),
        })
    }
}
