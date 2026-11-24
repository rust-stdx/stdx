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
pub struct Topup {
    #[doc = "Amount transferred."]
    #[json(default)]
    pub amount: i64,
    #[doc = "ID of the balance transaction that describes the impact of this top-up on your account balance."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<crate::Expandable<crate::balance_transactions::BalanceTransaction>>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Date the funds are expected to arrive in your Stripe account for payouts."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expected_availability_date: Option<i64>,
    #[doc = "Error code explaining reason for top-up failure if available (see [the errors section](/api/errors) for a list of cod..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[doc = "Message to user further explaining reason for top-up failure if available."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "Indicates whether the top-up was initiated by Stripe or by the user."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub initiated_by: Option<stripe_types_02::topups::TopupInitiatedBy>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::topups::TopupObject,
    #[doc = "The ID of a PaymentMethod representing the payment method used for the top-up."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[doc = "Payment-method-specific configuration for this top-up."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<stripe_types_08::topups::TopupResourcePaymentMethodOptions>,
    #[doc = "The source field is deprecated."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<stripe_types_10::sources::Source>,
    #[doc = "Extra information about a top-up."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[doc = "The status of the top-up is either `canceled`, `failed`, `pending`, `reversed`, or `succeeded`."]
    #[json(default)]
    pub status: stripe_types_00::topups::TopupStatus,
    #[doc = "A string that identifies this top-up as part of a group."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
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
impl crate::PagedItem for Topup {
    fn id(&self) -> &str {
        &self.id
    }
}
