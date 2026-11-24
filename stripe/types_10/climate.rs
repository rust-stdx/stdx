#[cfg(any(feature = "climate", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ClimateOrder {
    #[doc = "Total amount of [Frontier](https://frontierclimate.com/)'s service fees in the currency's smallest unit."]
    #[json(default)]
    pub amount_fees: i64,
    #[doc = "Total amount of the carbon removal in the currency's smallest unit."]
    #[json(default)]
    pub amount_subtotal: i64,
    #[doc = "Total amount of the order including fees in the currency's smallest unit."]
    #[json(default)]
    pub amount_total: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<stripe_types_02::climate::ClimateRemovalsBeneficiary>,
    #[doc = "Time at which the order was canceled."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    #[doc = "Reason for the cancellation of this order."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<stripe_types_00::climate::ClimateOrderCancellationReason>,
    #[doc = "For delivered orders, a URL to a delivery certificate for the order."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[doc = "Time at which the order was confirmed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub confirmed_at: Option<i64>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase, representing the cu..."]
    #[json(default)]
    pub currency: String,
    #[doc = "Time at which the order's expected_delivery_year was delayed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub delayed_at: Option<i64>,
    #[doc = "Time at which the order was delivered."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<i64>,
    #[doc = "Details about the delivery of carbon removal for this order."]
    #[json(default)]
    pub delivery_details: Vec<stripe_types_08::climate::ClimateRemovalsOrderDeliveries>,
    #[doc = "The year this order is expected to be delivered."]
    #[json(default)]
    pub expected_delivery_year: i64,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "Quantity of carbon removal that is included in this order."]
    #[json(default)]
    pub metric_tons: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::climate::ClimateOrderObject,
    #[doc = "Unique ID for the Climate `Product` this order is purchasing."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<crate::Expandable<stripe_types_07::climate::ClimateProduct>>,
    #[doc = "Time at which the order's product was substituted for a different product."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub product_substituted_at: Option<i64>,
    #[doc = "The current status of this order."]
    #[json(default)]
    pub status: stripe_types_03::climate::ClimateOrderStatus,
}

#[cfg(any(feature = "climate", feature = "events"))]
impl crate::PagedItem for ClimateOrder {
    fn id(&self) -> &str {
        &self.id
    }
}
