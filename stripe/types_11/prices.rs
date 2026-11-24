#[cfg(feature = "prices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePriceParams {
    #[doc = "Whether the price can be used for new purchases."]
    pub active: Option<bool>,
    #[doc = "Describes how to compute the price per period."]
    pub billing_scheme: Option<stripe_types_03::prices::CreatePriceParamsBillingScheme>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "Prices defined in each available currency option."]
    pub currency_options:
        Option<std::collections::HashMap<String, stripe_types_06::prices::CreatePriceParamsCurrencyOptionsValue>>,
    #[doc = "When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment L..."]
    pub custom_unit_amount: Option<stripe_types_06::prices::CreatePriceParamsCustomUnitAmount>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A lookup key used to retrieve prices dynamically from a static string."]
    pub lookup_key: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "A brief description of the price, hidden from customers."]
    pub nickname: Option<String>,
    #[doc = "The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) w..."]
    pub product: Option<String>,
    #[doc = "These fields can be used to create a new product that this price will belong to."]
    pub product_data: Option<stripe_types_06::prices::CreatePriceParamsProductData>,
    #[doc = "The recurring components of a price such as `interval` and `usage_type`."]
    pub recurring: Option<stripe_types_09::prices::CreatePriceParamsRecurring>,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_05::prices::CreatePriceParamsTaxBehavior>,
    #[doc = "Each element represents a pricing tier."]
    pub tiers: Option<Vec<stripe_types_05::prices::CreatePriceParamsTiersItem>>,
    #[doc = "Defines if the tiering price should be `graduated` or `volume` based."]
    pub tiers_mode: Option<stripe_types_00::prices::CreatePriceParamsTiersMode>,
    #[doc = "If set to true, will atomically remove the lookup key from the existing price, and assign it to this price."]
    pub transfer_lookup_key: Option<bool>,
    #[doc = "Apply a transformation to the reported usage or set quantity before computing the billed price."]
    pub transform_quantity: Option<stripe_types_06::prices::CreatePriceParamsTransformQuantity>,
    #[doc = "A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for CreatePriceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("billing_scheme", self.billing_scheme.as_ref());
        form.field("currency", &self.currency);
        if let Some(values) = self.currency_options.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("currency_options", key, value);
            }
        }
        form.field_opt("custom_unit_amount", self.custom_unit_amount.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("lookup_key", self.lookup_key.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("nickname", self.nickname.as_ref());
        form.field_opt("product", self.product.as_ref());
        form.field_opt("product_data", self.product_data.as_ref());
        form.field_opt("recurring", self.recurring.as_ref());
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        if let Some(values) = self.tiers.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("tiers", index, item);
            }
        }
        form.field_opt("tiers_mode", self.tiers_mode.as_ref());
        form.field_opt("transfer_lookup_key", self.transfer_lookup_key.as_ref());
        form.field_opt("transform_quantity", self.transform_quantity.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct Price {
    #[doc = "Whether the price can be used for new purchases."]
    #[json(default)]
    pub active: bool,
    #[doc = "Describes how to compute the price per period."]
    #[json(default)]
    pub billing_scheme: stripe_types_01::prices::PriceBillingScheme,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "Prices defined in each available currency option."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<std::collections::HashMap<String, stripe_types_06::common::CurrencyOption>>,
    #[doc = "When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment L..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<stripe_types_02::common::CustomUnitAmount>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "A lookup key used to retrieve prices dynamically from a static string."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub lookup_key: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "A brief description of the price, hidden from customers."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::prices::PriceObject,
    #[doc = "The ID of the product this price is associated with."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<crate::Expandable<crate::prices::PriceProductExpanded>>,
    #[doc = "The recurring components of a price such as `interval` and `usage_type`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub recurring: Option<stripe_types_05::common::Recurring>,
    #[doc = "Only required if a default tax behavior..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<stripe_types_02::prices::PriceTaxBehavior>,
    #[doc = "Each element represents a pricing tier."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<stripe_types_02::prices::PriceTier>>,
    #[doc = "Defines if the tiering price should be `graduated` or `volume` based."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<stripe_types_02::prices::PriceTiersMode>,
    #[doc = "Apply a transformation to the reported usage or set quantity before computing the amount billed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transform_quantity: Option<stripe_types_05::prices::TransformQuantity>,
    #[doc = "One of `one_time` or `recurring` depending on whether the price is for a one-time purchase or a recurring (subscripti..."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::prices::PriceType,
    #[doc = "The unit amount in cents (or local equivalent) to be charged, represented as a whole integer if possible."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    #[doc = "The unit amount in cents (or local equivalent) to be charged, represented as a decimal string with at most 12 decimal..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
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
impl crate::PagedItem for Price {
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
#[derive(Debug, Clone)]
pub enum PriceProductExpanded {
    /// Expanded `product`.
    Product(Box<crate::products::Product>),
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
impl<'json> json::Decode<'json> for PriceProductExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "product" => PriceProductExpanded::Product(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            _ => PriceProductExpanded::Other(value),
        })
    }
}
