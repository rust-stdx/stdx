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
pub struct Review {
    #[doc = "The ZIP or postal code of the card used, if applicable."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_zip: Option<String>,
    #[doc = "The charge associated with this review."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub charge: Option<crate::Expandable<crate::charges::Charge>>,
    #[doc = "The reason the review was closed, or null if it has not yet been closed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub closed_reason: Option<stripe_types_03::reviews::ReviewClosedReason>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The IP address where the payment originated."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "Information related to the location of the payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ip_address_location: Option<stripe_types_03::radar::RadarReviewResourceLocation>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::reviews::ReviewObject,
    #[doc = "If `true`, the review needs action."]
    #[json(default)]
    pub open: bool,
    #[doc = "The reason the review was opened."]
    #[json(default)]
    pub opened_reason: stripe_types_03::reviews::ReviewOpenedReason,
    #[doc = "The PaymentIntent ID associated with this review, if one exists."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<crate::Expandable<crate::payment_intents::PaymentIntent>>,
    #[doc = "The reason the review is currently open or closed."]
    #[json(default)]
    pub reason: String,
    #[doc = "Information related to the browsing session of the user who initiated the payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<stripe_types_02::radar::RadarReviewResourceSession>,
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
impl crate::PagedItem for Review {
    fn id(&self) -> &str {
        &self.id
    }
}
