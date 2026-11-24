#[cfg(any(feature = "climate", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ClimateProduct {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Current prices for a metric ton of carbon removal in a currency's smallest unit."]
    #[json(default)]
    pub current_prices_per_metric_ton:
        std::collections::HashMap<String, stripe_types_02::climate::ClimateRemovalsProductsPrice>,
    #[doc = "The year in which the carbon removal is expected to be delivered."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub delivery_year: Option<i64>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "The quantity of metric tons available for reservation."]
    #[json(default)]
    pub metric_tons_available: String,
    #[doc = "The Climate product's name."]
    #[json(default)]
    pub name: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::climate::ClimateProductObject,
    #[doc = "The carbon removal suppliers that fulfill orders for this Climate product."]
    #[json(default)]
    pub suppliers: Vec<stripe_types_06::climate::ClimateSupplier>,
}

#[cfg(any(feature = "climate", feature = "events"))]
impl crate::PagedItem for ClimateProduct {
    fn id(&self) -> &str {
        &self.id
    }
}
