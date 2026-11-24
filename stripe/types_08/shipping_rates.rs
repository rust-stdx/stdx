#[cfg(feature = "shipping_rates")]
#[derive(Debug, Clone, Default)]
pub struct CreateShippingRateParamsFixedAmount {
    #[doc = "A non-negative integer in cents representing how much to charge."]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "Shipping rates defined in each available currency option."]
    pub currency_options: Option<
        std::collections::HashMap<
            String,
            stripe_types_05::shipping_rates::CreateShippingRateParamsFixedAmountCurrencyOptionsValue,
        >,
    >,
}

#[cfg(feature = "shipping_rates")]
impl crate::params::FormEncode for CreateShippingRateParamsFixedAmount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("currency", &self.currency);
        if let Some(values) = self.currency_options.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("currency_options", key, value);
            }
        }
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
pub struct ShippingRateFixedAmount {
    #[doc = "A non-negative integer in cents representing how much to charge."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "Shipping rates defined in each available currency option."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency_options:
        Option<std::collections::HashMap<String, stripe_types_06::shipping_rates::ShippingRateCurrencyOption>>,
}

#[cfg(feature = "shipping_rates")]
#[derive(Debug, Clone, Default)]
pub struct UpdateShippingRateParamsFixedAmountCurrencyOptionsValue {
    #[doc = "A non-negative integer in cents representing how much to charge."]
    pub amount: Option<i64>,
    #[doc = "Specifies whether the rate is considered inclusive of taxes or exclusive of taxes."]
    pub tax_behavior:
        Option<stripe_types_05::shipping_rates::UpdateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior>,
}

#[cfg(feature = "shipping_rates")]
impl crate::params::FormEncode for UpdateShippingRateParamsFixedAmountCurrencyOptionsValue {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
    }
}
