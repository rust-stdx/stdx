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
pub struct ApplicationFee {
    #[doc = "ID of the Stripe account this fee was taken from."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<crate::Expandable<crate::account::Account>>,
    #[doc = "Amount earned, in cents (or local equivalent)."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Amount in cents (or local equivalent) refunded (can be less than the amount attribute on the fee if a partial refund ..."]
    #[json(default)]
    pub amount_refunded: i64,
    #[doc = "ID of the Connect application that earned the fee."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<crate::Expandable<stripe_types_05::common::Application>>,
    #[doc = "Balance transaction that describes the impact of this collected application fee on your account balance (not includin..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<crate::Expandable<crate::balance_transactions::BalanceTransaction>>,
    #[doc = "ID of the charge that the application fee was taken from."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub charge: Option<crate::Expandable<crate::charges::Charge>>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "Polymorphic source of the application fee."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fee_source: Option<stripe_types_06::application_fees::PlatformEarningFeeSource>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::application_fees::ApplicationFeeObject,
    #[doc = "ID of the corresponding charge on the platform account, if this fee was the result of a charge using the `destination..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub originating_transaction: Option<crate::Expandable<crate::charges::Charge>>,
    #[doc = "Whether the fee has been fully refunded."]
    #[json(default)]
    pub refunded: bool,
    #[doc = "A list of refunds that have been applied to the fee."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refunds: Option<crate::List<crate::common::FeeRefund>>,
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
impl crate::PagedItem for ApplicationFee {
    fn id(&self) -> &str {
        &self.id
    }
}
