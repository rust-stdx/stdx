#[cfg(feature = "issuing")]
#[derive(Debug, Clone)]
pub struct CreateIssuingCardParamsShippingAddressValidation {
    #[doc = "The address validation capabilities to use."]
    pub mode: stripe_types_02::issuing::CreateIssuingCardParamsShippingAddressValidationMode,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardParamsShippingAddressValidation {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("mode", &self.mode);
    }
}

#[cfg(feature = "issuing")]
pub type CreateIssuingCardParamsSpendingControlsAllowedCardPresencesItem =
    stripe_types_03::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCardPresencesItem;

#[cfg(feature = "issuing")]
pub type CreateIssuingCardParamsSpendingControlsBlockedCategoriesItem =
    stripe_types_01::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem;

#[cfg(feature = "issuing")]
pub type CreateIssuingCardParamsSpendingControlsSpendingLimitsItemCategoriesItem =
    stripe_types_01::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem;

#[cfg(feature = "issuing")]
pub type CreateIssuingCardParamsSpendingControlsSpendingLimitsItemInterval =
    stripe_types_01::issuing::CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval;

#[cfg(feature = "issuing")]
pub type CreateIssuingCardParamsStatus = stripe_types_04::issuing::CreateIssuingCardholderParamsStatus;

#[cfg(feature = "issuing")]
#[derive(Debug, Clone)]
pub struct CreateIssuingCardholderParamsBilling {
    #[doc = "The cardholder’s billing address."]
    pub address: stripe_types_04::issuing::CreateIssuingCardholderParamsBillingAddress,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardholderParamsBilling {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct CreateIssuingCardholderParamsIndividualCardIssuing {
    #[doc = "Information about cardholder acceptance of Celtic Authorized User Terms..."]
    pub user_terms_acceptance:
        Option<stripe_types_02::issuing::CreateIssuingCardholderParamsIndividualCardIssuingUserTermsAcceptance>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardholderParamsIndividualCardIssuing {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("user_terms_acceptance", self.user_terms_acceptance.as_ref());
    }
}

#[cfg(feature = "issuing")]
pub type CreateIssuingCardholderParamsSpendingControlsBlockedCategoriesItem =
    stripe_types_01::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem;

#[cfg(feature = "issuing")]
pub type CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemCategoriesItem =
    stripe_types_01::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem;

#[cfg(feature = "issuing")]
pub type CreateIssuingCardholderParamsType = stripe_types_00::issuing::ListIssuingCardholdersParamsType;

#[cfg(feature = "issuing")]
pub type CreateIssuingDisputeParamsEvidenceMerchandiseNotAsDescribedReturnStatus =
    stripe_types_00::issuing::CreateIssuingDisputeParamsEvidenceCanceledReturnStatus;

#[cfg(feature = "issuing")]
pub type CreateIssuingDisputeParamsEvidenceNoValidAuthorization =
    stripe_types_02::issuing::CreateIssuingDisputeParamsEvidenceFraudulent;

#[cfg(feature = "issuing")]
pub type CreateIssuingDisputeParamsEvidenceOtherProductType =
    stripe_types_04::issuing::CreateIssuingDisputeParamsEvidenceCanceledProductType;

#[cfg(feature = "issuing")]
pub type GetIssuingCardholderParams = stripe_types_04::issuing::GetIssuingAuthorizationParams;

#[cfg(feature = "issuing")]
pub type GetIssuingDisputeParams = stripe_types_04::issuing::GetIssuingAuthorizationParams;

#[cfg(feature = "issuing")]
pub type GetIssuingPersonalizationDesignParams = stripe_types_04::issuing::GetIssuingAuthorizationParams;

#[cfg(feature = "issuing")]
pub type GetIssuingPhysicalBundleParams = stripe_types_04::issuing::GetIssuingAuthorizationParams;

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
pub struct IssuingAuthorizationAuthenticationExemption {
    #[doc = "The entity that requested the exemption, either the acquiring merchant or the Issuing user."]
    #[json(default)]
    pub claimed_by: stripe_types_02::issuing::IssuingAuthorizationAuthenticationExemptionClaimedBy,
    #[doc = "The specific exemption claimed for this authorization."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::issuing::IssuingAuthorizationAuthenticationExemptionType,
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
pub struct IssuingAuthorizationFraudChallenge {
    #[doc = "The method by which the fraud challenge was delivered to the cardholder."]
    #[json(default)]
    pub channel: stripe_types_04::issuing::IssuingAuthorizationFraudChallengeChannel,
    #[doc = "The status of the fraud challenge."]
    #[json(default)]
    pub status: stripe_types_01::issuing::IssuingAuthorizationFraudChallengeStatus,
    #[doc = "If the challenge is not deliverable, the reason why."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub undeliverable_reason: Option<stripe_types_02::issuing::IssuingAuthorizationFraudChallengeUndeliverableReason>,
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
pub struct IssuingAuthorizationPendingRequest {
    #[doc = "The additional amount Stripe will hold if the authorization is approved, in the card's currency..."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Detailed breakdown of amount components."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_details: Option<stripe_types_01::issuing::IssuingAuthorizationAmountDetails>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "If set `true`, you may provide amount..."]
    #[json(default)]
    pub is_amount_controllable: bool,
    #[doc = "The amount the merchant is requesting to be authorized in the `merchant_currency`."]
    #[json(default)]
    pub merchant_amount: i64,
    #[doc = "The local currency the merchant is requesting to authorize."]
    #[json(default)]
    pub merchant_currency: String,
    #[doc = "The card network's estimate of the likelihood that an authorization is fraudulent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_risk_score: Option<i64>,
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
pub struct IssuingCardApplePay {
    #[doc = "Apple Pay Eligibility"]
    #[json(default)]
    pub eligible: bool,
    #[doc = "Reason the card is ineligible for Apple Pay"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ineligible_reason: Option<stripe_types_03::issuing::IssuingCardApplePayIneligibleReason>,
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
pub struct IssuingCardFraudWarning {
    #[doc = "Timestamp of the most recent fraud warning."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[doc = "The type of fraud warning that most recently took place on this card."]
    #[json(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<stripe_types_04::issuing::IssuingCardFraudWarningType>,
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
pub struct IssuingCardLifecycleControls {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cancel_after: Option<stripe_types_03::issuing::IssuingCardLifecycleConditions>,
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
pub struct IssuingCardSpendingLimit {
    #[doc = "Maximum amount allowed to spend per interval."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Array of strings containing categories..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<stripe_types_03::issuing::IssuingCardAuthorizationControlsAllowedCategoriesItem>>,
    #[doc = "Interval (or event) to which the amount applies."]
    #[json(default)]
    pub interval: stripe_types_01::issuing::IssuingCardSpendingLimitInterval,
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
pub struct IssuingCardholderAddress {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::common::Address>,
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
pub struct IssuingCardholderCardIssuing {
    #[doc = "Information about cardholder acceptance of Celtic Authorized User Terms..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub user_terms_acceptance: Option<stripe_types_02::issuing::IssuingCardholderUserTermsAcceptance>,
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
pub struct IssuingCardholderRequirements {
    #[doc = "If `disabled_reason` is present, all cards will decline authorizations with `cardholder_verification_required` reason."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<stripe_types_02::issuing::IssuingCardholderRequirementsDisabledReason>,
    #[doc = "Array of fields that need to be collected in order to verify and re-enable the cardholder."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub past_due: Option<Vec<stripe_types_03::issuing::IssuingCardholderRequirementsPastDueItem>>,
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
pub struct IssuingCardholderSpendingLimit {
    #[doc = "Maximum amount allowed to spend per interval."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Array of strings containing categories..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<stripe_types_03::issuing::IssuingCardAuthorizationControlsAllowedCategoriesItem>>,
    #[doc = "Interval (or event) to which the amount applies."]
    #[json(default)]
    pub interval: stripe_types_01::issuing::IssuingCardSpendingLimitInterval,
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
pub struct IssuingNetworkTokenDevice {
    #[doc = "An obfuscated ID derived from the device ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub device_fingerprint: Option<String>,
    #[doc = "The IP address of the device at provisioning time."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The geographic latitude/longitude coordinates of the device at provisioning time."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The name of the device used for tokenization."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The phone number of the device used for tokenization."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[doc = "The type of device used for tokenization."]
    #[json(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<stripe_types_02::issuing::IssuingNetworkTokenDeviceType>,
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
pub struct IssuingNetworkTokenWalletProvider {
    #[doc = "The wallet provider-given account ID of the digital wallet the token belongs to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "An evaluation on the trustworthiness of the wallet account between 1 and 5."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_trust_score: Option<i64>,
    #[doc = "The method used for tokenizing a card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_number_source: Option<stripe_types_02::issuing::IssuingNetworkTokenWalletProviderCardNumberSource>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cardholder_address: Option<stripe_types_04::issuing::IssuingNetworkTokenAddress>,
    #[doc = "The name of the cardholder tokenizing the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<String>,
    #[doc = "An evaluation on the trustworthiness of the device."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub device_trust_score: Option<i64>,
    #[doc = "The hashed email address of the cardholder's account with the wallet provider."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hashed_account_email_address: Option<String>,
    #[doc = "The reasons for suggested tokenization given by the card network."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason_codes: Option<Vec<stripe_types_04::issuing::IssuingNetworkTokenWalletProviderReasonCodesItem>>,
    #[doc = "The recommendation on responding to the tokenization request."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub suggested_decision: Option<stripe_types_03::issuing::IssuingNetworkTokenWalletProviderSuggestedDecision>,
    #[doc = "The version of the standard for mapping reason codes followed by the wallet provider."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub suggested_decision_version: Option<String>,
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
pub struct IssuingPhysicalBundleFeatures {
    #[doc = "The policy for how to use card logo images in a card design with this physical bundle."]
    #[json(default)]
    pub card_logo: stripe_types_04::issuing::IssuingPhysicalBundleFeaturesCardLogo,
    #[doc = "The policy for how to use carrier letter text in a card design with this physical bundle."]
    #[json(default)]
    pub carrier_text: stripe_types_04::issuing::IssuingPhysicalBundleFeaturesCardLogo,
    #[doc = "The policy for how to use a second line on a card with this physical bundle."]
    #[json(default)]
    pub second_line: stripe_types_04::issuing::IssuingPhysicalBundleFeaturesCardLogo,
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
pub struct IssuingTransactionFleetReportedBreakdown {
    #[doc = "Breakdown of fuel portion of the purchase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fuel: Option<stripe_types_01::issuing::IssuingTransactionFleetFuelPriceData>,
    #[doc = "Breakdown of non-fuel portion of the purchase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub non_fuel: Option<stripe_types_03::issuing::IssuingTransactionFleetNonFuelPriceData>,
    #[doc = "Information about tax included in this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<stripe_types_04::issuing::IssuingTransactionFleetTaxData>,
}

#[cfg(feature = "issuing")]
pub type ListIssuingCardholdersParamsCreated = stripe_types_04::issuing::ListIssuingAuthorizationsParamsCreated;

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct ListIssuingPersonalizationDesignsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub lookup_keys: Option<Vec<String>>,
    pub preferences: Option<stripe_types_04::issuing::ListIssuingPersonalizationDesignsParamsPreferences>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_03::issuing::ListIssuingPersonalizationDesignsParamsStatus>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for ListIssuingPersonalizationDesignsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        if let Some(values) = self.lookup_keys.as_ref() {
            form.field_array("lookup_keys", values.as_slice());
        }
        form.field_opt("preferences", self.preferences.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct ListIssuingPhysicalBundlesParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_01::issuing::ListIssuingPhysicalBundlesParamsStatus>,
    pub r#type: Option<stripe_types_00::issuing::ListIssuingPhysicalBundlesParamsType>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for ListIssuingPhysicalBundlesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "issuing")]
pub type ListIssuingTransactionsParamsCreated = stripe_types_04::issuing::ListIssuingAuthorizationsParamsCreated;

#[cfg(feature = "issuing")]
pub type SubmitIssuingDisputeParams = stripe_types_01::issuing::UpdateIssuingAuthorizationParams;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardParamsShippingAddressValidationMode =
    stripe_types_02::issuing::CreateIssuingCardParamsShippingAddressValidationMode;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardParamsShippingCustoms = stripe_types_03::issuing::CreateIssuingCardParamsShippingCustoms;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardParamsShippingType = stripe_types_02::issuing::CreateIssuingCardParamsShippingType;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardParamsSpendingControlsBlockedCardPresencesItem =
    stripe_types_03::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCardPresencesItem;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardParamsSpendingControlsBlockedCategoriesItem =
    stripe_types_01::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardParamsStatus = stripe_types_04::issuing::ListIssuingCardsParamsStatus;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardholderParamsBillingAddress =
    stripe_types_04::issuing::CreateIssuingCardholderParamsBillingAddress;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardholderParamsIndividualCardIssuingUserTermsAcceptance =
    stripe_types_02::issuing::CreateIssuingCardholderParamsIndividualCardIssuingUserTermsAcceptance;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardholderParamsIndividualDob =
    stripe_types_02::issuing::CreateIssuingCardholderParamsIndividualDob;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardholderParamsIndividualVerificationDocument =
    stripe_types_02::issuing::CreateIssuingCardholderParamsIndividualVerificationDocument;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardholderParamsSpendingControlsBlockedCardPresencesItem =
    stripe_types_03::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCardPresencesItem;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardholderParamsSpendingControlsBlockedCategoriesItem =
    stripe_types_01::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval =
    stripe_types_01::issuing::CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval;

#[cfg(feature = "issuing")]
pub type UpdateIssuingDisputeParamsEvidenceCanceledReturnStatus =
    stripe_types_00::issuing::CreateIssuingDisputeParamsEvidenceCanceledReturnStatus;

#[cfg(feature = "issuing")]
pub type UpdateIssuingDisputeParamsEvidenceFraudulent =
    stripe_types_02::issuing::CreateIssuingDisputeParamsEvidenceFraudulent;

#[cfg(feature = "issuing")]
pub type UpdateIssuingDisputeParamsEvidenceMerchandiseNotAsDescribedReturnStatus =
    stripe_types_00::issuing::CreateIssuingDisputeParamsEvidenceCanceledReturnStatus;

#[cfg(feature = "issuing")]
pub type UpdateIssuingDisputeParamsEvidenceNoValidAuthorization =
    stripe_types_02::issuing::CreateIssuingDisputeParamsEvidenceFraudulent;

#[cfg(feature = "issuing")]
pub type UpdateIssuingDisputeParamsEvidenceNotReceivedProductType =
    stripe_types_04::issuing::CreateIssuingDisputeParamsEvidenceCanceledProductType;

#[cfg(feature = "issuing")]
pub type UpdateIssuingDisputeParamsEvidenceReason = stripe_types_02::issuing::CreateIssuingDisputeParamsEvidenceReason;

#[cfg(feature = "issuing")]
pub type UpdateIssuingPersonalizationDesignParamsPreferences =
    stripe_types_03::issuing::CreateIssuingPersonalizationDesignParamsPreferences;

#[cfg(feature = "issuing")]
pub type UpdateIssuingTransactionParams = stripe_types_01::issuing::UpdateIssuingAuthorizationParams;
