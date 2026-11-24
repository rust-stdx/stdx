#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct CreateIssuingCardParamsSpendingControls {
    #[doc = "Array of card presence statuses from which authorizations will be allowed."]
    pub allowed_card_presences:
        Option<Vec<stripe_types_05::issuing::CreateIssuingCardParamsSpendingControlsAllowedCardPresencesItem>>,
    #[doc = "Array of strings containing categories..."]
    pub allowed_categories:
        Option<Vec<stripe_types_06::issuing::CreateIssuingCardParamsSpendingControlsAllowedCategoriesItem>>,
    #[doc = "Array of strings containing representing countries from which authorizations will be allowed."]
    pub allowed_merchant_countries: Option<Vec<String>>,
    #[doc = "Array of card presence statuses from which authorizations will be declined."]
    pub blocked_card_presences:
        Option<Vec<stripe_types_06::issuing::CreateIssuingCardParamsSpendingControlsBlockedCardPresencesItem>>,
    #[doc = "Array of strings containing categories..."]
    pub blocked_categories:
        Option<Vec<stripe_types_05::issuing::CreateIssuingCardParamsSpendingControlsBlockedCategoriesItem>>,
    #[doc = "Array of strings containing representing countries from which authorizations will be declined."]
    pub blocked_merchant_countries: Option<Vec<String>>,
    #[doc = "Limit spending with amount-based rules that apply across any cards this card replaced (i.e., its `replacement_for` ca..."]
    pub spending_limits:
        Option<Vec<stripe_types_09::issuing::CreateIssuingCardParamsSpendingControlsSpendingLimitsItem>>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardParamsSpendingControls {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.allowed_card_presences.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("allowed_card_presences", index, item);
            }
        }
        if let Some(values) = self.allowed_categories.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("allowed_categories", index, item);
            }
        }
        if let Some(values) = self.allowed_merchant_countries.as_ref() {
            form.field_array("allowed_merchant_countries", values.as_slice());
        }
        if let Some(values) = self.blocked_card_presences.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("blocked_card_presences", index, item);
            }
        }
        if let Some(values) = self.blocked_categories.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("blocked_categories", index, item);
            }
        }
        if let Some(values) = self.blocked_merchant_countries.as_ref() {
            form.field_array("blocked_merchant_countries", values.as_slice());
        }
        if let Some(values) = self.spending_limits.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("spending_limits", index, item);
            }
        }
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct CreateIssuingCardholderParamsSpendingControls {
    #[doc = "Array of card presence statuses from which authorizations will be allowed."]
    pub allowed_card_presences:
        Option<Vec<stripe_types_03::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCardPresencesItem>>,
    #[doc = "Array of strings containing categories..."]
    pub allowed_categories:
        Option<Vec<stripe_types_01::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem>>,
    #[doc = "Array of strings containing representing countries from which authorizations will be allowed."]
    pub allowed_merchant_countries: Option<Vec<String>>,
    #[doc = "Array of card presence statuses from which authorizations will be declined."]
    pub blocked_card_presences:
        Option<Vec<stripe_types_06::issuing::CreateIssuingCardholderParamsSpendingControlsBlockedCardPresencesItem>>,
    #[doc = "Array of strings containing categories..."]
    pub blocked_categories:
        Option<Vec<stripe_types_05::issuing::CreateIssuingCardholderParamsSpendingControlsBlockedCategoriesItem>>,
    #[doc = "Array of strings containing representing countries from which authorizations will be declined."]
    pub blocked_merchant_countries: Option<Vec<String>>,
    #[doc = "Limit spending with amount-based rules that apply across this cardholder's cards."]
    pub spending_limits:
        Option<Vec<stripe_types_08::issuing::CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItem>>,
    #[doc = "Currency of amounts within `spending_limits`."]
    pub spending_limits_currency: Option<String>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardholderParamsSpendingControls {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.allowed_card_presences.as_ref() {
            form.field_array("allowed_card_presences", values.as_slice());
        }
        if let Some(values) = self.allowed_categories.as_ref() {
            form.field_array("allowed_categories", values.as_slice());
        }
        if let Some(values) = self.allowed_merchant_countries.as_ref() {
            form.field_array("allowed_merchant_countries", values.as_slice());
        }
        if let Some(values) = self.blocked_card_presences.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("blocked_card_presences", index, item);
            }
        }
        if let Some(values) = self.blocked_categories.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("blocked_categories", index, item);
            }
        }
        if let Some(values) = self.blocked_merchant_countries.as_ref() {
            form.field_array("blocked_merchant_countries", values.as_slice());
        }
        if let Some(values) = self.spending_limits.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("spending_limits", index, item);
            }
        }
        form.field_opt("spending_limits_currency", self.spending_limits_currency.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct CreateIssuingDisputeParamsEvidence {
    #[doc = "Evidence provided when `reason` is 'canceled'."]
    pub canceled: Option<stripe_types_06::issuing::CreateIssuingDisputeParamsEvidenceCanceled>,
    #[doc = "Evidence provided when `reason` is 'duplicate'."]
    pub duplicate: Option<stripe_types_01::issuing::CreateIssuingDisputeParamsEvidenceDuplicate>,
    #[doc = "Evidence provided when `reason` is 'fraudulent'."]
    pub fraudulent: Option<stripe_types_02::issuing::CreateIssuingDisputeParamsEvidenceFraudulent>,
    #[doc = "Evidence provided when `reason` is 'merchandise_not_as_described'."]
    pub merchandise_not_as_described:
        Option<stripe_types_09::issuing::CreateIssuingDisputeParamsEvidenceMerchandiseNotAsDescribed>,
    #[doc = "Evidence provided when `reason` is 'no_valid_authorization'."]
    pub no_valid_authorization:
        Option<stripe_types_05::issuing::CreateIssuingDisputeParamsEvidenceNoValidAuthorization>,
    #[doc = "Evidence provided when `reason` is 'not_received'."]
    pub not_received: Option<stripe_types_07::issuing::CreateIssuingDisputeParamsEvidenceNotReceived>,
    #[doc = "Evidence provided when `reason` is 'other'."]
    pub other: Option<stripe_types_07::issuing::CreateIssuingDisputeParamsEvidenceOther>,
    #[doc = "The reason for filing the dispute."]
    pub reason: Option<stripe_types_02::issuing::CreateIssuingDisputeParamsEvidenceReason>,
    #[doc = "Evidence provided when `reason` is 'service_not_as_described'."]
    pub service_not_as_described:
        Option<stripe_types_00::issuing::CreateIssuingDisputeParamsEvidenceServiceNotAsDescribed>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingDisputeParamsEvidence {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("canceled", self.canceled.as_ref());
        form.field_opt("duplicate", self.duplicate.as_ref());
        form.field_opt("fraudulent", self.fraudulent.as_ref());
        form.field_opt("merchandise_not_as_described", self.merchandise_not_as_described.as_ref());
        form.field_opt("no_valid_authorization", self.no_valid_authorization.as_ref());
        form.field_opt("not_received", self.not_received.as_ref());
        form.field_opt("other", self.other.as_ref());
        form.field_opt("reason", self.reason.as_ref());
        form.field_opt("service_not_as_described", self.service_not_as_described.as_ref());
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
pub struct IssuingCardholderVerification {
    #[doc = "An identifying document, either a passport or local ID card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<stripe_types_07::issuing::IssuingCardholderIdDocument>,
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
pub struct IssuingDisputeEvidence {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub canceled: Option<stripe_types_09::issuing::IssuingDisputeCanceledEvidence>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<stripe_types_08::issuing::IssuingDisputeDuplicateEvidence>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fraudulent: Option<stripe_types_09::issuing::IssuingDisputeFraudulentEvidence>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub merchandise_not_as_described: Option<stripe_types_07::issuing::IssuingDisputeMerchandiseNotAsDescribedEvidence>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub no_valid_authorization: Option<stripe_types_08::issuing::IssuingDisputeNoValidAuthorizationEvidence>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub not_received: Option<stripe_types_09::issuing::IssuingDisputeNotReceivedEvidence>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub other: Option<stripe_types_07::issuing::IssuingDisputeOtherEvidence>,
    #[doc = "The reason for filing the dispute."]
    #[json(default)]
    pub reason: stripe_types_02::issuing::IssuingDisputeEvidenceReason,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub service_not_as_described: Option<stripe_types_07::issuing::IssuingDisputeServiceNotAsDescribedEvidence>,
}
