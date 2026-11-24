#[cfg(feature = "disputes")]
pub type CloseDisputeParams = stripe_types_01::disputes::GetDisputeParams;

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
pub struct DisputeEnhancedEligibilityMastercardCompliance {
    #[doc = "Mastercard compliance eligibility status."]
    #[json(default)]
    pub status: stripe_types_00::disputes::DisputeEnhancedEligibilityMastercardComplianceStatus,
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
pub struct DisputeEnhancedEligibilityVisaCompellingEvidence3 {
    #[doc = "List of actions required to qualify dispute for Visa Compelling Evidence 3.0 evidence submission."]
    #[json(default)]
    pub required_actions:
        Vec<stripe_types_00::disputes::DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActionsItem>,
    #[doc = "Visa Compelling Evidence 3.0 eligibility status."]
    #[json(default)]
    pub status: stripe_types_03::disputes::DisputeEnhancedEligibilityVisaCompellingEvidence3Status,
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
pub struct DisputeEnhancedEligibilityVisaCompliance {
    #[doc = "Visa compliance eligibility status."]
    #[json(default)]
    pub status: stripe_types_00::disputes::DisputeEnhancedEligibilityMastercardComplianceStatus,
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
pub struct DisputePaymentMethodDetailsAmazonPay {
    #[doc = "The AmazonPay dispute type, chargeback or claim"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dispute_type: Option<stripe_types_04::disputes::DisputePaymentMethodDetailsAmazonPayDisputeType>,
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
pub struct DisputePaymentMethodDetailsCard {
    #[doc = "Card brand."]
    #[json(default)]
    pub brand: String,
    #[doc = "The type of dispute opened."]
    #[json(default)]
    pub case_type: stripe_types_00::disputes::DisputePaymentMethodDetailsCardCaseType,
    #[doc = "Identifies which network this charge was processed on."]
    #[json(default)]
    pub network: String,
    #[doc = "The card network's specific dispute reason code, which maps to one of Stripe's primary dispute categories to simplify..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_reason_code: Option<String>,
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
pub struct DisputeVisaCompellingEvidence3PriorUndisputedTransaction {
    #[doc = "Stripe charge ID for the Visa Compelling Evidence 3.0 eligible prior charge."]
    #[json(default)]
    pub charge: String,
    #[doc = "User Account ID used to log into business platform."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account_id: Option<String>,
    #[doc = "Unique identifier of the cardholder’s device derived from a combination of at least two hardware and software attribu..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_device_fingerprint: Option<String>,
    #[doc = "Unique identifier of the cardholder’s device such as a device serial number (e.g., International Mobile Equipment Ide..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_device_id: Option<String>,
    #[doc = "The email address of the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_email_address: Option<String>,
    #[doc = "The IP address that the customer used when making the purchase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_purchase_ip: Option<String>,
    #[doc = "A description of the product or service that was sold."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[doc = "The address to which a physical product was shipped."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<stripe_types_03::disputes::DisputeTransactionShippingAddress>,
}

#[cfg(feature = "disputes")]
#[derive(Debug, Clone, Default)]
pub struct UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransaction {
    #[doc = "User Account ID used to log into business platform."]
    pub customer_account_id: Option<json::OwnedValue>,
    #[doc = "Unique identifier of the cardholder’s device derived from a combination of at least two hardware and software attribu..."]
    pub customer_device_fingerprint: Option<json::OwnedValue>,
    #[doc = "Unique identifier of the cardholder’s device such as a device serial number (e.g., International Mobile Equipment Ide..."]
    pub customer_device_id: Option<json::OwnedValue>,
    #[doc = "The email address of the customer."]
    pub customer_email_address: Option<json::OwnedValue>,
    #[doc = "The IP address that the customer used when making the purchase."]
    pub customer_purchase_ip: Option<json::OwnedValue>,
    #[doc = "Categorization of disputed payment."]
    pub merchandise_or_services: Option<stripe_types_01::disputes::UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices>,
    #[doc = "A description of the product or service that was sold."]
    pub product_description: Option<json::OwnedValue>,
    #[doc = "The address to which a physical product was shipped."]
    pub shipping_address: Option<stripe_types_01::disputes::UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionShippingAddress>,
}

#[cfg(feature = "disputes")]
impl crate::params::FormEncode
    for UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransaction
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("merchandise_or_services", self.merchandise_or_services.as_ref());
        form.field_opt("shipping_address", self.shipping_address.as_ref());
    }
}
