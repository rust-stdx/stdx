#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedDisputesListConfig {
    #[doc = "Whether the embedded component is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<stripe_types_02::accounts::ConnectEmbeddedDisputesListFeatures>,
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedFinancialAccountTransactionsConfigClaim {
    #[doc = "Whether the embedded component is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<stripe_types_04::accounts::ConnectEmbeddedFinancialAccountTransactionsFeatures>,
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedIssuingCardsListConfigClaim {
    #[doc = "Whether the embedded component is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<stripe_types_04::accounts::ConnectEmbeddedIssuingCardsListFeatures>,
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedPaymentDisputesConfig {
    #[doc = "Whether the embedded component is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<stripe_types_01::accounts::ConnectEmbeddedPaymentDisputesFeatures>,
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedPaymentMethodSettingsConfigClaim {
    #[doc = "Whether the embedded component is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<stripe_types_04::accounts::ConnectEmbeddedPaymentMethodSettingsFeatures>,
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedPaymentsConfigClaim {
    #[doc = "Whether the embedded component is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<stripe_types_00::accounts::ConnectEmbeddedPaymentsFeatures>,
}

#[cfg(feature = "accounts")]
pub type CreateAccountLoginLinkParams = stripe_types_03::accounts::GetAccountParams;

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsBusinessProfile {
    #[doc = "The applicant's gross annual revenue for its preceding fiscal year."]
    pub annual_revenue: Option<stripe_types_03::accounts::CreateAccountParamsBusinessProfileAnnualRevenue>,
    #[doc = "An estimated upper bound of employees, contractors, vendors, etc."]
    pub estimated_worker_count: Option<i64>,
    #[doc = "[The merchant category code for the account](/connect/setting-mcc)."]
    pub mcc: Option<String>,
    #[doc = "Whether the business is a minority-owned, women-owned, and/or LGBTQI+ -owned business."]
    pub minority_owned_business_designation:
        Option<Vec<stripe_types_00::accounts::CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem>>,
    #[doc = "An estimate of the monthly revenue of the business."]
    pub monthly_estimated_revenue:
        Option<stripe_types_00::accounts::CreateAccountParamsBusinessProfileMonthlyEstimatedRevenue>,
    #[doc = "The customer-facing business name."]
    pub name: Option<String>,
    #[doc = "Internal-only description of the product sold by, or service provided by, the business."]
    pub product_description: Option<String>,
    #[doc = "A link to the business's publicly available terms related to the Specified Commercial Transaction Act."]
    pub specified_commercial_transactions_act_url: Option<json::OwnedValue>,
    #[doc = "A publicly available mailing address for sending support issues to."]
    pub support_address: Option<stripe_types_03::accounts::CreateAccountParamsBusinessProfileSupportAddress>,
    #[doc = "A publicly available email address for sending support issues to."]
    pub support_email: Option<String>,
    #[doc = "A publicly available phone number to call with support issues."]
    pub support_phone: Option<String>,
    #[doc = "A publicly available website for handling support issues."]
    pub support_url: Option<json::OwnedValue>,
    #[doc = "The business's publicly available website."]
    pub url: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsBusinessProfile {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("annual_revenue", self.annual_revenue.as_ref());
        form.field_opt("estimated_worker_count", self.estimated_worker_count.as_ref());
        form.field_opt("mcc", self.mcc.as_ref());
        if let Some(values) = self.minority_owned_business_designation.as_ref() {
            form.field_array("minority_owned_business_designation", values.as_slice());
        }
        form.field_opt("monthly_estimated_revenue", self.monthly_estimated_revenue.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("product_description", self.product_description.as_ref());
        form.field_opt("support_address", self.support_address.as_ref());
        form.field_opt("support_email", self.support_email.as_ref());
        form.field_opt("support_phone", self.support_phone.as_ref());
        form.field_opt("url", self.url.as_ref());
    }
}

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesAffirmPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesAfterpayClearpayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesAlmaPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesAmazonPayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesAppDistribution =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesBancontactPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesBlikPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesBoletoPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesCardIssuing =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesCartesBancairesPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesFpxPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesGiropayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesIndiaInternationalPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesKakaoPayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesKlarnaPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesKonbiniPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesLegacyPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesLinkPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesMbWayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesMobilepayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesMultibancoPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesMxBankTransferPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesP24Payments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesPaypayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesPaytoPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesPromptpayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesSamsungPayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesSatispayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesSepaBankTransferPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesSofortPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesTaxReportingUs1099K =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesTransfers =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesTwintPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesUpiPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesUsBankTransferPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCompanyAddress =
    stripe_types_03::accounts::CreateAccountParamsBusinessProfileSupportAddress;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCompanyAdministrativeAddress =
    stripe_types_03::accounts::CreateAccountParamsBusinessProfileSupportAddress;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCompanyRepresentativeDeclaration =
    stripe_types_04::accounts::CreateAccountParamsCompanyDirectorshipDeclaration;

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsControllerFees {
    #[doc = "A value indicating the responsible payer of Stripe fees on this account."]
    pub payer: Option<stripe_types_03::accounts::CreateAccountParamsControllerFeesPayer>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsControllerFees {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("payer", self.payer.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsControllerLosses {
    #[doc = "A value indicating who is liable when this account can't pay back negative balances resulting from payments."]
    pub payments: Option<stripe_types_04::accounts::CreateAccountParamsControllerLossesPayments>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsControllerLosses {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("payments", self.payments.as_ref());
    }
}

#[cfg(feature = "accounts")]
pub type CreateAccountParamsControllerRequirementCollection =
    stripe_types_04::accounts::CreateAccountParamsControllerLossesPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsDocumentsCompanyMemorandumOfAssociation =
    stripe_types_00::accounts::CreateAccountParamsDocumentsBankAccountOwnershipVerification;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsDocumentsCompanyRegistrationVerification =
    stripe_types_00::accounts::CreateAccountParamsDocumentsBankAccountOwnershipVerification;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsDocumentsCompanyTaxIdVerification =
    stripe_types_00::accounts::CreateAccountParamsDocumentsBankAccountOwnershipVerification;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsDocumentsProofOfAddress =
    stripe_types_00::accounts::CreateAccountParamsDocumentsBankAccountOwnershipVerification;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsIndividualAddressKana = stripe_types_04::accounts::CreateAccountParamsCompanyAddressKana;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsIndividualAddressKanji = stripe_types_04::accounts::CreateAccountParamsCompanyAddressKana;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsIndividualVerificationDocument =
    stripe_types_02::accounts::CreateAccountParamsCompanyVerificationDocument;

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettingsCardPayments {
    #[doc = "Automatically declines certain charge types regardless of whether the card issuer accepted or declined the charge."]
    pub decline_on: Option<stripe_types_02::accounts::CreateAccountParamsSettingsCardPaymentsDeclineOn>,
    #[doc = "The default text that appears on credit card statements when a charge is made."]
    pub statement_descriptor_prefix: Option<String>,
    #[doc = "The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only)."]
    pub statement_descriptor_prefix_kana: Option<json::OwnedValue>,
    #[doc = "The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only)."]
    pub statement_descriptor_prefix_kanji: Option<json::OwnedValue>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsCardPayments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("decline_on", self.decline_on.as_ref());
        form.field_opt("statement_descriptor_prefix", self.statement_descriptor_prefix.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettingsPayoutsSchedule {
    #[doc = "The number of days charge funds are held before being paid out."]
    pub delay_days: Option<json::OwnedValue>,
    #[doc = "How frequently available funds are paid out."]
    pub interval: Option<stripe_types_01::accounts::CreateAccountParamsSettingsPayoutsScheduleInterval>,
    #[doc = "The day of the month when available funds are paid out, specified as a number between 1--31."]
    pub monthly_anchor: Option<i64>,
    #[doc = "The days of the month when available funds are paid out, specified as an array of numbers between 1--31."]
    pub monthly_payout_days: Option<Vec<i64>>,
    #[doc = "The day of the week when available funds are paid out, specified as `monday`, `tuesday`, etc."]
    pub weekly_anchor: Option<stripe_types_02::accounts::CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor>,
    #[doc = "The days of the week when available funds are paid out, specified as an array, e.g., `monday`, `tuesday`."]
    pub weekly_payout_days:
        Option<Vec<stripe_types_03::accounts::CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem>>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsPayoutsSchedule {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("interval", self.interval.as_ref());
        form.field_opt("monthly_anchor", self.monthly_anchor.as_ref());
        if let Some(values) = self.monthly_payout_days.as_ref() {
            form.field_array("monthly_payout_days", values.as_slice());
        }
        form.field_opt("weekly_anchor", self.weekly_anchor.as_ref());
        if let Some(values) = self.weekly_payout_days.as_ref() {
            form.field_array("weekly_payout_days", values.as_slice());
        }
    }
}

#[cfg(feature = "accounts")]
pub type CreateAccountParamsSettingsTreasuryTosAcceptance =
    stripe_types_03::accounts::CreateAccountParamsSettingsCardIssuingTosAcceptance;

#[cfg(feature = "accounts")]
pub type CreateAccountPersonParamsAddress = stripe_types_03::accounts::CreateAccountParamsBusinessProfileSupportAddress;

#[cfg(feature = "accounts")]
pub type CreateAccountPersonParamsAddressKana = stripe_types_04::accounts::CreateAccountParamsCompanyAddressKana;

#[cfg(feature = "accounts")]
pub type CreateAccountPersonParamsAddressKanji = stripe_types_04::accounts::CreateAccountParamsCompanyAddressKana;

#[cfg(feature = "accounts")]
pub type CreateAccountPersonParamsDob = stripe_types_03::accounts::CreateAccountParamsCompanyRegistrationDate;

#[cfg(feature = "accounts")]
pub type CreateAccountPersonParamsDocumentsPassport =
    stripe_types_01::accounts::CreateAccountPersonParamsDocumentsCompanyAuthorization;

#[cfg(feature = "accounts")]
pub type CreateAccountPersonParamsPoliticalExposure =
    stripe_types_03::accounts::CreateAccountParamsIndividualPoliticalExposure;

#[cfg(feature = "accounts")]
pub type CreateAccountPersonParamsRegisteredAddress =
    stripe_types_03::accounts::CreateAccountParamsBusinessProfileSupportAddress;

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountPersonParamsUsCfpbDataEthnicityDetails {
    #[doc = "The persons ethnicity"]
    pub ethnicity:
        Option<Vec<stripe_types_04::accounts::CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem>>,
    #[doc = "Please specify your origin, when other is selected."]
    pub ethnicity_other: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountPersonParamsUsCfpbDataEthnicityDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.ethnicity.as_ref() {
            form.field_array("ethnicity", values.as_slice());
        }
        form.field_opt("ethnicity_other", self.ethnicity_other.as_ref());
    }
}

#[cfg(feature = "accounts")]
pub type CreateAccountPersonParamsVerificationDocument =
    stripe_types_02::accounts::CreateAccountParamsCompanyVerificationDocument;

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedPerson {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::accounts::PersonObject,
}

#[cfg(feature = "accounts")]
impl crate::PagedItem for DeletedPerson {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "accounts")]
pub type GetAccountExternalAccountParams = stripe_types_03::accounts::GetAccountParams;

#[cfg(feature = "accounts")]
pub type ListAccountCapabilitiesParams = stripe_types_03::accounts::GetAccountParams;

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct ListAccountExternalAccountsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub object: Option<stripe_types_04::accounts::ListAccountExternalAccountsParamsObject>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for ListAccountExternalAccountsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("object", self.object.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct ListAccountPersonsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub relationship: Option<stripe_types_02::accounts::ListAccountPersonsParamsRelationship>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for ListAccountPersonsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("relationship", self.relationship.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct ListAccountsParams {
    pub created: Option<stripe_types_04::accounts::ListAccountsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for ListAccountsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
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
pub struct PersonAdditionalTosAcceptances {
    #[doc = "Details on the legal guardian's acceptance of the main Stripe service agreement."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<stripe_types_00::accounts::PersonAdditionalTosAcceptance>,
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
pub struct PersonEthnicityDetails {
    #[doc = "The persons ethnicity"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ethnicity: Option<Vec<stripe_types_02::accounts::PersonEthnicityDetailsEthnicityItem>>,
    #[doc = "Please specify your origin, when other is selected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ethnicity_other: Option<String>,
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone)]
pub struct RejectAccountParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Whether to pause payouts on the account as part of the rejection."]
    pub payouts_action: Option<stripe_types_03::accounts::RejectAccountParamsPayoutsAction>,
    #[doc = "The reason for rejecting the account."]
    pub reason: stripe_types_04::accounts::RejectAccountParamsReason,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for RejectAccountParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("payouts_action", self.payouts_action.as_ref());
        form.field("reason", &self.reason);
    }
}

#[cfg(feature = "accounts")]
pub type UnrejectAccountParams = stripe_types_03::accounts::GetAccountParams;

#[cfg(feature = "accounts")]
pub type UpdateAccountExternalAccountParamsDocumentsBankAccountOwnershipVerification =
    stripe_types_00::accounts::CreateAccountParamsDocumentsBankAccountOwnershipVerification;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsBusinessProfileAnnualRevenue =
    stripe_types_03::accounts::CreateAccountParamsBusinessProfileAnnualRevenue;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesAcssDebitPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesAfterpayClearpayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesAuBecsDebitPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesBacsDebitPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesBilliePayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesBizumPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesBlikPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesCashappPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesCryptoPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesEpsPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesGbBankTransferPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesGrabpayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesIndiaInternationalPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesJcbPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesKrCardPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesLinkPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesMbWayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesMultibancoPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesMxBankTransferPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesNaverPayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesNzBankAccountBecsDebitPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesPayByBankPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesPaynowPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesPaytoPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesPixPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesSamsungPayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesScalapayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesSepaDebitPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesSequraPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesSunbitPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesTwintPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesUsBankAccountAchPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesUsBankTransferPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesZipPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCompanyAddress =
    stripe_types_03::accounts::CreateAccountParamsBusinessProfileSupportAddress;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCompanyAddressKanji = stripe_types_04::accounts::CreateAccountParamsCompanyAddressKana;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCompanyDirectorshipDeclaration =
    stripe_types_04::accounts::CreateAccountParamsCompanyDirectorshipDeclaration;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCompanyOwnershipDeclaration =
    stripe_types_04::accounts::CreateAccountParamsCompanyDirectorshipDeclaration;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCompanyOwnershipExemptionReason =
    stripe_types_03::accounts::CreateAccountParamsCompanyOwnershipExemptionReason;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsDocumentsCompanyLicense =
    stripe_types_00::accounts::CreateAccountParamsDocumentsBankAccountOwnershipVerification;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsDocumentsCompanyMemorandumOfAssociation =
    stripe_types_00::accounts::CreateAccountParamsDocumentsBankAccountOwnershipVerification;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsDocumentsCompanyTaxIdVerification =
    stripe_types_00::accounts::CreateAccountParamsDocumentsBankAccountOwnershipVerification;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsDocumentsProofOfRegistrationSigner =
    stripe_types_04::accounts::CreateAccountParamsDocumentsProofOfUltimateBeneficialOwnershipSigner;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsGroups = stripe_types_04::accounts::CreateAccountParamsGroups;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsIndividualAddress =
    stripe_types_03::accounts::CreateAccountParamsBusinessProfileSupportAddress;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsIndividualAddressKanji = stripe_types_04::accounts::CreateAccountParamsCompanyAddressKana;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsIndividualDob = stripe_types_03::accounts::CreateAccountParamsCompanyRegistrationDate;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsIndividualPoliticalExposure =
    stripe_types_03::accounts::CreateAccountParamsIndividualPoliticalExposure;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsIndividualRegisteredAddress =
    stripe_types_03::accounts::CreateAccountParamsBusinessProfileSupportAddress;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsSettingsBacsDebitPayments =
    stripe_types_03::accounts::CreateAccountParamsSettingsBacsDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsSettingsBranding = stripe_types_01::accounts::CreateAccountParamsSettingsBranding;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsSettingsCardIssuingTosAcceptance =
    stripe_types_03::accounts::CreateAccountParamsSettingsCardIssuingTosAcceptance;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsSettingsPayoutsScheduleInterval =
    stripe_types_01::accounts::CreateAccountParamsSettingsPayoutsScheduleInterval;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsSettingsPaypayPaymentsSiteRestricted =
    stripe_types_04::accounts::CreateAccountParamsSettingsPaypayPaymentsSiteRestricted;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsSettingsPaypayPaymentsSiteType =
    stripe_types_03::accounts::CreateAccountParamsSettingsPaypayPaymentsSiteType;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsSettingsTreasuryTosAcceptance =
    stripe_types_03::accounts::CreateAccountParamsSettingsCardIssuingTosAcceptance;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsTosAcceptance = stripe_types_01::accounts::CreateAccountParamsTosAcceptance;

#[cfg(feature = "accounts")]
pub type UpdateAccountPersonParamsAdditionalTosAcceptancesAccount =
    stripe_types_03::accounts::CreateAccountParamsSettingsCardIssuingTosAcceptance;

#[cfg(feature = "accounts")]
pub type UpdateAccountPersonParamsAddressKana = stripe_types_04::accounts::CreateAccountParamsCompanyAddressKana;

#[cfg(feature = "accounts")]
pub type UpdateAccountPersonParamsDob = stripe_types_03::accounts::CreateAccountParamsCompanyRegistrationDate;

#[cfg(feature = "accounts")]
pub type UpdateAccountPersonParamsDocumentsCompanyAuthorization =
    stripe_types_01::accounts::CreateAccountPersonParamsDocumentsCompanyAuthorization;

#[cfg(feature = "accounts")]
pub type UpdateAccountPersonParamsDocumentsPassport =
    stripe_types_01::accounts::CreateAccountPersonParamsDocumentsCompanyAuthorization;

#[cfg(feature = "accounts")]
pub type UpdateAccountPersonParamsPoliticalExposure =
    stripe_types_03::accounts::CreateAccountParamsIndividualPoliticalExposure;

#[cfg(feature = "accounts")]
pub type UpdateAccountPersonParamsRelationship = stripe_types_01::accounts::CreateAccountPersonParamsRelationship;

#[cfg(feature = "accounts")]
pub type UpdateAccountPersonParamsVerificationDocument =
    stripe_types_02::accounts::CreateAccountParamsCompanyVerificationDocument;
