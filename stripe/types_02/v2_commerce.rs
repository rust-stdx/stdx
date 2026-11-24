#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CommerceProductCatalogImportMode {
    Replace,
    Upsert,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_commerce")]
impl V2CommerceProductCatalogImportMode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CommerceProductCatalogImportMode::Replace => "replace",
            V2CommerceProductCatalogImportMode::Upsert => "upsert",
            V2CommerceProductCatalogImportMode::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_commerce")]
impl Default for V2CommerceProductCatalogImportMode {
    fn default() -> Self {
        V2CommerceProductCatalogImportMode::Unknown(String::new())
    }
}

#[cfg(feature = "v2_commerce")]
impl<'json> json::Decode<'json> for V2CommerceProductCatalogImportMode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CommerceProductCatalogImportMode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "replace" => V2CommerceProductCatalogImportMode::Replace,
            "upsert" => V2CommerceProductCatalogImportMode::Upsert,
            other => V2CommerceProductCatalogImportMode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CommerceProductCatalogImportObject {
    V2CommerceProductCatalogImport,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_commerce")]
impl V2CommerceProductCatalogImportObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CommerceProductCatalogImportObject::V2CommerceProductCatalogImport => {
                "v2.commerce.product_catalog_import"
            }
            V2CommerceProductCatalogImportObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_commerce")]
impl Default for V2CommerceProductCatalogImportObject {
    fn default() -> Self {
        V2CommerceProductCatalogImportObject::Unknown(String::new())
    }
}

#[cfg(feature = "v2_commerce")]
impl<'json> json::Decode<'json> for V2CommerceProductCatalogImportObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CommerceProductCatalogImportObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.commerce.product_catalog_import" => {
                V2CommerceProductCatalogImportObject::V2CommerceProductCatalogImport
            }
            other => V2CommerceProductCatalogImportObject::Unknown(other.to_string()),
        })
    }
}
