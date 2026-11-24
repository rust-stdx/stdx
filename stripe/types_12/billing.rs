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
pub struct BillingBillResourceInvoicingPricingPricingPriceDetails {
    #[doc = "The ID of the price this item is associated with."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<crate::Expandable<stripe_types_11::prices::Price>>,
    #[doc = "The ID of the product this item is associated with."]
    #[json(default)]
    pub product: String,
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone)]
pub struct CreateBillingCreditGrantParams {
    #[doc = "Amount of this credit grant."]
    pub amount: stripe_types_06::billing::CreateBillingCreditGrantParamsAmount,
    #[doc = "Configuration specifying what this credit grant applies to."]
    pub applicability_config: stripe_types_10::billing::CreateBillingCreditGrantParamsApplicabilityConfig,
    #[doc = "The category of this credit grant."]
    pub category: Option<stripe_types_01::billing::CreateBillingCreditGrantParamsCategory>,
    #[doc = "ID of the customer receiving the billing credits."]
    pub customer: Option<String>,
    #[doc = "ID of the account representing the customer receiving the billing credits."]
    pub customer_account: Option<String>,
    #[doc = "The time when the billing credits become effective-when they're eligible for use."]
    pub effective_at: Option<i64>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The time when the billing credits expire."]
    pub expires_at: Option<i64>,
    #[doc = "Set of key-value pairs that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "A descriptive name shown in the Dashboard."]
    pub name: Option<String>,
    #[doc = "The desired priority for applying this credit grant."]
    pub priority: Option<i64>,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingCreditGrantParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("applicability_config", &self.applicability_config);
        form.field_opt("category", self.category.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("effective_at", self.effective_at.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("expires_at", self.expires_at.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("name", self.name.as_ref());
        form.field_opt("priority", self.priority.as_ref());
    }
}
