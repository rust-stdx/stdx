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
pub struct Refund {
    #[doc = "Amount, in cents (or local equivalent)."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Balance transaction that describes the impact on your account balance."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<crate::Expandable<crate::balance_transactions::BalanceTransaction>>,
    #[doc = "ID of the charge that's refunded."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub charge: Option<crate::Expandable<crate::charges::Charge>>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "ID of the customer of this refund."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<crate::refunds::RefundCustomerExpanded>>,
    #[doc = "ID of the account of this refund."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub destination_details: Option<stripe_types_07::refunds::RefundDestinationDetails>,
    #[doc = "After the refund fails, this balance transaction describes the adjustment made on your account balance that reverses ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failure_balance_transaction: Option<crate::Expandable<crate::balance_transactions::BalanceTransaction>>,
    #[doc = "Provides the reason for the refund failure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "For payment methods without native refund support (for example, Konbini, PromptPay), provide an email address for the..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub instructions_email: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub next_action: Option<stripe_types_09::refunds::RefundNextAction>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::refunds::RefundObject,
    #[doc = "ID of the PaymentIntent that's refunded."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<crate::Expandable<crate::payment_intents::PaymentIntent>>,
    #[doc = "ID of the payment method associated with this refund."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[doc = "Provides the reason for why the refund is pending."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pending_reason: Option<stripe_types_00::refunds::RefundPendingReason>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub presentment_details: Option<stripe_types_04::payment_intents::PaymentFlowsPaymentIntentPresentmentDetails>,
    #[doc = "Reason for the refund, which is either user-provided (`duplicate`, `fraudulent`, or `requested_by_customer`) or gener..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<stripe_types_01::refunds::RefundReason>,
    #[doc = "This is the transaction number that appears on email receipts sent for this refund."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    #[doc = "The transfer reversal that's associated with the refund."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source_transfer_reversal: Option<crate::Expandable<crate::transfers::TransferReversal>>,
    #[doc = "Status of the refund."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "This refers to the transfer reversal object if the accompanying transfer reverses."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transfer_reversal: Option<crate::Expandable<crate::transfers::TransferReversal>>,
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
impl crate::PagedItem for Refund {
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
pub enum RefundCustomerExpanded {
    /// Expanded `customer`.
    Customer(Box<crate::customers::Customer>),
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
impl<'json> json::Decode<'json> for RefundCustomerExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "customer" => RefundCustomerExpanded::Customer(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            _ => RefundCustomerExpanded::Other(value),
        })
    }
}
