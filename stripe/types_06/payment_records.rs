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
pub struct PaymentsPrimitivesPaymentRecordsResourceBillingDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_03::payment_records::PaymentsPrimitivesPaymentRecordsResourceAddress>,
    #[doc = "The billing email associated with the method of payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The billing name associated with the method of payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The billing phone number associated with the method of payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
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
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodAmazonPayDetailsResourceFunding {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_03::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodAmazonPayDetailsResourceFundingResourceFundingCard>,
    #[doc = "funding type of the underlying payment method."]
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
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks {
    #[doc = "If you provide a value for `address.line1`, the check result is one of `pass`, `fail`, `unavailable`, or `unchecked`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_line1_check: Option<stripe_types_00::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check>,
    #[doc = "If you provide a address postal code, the check result is one of `pass`, `fail`, `unavailable`, or `unchecked`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_postal_code_check: Option<stripe_types_00::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check>,
    #[doc = "If you provide a CVC, the check results is one of `pass`, `fail`, `unavailable`, or `unchecked`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<stripe_types_00::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check>,
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
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallmentPlan {
    #[doc = "For `fixed_count` installment plans, this is the number of installment payments your customer will make to their cred..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "For `fixed_count` installment plans, this is the interval between installment payments your customer will make to the..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<stripe_types_03::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallmentPlanInterval>,
    #[doc = "Type of installment plan, one of `fixed_count`, `revolving`, or `bonus`."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallmentPlanType,
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
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceThreeDSecure {
    #[doc = "For authenticated transactions: Indicates how the issuing bank authenticated the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub authentication_flow: Option<stripe_types_01::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceThreeDSecureAuthenticationFlow>,
    #[doc = "The Electronic Commerce Indicator (ECI)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub electronic_commerce_indicator: Option<stripe_types_00::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceThreeDSecureElectronicCommerceIndicator>,
    #[doc = "The exemption requested via 3DS and accepted by the issuer at authentication time."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub exemption_indicator: Option<stripe_types_02::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceThreeDSecureExemptionIndicator>,
    #[doc = "Whether Stripe requested the value of `exemption_indicator` in the transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub exemption_indicator_applied: Option<bool>,
    #[doc = "Indicates the outcome of 3D Secure authentication."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<stripe_types_00::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceThreeDSecureResult>,
    #[doc = "Additional information about why 3D Secure succeeded or failed, based on the `result`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub result_reason: Option<stripe_types_01::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceThreeDSecureResultReason>,
    #[doc = "The version of 3D Secure that was used."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<stripe_types_02::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceThreeDSecureVersion>,
}

#[cfg(feature = "payment_records")]
pub type ReportPaymentAttemptInformationalParamsCustomerDetails =
    stripe_types_01::payment_records::ReportPaymentParamsCustomerDetails;

#[cfg(feature = "payment_records")]
pub type ReportPaymentAttemptParamsGuaranteed = stripe_types_00::payment_records::ReportPaymentParamsGuaranteed;

#[cfg(feature = "payment_records")]
pub type ReportPaymentAttemptParamsOutcome = stripe_types_00::payment_records::ReportPaymentParamsOutcome;

#[cfg(feature = "payment_records")]
pub type ReportPaymentAttemptParamsPaymentMethodDetailsBillingDetailsAddress =
    stripe_types_04::payment_records::ReportPaymentParamsPaymentMethodDetailsBillingDetailsAddress;

#[cfg(feature = "payment_records")]
#[derive(Debug, Clone, Default)]
pub struct ReportPaymentParamsPaymentMethodDetailsBillingDetails {
    #[doc = "The billing address associated with the method of payment."]
    pub address: Option<stripe_types_04::payment_records::ReportPaymentParamsPaymentMethodDetailsBillingDetailsAddress>,
    #[doc = "The billing email associated with the method of payment."]
    pub email: Option<String>,
    #[doc = "The billing name associated with the method of payment."]
    pub name: Option<String>,
    #[doc = "The billing phone number associated with the method of payment."]
    pub phone: Option<String>,
}

#[cfg(feature = "payment_records")]
impl crate::params::FormEncode for ReportPaymentParamsPaymentMethodDetailsBillingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("email", self.email.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("phone", self.phone.as_ref());
    }
}

#[cfg(feature = "payment_records")]
pub type ReportRefundParamsAmount = stripe_types_03::payment_records::ReportPaymentParamsAmountRequested;
