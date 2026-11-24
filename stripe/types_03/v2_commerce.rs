#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCommerceProductCatalogImportParamsFeedType {
    Inventory,
    Pricing,
    Product,
    Promotion,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_commerce")]
impl CreateCommerceProductCatalogImportParamsFeedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCommerceProductCatalogImportParamsFeedType::Inventory => "inventory",
            CreateCommerceProductCatalogImportParamsFeedType::Pricing => "pricing",
            CreateCommerceProductCatalogImportParamsFeedType::Product => "product",
            CreateCommerceProductCatalogImportParamsFeedType::Promotion => "promotion",
            CreateCommerceProductCatalogImportParamsFeedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_commerce")]
impl json::Encode for CreateCommerceProductCatalogImportParamsFeedType {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_commerce")]
impl<'json> json::Decode<'json> for CreateCommerceProductCatalogImportParamsFeedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCommerceProductCatalogImportParamsFeedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "inventory" => CreateCommerceProductCatalogImportParamsFeedType::Inventory,
            "pricing" => CreateCommerceProductCatalogImportParamsFeedType::Pricing,
            "product" => CreateCommerceProductCatalogImportParamsFeedType::Product,
            "promotion" => CreateCommerceProductCatalogImportParamsFeedType::Promotion,
            other => CreateCommerceProductCatalogImportParamsFeedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CommerceProductCatalogImportFeedType {
    Inventory,
    Pricing,
    Product,
    Promotion,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_commerce")]
impl V2CommerceProductCatalogImportFeedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CommerceProductCatalogImportFeedType::Inventory => "inventory",
            V2CommerceProductCatalogImportFeedType::Pricing => "pricing",
            V2CommerceProductCatalogImportFeedType::Product => "product",
            V2CommerceProductCatalogImportFeedType::Promotion => "promotion",
            V2CommerceProductCatalogImportFeedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_commerce")]
impl Default for V2CommerceProductCatalogImportFeedType {
    fn default() -> Self {
        V2CommerceProductCatalogImportFeedType::Unknown(String::new())
    }
}

#[cfg(feature = "v2_commerce")]
impl<'json> json::Decode<'json> for V2CommerceProductCatalogImportFeedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CommerceProductCatalogImportFeedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "inventory" => V2CommerceProductCatalogImportFeedType::Inventory,
            "pricing" => V2CommerceProductCatalogImportFeedType::Pricing,
            "product" => V2CommerceProductCatalogImportFeedType::Product,
            "promotion" => V2CommerceProductCatalogImportFeedType::Promotion,
            other => V2CommerceProductCatalogImportFeedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CommerceProductCatalogImportStatusDetailsSucceededWithErrorsSamplesItem {
    #[doc = "A description of what went wrong with this record."]
    #[json(default)]
    pub error_message: String,
    #[doc = "The name of the field that caused the error."]
    #[json(default)]
    pub field: String,
    #[doc = "The identifier of the record that failed to process."]
    #[json(default)]
    pub id: String,
    #[doc = "The row number in the import file where the error occurred."]
    #[json(default)]
    pub row: String,
}

#[cfg(feature = "v2_commerce")]
impl crate::PagedItem for V2CommerceProductCatalogImportStatusDetailsSucceededWithErrorsSamplesItem {
    fn id(&self) -> &str {
        &self.id
    }
}
