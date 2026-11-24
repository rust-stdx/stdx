#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct CreateIssuingDisputeParamsEvidenceNotReceived {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute."]
    pub additional_documentation: Option<json::OwnedValue>,
    #[doc = "Date when the cardholder expected to receive the product."]
    pub expected_at: Option<json::OwnedValue>,
    #[doc = "Explanation of why the cardholder is disputing this transaction."]
    pub explanation: Option<json::OwnedValue>,
    #[doc = "Description of the merchandise or service that was purchased."]
    pub product_description: Option<json::OwnedValue>,
    #[doc = "Whether the product was a merchandise or service."]
    pub product_type: Option<stripe_types_06::issuing::CreateIssuingDisputeParamsEvidenceNotReceivedProductType>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingDisputeParamsEvidenceNotReceived {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("product_type", self.product_type.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct CreateIssuingDisputeParamsEvidenceOther {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute."]
    pub additional_documentation: Option<json::OwnedValue>,
    #[doc = "Explanation of why the cardholder is disputing this transaction."]
    pub explanation: Option<json::OwnedValue>,
    #[doc = "Description of the merchandise or service that was purchased."]
    pub product_description: Option<json::OwnedValue>,
    #[doc = "Whether the product was a merchandise or service."]
    pub product_type: Option<stripe_types_05::issuing::CreateIssuingDisputeParamsEvidenceOtherProductType>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingDisputeParamsEvidenceOther {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("product_type", self.product_type.as_ref());
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingAuthorizationVerificationData {
    #[doc = "Whether the cardholder provided an address first line and if it matched the cardholder’s `billing.address.line1`."]
    #[json(default)]
    pub address_line1_check: stripe_types_02::issuing::IssuingAuthorizationVerificationDataAddressLine1Check,
    #[doc = "Whether the cardholder provided a postal code and if it matched the cardholder’s `billing.address.postal_code`."]
    #[json(default)]
    pub address_postal_code_check: stripe_types_02::issuing::IssuingAuthorizationVerificationDataAddressLine1Check,
    #[doc = "The exemption applied to this authorization."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub authentication_exemption: Option<stripe_types_05::issuing::IssuingAuthorizationAuthenticationExemption>,
    #[doc = "Whether the cardholder provided a CVC and if it matched Stripe’s record."]
    #[json(default)]
    pub cvc_check: stripe_types_02::issuing::IssuingAuthorizationVerificationDataAddressLine1Check,
    #[doc = "Whether the cardholder provided an expiry date and if it matched Stripe’s record."]
    #[json(default)]
    pub expiry_check: stripe_types_02::issuing::IssuingAuthorizationVerificationDataAddressLine1Check,
    #[doc = "The postal code submitted as part of the authorization used for postal code verification."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "3D Secure details."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<stripe_types_06::issuing::IssuingAuthorizationThreeDSecure>,
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
pub struct IssuingCardAuthorizationControls {
    #[doc = "Array of card presence statuses from which authorizations will be allowed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub allowed_card_presences: Option<Vec<stripe_types_01::issuing::IssuingAuthorizationCardPresence>>,
    #[doc = "Array of strings containing categories..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub allowed_categories:
        Option<Vec<stripe_types_03::issuing::IssuingCardAuthorizationControlsAllowedCategoriesItem>>,
    #[doc = "Array of strings containing representing countries from which authorizations will be allowed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub allowed_merchant_countries: Option<Vec<String>>,
    #[doc = "Array of card presence statuses from which authorizations will be declined."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blocked_card_presences: Option<Vec<stripe_types_01::issuing::IssuingAuthorizationCardPresence>>,
    #[doc = "Array of strings containing categories..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blocked_categories:
        Option<Vec<stripe_types_03::issuing::IssuingCardAuthorizationControlsAllowedCategoriesItem>>,
    #[doc = "Array of strings containing representing countries from which authorizations will be declined."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blocked_merchant_countries: Option<Vec<String>>,
    #[doc = "Limit spending with amount-based rules that apply across any cards this card replaced (i.e., its `replacement_for` ca..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub spending_limits: Option<Vec<stripe_types_05::issuing::IssuingCardSpendingLimit>>,
    #[doc = "Currency of the amounts within `spending_limits`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub spending_limits_currency: Option<String>,
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
pub struct IssuingCardWallets {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<stripe_types_05::issuing::IssuingCardApplePay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<stripe_types_06::issuing::IssuingCardGooglePay>,
    #[doc = "Unique identifier for a card used with digital wallets"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub primary_account_identifier: Option<String>,
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
pub struct IssuingCardholderAuthorizationControls {
    #[doc = "Array of card presence statuses from which authorizations will be allowed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub allowed_card_presences: Option<Vec<stripe_types_01::issuing::IssuingAuthorizationCardPresence>>,
    #[doc = "Array of strings containing categories..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub allowed_categories:
        Option<Vec<stripe_types_03::issuing::IssuingCardAuthorizationControlsAllowedCategoriesItem>>,
    #[doc = "Array of strings containing representing countries from which authorizations will be allowed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub allowed_merchant_countries: Option<Vec<String>>,
    #[doc = "Array of card presence statuses from which authorizations will be declined."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blocked_card_presences: Option<Vec<stripe_types_01::issuing::IssuingAuthorizationCardPresence>>,
    #[doc = "Array of strings containing categories..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blocked_categories:
        Option<Vec<stripe_types_03::issuing::IssuingCardAuthorizationControlsAllowedCategoriesItem>>,
    #[doc = "Array of strings containing representing countries from which authorizations will be declined."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blocked_merchant_countries: Option<Vec<String>>,
    #[doc = "Limit spending with amount-based rules that apply across this cardholder's cards."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub spending_limits: Option<Vec<stripe_types_05::issuing::IssuingCardholderSpendingLimit>>,
    #[doc = "Currency of the amounts within `spending_limits`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub spending_limits_currency: Option<String>,
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
pub struct IssuingCardholderIdDocument {
    #[doc = "The back of a document returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value of..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub back: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "The front of a document returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value o..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub front: Option<crate::Expandable<stripe_types_06::files::File>>,
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
pub struct IssuingDisputeMerchandiseNotAsDescribedEvidence {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "Explanation of why the cardholder is disputing this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[doc = "Date when the product was received."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub received_at: Option<i64>,
    #[doc = "Description of the cardholder's attempt to return the product."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub return_description: Option<String>,
    #[doc = "Result of cardholder's attempt to return the product."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub return_status: Option<stripe_types_03::issuing::IssuingDisputeCanceledEvidenceReturnStatus>,
    #[doc = "Date when the product was returned or attempted to be returned."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<i64>,
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
pub struct IssuingDisputeOtherEvidence {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "Explanation of why the cardholder is disputing this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[doc = "Description of the merchandise or service that was purchased."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[doc = "Whether the product was a merchandise or service."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<stripe_types_04::issuing::IssuingDisputeCanceledEvidenceProductType>,
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
pub struct IssuingDisputeServiceNotAsDescribedEvidence {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "Date when order was canceled."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    #[doc = "Reason for canceling the order."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    #[doc = "Explanation of why the cardholder is disputing this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[doc = "Date when the product was received."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub received_at: Option<i64>,
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
pub struct IssuingPhysicalBundle {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<stripe_types_05::issuing::IssuingPhysicalBundleFeatures>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Friendly display name."]
    #[json(default)]
    pub name: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::issuing::IssuingPhysicalBundleObject,
    #[doc = "Whether this physical bundle can be used to create cards."]
    #[json(default)]
    pub status: stripe_types_00::issuing::IssuingPhysicalBundleStatus,
    #[doc = "Whether this physical bundle is a standard Stripe offering or custom-made for you."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::issuing::IssuingPhysicalBundleType,
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
impl crate::PagedItem for IssuingPhysicalBundle {
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingTransactionFleetData {
    #[doc = "Answers to prompts presented to cardholder at point of sale."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cardholder_prompt_data: Option<stripe_types_00::issuing::IssuingTransactionFleetCardholderPromptData>,
    #[doc = "The type of purchase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub purchase_type: Option<String>,
    #[doc = "More information about the total amount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reported_breakdown: Option<stripe_types_05::issuing::IssuingTransactionFleetReportedBreakdown>,
    #[doc = "The type of fuel service."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct ListIssuingCardsParams {
    pub cardholder: Option<String>,
    pub created: Option<stripe_types_06::issuing::ListIssuingCardsParamsCreated>,
    pub ending_before: Option<String>,
    pub exp_month: Option<i64>,
    pub exp_year: Option<i64>,
    pub expand: Option<Vec<String>>,
    pub last4: Option<String>,
    pub limit: Option<i64>,
    pub personalization_design: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_04::issuing::ListIssuingCardsParamsStatus>,
    pub r#type: Option<stripe_types_01::issuing::ListIssuingCardsParamsType>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for ListIssuingCardsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cardholder", self.cardholder.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        form.field_opt("exp_month", self.exp_month.as_ref());
        form.field_opt("exp_year", self.exp_year.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("last4", self.last4.as_ref());
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("personalization_design", self.personalization_design.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct ListIssuingDisputesParams {
    pub created: Option<stripe_types_06::issuing::ListIssuingDisputesParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_01::issuing::ListIssuingDisputesParamsStatus>,
    pub transaction: Option<String>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for ListIssuingDisputesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
        form.field_opt("transaction", self.transaction.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone)]
pub struct UpdateIssuingCardParamsSpendingControlsSpendingLimitsItem {
    #[doc = "Maximum amount allowed to spend per interval."]
    pub amount: i64,
    #[doc = "Array of strings containing categories..."]
    pub categories:
        Option<Vec<stripe_types_06::issuing::UpdateIssuingCardParamsSpendingControlsSpendingLimitsItemCategoriesItem>>,
    #[doc = "Interval (or event) to which the amount applies."]
    pub interval: stripe_types_06::issuing::UpdateIssuingCardParamsSpendingControlsSpendingLimitsItemInterval,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingCardParamsSpendingControlsSpendingLimitsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        if let Some(values) = self.categories.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("categories", index, item);
            }
        }
        form.field("interval", &self.interval);
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct UpdateIssuingCardholderParamsIndividualCardIssuing {
    #[doc = "Information about cardholder acceptance of Celtic Authorized User Terms..."]
    pub user_terms_acceptance:
        Option<stripe_types_05::issuing::UpdateIssuingCardholderParamsIndividualCardIssuingUserTermsAcceptance>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingCardholderParamsIndividualCardIssuing {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("user_terms_acceptance", self.user_terms_acceptance.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct UpdateIssuingDisputeParamsEvidenceCanceled {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute."]
    pub additional_documentation: Option<json::OwnedValue>,
    #[doc = "Date when order was canceled."]
    pub canceled_at: Option<json::OwnedValue>,
    #[doc = "Whether the cardholder was provided with a cancellation policy."]
    pub cancellation_policy_provided: Option<json::OwnedValue>,
    #[doc = "Reason for canceling the order."]
    pub cancellation_reason: Option<json::OwnedValue>,
    #[doc = "Date when the cardholder expected to receive the product."]
    pub expected_at: Option<json::OwnedValue>,
    #[doc = "Explanation of why the cardholder is disputing this transaction."]
    pub explanation: Option<json::OwnedValue>,
    #[doc = "Description of the merchandise or service that was purchased."]
    pub product_description: Option<json::OwnedValue>,
    #[doc = "Whether the product was a merchandise or service."]
    pub product_type: Option<stripe_types_06::issuing::UpdateIssuingDisputeParamsEvidenceCanceledProductType>,
    #[doc = "Result of cardholder's attempt to return the product."]
    pub return_status: Option<stripe_types_05::issuing::UpdateIssuingDisputeParamsEvidenceCanceledReturnStatus>,
    #[doc = "Date when the product was returned or attempted to be returned."]
    pub returned_at: Option<json::OwnedValue>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingDisputeParamsEvidenceCanceled {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("product_type", self.product_type.as_ref());
        form.field_opt("return_status", self.return_status.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct UpdateIssuingPersonalizationDesignParams {
    #[doc = "The file for the card logo, for use with physical bundles that support card logos."]
    pub card_logo: Option<json::OwnedValue>,
    #[doc = "Hash containing carrier text, for use with physical bundles that support carrier text."]
    pub carrier_text: Option<stripe_types_06::issuing::UpdateIssuingPersonalizationDesignParamsCarrierText>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A lookup key used to retrieve personalization designs dynamically from a static string."]
    pub lookup_key: Option<json::OwnedValue>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Friendly display name."]
    pub name: Option<json::OwnedValue>,
    #[doc = "The physical bundle object belonging to this personalization design."]
    pub physical_bundle: Option<String>,
    #[doc = "Information on whether this personalization design is used to create cards when one is not specified."]
    pub preferences: Option<stripe_types_05::issuing::UpdateIssuingPersonalizationDesignParamsPreferences>,
    #[doc = "If set to true, will atomically remove the lookup key from the existing personalization design, and assign it to this..."]
    pub transfer_lookup_key: Option<bool>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingPersonalizationDesignParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("carrier_text", self.carrier_text.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("physical_bundle", self.physical_bundle.as_ref());
        form.field_opt("preferences", self.preferences.as_ref());
        form.field_opt("transfer_lookup_key", self.transfer_lookup_key.as_ref());
    }
}
