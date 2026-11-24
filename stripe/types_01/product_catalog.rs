#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateProductCatalogTrialOfferParamsDurationType {
    Relative,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "product_catalog")]
impl CreateProductCatalogTrialOfferParamsDurationType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateProductCatalogTrialOfferParamsDurationType::Relative => "relative",
            CreateProductCatalogTrialOfferParamsDurationType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "product_catalog")]
impl<'json> json::Decode<'json> for CreateProductCatalogTrialOfferParamsDurationType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateProductCatalogTrialOfferParamsDurationType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "relative" => CreateProductCatalogTrialOfferParamsDurationType::Relative,
            other => CreateProductCatalogTrialOfferParamsDurationType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "product_catalog")]
impl crate::params::FormEncode for CreateProductCatalogTrialOfferParamsDurationType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
