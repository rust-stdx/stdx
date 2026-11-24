#[cfg(feature = "issuing")]
#[derive(Debug, Clone)]
pub struct CreateIssuingCardParamsShipping {
    #[doc = "The address that the card is shipped to."]
    pub address: stripe_types_06::issuing::CreateIssuingCardParamsShippingAddress,
    #[doc = "Address validation settings."]
    pub address_validation: Option<stripe_types_05::issuing::CreateIssuingCardParamsShippingAddressValidation>,
    #[doc = "The name of the business at the shipping address, used on the shipping label to ensure delivery when the card is ship..."]
    pub business_name: Option<String>,
    #[doc = "Customs information for the shipment."]
    pub customs: Option<stripe_types_03::issuing::CreateIssuingCardParamsShippingCustoms>,
    #[doc = "The name printed on the shipping label when shipping the card."]
    pub name: String,
    #[doc = "Phone number of the recipient of the shipment."]
    pub phone_number: Option<String>,
    #[doc = "Whether a signature is required for card delivery."]
    pub require_signature: Option<bool>,
    #[doc = "Shipment service."]
    pub service: Option<stripe_types_02::issuing::CreateIssuingCardParamsShippingService>,
    #[doc = "Packaging options."]
    pub r#type: Option<stripe_types_02::issuing::CreateIssuingCardParamsShippingType>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardParamsShipping {
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
#[derive(Debug, Clone)]
pub struct CreateIssuingCardParamsSpendingControlsSpendingLimitsItem {
    #[doc = "Maximum amount allowed to spend per interval."]
    pub amount: i64,
    #[doc = "Array of strings containing categories..."]
    pub categories:
        Option<Vec<stripe_types_05::issuing::CreateIssuingCardParamsSpendingControlsSpendingLimitsItemCategoriesItem>>,
    #[doc = "Interval (or event) to which the amount applies."]
    pub interval: stripe_types_05::issuing::CreateIssuingCardParamsSpendingControlsSpendingLimitsItemInterval,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardParamsSpendingControlsSpendingLimitsItem {
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
pub struct CreateIssuingCardholderParamsIndividual {
    #[doc = "Information related to the card_issuing program for this cardholder."]
    pub card_issuing: Option<stripe_types_05::issuing::CreateIssuingCardholderParamsIndividualCardIssuing>,
    #[doc = "The date of birth of this cardholder."]
    pub dob: Option<stripe_types_02::issuing::CreateIssuingCardholderParamsIndividualDob>,
    #[doc = "The first name of this cardholder."]
    pub first_name: Option<String>,
    #[doc = "The last name of this cardholder."]
    pub last_name: Option<String>,
    #[doc = "Government-issued ID document for this cardholder."]
    pub verification: Option<stripe_types_06::issuing::CreateIssuingCardholderParamsIndividualVerification>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardholderParamsIndividual {
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
pub struct CreateIssuingDisputeParamsEvidenceMerchandiseNotAsDescribed {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute."]
    pub additional_documentation: Option<json::OwnedValue>,
    #[doc = "Explanation of why the cardholder is disputing this transaction."]
    pub explanation: Option<json::OwnedValue>,
    #[doc = "Date when the product was received."]
    pub received_at: Option<json::OwnedValue>,
    #[doc = "Description of the cardholder's attempt to return the product."]
    pub return_description: Option<json::OwnedValue>,
    #[doc = "Result of cardholder's attempt to return the product."]
    pub return_status:
        Option<stripe_types_05::issuing::CreateIssuingDisputeParamsEvidenceMerchandiseNotAsDescribedReturnStatus>,
    #[doc = "Date when the product was returned or attempted to be returned."]
    pub returned_at: Option<json::OwnedValue>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingDisputeParamsEvidenceMerchandiseNotAsDescribed {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("return_status", self.return_status.as_ref());
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
pub struct IssuingCardShipping {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::common::Address>,
    #[doc = "Address validation details for the shipment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_validation: Option<stripe_types_06::issuing::IssuingCardShippingAddressValidation>,
    #[doc = "The name of the business at the shipping address, used on the shipping label to ensure delivery when the card is ship..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[doc = "The delivery company that shipped a card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub carrier: Option<stripe_types_00::issuing::IssuingCardShippingCarrier>,
    #[doc = "Additional information that may be required for clearing customs."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customs: Option<stripe_types_02::issuing::IssuingCardShippingCustoms>,
    #[doc = "A unix timestamp representing a best estimate of when the card will be delivered."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eta: Option<i64>,
    #[doc = "Recipient name."]
    #[json(default)]
    pub name: String,
    #[doc = "The phone number of the receiver of the shipment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[doc = "Whether a signature is required for card delivery."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub require_signature: Option<bool>,
    #[doc = "Shipment service, such as `standard` or `express`."]
    #[json(default)]
    pub service: stripe_types_01::issuing::IssuingCardShippingService,
    #[doc = "The delivery status of the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_types_04::issuing::IssuingCardShippingStatus>,
    #[doc = "A tracking number for a card shipment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
    #[doc = "A link to the shipping carrier's site where you can view detailed information about a card shipment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
    #[doc = "Packaging options."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::issuing::IssuingCardShippingType,
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
pub struct IssuingDisputeCanceledEvidence {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "Date when order was canceled."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    #[doc = "Whether the cardholder was provided with a cancellation policy."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_provided: Option<bool>,
    #[doc = "Reason for canceling the order."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    #[doc = "Date when the cardholder expected to receive the product."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expected_at: Option<i64>,
    #[doc = "Explanation of why the cardholder is disputing this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[doc = "Description of the merchandise or service that was purchased."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[doc = "Whether the product was a merchandise or service."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<stripe_types_04::issuing::IssuingDisputeCanceledEvidenceProductType>,
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
pub struct IssuingDisputeFraudulentEvidence {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "Explanation of why the cardholder is disputing this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
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
pub struct IssuingDisputeNotReceivedEvidence {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "Date when the cardholder expected to receive the product."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expected_at: Option<i64>,
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
pub struct IssuingNetworkTokenNetworkData {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<stripe_types_05::issuing::IssuingNetworkTokenDevice>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mastercard: Option<stripe_types_00::issuing::IssuingNetworkTokenMastercard>,
    #[doc = "The network that the token is associated with."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::issuing::IssuingTokenNetwork,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub visa: Option<stripe_types_01::issuing::IssuingNetworkTokenVisa>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wallet_provider: Option<stripe_types_05::issuing::IssuingNetworkTokenWalletProvider>,
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct ListIssuingCardholdersParams {
    pub created: Option<stripe_types_05::issuing::ListIssuingCardholdersParamsCreated>,
    pub email: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub phone_number: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_00::issuing::ListIssuingCardholdersParamsStatus>,
    pub r#type: Option<stripe_types_00::issuing::ListIssuingCardholdersParamsType>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for ListIssuingCardholdersParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("email", self.email.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("phone_number", self.phone_number.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone)]
pub struct UpdateIssuingCardholderParamsSpendingControlsSpendingLimitsItem {
    #[doc = "Maximum amount allowed to spend per interval."]
    pub amount: i64,
    #[doc = "Array of strings containing categories..."]
    pub categories: Option<
        Vec<stripe_types_06::issuing::UpdateIssuingCardholderParamsSpendingControlsSpendingLimitsItemCategoriesItem>,
    >,
    #[doc = "Interval (or event) to which the amount applies."]
    pub interval: stripe_types_05::issuing::UpdateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingCardholderParamsSpendingControlsSpendingLimitsItem {
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
pub struct UpdateIssuingDisputeParamsEvidenceNotReceived {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute."]
    pub additional_documentation: Option<json::OwnedValue>,
    #[doc = "Date when the cardholder expected to receive the product."]
    pub expected_at: Option<json::OwnedValue>,
    #[doc = "Explanation of why the cardholder is disputing this transaction."]
    pub explanation: Option<json::OwnedValue>,
    #[doc = "Description of the merchandise or service that was purchased."]
    pub product_description: Option<json::OwnedValue>,
    #[doc = "Whether the product was a merchandise or service."]
    pub product_type: Option<stripe_types_05::issuing::UpdateIssuingDisputeParamsEvidenceNotReceivedProductType>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingDisputeParamsEvidenceNotReceived {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("product_type", self.product_type.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct UpdateIssuingDisputeParamsEvidenceOther {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute."]
    pub additional_documentation: Option<json::OwnedValue>,
    #[doc = "Explanation of why the cardholder is disputing this transaction."]
    pub explanation: Option<json::OwnedValue>,
    #[doc = "Description of the merchandise or service that was purchased."]
    pub product_description: Option<json::OwnedValue>,
    #[doc = "Whether the product was a merchandise or service."]
    pub product_type: Option<stripe_types_06::issuing::UpdateIssuingDisputeParamsEvidenceOtherProductType>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingDisputeParamsEvidenceOther {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("product_type", self.product_type.as_ref());
    }
}
