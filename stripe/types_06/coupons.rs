#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "coupons",
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
pub struct Coupon {
    #[doc = "Amount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_off: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<stripe_types_02::coupons::CouponAppliesTo>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "If `amount_off` has been set, the three-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "Coupons defined in each available currency option."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<std::collections::HashMap<String, stripe_types_04::coupons::CouponCurrencyOption>>,
    #[doc = "One of `forever`, `once`, or `repeating`."]
    #[json(default)]
    pub duration: stripe_types_04::coupons::CouponDuration,
    #[doc = "If `duration` is `repeating`, the number of months the coupon applies."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub duration_in_months: Option<i64>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Maximum number of times this coupon can be redeemed, in total, across all customers, before it is no longer valid."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i64>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Name of the coupon displayed to customers on for instance invoices or receipts."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::coupons::CouponObject,
    #[doc = "Percent that will be taken off the subtotal of any invoices for this customer for the duration of the coupon."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub percent_off: Option<f64>,
    #[doc = "Date after which the coupon can no longer be redeemed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub redeem_by: Option<i64>,
    #[doc = "Number of times this coupon has been applied to a customer."]
    #[json(default)]
    pub times_redeemed: i64,
    #[doc = "Taking account of the above properties, whether this coupon can still be applied to a customer."]
    #[json(default)]
    pub valid: bool,
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
    feature = "coupons",
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
impl crate::PagedItem for Coupon {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "coupons")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedCoupon {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::coupons::CouponObject,
}

#[cfg(feature = "coupons")]
impl crate::PagedItem for DeletedCoupon {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "coupons")]
#[derive(Debug, Clone, Default)]
pub struct ListCouponsParams {
    pub created: Option<stripe_types_04::coupons::ListCouponsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "coupons")]
impl crate::params::FormEncode for ListCouponsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "coupons")]
pub type UpdateCouponParamsCurrencyOptionsValue = stripe_types_03::coupons::CreateCouponParamsCurrencyOptionsValue;
