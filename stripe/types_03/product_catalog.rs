#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone, PartialEq)]
pub enum ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType {
    Transition,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "product_catalog")]
impl ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType::Transition => "transition",
            ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "product_catalog")]
impl Default for ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType {
    fn default() -> Self {
        ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType::Unknown(String::new())
    }
}

#[cfg(feature = "product_catalog")]
impl<'json> json::Decode<'json> for ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "transition" => ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType::Transition,
            other => ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone, Default)]
pub struct UpdateProductCatalogTrialOfferParams {
    #[doc = "Whether the trial offer can be used for new purchases."]
    pub active: Option<bool>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "product_catalog")]
impl crate::params::FormEncode for UpdateProductCatalogTrialOfferParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
