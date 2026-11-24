#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links",
    feature = "quotes"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct Item {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<stripe_types_04::common::LineItemsAdjustableQuantity>,
    #[doc = "Total discount amount applied."]
    #[json(default)]
    pub amount_discount: i64,
    #[doc = "Total before any discounts or taxes are applied."]
    #[json(default)]
    pub amount_subtotal: i64,
    #[doc = "Total tax amount applied."]
    #[json(default)]
    pub amount_tax: i64,
    #[doc = "Total after discounts and taxes."]
    #[json(default)]
    pub amount_total: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The discounts applied to the line item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<crate::common::LineItemsDiscountAmount>>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::common::ItemObject,
    #[doc = "The price used to generate the line item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<stripe_types_11::prices::Price>,
    #[doc = "The quantity of products being purchased."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "The taxes applied to the line item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<stripe_types_07::common::LineItemsTaxAmount>>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links",
    feature = "quotes"
))]
impl crate::PagedItem for Item {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links",
    feature = "quotes"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct LineItemsDiscountAmount {
    #[doc = "The amount discounted."]
    #[json(default)]
    pub amount: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub discount: Option<stripe_types_14::coupons::Discount>,
}
