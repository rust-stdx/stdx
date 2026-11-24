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
    feature = "tax_rates",
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
pub struct TaxRate {
    #[doc = "Defaults to `true`."]
    #[json(default)]
    pub active: bool,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "An arbitrary string attached to the tax rate for your internal use only."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name of the tax rates as it will appear to your customer on their receipt email, PDF, and the hosted invo..."]
    #[json(default)]
    pub display_name: String,
    #[doc = "Actual/effective tax rate percentage out of 100."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub effective_percentage: Option<f64>,
    #[doc = "The amount of the tax rate when the `rate_type` is `flat_amount`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub flat_amount: Option<stripe_types_00::tax_rates::TaxRateFlatAmount>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "This specifies if the tax rate is inclusive or exclusive."]
    #[json(default)]
    pub inclusive: bool,
    #[doc = "The jurisdiction for the tax rate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
    #[doc = "The level of the jurisdiction that imposes this tax rate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub jurisdiction_level: Option<stripe_types_01::tax_rates::TaxRateJurisdictionLevel>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::tax_rates::TaxRateObject,
    #[doc = "Tax rate percentage out of 100."]
    #[json(default)]
    pub percentage: f64,
    #[doc = "Indicates the type of tax rate applied to the taxable amount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub rate_type: Option<stripe_types_03::tax_rates::TaxRateRateType>,
    #[doc = "[ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2), without country prefix."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The high-level tax type, such as `vat` or `sales_tax`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<stripe_types_02::tax_rates::TaxRateTaxType>,
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
    feature = "tax_rates",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl crate::PagedItem for TaxRate {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "tax_rates")]
pub type UpdateTaxRateParamsTaxType = stripe_types_01::tax_rates::CreateTaxRateParamsTaxType;
