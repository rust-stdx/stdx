#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone)]
pub enum ProductCatalogTrialOfferPriceExpanded {
    /// Expanded `price`.
    Price(Box<stripe_types_11::prices::Price>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(feature = "product_catalog")]
impl<'json> json::Decode<'json> for ProductCatalogTrialOfferPriceExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "price" => {
                ProductCatalogTrialOfferPriceExpanded::Price(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            _ => ProductCatalogTrialOfferPriceExpanded::Other(value),
        })
    }
}
