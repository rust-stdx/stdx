#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone, PartialEq)]
pub enum ProductCatalogImplEndpointsTrialOfferResourceTrialOfferDurationType {
    Relative,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "product_catalog")]
impl ProductCatalogImplEndpointsTrialOfferResourceTrialOfferDurationType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ProductCatalogImplEndpointsTrialOfferResourceTrialOfferDurationType::Relative => "relative",
            ProductCatalogImplEndpointsTrialOfferResourceTrialOfferDurationType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "product_catalog")]
impl Default for ProductCatalogImplEndpointsTrialOfferResourceTrialOfferDurationType {
    fn default() -> Self {
        ProductCatalogImplEndpointsTrialOfferResourceTrialOfferDurationType::Unknown(String::new())
    }
}

#[cfg(feature = "product_catalog")]
impl<'json> json::Decode<'json> for ProductCatalogImplEndpointsTrialOfferResourceTrialOfferDurationType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ProductCatalogImplEndpointsTrialOfferResourceTrialOfferDurationType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "relative" => ProductCatalogImplEndpointsTrialOfferResourceTrialOfferDurationType::Relative,
            other => ProductCatalogImplEndpointsTrialOfferResourceTrialOfferDurationType::Unknown(other.to_string()),
        })
    }
}
