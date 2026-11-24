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
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWallet {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<stripe_types_03::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWalletResourceApplePay>,
    #[doc = "(For tokenized numbers only.) The last four digits of the device account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dynamic_last4: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<stripe_types_03::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWalletResourceGooglePay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types_04::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWalletResourceLink>,
    #[doc = "The type of the card wallet, one of `apple_pay`, `google_pay`, or `link`."]
    #[json(rename = "type", default)]
    pub r#type: String,
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
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKlarnaDetailsResourcePayerDetails {
    #[doc = "The payer's address"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_02::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKlarnaDetailsResourcePayerDetailsResourcePayerDetailsAddress>,
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
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStore {
    #[doc = "The name of the convenience store chain where the payment was completed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub chain: Option<stripe_types_01::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain>,
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
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodRevolutPayDetailsResourceFunding {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_04::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodRevolutPayDetailsResourceFundingResourceFundingCard>,
    #[doc = "Funding type of the underlying payment method."]
    #[json(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<stripe_types_02::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodAmazonPayDetailsResourceFundingType>,
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
pub struct PaymentsPrimitivesPaymentRecordsResourceProcessorDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<
        stripe_types_02::payment_records::PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails,
    >,
    #[doc = "The processor used for this payment attempt."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::payment_records::PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType,
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
pub struct PaymentsPrimitivesPaymentRecordsResourceShippingDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_03::payment_records::PaymentsPrimitivesPaymentRecordsResourceAddress>,
    #[doc = "The shipping recipient's name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The shipping recipient's phone number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[cfg(feature = "payment_records")]
pub type ReportPaymentAttemptInformationalParamsShippingDetailsAddress =
    stripe_types_04::payment_records::ReportPaymentParamsPaymentMethodDetailsBillingDetailsAddress;

#[cfg(feature = "payment_records")]
pub type ReportPaymentAttemptParamsCanceled = stripe_types_01::payment_records::ReportPaymentParamsCanceled;

#[cfg(feature = "payment_records")]
pub type ReportPaymentAttemptParamsFailed = stripe_types_01::payment_records::ReportPaymentParamsFailed;

#[cfg(feature = "payment_records")]
pub type ReportPaymentAttemptParamsPaymentMethodDetailsCustom =
    stripe_types_01::payment_records::ReportPaymentParamsPaymentMethodDetailsCustom;

#[cfg(feature = "payment_records")]
pub type ReportPaymentAttemptParamsPaymentMethodDetailsType =
    stripe_types_01::payment_records::ReportPaymentParamsPaymentMethodDetailsType;

#[cfg(feature = "payment_records")]
pub type ReportPaymentAttemptParamsShippingDetailsAddress =
    stripe_types_04::payment_records::ReportPaymentParamsPaymentMethodDetailsBillingDetailsAddress;

#[cfg(feature = "payment_records")]
pub type ReportPaymentParamsProcessorDetailsType =
    stripe_types_01::payment_records::ReportPaymentParamsPaymentMethodDetailsType;

#[cfg(feature = "payment_records")]
pub type ReportPaymentParamsShippingDetailsAddress =
    stripe_types_04::payment_records::ReportPaymentParamsPaymentMethodDetailsBillingDetailsAddress;

#[cfg(feature = "payment_records")]
pub type ReportRefundParamsProcessorDetailsType =
    stripe_types_01::payment_records::ReportPaymentParamsPaymentMethodDetailsType;
