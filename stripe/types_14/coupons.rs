#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct Discount {
    #[doc = "The Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub checkout_session: Option<String>,
    #[doc = "The ID of the customer associated with this discount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<crate::coupons::DiscountCustomerExpanded>>,
    #[doc = "The ID of the account representing the customer associated with this discount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "If the coupon has a duration of `repeating`, the date that this discount will end."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
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
    pub promotion_code: Option<crate::Expandable<crate::promotion_codes::PromotionCode>>,
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

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl crate::PagedItem for Discount {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone)]
pub enum DiscountCustomerExpanded {
    /// Expanded `customer`.
    Customer(Box<crate::customers::Customer>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl<'json> json::Decode<'json> for DiscountCustomerExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "customer" => DiscountCustomerExpanded::Customer(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            _ => DiscountCustomerExpanded::Other(value),
        })
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct StackableDiscountWithDiscountSettings {
    #[doc = "ID of the coupon to create a new discount for."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub coupon: Option<crate::Expandable<stripe_types_06::coupons::Coupon>>,
    #[doc = "ID of an existing discount on the object (or one of its ancestors) to reuse."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub discount: Option<crate::Expandable<crate::coupons::Discount>>,
    #[doc = "ID of the promotion code to create a new discount for."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<crate::Expandable<crate::promotion_codes::PromotionCode>>,
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct StackableDiscountWithDiscountSettingsAndDiscountEnd {
    #[doc = "ID of the coupon to create a new discount for."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub coupon: Option<crate::Expandable<stripe_types_06::coupons::Coupon>>,
    #[doc = "ID of an existing discount on the object (or one of its ancestors) to reuse."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub discount: Option<crate::Expandable<crate::coupons::Discount>>,
    #[doc = "ID of the promotion code to create a new discount for."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<crate::Expandable<crate::promotion_codes::PromotionCode>>,
}
