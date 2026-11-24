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
    feature = "shipping_rates",
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
pub struct ShippingRate {
    #[doc = "Whether the shipping rate can be used for new purchases."]
    #[json(default)]
    pub active: bool,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The estimated range for how long shipping will take, meant to be displayable to the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<stripe_types_07::shipping_rates::ShippingRateDeliveryEstimate>,
    #[doc = "The name of the shipping rate, meant to be displayable to the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<stripe_types_08::shipping_rates::ShippingRateFixedAmount>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::shipping_rates::ShippingRateObject,
    #[doc = "Specifies whether the rate is considered inclusive of taxes or exclusive of taxes."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<stripe_types_04::shipping_rates::ShippingRateTaxBehavior>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<crate::Expandable<stripe_types_08::tax_codes::TaxCode>>,
    #[doc = "The type of calculation to use on the shipping rate."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::shipping_rates::ShippingRateType,
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
    feature = "shipping_rates",
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
impl crate::PagedItem for ShippingRate {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "shipping_rates")]
#[derive(Debug, Clone, Default)]
pub struct UpdateShippingRateParamsFixedAmount {
    #[doc = "Shipping rates defined in each available currency option."]
    pub currency_options: Option<
        std::collections::HashMap<
            String,
            stripe_types_08::shipping_rates::UpdateShippingRateParamsFixedAmountCurrencyOptionsValue,
        >,
    >,
}

#[cfg(feature = "shipping_rates")]
impl crate::params::FormEncode for UpdateShippingRateParamsFixedAmount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.currency_options.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("currency_options", key, value);
            }
        }
    }
}
