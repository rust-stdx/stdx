#[cfg(feature = "prices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePriceParamsCurrencyOptionsValue {
    #[doc = "When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment L..."]
    pub custom_unit_amount: Option<stripe_types_04::prices::CreatePriceParamsCurrencyOptionsValueCustomUnitAmount>,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_02::prices::CreatePriceParamsCurrencyOptionsValueTaxBehavior>,
    #[doc = "Each element represents a pricing tier."]
    pub tiers: Option<Vec<stripe_types_01::prices::CreatePriceParamsCurrencyOptionsValueTiersItem>>,
    #[doc = "A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for CreatePriceParamsCurrencyOptionsValue {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("custom_unit_amount", self.custom_unit_amount.as_ref());
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        if let Some(values) = self.tiers.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("tiers", index, item);
            }
        }
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "prices")]
pub type CreatePriceParamsCustomUnitAmount =
    stripe_types_04::prices::CreatePriceParamsCurrencyOptionsValueCustomUnitAmount;

#[cfg(feature = "prices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePriceParamsProductData {
    #[doc = "Whether the product is currently available for purchase."]
    pub active: Option<bool>,
    #[doc = "The identifier for the product."]
    pub id: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The product's name, meant to be displayable to the customer."]
    pub name: String,
    #[doc = "An arbitrary string to be displayed on your customer's credit card or bank statement."]
    pub statement_descriptor: Option<String>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<String>,
    #[doc = "Tax details for this product, including the [tax code](/tax/tax-codes) and an optional performance location."]
    pub tax_details: Option<stripe_types_03::prices::CreatePriceParamsProductDataTaxDetails>,
    #[doc = "A label that represents units of this product."]
    pub unit_label: Option<String>,
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for CreatePriceParamsProductData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("id", self.id.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("name", &self.name);
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
        form.field_opt("tax_code", self.tax_code.as_ref());
        form.field_opt("tax_details", self.tax_details.as_ref());
        form.field_opt("unit_label", self.unit_label.as_ref());
    }
}

#[cfg(feature = "prices")]
#[derive(Debug, Clone)]
pub struct CreatePriceParamsTransformQuantity {
    #[doc = "Divide usage by this number."]
    pub divide_by: i64,
    #[doc = "After division, either round the result `up` or `down`."]
    pub round: stripe_types_02::prices::CreatePriceParamsTransformQuantityRound,
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for CreatePriceParamsTransformQuantity {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("divide_by", &self.divide_by);
        form.field("round", &self.round);
    }
}

#[cfg(feature = "prices")]
#[derive(Debug, Clone, Default)]
pub struct ListPricesParamsRecurring {
    #[doc = "Filter by billing frequency."]
    pub interval: Option<stripe_types_01::prices::ListPricesParamsRecurringInterval>,
    #[doc = "Filter by the price's meter."]
    pub meter: Option<String>,
    #[doc = "Filter by the usage type for this price."]
    pub usage_type: Option<stripe_types_03::prices::ListPricesParamsRecurringUsageType>,
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for ListPricesParamsRecurring {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("interval", self.interval.as_ref());
        form.field_opt("meter", self.meter.as_ref());
        form.field_opt("usage_type", self.usage_type.as_ref());
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
pub struct TransformUsage {
    #[doc = "Divide usage by this number."]
    #[json(default)]
    pub divide_by: i64,
    #[doc = "After division, either round the result `up` or `down`."]
    #[json(default)]
    pub round: stripe_types_04::prices::TransformQuantityRound,
}

#[cfg(feature = "prices")]
pub type UpdatePriceParamsCurrencyOptionsValueTaxBehavior =
    stripe_types_02::prices::CreatePriceParamsCurrencyOptionsValueTaxBehavior;

#[cfg(feature = "prices")]
pub type UpdatePriceParamsTaxBehavior = stripe_types_02::prices::CreatePriceParamsCurrencyOptionsValueTaxBehavior;
