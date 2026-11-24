#[cfg(feature = "issuing")]
#[derive(Debug, Clone)]
pub struct CreateIssuingCardParams {
    #[doc = "The [Cardholder](https://docs.stripe.com/api#issuing_cardholder_object) object with which the card will be associated."]
    pub cardholder: Option<String>,
    #[doc = "The currency for the card."]
    pub currency: String,
    #[doc = "The desired expiration month (1-12) for this card if specifying a custom expiration date..."]
    pub exp_month: Option<i64>,
    #[doc = "The desired 4-digit expiration year for this card if specifying a custom expiration date..."]
    pub exp_year: Option<i64>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The new financial account ID the card will be associated with."]
    pub financial_account: Option<String>,
    #[doc = "Rules that control the lifecycle of this card, such as automatic cancellation."]
    pub lifecycle_controls: Option<stripe_types_06::issuing::CreateIssuingCardParamsLifecycleControls>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The personalization design object belonging to this card."]
    pub personalization_design: Option<String>,
    #[doc = "The desired PIN for this card."]
    pub pin: Option<stripe_types_04::issuing::CreateIssuingCardParamsPin>,
    #[doc = "The card this is meant to be a replacement for (if any)."]
    pub replacement_for: Option<String>,
    #[doc = "If `replacement_for` is specified, this should indicate why that card is being replaced."]
    pub replacement_reason: Option<stripe_types_02::issuing::CreateIssuingCardParamsReplacementReason>,
    #[doc = "The second line to print on the card."]
    pub second_line: Option<json::OwnedValue>,
    #[doc = "The address where the card will be shipped."]
    pub shipping: Option<stripe_types_09::issuing::CreateIssuingCardParamsShipping>,
    #[doc = "Rules that control spending for this card."]
    pub spending_controls: Option<stripe_types_11::issuing::CreateIssuingCardParamsSpendingControls>,
    #[doc = "Whether authorizations can be approved on this card."]
    pub status: Option<stripe_types_05::issuing::CreateIssuingCardParamsStatus>,
    #[doc = "The type of card to issue."]
    pub r#type: stripe_types_06::issuing::CreateIssuingCardParamsType,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cardholder", self.cardholder.as_ref());
        form.field("currency", &self.currency);
        form.field_opt("exp_month", self.exp_month.as_ref());
        form.field_opt("exp_year", self.exp_year.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("financial_account", self.financial_account.as_ref());
        form.field_opt("lifecycle_controls", self.lifecycle_controls.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("personalization_design", self.personalization_design.as_ref());
        form.field_opt("pin", self.pin.as_ref());
        form.field_opt("replacement_for", self.replacement_for.as_ref());
        form.field_opt("replacement_reason", self.replacement_reason.as_ref());
        form.field_opt("shipping", self.shipping.as_ref());
        form.field_opt("spending_controls", self.spending_controls.as_ref());
        form.field_opt("status", self.status.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone)]
pub struct CreateIssuingCardholderParams {
    #[doc = "The cardholder's billing address."]
    pub billing: stripe_types_05::issuing::CreateIssuingCardholderParamsBilling,
    #[doc = "Additional information about a `company` cardholder."]
    pub company: Option<stripe_types_04::issuing::CreateIssuingCardholderParamsCompany>,
    #[doc = "The cardholder's email address."]
    pub email: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Additional information about an `individual` cardholder."]
    pub individual: Option<stripe_types_09::issuing::CreateIssuingCardholderParamsIndividual>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The cardholder's name."]
    pub name: String,
    #[doc = "The cardholder's phone number."]
    pub phone_number: Option<String>,
    #[doc = "The cardholder’s preferred locales (languages), ordered by preference."]
    pub preferred_locales: Option<Vec<stripe_types_01::issuing::CreateIssuingCardholderParamsPreferredLocalesItem>>,
    #[doc = "Rules that control spending across this cardholder's cards."]
    pub spending_controls: Option<stripe_types_11::issuing::CreateIssuingCardholderParamsSpendingControls>,
    #[doc = "Specifies whether to permit authorizations on this cardholder's cards."]
    pub status: Option<stripe_types_04::issuing::CreateIssuingCardholderParamsStatus>,
    #[doc = "One of `individual` or `company`."]
    pub r#type: Option<stripe_types_05::issuing::CreateIssuingCardholderParamsType>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardholderParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("billing", &self.billing);
        form.field_opt("company", self.company.as_ref());
        form.field_opt("email", self.email.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("individual", self.individual.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("name", &self.name);
        form.field_opt("phone_number", self.phone_number.as_ref());
        if let Some(values) = self.preferred_locales.as_ref() {
            form.field_array("preferred_locales", values.as_slice());
        }
        form.field_opt("spending_controls", self.spending_controls.as_ref());
        form.field_opt("status", self.status.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct CreateIssuingDisputeParams {
    #[doc = "The dispute amount in the card's currency and in the smallest currency unit..."]
    pub amount: Option<i64>,
    #[doc = "Evidence provided for the dispute."]
    pub evidence: Option<stripe_types_11::issuing::CreateIssuingDisputeParamsEvidence>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The ID of the issuing transaction to create a dispute for."]
    pub transaction: Option<String>,
    #[doc = "Params for disputes related to Treasury FinancialAccounts"]
    pub treasury: Option<stripe_types_02::issuing::CreateIssuingDisputeParamsTreasury>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingDisputeParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("evidence", self.evidence.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("transaction", self.transaction.as_ref());
        form.field_opt("treasury", self.treasury.as_ref());
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
pub struct IssuingCardholderIndividual {
    #[doc = "Information related to the card_issuing program for this cardholder."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<stripe_types_05::issuing::IssuingCardholderCardIssuing>,
    #[doc = "The date of birth of this cardholder."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dob: Option<stripe_types_03::issuing::IssuingCardholderIndividualDob>,
    #[doc = "The first name of this cardholder."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "The last name of this cardholder."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Government-issued ID document for this cardholder."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<stripe_types_11::issuing::IssuingCardholderVerification>,
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct UpdateIssuingCardParams {
    #[doc = "Reason why the `status` of this card is `canceled`."]
    pub cancellation_reason: Option<stripe_types_04::issuing::UpdateIssuingCardParamsCancellationReason>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub personalization_design: Option<String>,
    #[doc = "The desired new PIN for this card."]
    pub pin: Option<stripe_types_06::issuing::UpdateIssuingCardParamsPin>,
    #[doc = "Updated shipping information for the card."]
    pub shipping: Option<stripe_types_10::issuing::UpdateIssuingCardParamsShipping>,
    #[doc = "Rules that control spending for this card."]
    pub spending_controls: Option<stripe_types_10::issuing::UpdateIssuingCardParamsSpendingControls>,
    #[doc = "Dictates whether authorizations can be approved on this card."]
    pub status: Option<stripe_types_05::issuing::UpdateIssuingCardParamsStatus>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingCardParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cancellation_reason", self.cancellation_reason.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("personalization_design", self.personalization_design.as_ref());
        form.field_opt("pin", self.pin.as_ref());
        form.field_opt("shipping", self.shipping.as_ref());
        form.field_opt("spending_controls", self.spending_controls.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct UpdateIssuingCardholderParams {
    #[doc = "The cardholder's billing address."]
    pub billing: Option<stripe_types_08::issuing::UpdateIssuingCardholderParamsBilling>,
    #[doc = "Additional information about a `company` cardholder."]
    pub company: Option<stripe_types_06::issuing::UpdateIssuingCardholderParamsCompany>,
    #[doc = "The cardholder's email address."]
    pub email: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Additional information about an `individual` cardholder."]
    pub individual: Option<stripe_types_10::issuing::UpdateIssuingCardholderParamsIndividual>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The cardholder's phone number."]
    pub phone_number: Option<String>,
    #[doc = "The cardholder’s preferred locales (languages), ordered by preference."]
    pub preferred_locales: Option<Vec<stripe_types_06::issuing::UpdateIssuingCardholderParamsPreferredLocalesItem>>,
    #[doc = "Rules that control spending across this cardholder's cards."]
    pub spending_controls: Option<stripe_types_10::issuing::UpdateIssuingCardholderParamsSpendingControls>,
    #[doc = "Specifies whether to permit authorizations on this cardholder's cards."]
    pub status: Option<stripe_types_06::issuing::UpdateIssuingCardholderParamsStatus>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingCardholderParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing", self.billing.as_ref());
        form.field_opt("company", self.company.as_ref());
        form.field_opt("email", self.email.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("individual", self.individual.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("phone_number", self.phone_number.as_ref());
        if let Some(values) = self.preferred_locales.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("preferred_locales", index, item);
            }
        }
        form.field_opt("spending_controls", self.spending_controls.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct UpdateIssuingDisputeParams {
    #[doc = "The dispute amount in the card's currency and in the smallest currency unit..."]
    pub amount: Option<i64>,
    #[doc = "Evidence provided for the dispute."]
    pub evidence: Option<stripe_types_10::issuing::UpdateIssuingDisputeParamsEvidence>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingDisputeParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("evidence", self.evidence.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
    }
}
