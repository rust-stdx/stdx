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
pub struct Mandate {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_acceptance: Option<stripe_types_06::customers::CustomerAcceptance>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub multi_use: Option<stripe_types_04::mandates::MandateMultiUse>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::mandates::MandateObject,
    #[doc = "The account (if any) that the mandate is intended for."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    #[doc = "ID of the payment method associated with this mandate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_details: Option<stripe_types_10::mandates::MandatePaymentMethodDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub single_use: Option<stripe_types_04::mandates::MandateSingleUse>,
    #[doc = "The mandate status indicates whether or not you can use it to initiate a payment."]
    #[json(default)]
    pub status: stripe_types_00::mandates::MandateStatus,
    #[doc = "The type of the mandate."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::mandates::MandateType,
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
impl crate::PagedItem for Mandate {
    fn id(&self) -> &str {
        &self.id
    }
}
