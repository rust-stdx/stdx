#[cfg(feature = "exchange_rates")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ExchangeRate {
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::exchange_rates::ExchangeRateObject,
    #[doc = "Hash where the keys are supported currencies and the values are the exchange rate at which the base id currency conve..."]
    #[json(default)]
    pub rates: std::collections::HashMap<String, f64>,
}

#[cfg(feature = "exchange_rates")]
impl crate::PagedItem for ExchangeRate {
    fn id(&self) -> &str {
        &self.id
    }
}
