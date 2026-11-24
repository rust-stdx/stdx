#[cfg(any(feature = "climate", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ClimateRemovalsOrderDeliveries {
    #[doc = "Time at which the delivery occurred."]
    #[json(default)]
    pub delivered_at: i64,
    #[doc = "Specific location of this delivery."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<stripe_types_04::climate::ClimateRemovalsLocation>,
    #[doc = "Quantity of carbon removal supplied by this delivery."]
    #[json(default)]
    pub metric_tons: String,
    #[doc = "Once retired, a URL to the registry entry for the tons from this delivery."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registry_url: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub supplier: Option<stripe_types_06::climate::ClimateSupplier>,
}
