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
pub struct MandateAcssDebit {
    #[doc = "List of Stripe products where this mandate can be selected automatically."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_for: Option<Vec<stripe_types_03::mandates::MandateAcssDebitDefaultForItem>>,
    #[doc = "Description of the interval."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,
    #[doc = "Payment schedule for the mandate."]
    #[json(default)]
    pub payment_schedule: stripe_types_01::mandates::MandateAcssDebitPaymentSchedule,
    #[doc = "Transaction type of the mandate."]
    #[json(default)]
    pub transaction_type: stripe_types_02::mandates::MandateAcssDebitTransactionType,
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct MandateBacsDebit {
    #[doc = "The display name for the account on this mandate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The status of the mandate on the Bacs network."]
    #[json(default)]
    pub network_status: stripe_types_02::mandates::MandateBacsDebitNetworkStatus,
    #[doc = "The unique reference identifying the mandate on the Bacs network."]
    #[json(default)]
    pub reference: String,
    #[doc = "When the mandate is revoked on the Bacs network this field displays the reason for the revocation."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<stripe_types_04::mandates::MandateBacsDebitRevocationReason>,
    #[doc = "The service user number for the account on this mandate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub service_user_number: Option<String>,
    #[doc = "The URL that will contain the mandate that the customer has signed."]
    #[json(default)]
    pub url: String,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct MandateOptionsPayto {
    #[doc = "Amount that will be collected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[doc = "The type of amount that will be collected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<stripe_types_00::mandates::MandateOptionsPaytoAmountType>,
    #[doc = "Date, in YYYY-MM-DD format, after which payments will not be collected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[doc = "The periodicity at which payments will be collected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<stripe_types_02::mandates::MandateOptionsPaytoPaymentSchedule>,
    #[doc = "The number of payments that will be made during a payment period."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payments_per_period: Option<i64>,
    #[doc = "The purpose for which payments are made."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<stripe_types_01::mandates::MandateOptionsPaytoPurpose>,
    #[doc = "Date, in YYYY-MM-DD format, from which payments will be collected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct MandateOptionsUpi {
    #[doc = "Amount to be charged for future payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<stripe_types_00::mandates::MandateOptionsPaytoAmountType>,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i64>,
}
