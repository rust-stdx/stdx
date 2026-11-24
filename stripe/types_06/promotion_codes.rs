#[cfg(feature = "promotion_codes")]
#[derive(Debug, Clone)]
pub struct CreatePromotionCodeParamsPromotion {
    #[doc = "If promotion `type` is `coupon`, the coupon for this promotion code."]
    pub coupon: Option<String>,
    #[doc = "Specifies the type of promotion."]
    pub r#type: stripe_types_03::promotion_codes::CreatePromotionCodeParamsPromotionType,
}

#[cfg(feature = "promotion_codes")]
impl crate::params::FormEncode for CreatePromotionCodeParamsPromotion {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("coupon", self.coupon.as_ref());
        form.field("type", &self.r#type);
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
pub struct PromotionCodesResourceRestrictions {
    #[doc = "Promotion code restrictions defined in each available currency option."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency_options:
        Option<std::collections::HashMap<String, stripe_types_02::promotion_codes::PromotionCodeCurrencyOption>>,
    #[doc = "A Boolean indicating if the Promotion Code should only be redeemed for Customers without any successful payments or i..."]
    #[json(default)]
    pub first_time_transaction: bool,
    #[doc = "Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<i64>,
    #[doc = "Three-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub minimum_amount_currency: Option<String>,
}

#[cfg(feature = "promotion_codes")]
pub type UpdatePromotionCodeParamsRestrictionsCurrencyOptionsValue =
    stripe_types_01::promotion_codes::CreatePromotionCodeParamsRestrictionsCurrencyOptionsValue;
