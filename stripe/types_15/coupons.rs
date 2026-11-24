#[cfg(any(feature = "customers", feature = "subscriptions"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedDiscount {
    #[doc = "The Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub checkout_session: Option<String>,
    #[doc = "The ID of the customer associated with this discount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<stripe_types_14::coupons::DiscountCustomerExpanded>>,
    #[doc = "The ID of the account representing the customer associated with this discount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "The ID of the discount object."]
    #[json(default)]
    pub id: String,
    #[doc = "The invoice that the discount's coupon was applied to, if it was applied directly to a particular invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    #[doc = "The invoice item `id` (or invoice line item `id` for invoice line items of type='subscription') that the discount's c..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_item: Option<String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::coupons::DiscountObject,
    #[doc = "The promotion code applied to create this discount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<crate::Expandable<stripe_types_14::promotion_codes::PromotionCode>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<stripe_types_07::coupons::DiscountSource>,
    #[doc = "Date that the coupon was applied."]
    #[json(default)]
    pub start: i64,
    #[doc = "The subscription that this coupon is applied to, if it is applied to a particular subscription."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    #[doc = "The subscription item that this coupon is applied to, if it is applied to a particular subscription item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<String>,
}

#[cfg(any(feature = "customers", feature = "subscriptions"))]
impl crate::PagedItem for DeletedDiscount {
    fn id(&self) -> &str {
        &self.id
    }
}
