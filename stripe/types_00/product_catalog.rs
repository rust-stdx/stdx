#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone, Default)]
pub struct CreateProductCatalogTrialOfferParamsEndBehaviorTransition {
    #[doc = "The price to transition the recurring item to when the trial offer ends."]
    pub price: String,
}

#[cfg(feature = "product_catalog")]
impl crate::params::FormEncode for CreateProductCatalogTrialOfferParamsEndBehaviorTransition {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("price", &self.price);
    }
}

#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone, Default)]
pub struct GetProductCatalogTrialOfferParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "product_catalog")]
impl crate::params::FormEncode for GetProductCatalogTrialOfferParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone, Default)]
pub struct ListProductCatalogTrialOffersParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "product_catalog")]
impl crate::params::FormEncode for ListProductCatalogTrialOffersParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}

#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ProductCatalogImplEndpointsTrialOfferResourceRelativeDuration {
    #[doc = "The number of iterations of the price's interval for this trial offer."]
    #[json(default)]
    pub iterations: i64,
}

#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone, PartialEq)]
pub enum ProductCatalogTrialOfferObject {
    ProductCatalogTrialOffer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "product_catalog")]
impl ProductCatalogTrialOfferObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ProductCatalogTrialOfferObject::ProductCatalogTrialOffer => "product_catalog.trial_offer",
            ProductCatalogTrialOfferObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "product_catalog")]
impl Default for ProductCatalogTrialOfferObject {
    fn default() -> Self {
        ProductCatalogTrialOfferObject::Unknown(String::new())
    }
}

#[cfg(feature = "product_catalog")]
impl<'json> json::Decode<'json> for ProductCatalogTrialOfferObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ProductCatalogTrialOfferObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "product_catalog.trial_offer" => ProductCatalogTrialOfferObject::ProductCatalogTrialOffer,
            other => ProductCatalogTrialOfferObject::Unknown(other.to_string()),
        })
    }
}
