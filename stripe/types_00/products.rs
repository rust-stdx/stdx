#[cfg(feature = "products")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateProductParamsDefaultPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "products")]
impl CreateProductParamsDefaultPriceDataRecurringInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateProductParamsDefaultPriceDataRecurringInterval::Day => "day",
            CreateProductParamsDefaultPriceDataRecurringInterval::Month => "month",
            CreateProductParamsDefaultPriceDataRecurringInterval::Week => "week",
            CreateProductParamsDefaultPriceDataRecurringInterval::Year => "year",
            CreateProductParamsDefaultPriceDataRecurringInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "products")]
impl<'json> json::Decode<'json> for CreateProductParamsDefaultPriceDataRecurringInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateProductParamsDefaultPriceDataRecurringInterval::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => CreateProductParamsDefaultPriceDataRecurringInterval::Day,
            "month" => CreateProductParamsDefaultPriceDataRecurringInterval::Month,
            "week" => CreateProductParamsDefaultPriceDataRecurringInterval::Week,
            "year" => CreateProductParamsDefaultPriceDataRecurringInterval::Year,
            other => CreateProductParamsDefaultPriceDataRecurringInterval::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "products")]
impl crate::params::FormEncode for CreateProductParamsDefaultPriceDataRecurringInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "products")]
#[derive(Debug, Clone, Default)]
pub struct CreateProductParamsPackageDimensions {
    #[doc = "Height, in inches."]
    pub height: f64,
    #[doc = "Length, in inches."]
    pub length: f64,
    #[doc = "Weight, in ounces."]
    pub weight: f64,
    #[doc = "Width, in inches."]
    pub width: f64,
}

#[cfg(feature = "products")]
impl crate::params::FormEncode for CreateProductParamsPackageDimensions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("height", &self.height);
        form.field("length", &self.length);
        form.field("weight", &self.weight);
        form.field("width", &self.width);
    }
}

#[cfg(feature = "products")]
#[derive(Debug, Clone, Default)]
pub struct ListProductFeaturesParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "products")]
impl crate::params::FormEncode for ListProductFeaturesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
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
pub enum ProductObject {
    Product,
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
impl ProductObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ProductObject::Product => "product",
            ProductObject::Unknown(value) => value,
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
impl Default for ProductObject {
    fn default() -> Self {
        ProductObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for ProductObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ProductObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "product" => ProductObject::Product,
            other => ProductObject::Unknown(other.to_string()),
        })
    }
}
