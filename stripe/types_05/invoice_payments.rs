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
pub struct InvoicePaymentMethodOptionsAcssDebitMandateOptions {
    #[doc = "Transaction type of the mandate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<stripe_types_04::invoice_payments::InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
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
pub struct InvoicePaymentMethodOptionsBillieCompanyDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registered_address:
        Option<stripe_types_02::invoice_payments::InvoicePaymentMethodOptionsBillieCompanyDetailsRegisteredAddress>,
    #[doc = "Company or entity name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registered_name: Option<String>,
    #[doc = "The official registration number for the given registration type."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    #[doc = "Type of registration the company or entity holds in their registered country."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registration_type:
        Option<stripe_types_02::invoice_payments::InvoicePaymentMethodOptionsBillieCompanyDetailsRegistrationType>,
    #[doc = "VAT ID number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub vat: Option<String>,
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
pub struct InvoicePaymentMethodOptionsCard {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub installments: Option<stripe_types_04::invoices::InvoiceInstallmentsCard>,
    #[doc = "We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<stripe_types_02::invoice_payments::InvoicePaymentMethodOptionsCardRequestThreeDSecure>,
}
