#[cfg(feature = "prices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePriceParamsBillingScheme {
    PerUnit,
    Tiered,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "prices")]
impl CreatePriceParamsBillingScheme {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePriceParamsBillingScheme::PerUnit => "per_unit",
            CreatePriceParamsBillingScheme::Tiered => "tiered",
            CreatePriceParamsBillingScheme::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "prices")]
impl<'json> json::Decode<'json> for CreatePriceParamsBillingScheme {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePriceParamsBillingScheme::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "per_unit" => CreatePriceParamsBillingScheme::PerUnit,
            "tiered" => CreatePriceParamsBillingScheme::Tiered,
            other => CreatePriceParamsBillingScheme::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for CreatePriceParamsBillingScheme {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "prices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePriceParamsProductDataTaxDetails {
    #[doc = "A tax location ID."]
    pub performance_location: Option<String>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<json::OwnedValue>,
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for CreatePriceParamsProductDataTaxDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("performance_location", self.performance_location.as_ref());
    }
}

#[cfg(feature = "prices")]
#[derive(Debug, Clone, Default)]
pub struct GetPriceParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for GetPriceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "prices")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListPricesParamsRecurringUsageType {
    Licensed,
    Metered,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "prices")]
impl ListPricesParamsRecurringUsageType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListPricesParamsRecurringUsageType::Licensed => "licensed",
            ListPricesParamsRecurringUsageType::Metered => "metered",
            ListPricesParamsRecurringUsageType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "prices")]
impl<'json> json::Decode<'json> for ListPricesParamsRecurringUsageType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListPricesParamsRecurringUsageType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "licensed" => ListPricesParamsRecurringUsageType::Licensed,
            "metered" => ListPricesParamsRecurringUsageType::Metered,
            other => ListPricesParamsRecurringUsageType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for ListPricesParamsRecurringUsageType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "prices")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListPricesParamsType {
    OneTime,
    Recurring,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "prices")]
impl ListPricesParamsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListPricesParamsType::OneTime => "one_time",
            ListPricesParamsType::Recurring => "recurring",
            ListPricesParamsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "prices")]
impl<'json> json::Decode<'json> for ListPricesParamsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListPricesParamsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "one_time" => ListPricesParamsType::OneTime,
            "recurring" => ListPricesParamsType::Recurring,
            other => ListPricesParamsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for ListPricesParamsType {
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
pub enum PriceObject {
    Price,
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
impl PriceObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PriceObject::Price => "price",
            PriceObject::Unknown(value) => value,
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
impl Default for PriceObject {
    fn default() -> Self {
        PriceObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for PriceObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PriceObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "price" => PriceObject::Price,
            other => PriceObject::Unknown(other.to_string()),
        })
    }
}
