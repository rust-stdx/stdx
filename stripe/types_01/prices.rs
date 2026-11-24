#[cfg(feature = "prices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePriceParamsCurrencyOptionsValueTiersItem {
    #[doc = "The flat billing amount for an entire tier, regardless of the number of units in the tier."]
    pub flat_amount: Option<i64>,
    #[doc = "Same as `flat_amount`, but accepts a decimal value representing an integer in the minor units of the currency."]
    pub flat_amount_decimal: Option<String>,
    #[doc = "The per unit billing amount for each individual unit for which this tier applies."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
    #[doc = "Specifies the upper bound of this tier."]
    pub up_to: json::OwnedValue,
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for CreatePriceParamsCurrencyOptionsValueTiersItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("flat_amount", self.flat_amount.as_ref());
        form.field_opt("flat_amount_decimal", self.flat_amount_decimal.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "prices")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListPricesParamsRecurringInterval {
    Day,
    Month,
    Week,
    Year,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "prices")]
impl ListPricesParamsRecurringInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListPricesParamsRecurringInterval::Day => "day",
            ListPricesParamsRecurringInterval::Month => "month",
            ListPricesParamsRecurringInterval::Week => "week",
            ListPricesParamsRecurringInterval::Year => "year",
            ListPricesParamsRecurringInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "prices")]
impl<'json> json::Decode<'json> for ListPricesParamsRecurringInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListPricesParamsRecurringInterval::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => ListPricesParamsRecurringInterval::Day,
            "month" => ListPricesParamsRecurringInterval::Month,
            "week" => ListPricesParamsRecurringInterval::Week,
            "year" => ListPricesParamsRecurringInterval::Year,
            other => ListPricesParamsRecurringInterval::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for ListPricesParamsRecurringInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
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
    feature = "plans",
    feature = "prices",
    feature = "product_catalog",
    feature = "products",
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
#[derive(Debug, Clone, PartialEq)]
pub enum PriceBillingScheme {
    PerUnit,
    Tiered,
    /// A value not known to this version of the crate.
    Unknown(String),
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
    feature = "plans",
    feature = "prices",
    feature = "product_catalog",
    feature = "products",
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
impl PriceBillingScheme {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PriceBillingScheme::PerUnit => "per_unit",
            PriceBillingScheme::Tiered => "tiered",
            PriceBillingScheme::Unknown(value) => value,
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
    feature = "plans",
    feature = "prices",
    feature = "product_catalog",
    feature = "products",
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
impl Default for PriceBillingScheme {
    fn default() -> Self {
        PriceBillingScheme::Unknown(String::new())
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
    feature = "plans",
    feature = "prices",
    feature = "product_catalog",
    feature = "products",
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
impl<'json> json::Decode<'json> for PriceBillingScheme {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PriceBillingScheme::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "per_unit" => PriceBillingScheme::PerUnit,
            "tiered" => PriceBillingScheme::Tiered,
            other => PriceBillingScheme::Unknown(other.to_string()),
        })
    }
}
