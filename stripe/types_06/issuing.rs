#[cfg(feature = "issuing")]
#[derive(Debug, Clone)]
pub struct CreateIssuingCardParamsLifecycleControls {
    #[doc = "Cancels the card after the specified conditions are met."]
    pub cancel_after: stripe_types_03::issuing::CreateIssuingCardParamsLifecycleControlsCancelAfter,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardParamsLifecycleControls {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("cancel_after", &self.cancel_after);
    }
}

#[cfg(feature = "issuing")]
pub type CreateIssuingCardParamsShippingAddress = stripe_types_04::issuing::CreateIssuingCardholderParamsBillingAddress;

#[cfg(feature = "issuing")]
pub type CreateIssuingCardParamsSpendingControlsAllowedCategoriesItem =
    stripe_types_01::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem;

#[cfg(feature = "issuing")]
pub type CreateIssuingCardParamsSpendingControlsBlockedCardPresencesItem =
    stripe_types_03::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCardPresencesItem;

#[cfg(feature = "issuing")]
pub type CreateIssuingCardParamsType = stripe_types_01::issuing::ListIssuingCardsParamsType;

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct CreateIssuingCardholderParamsIndividualVerification {
    #[doc = "An identifying document, either a passport or local ID card."]
    pub document: Option<stripe_types_02::issuing::CreateIssuingCardholderParamsIndividualVerificationDocument>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardholderParamsIndividualVerification {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("document", self.document.as_ref());
    }
}

#[cfg(feature = "issuing")]
pub type CreateIssuingCardholderParamsSpendingControlsBlockedCardPresencesItem =
    stripe_types_03::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCardPresencesItem;

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct CreateIssuingDisputeParamsEvidenceCanceled {
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
    pub product_type: Option<stripe_types_04::issuing::CreateIssuingDisputeParamsEvidenceCanceledProductType>,
    #[doc = "Result of cardholder's attempt to return the product."]
    pub return_status: Option<stripe_types_00::issuing::CreateIssuingDisputeParamsEvidenceCanceledReturnStatus>,
    #[doc = "Date when the product was returned or attempted to be returned."]
    pub returned_at: Option<json::OwnedValue>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingDisputeParamsEvidenceCanceled {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("product_type", self.product_type.as_ref());
        form.field_opt("return_status", self.return_status.as_ref());
    }
}

#[cfg(feature = "issuing")]
pub type CreateIssuingDisputeParamsEvidenceNotReceivedProductType =
    stripe_types_04::issuing::CreateIssuingDisputeParamsEvidenceCanceledProductType;

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct CreateIssuingPersonalizationDesignParams {
    #[doc = "The file for the card logo, for use with physical bundles that support card logos."]
    pub card_logo: Option<String>,
    #[doc = "Hash containing carrier text, for use with physical bundles that support carrier text."]
    pub carrier_text: Option<stripe_types_03::issuing::CreateIssuingPersonalizationDesignParamsCarrierText>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A lookup key used to retrieve personalization designs dynamically from a static string."]
    pub lookup_key: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Friendly display name."]
    pub name: Option<String>,
    #[doc = "The physical bundle object belonging to this personalization design."]
    pub physical_bundle: String,
    #[doc = "Information on whether this personalization design is used to create cards when one is not specified."]
    pub preferences: Option<stripe_types_03::issuing::CreateIssuingPersonalizationDesignParamsPreferences>,
    #[doc = "If set to true, will atomically remove the lookup key from the existing personalization design, and assign it to this..."]
    pub transfer_lookup_key: Option<bool>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingPersonalizationDesignParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("card_logo", self.card_logo.as_ref());
        form.field_opt("carrier_text", self.carrier_text.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("lookup_key", self.lookup_key.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("name", self.name.as_ref());
        form.field("physical_bundle", &self.physical_bundle);
        form.field_opt("preferences", self.preferences.as_ref());
        form.field_opt("transfer_lookup_key", self.transfer_lookup_key.as_ref());
    }
}

#[cfg(feature = "issuing")]
pub type DeclineIssuingAuthorizationParams = stripe_types_01::issuing::UpdateIssuingAuthorizationParams;

#[cfg(feature = "issuing")]
pub type GetIssuingCardParams = stripe_types_04::issuing::GetIssuingAuthorizationParams;

#[cfg(feature = "issuing")]
pub type GetIssuingTokenParams = stripe_types_04::issuing::GetIssuingAuthorizationParams;

#[cfg(feature = "issuing")]
pub type GetIssuingTransactionParams = stripe_types_04::issuing::GetIssuingAuthorizationParams;

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
pub struct IssuingAuthorizationFleetReportedBreakdown {
    #[doc = "Breakdown of fuel portion of the purchase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fuel: Option<stripe_types_00::issuing::IssuingAuthorizationFleetFuelPriceData>,
    #[doc = "Breakdown of non-fuel portion of the purchase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub non_fuel: Option<stripe_types_00::issuing::IssuingAuthorizationFleetNonFuelPriceData>,
    #[doc = "Information about tax included in this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<stripe_types_00::issuing::IssuingAuthorizationFleetTaxData>,
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
pub struct IssuingAuthorizationFuelData {
    #[doc = "[Conexxus Payment System Product Code](https://www.conexxus.org/conexxus-payment-system-product-codes) identifying th..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub industry_product_code: Option<String>,
    #[doc = "The quantity of `unit`s of fuel that was dispensed, represented as a decimal string with at most 12 decimal places."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub quantity_decimal: Option<String>,
    #[doc = "The type of fuel that was purchased."]
    #[json(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<stripe_types_04::issuing::IssuingAuthorizationFuelDataType>,
    #[doc = "The units for `quantity_decimal`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<stripe_types_04::issuing::IssuingAuthorizationFuelDataUnit>,
    #[doc = "The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub unit_cost_decimal: Option<String>,
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
pub struct IssuingAuthorizationRequest {
    #[doc = "The `pending_request.amount` at the time of the request, presented in your card's currency and in the smallest curre..."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Detailed breakdown of amount components."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_details: Option<stripe_types_01::issuing::IssuingAuthorizationAmountDetails>,
    #[doc = "Whether this request was approved."]
    #[json(default)]
    pub approved: bool,
    #[doc = "A code created by Stripe which is shared with the merchant to validate the authorization."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "The `pending_request.merchant_amount` at the time of the request, presented in the `merchant_currency` and in the sm..."]
    #[json(default)]
    pub merchant_amount: i64,
    #[doc = "The currency that was collected by the merchant and presented to the cardholder for the authorization."]
    #[json(default)]
    pub merchant_currency: String,
    #[doc = "The card network's estimate of the likelihood that an authorization is fraudulent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_risk_score: Option<i64>,
    #[doc = "When an authorization is approved or declined by you or by Stripe, this field provides additional detail on the reaso..."]
    #[json(default)]
    pub reason: stripe_types_01::issuing::IssuingAuthorizationRequestReason,
    #[doc = "If the `request_history.reason` is `webhook_error` because the direct webhook response is invalid (for example, parsi..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason_message: Option<String>,
    #[doc = "Time when the card network received an authorization request from the acquirer in UTC."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<i64>,
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
pub struct IssuingAuthorizationThreeDSecure {
    #[doc = "The outcome of the 3D Secure authentication request."]
    #[json(default)]
    pub result: stripe_types_04::issuing::IssuingAuthorizationThreeDSecureResult,
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
pub struct IssuingCardGooglePay {
    #[doc = "Google Pay Eligibility"]
    #[json(default)]
    pub eligible: bool,
    #[doc = "Reason the card is ineligible for Google Pay"]
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
pub struct IssuingCardShippingAddressValidation {
    #[doc = "The address validation capabilities to use."]
    #[json(default)]
    pub mode: stripe_types_03::issuing::IssuingCardShippingAddressValidationMode,
    #[doc = "The normalized shipping address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub normalized_address: Option<stripe_types_01::common::Address>,
    #[doc = "The validation result for the shipping address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<stripe_types_01::issuing::IssuingCardShippingAddressValidationResult>,
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
pub struct IssuingPersonalizationDesignRejectionReasons {
    #[doc = "The reason(s) the card logo was rejected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_logo: Option<Vec<stripe_types_02::issuing::IssuingPersonalizationDesignRejectionReasonsCardLogoItem>>,
    #[doc = "The reason(s) the carrier text was rejected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub carrier_text:
        Option<Vec<stripe_types_04::issuing::IssuingPersonalizationDesignRejectionReasonsCarrierTextItem>>,
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
pub struct IssuingTransactionFlightData {
    #[doc = "The time that the flight departed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub departure_at: Option<i64>,
    #[doc = "The name of the passenger."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub passenger_name: Option<String>,
    #[doc = "Whether the ticket is refundable."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refundable: Option<bool>,
    #[doc = "The legs of the trip."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<stripe_types_03::issuing::IssuingTransactionFlightDataLeg>>,
    #[doc = "The travel agency that issued the ticket."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub travel_agency: Option<String>,
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct ListIssuingAuthorizationsParams {
    pub card: Option<String>,
    pub cardholder: Option<String>,
    pub created: Option<stripe_types_04::issuing::ListIssuingAuthorizationsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_04::issuing::ListIssuingAuthorizationsParamsStatus>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for ListIssuingAuthorizationsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("card", self.card.as_ref());
        form.field_opt("cardholder", self.cardholder.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}

#[cfg(feature = "issuing")]
pub type ListIssuingCardsParamsCreated = stripe_types_04::issuing::ListIssuingAuthorizationsParamsCreated;

#[cfg(feature = "issuing")]
pub type ListIssuingDisputesParamsCreated = stripe_types_04::issuing::ListIssuingAuthorizationsParamsCreated;

#[cfg(feature = "issuing")]
pub type ListIssuingTokensParamsCreated = stripe_types_04::issuing::ListIssuingAuthorizationsParamsCreated;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardParamsPin = stripe_types_04::issuing::CreateIssuingCardParamsPin;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardParamsShippingAddress = stripe_types_04::issuing::CreateIssuingCardholderParamsBillingAddress;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardParamsShippingService = stripe_types_02::issuing::CreateIssuingCardParamsShippingService;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardParamsSpendingControlsAllowedCardPresencesItem =
    stripe_types_03::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCardPresencesItem;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardParamsSpendingControlsAllowedCategoriesItem =
    stripe_types_01::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardParamsSpendingControlsSpendingLimitsItemCategoriesItem =
    stripe_types_01::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardParamsSpendingControlsSpendingLimitsItemInterval =
    stripe_types_01::issuing::CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardholderParamsCompany = stripe_types_04::issuing::CreateIssuingCardholderParamsCompany;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardholderParamsPreferredLocalesItem =
    stripe_types_01::issuing::CreateIssuingCardholderParamsPreferredLocalesItem;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardholderParamsSpendingControlsAllowedCardPresencesItem =
    stripe_types_03::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCardPresencesItem;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem =
    stripe_types_01::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardholderParamsSpendingControlsSpendingLimitsItemCategoriesItem =
    stripe_types_01::issuing::CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem;

#[cfg(feature = "issuing")]
pub type UpdateIssuingCardholderParamsStatus = stripe_types_04::issuing::CreateIssuingCardholderParamsStatus;

#[cfg(feature = "issuing")]
pub type UpdateIssuingDisputeParamsEvidenceCanceledProductType =
    stripe_types_04::issuing::CreateIssuingDisputeParamsEvidenceCanceledProductType;

#[cfg(feature = "issuing")]
pub type UpdateIssuingDisputeParamsEvidenceDuplicate =
    stripe_types_01::issuing::CreateIssuingDisputeParamsEvidenceDuplicate;

#[cfg(feature = "issuing")]
pub type UpdateIssuingDisputeParamsEvidenceOtherProductType =
    stripe_types_04::issuing::CreateIssuingDisputeParamsEvidenceCanceledProductType;

#[cfg(feature = "issuing")]
pub type UpdateIssuingDisputeParamsEvidenceServiceNotAsDescribed =
    stripe_types_00::issuing::CreateIssuingDisputeParamsEvidenceServiceNotAsDescribed;

#[cfg(feature = "issuing")]
pub type UpdateIssuingPersonalizationDesignParamsCarrierText =
    stripe_types_03::issuing::CreateIssuingPersonalizationDesignParamsCarrierText;

#[cfg(feature = "issuing")]
#[derive(Debug, Clone)]
pub struct UpdateIssuingTokenParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Specifies which status the token should be updated to."]
    pub status: stripe_types_00::issuing::UpdateIssuingTokenParamsStatus,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingTokenParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("status", &self.status);
    }
}
