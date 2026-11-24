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
pub struct IssuingPersonalizationDesign {
    #[doc = "The file for the card logo to use with physical bundles that support card logos."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_logo: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "Hash containing carrier text, for use with physical bundles that support carrier text."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub carrier_text: Option<stripe_types_02::issuing::IssuingPersonalizationDesignCarrierText>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "A lookup key used to retrieve personalization designs dynamically from a static string."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub lookup_key: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "Friendly display name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::issuing::IssuingPersonalizationDesignObject,
    #[doc = "The physical bundle object belonging to this personalization design."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub physical_bundle: Option<crate::Expandable<stripe_types_07::issuing::IssuingPhysicalBundle>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<stripe_types_03::issuing::IssuingPersonalizationDesignPreferences>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub rejection_reasons: Option<stripe_types_06::issuing::IssuingPersonalizationDesignRejectionReasons>,
    #[doc = "Whether this personalization design can be used to create cards."]
    #[json(default)]
    pub status: stripe_types_04::issuing::IssuingPersonalizationDesignStatus,
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
impl crate::PagedItem for IssuingPersonalizationDesign {
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
pub struct IssuingTransactionPurchaseDetails {
    #[doc = "Fleet-specific information for transactions using Fleet cards."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fleet: Option<stripe_types_07::issuing::IssuingTransactionFleetData>,
    #[doc = "Information about the flight that was purchased with this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub flight: Option<stripe_types_06::issuing::IssuingTransactionFlightData>,
    #[doc = "Information about fuel that was purchased with this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fuel: Option<stripe_types_01::issuing::IssuingTransactionFuelData>,
    #[doc = "Information about lodging that was purchased with this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub lodging: Option<stripe_types_01::issuing::IssuingTransactionLodgingData>,
    #[doc = "The line items in the purchase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub receipt: Option<Vec<stripe_types_04::issuing::IssuingTransactionReceiptData>>,
    #[doc = "A merchant-specific order number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone)]
pub struct UpdateIssuingCardParamsShipping {
    #[doc = "The address that the card is shipped to."]
    pub address: stripe_types_06::issuing::UpdateIssuingCardParamsShippingAddress,
    #[doc = "Address validation settings."]
    pub address_validation: Option<stripe_types_08::issuing::UpdateIssuingCardParamsShippingAddressValidation>,
    #[doc = "The name of the business at the shipping address, used on the shipping label to ensure delivery when the card is ship..."]
    pub business_name: Option<String>,
    #[doc = "Customs information for the shipment."]
    pub customs: Option<stripe_types_05::issuing::UpdateIssuingCardParamsShippingCustoms>,
    #[doc = "The name printed on the shipping label when shipping the card."]
    pub name: String,
    #[doc = "Phone number of the recipient of the shipment."]
    pub phone_number: Option<String>,
    #[doc = "Whether a signature is required for card delivery."]
    pub require_signature: Option<bool>,
    #[doc = "Shipment service."]
    pub service: Option<stripe_types_06::issuing::UpdateIssuingCardParamsShippingService>,
    #[doc = "Packaging options."]
    pub r#type: Option<stripe_types_05::issuing::UpdateIssuingCardParamsShippingType>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingCardParamsShipping {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
        form.field_opt("address_validation", self.address_validation.as_ref());
        form.field_opt("business_name", self.business_name.as_ref());
        form.field_opt("customs", self.customs.as_ref());
        form.field("name", &self.name);
        form.field_opt("phone_number", self.phone_number.as_ref());
        form.field_opt("require_signature", self.require_signature.as_ref());
        form.field_opt("service", self.service.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct UpdateIssuingCardParamsSpendingControls {
    #[doc = "Array of card presence statuses from which authorizations will be allowed."]
    pub allowed_card_presences:
        Option<Vec<stripe_types_06::issuing::UpdateIssuingCardParamsSpendingControlsAllowedCardPresencesItem>>,
    #[doc = "Array of strings containing categories..."]
    pub allowed_categories:
        Option<Vec<stripe_types_06::issuing::UpdateIssuingCardParamsSpendingControlsAllowedCategoriesItem>>,
    #[doc = "Array of strings containing representing countries from which authorizations will be allowed."]
    pub allowed_merchant_countries: Option<Vec<String>>,
    #[doc = "Array of card presence statuses from which authorizations will be declined."]
    pub blocked_card_presences:
        Option<Vec<stripe_types_05::issuing::UpdateIssuingCardParamsSpendingControlsBlockedCardPresencesItem>>,
    #[doc = "Array of strings containing categories..."]
    pub blocked_categories:
        Option<Vec<stripe_types_05::issuing::UpdateIssuingCardParamsSpendingControlsBlockedCategoriesItem>>,
    #[doc = "Array of strings containing representing countries from which authorizations will be declined."]
    pub blocked_merchant_countries: Option<Vec<String>>,
    #[doc = "Limit spending with amount-based rules that apply across any cards this card replaced (i.e., its `replacement_for` ca..."]
    pub spending_limits:
        Option<Vec<stripe_types_07::issuing::UpdateIssuingCardParamsSpendingControlsSpendingLimitsItem>>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingCardParamsSpendingControls {
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
pub struct UpdateIssuingCardholderParamsIndividual {
    #[doc = "Information related to the card_issuing program for this cardholder."]
    pub card_issuing: Option<stripe_types_07::issuing::UpdateIssuingCardholderParamsIndividualCardIssuing>,
    #[doc = "The date of birth of this cardholder."]
    pub dob: Option<stripe_types_05::issuing::UpdateIssuingCardholderParamsIndividualDob>,
    #[doc = "The first name of this cardholder."]
    pub first_name: Option<String>,
    #[doc = "The last name of this cardholder."]
    pub last_name: Option<String>,
    #[doc = "Government-issued ID document for this cardholder."]
    pub verification: Option<stripe_types_08::issuing::UpdateIssuingCardholderParamsIndividualVerification>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingCardholderParamsIndividual {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("card_issuing", self.card_issuing.as_ref());
        form.field_opt("dob", self.dob.as_ref());
        form.field_opt("first_name", self.first_name.as_ref());
        form.field_opt("last_name", self.last_name.as_ref());
        form.field_opt("verification", self.verification.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct UpdateIssuingCardholderParamsSpendingControls {
    #[doc = "Array of card presence statuses from which authorizations will be allowed."]
    pub allowed_card_presences:
        Option<Vec<stripe_types_06::issuing::UpdateIssuingCardholderParamsSpendingControlsAllowedCardPresencesItem>>,
    #[doc = "Array of strings containing categories..."]
    pub allowed_categories:
        Option<Vec<stripe_types_06::issuing::UpdateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem>>,
    #[doc = "Array of strings containing representing countries from which authorizations will be allowed."]
    pub allowed_merchant_countries: Option<Vec<String>>,
    #[doc = "Array of card presence statuses from which authorizations will be declined."]
    pub blocked_card_presences:
        Option<Vec<stripe_types_05::issuing::UpdateIssuingCardholderParamsSpendingControlsBlockedCardPresencesItem>>,
    #[doc = "Array of strings containing categories..."]
    pub blocked_categories:
        Option<Vec<stripe_types_05::issuing::UpdateIssuingCardholderParamsSpendingControlsBlockedCategoriesItem>>,
    #[doc = "Array of strings containing representing countries from which authorizations will be declined."]
    pub blocked_merchant_countries: Option<Vec<String>>,
    #[doc = "Limit spending with amount-based rules that apply across this cardholder's cards."]
    pub spending_limits:
        Option<Vec<stripe_types_09::issuing::UpdateIssuingCardholderParamsSpendingControlsSpendingLimitsItem>>,
    #[doc = "Currency of amounts within `spending_limits`."]
    pub spending_limits_currency: Option<String>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingCardholderParamsSpendingControls {
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
        form.field_opt("spending_limits_currency", self.spending_limits_currency.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct UpdateIssuingDisputeParamsEvidence {
    #[doc = "Evidence provided when `reason` is 'canceled'."]
    pub canceled: Option<stripe_types_07::issuing::UpdateIssuingDisputeParamsEvidenceCanceled>,
    #[doc = "Evidence provided when `reason` is 'duplicate'."]
    pub duplicate: Option<stripe_types_06::issuing::UpdateIssuingDisputeParamsEvidenceDuplicate>,
    #[doc = "Evidence provided when `reason` is 'fraudulent'."]
    pub fraudulent: Option<stripe_types_05::issuing::UpdateIssuingDisputeParamsEvidenceFraudulent>,
    #[doc = "Evidence provided when `reason` is 'merchandise_not_as_described'."]
    pub merchandise_not_as_described:
        Option<stripe_types_08::issuing::UpdateIssuingDisputeParamsEvidenceMerchandiseNotAsDescribed>,
    #[doc = "Evidence provided when `reason` is 'no_valid_authorization'."]
    pub no_valid_authorization:
        Option<stripe_types_05::issuing::UpdateIssuingDisputeParamsEvidenceNoValidAuthorization>,
    #[doc = "Evidence provided when `reason` is 'not_received'."]
    pub not_received: Option<stripe_types_09::issuing::UpdateIssuingDisputeParamsEvidenceNotReceived>,
    #[doc = "Evidence provided when `reason` is 'other'."]
    pub other: Option<stripe_types_09::issuing::UpdateIssuingDisputeParamsEvidenceOther>,
    #[doc = "The reason for filing the dispute."]
    pub reason: Option<stripe_types_05::issuing::UpdateIssuingDisputeParamsEvidenceReason>,
    #[doc = "Evidence provided when `reason` is 'service_not_as_described'."]
    pub service_not_as_described:
        Option<stripe_types_06::issuing::UpdateIssuingDisputeParamsEvidenceServiceNotAsDescribed>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingDisputeParamsEvidence {
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
