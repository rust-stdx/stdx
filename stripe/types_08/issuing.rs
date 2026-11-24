#[cfg(feature = "issuing")]
#[derive(Debug, Clone)]
pub struct CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItem {
    #[doc = "Maximum amount allowed to spend per interval."]
    pub amount: i64,
    #[doc = "Array of strings containing categories..."]
    pub categories: Option<
        Vec<stripe_types_05::issuing::CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemCategoriesItem>,
    >,
    #[doc = "Interval (or event) to which the amount applies."]
    pub interval: stripe_types_01::issuing::CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItem {
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
pub struct IssuingAuthorizationFleetData {
    #[doc = "Answers to prompts presented to the cardholder at the point of sale."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cardholder_prompt_data: Option<stripe_types_02::issuing::IssuingAuthorizationFleetCardholderPromptData>,
    #[doc = "The type of purchase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub purchase_type: Option<stripe_types_01::issuing::IssuingAuthorizationFleetDataPurchaseType>,
    #[doc = "More information about the total amount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reported_breakdown: Option<stripe_types_06::issuing::IssuingAuthorizationFleetReportedBreakdown>,
    #[doc = "The type of fuel service."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<stripe_types_02::issuing::IssuingAuthorizationFleetDataServiceType>,
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
pub struct IssuingDisputeDuplicateEvidence {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the card statement showing that the produ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_statement: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the receipt showing that the product had ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cash_receipt: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Image of the front and back of the check that was..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub check_image: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "Explanation of why the cardholder is disputing this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[doc = "Transaction (e.g., ipi_...) that the disputed transaction is a duplicate of."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub original_transaction: Option<String>,
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
pub struct IssuingDisputeNoValidAuthorizationEvidence {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "Explanation of why the cardholder is disputing this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct ListIssuingTokensParams {
    pub card: String,
    pub created: Option<stripe_types_06::issuing::ListIssuingTokensParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_02::issuing::ListIssuingTokensParamsStatus>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for ListIssuingTokensParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("card", &self.card);
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
#[derive(Debug, Clone, Default)]
pub struct ListIssuingTransactionsParams {
    pub card: Option<String>,
    pub cardholder: Option<String>,
    pub created: Option<stripe_types_05::issuing::ListIssuingTransactionsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub r#type: Option<stripe_types_01::issuing::ListIssuingTransactionsParamsType>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for ListIssuingTransactionsParams {
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
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone)]
pub struct UpdateIssuingCardParamsShippingAddressValidation {
    #[doc = "The address validation capabilities to use."]
    pub mode: stripe_types_05::issuing::UpdateIssuingCardParamsShippingAddressValidationMode,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingCardParamsShippingAddressValidation {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("mode", &self.mode);
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone)]
pub struct UpdateIssuingCardholderParamsBilling {
    #[doc = "The cardholder’s billing address."]
    pub address: stripe_types_05::issuing::UpdateIssuingCardholderParamsBillingAddress,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingCardholderParamsBilling {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct UpdateIssuingCardholderParamsIndividualVerification {
    #[doc = "An identifying document, either a passport or local ID card."]
    pub document: Option<stripe_types_05::issuing::UpdateIssuingCardholderParamsIndividualVerificationDocument>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingCardholderParamsIndividualVerification {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("document", self.document.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct UpdateIssuingDisputeParamsEvidenceMerchandiseNotAsDescribed {
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
        Option<stripe_types_05::issuing::UpdateIssuingDisputeParamsEvidenceMerchandiseNotAsDescribedReturnStatus>,
    #[doc = "Date when the product was returned or attempted to be returned."]
    pub returned_at: Option<json::OwnedValue>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingDisputeParamsEvidenceMerchandiseNotAsDescribed {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("return_status", self.return_status.as_ref());
    }
}
