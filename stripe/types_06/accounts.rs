#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedAccountConfigClaim {
    #[doc = "Whether the embedded component is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<stripe_types_01::accounts::ConnectEmbeddedAccountFeaturesClaim>,
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedBaseConfigClaim {
    #[doc = "Whether the embedded component is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<stripe_types_02::accounts::ConnectEmbeddedBaseFeatures>,
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedFinancialAccountConfigClaim {
    #[doc = "Whether the embedded component is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<stripe_types_00::accounts::ConnectEmbeddedFinancialAccountFeatures>,
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedInstantPayoutsPromotionConfig {
    #[doc = "Whether the embedded component is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<stripe_types_04::accounts::ConnectEmbeddedInstantPayoutsPromotionFeatures>,
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedIssuingCardConfigClaim {
    #[doc = "Whether the embedded component is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<stripe_types_02::accounts::ConnectEmbeddedIssuingCardFeatures>,
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedPayoutsConfig {
    #[doc = "Whether the embedded component is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<stripe_types_01::accounts::ConnectEmbeddedPayoutsFeatures>,
}

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesAuBecsDebitPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesBacsDebitPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesBankTransferPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesBilliePayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesBizumPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesBlikRecurringPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesCardPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesCashappPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesCryptoPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesEpsPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesGbBankTransferPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesGrabpayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesIdealPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesJcbPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesJpBankTransferPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesKrCardPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesNaverPayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesNzBankAccountBecsDebitPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesOxxoPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesPayByBankPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesPaycoPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesPaynowPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesPixPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesRevolutPayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesScalapayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesSepaDebitPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesSequraPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesSunbitPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesSwishPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesTaxReportingUs1099Misc =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesTreasury =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesUsBankAccountAchPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCapabilitiesZipPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCompanyAddressKanji = stripe_types_04::accounts::CreateAccountParamsCompanyAddressKana;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCompanyOwnershipDeclaration =
    stripe_types_04::accounts::CreateAccountParamsCompanyDirectorshipDeclaration;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsCompanyPrincipalPlaceOfBusiness =
    stripe_types_03::accounts::CreateAccountParamsBusinessProfileSupportAddress;

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsCompanyVerification {
    #[doc = "A document verifying the business."]
    pub document: Option<stripe_types_02::accounts::CreateAccountParamsCompanyVerificationDocument>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsCompanyVerification {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("document", self.document.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsControllerStripeDashboard {
    #[doc = "Whether this account should have access to the full Stripe Dashboard (`full`), to the Express Dashboard (`express`), ..."]
    pub r#type: Option<stripe_types_03::accounts::CreateAccountParamsControllerStripeDashboardType>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsControllerStripeDashboard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "accounts")]
pub type CreateAccountParamsDocumentsCompanyLicense =
    stripe_types_00::accounts::CreateAccountParamsDocumentsBankAccountOwnershipVerification;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsDocumentsCompanyMinisterialDecree =
    stripe_types_00::accounts::CreateAccountParamsDocumentsBankAccountOwnershipVerification;

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsDocumentsProofOfUltimateBeneficialOwnership {
    #[doc = "One or more document ids returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value ..."]
    pub files: Option<Vec<String>>,
    #[doc = "Information regarding the person signing the document if applicable."]
    pub signer: Option<stripe_types_04::accounts::CreateAccountParamsDocumentsProofOfUltimateBeneficialOwnershipSigner>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsDocumentsProofOfUltimateBeneficialOwnership {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.files.as_ref() {
            form.field_array("files", values.as_slice());
        }
        form.field_opt("signer", self.signer.as_ref());
    }
}

#[cfg(feature = "accounts")]
pub type CreateAccountParamsIndividualAddress =
    stripe_types_03::accounts::CreateAccountParamsBusinessProfileSupportAddress;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsIndividualDob = stripe_types_03::accounts::CreateAccountParamsCompanyRegistrationDate;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsIndividualRegisteredAddress =
    stripe_types_03::accounts::CreateAccountParamsBusinessProfileSupportAddress;

#[cfg(feature = "accounts")]
pub type CreateAccountParamsIndividualVerificationAdditionalDocument =
    stripe_types_02::accounts::CreateAccountParamsCompanyVerificationDocument;

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettingsCardIssuing {
    #[doc = "Details on the account's acceptance of the [Stripe Issuing Terms and Disclosures](/issuing/connect/tos_acceptance)."]
    pub tos_acceptance: Option<stripe_types_03::accounts::CreateAccountParamsSettingsCardIssuingTosAcceptance>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsCardIssuing {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("tos_acceptance", self.tos_acceptance.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettingsInvoices {
    #[doc = "Whether to save the payment method after a payment is completed for a one-time invoice or a subscription invoice when..."]
    pub hosted_payment_method_save:
        Option<stripe_types_03::accounts::CreateAccountParamsSettingsInvoicesHostedPaymentMethodSave>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsInvoices {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("hosted_payment_method_save", self.hosted_payment_method_save.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettingsPaypayPaymentsSite {
    #[doc = "Additional information about your business's website."]
    pub accessible: Option<stripe_types_03::accounts::CreateAccountParamsSettingsPaypayPaymentsSiteAccessible>,
    #[doc = "Additional information about your business's website."]
    pub in_development: Option<stripe_types_04::accounts::CreateAccountParamsSettingsPaypayPaymentsSiteInDevelopment>,
    #[doc = "Additional information about your business's website."]
    pub restricted: Option<stripe_types_04::accounts::CreateAccountParamsSettingsPaypayPaymentsSiteRestricted>,
    #[doc = "The status of your business's website."]
    pub r#type: Option<stripe_types_03::accounts::CreateAccountParamsSettingsPaypayPaymentsSiteType>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsPaypayPaymentsSite {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("accessible", self.accessible.as_ref());
        form.field_opt("in_development", self.in_development.as_ref());
        form.field_opt("restricted", self.restricted.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "accounts")]
pub type CreateAccountPersonParamsAdditionalTosAcceptancesAccount =
    stripe_types_03::accounts::CreateAccountParamsSettingsCardIssuingTosAcceptance;

#[cfg(feature = "accounts")]
pub type CreateAccountPersonParamsDocumentsVisa =
    stripe_types_01::accounts::CreateAccountPersonParamsDocumentsCompanyAuthorization;

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountPersonParamsUsCfpbDataRaceDetails {
    #[doc = "The persons race."]
    pub race: Option<Vec<stripe_types_03::accounts::CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem>>,
    #[doc = "Please specify your race, when other is selected."]
    pub race_other: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountPersonParamsUsCfpbDataRaceDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.race.as_ref() {
            form.field_array("race", values.as_slice());
        }
        form.field_opt("race_other", self.race_other.as_ref());
    }
}

#[cfg(feature = "accounts")]
pub type CreateAccountPersonParamsVerificationAdditionalDocument =
    stripe_types_02::accounts::CreateAccountParamsCompanyVerificationDocument;

#[cfg(feature = "accounts")]
pub type GetAccountCapabilityParams = stripe_types_03::accounts::GetAccountParams;

#[cfg(feature = "accounts")]
pub type GetAccountPersonParams = stripe_types_03::accounts::GetAccountParams;

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
pub struct PersonRaceDetails {
    #[doc = "The persons race."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub race: Option<Vec<stripe_types_03::accounts::PersonRaceDetailsRaceItem>>,
    #[doc = "Please specify your race, when other is selected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub race_other: Option<String>,
}

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem =
    stripe_types_00::accounts::CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsBusinessProfileMonthlyEstimatedRevenue =
    stripe_types_00::accounts::CreateAccountParamsBusinessProfileMonthlyEstimatedRevenue;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsBusinessProfileSupportAddress =
    stripe_types_03::accounts::CreateAccountParamsBusinessProfileSupportAddress;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsBusinessType = stripe_types_03::accounts::CreateAccountParamsBusinessType;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesAffirmPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesAlmaPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesAmazonPayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesAppDistribution =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesBancontactPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesBankTransferPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesBlikRecurringPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesBoletoPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesCardIssuing =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesCardPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesCartesBancairesPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesFpxPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesGiropayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesIdealPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesJpBankTransferPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesKakaoPayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesKlarnaPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesKonbiniPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesLegacyPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesMobilepayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesOxxoPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesP24Payments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesPaycoPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesPaypayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesPromptpayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesRevolutPayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesSatispayPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesSepaBankTransferPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesSofortPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesSwishPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesTaxReportingUs1099K =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesTaxReportingUs1099Misc =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesTransfers =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesTreasury =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCapabilitiesUpiPayments =
    stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCompanyAddressKana = stripe_types_04::accounts::CreateAccountParamsCompanyAddressKana;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCompanyAdministrativeAddress =
    stripe_types_03::accounts::CreateAccountParamsBusinessProfileSupportAddress;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCompanyPrincipalPlaceOfBusiness =
    stripe_types_03::accounts::CreateAccountParamsBusinessProfileSupportAddress;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCompanyRegistrationDate =
    stripe_types_03::accounts::CreateAccountParamsCompanyRegistrationDate;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCompanyRepresentativeDeclaration =
    stripe_types_04::accounts::CreateAccountParamsCompanyDirectorshipDeclaration;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCompanyStructure = stripe_types_04::accounts::CreateAccountParamsCompanyStructure;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsCompanyVerificationDocument =
    stripe_types_02::accounts::CreateAccountParamsCompanyVerificationDocument;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsDocumentsBankAccountOwnershipVerification =
    stripe_types_00::accounts::CreateAccountParamsDocumentsBankAccountOwnershipVerification;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsDocumentsCompanyMinisterialDecree =
    stripe_types_00::accounts::CreateAccountParamsDocumentsBankAccountOwnershipVerification;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsDocumentsCompanyRegistrationVerification =
    stripe_types_00::accounts::CreateAccountParamsDocumentsBankAccountOwnershipVerification;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsDocumentsProofOfAddress =
    stripe_types_00::accounts::CreateAccountParamsDocumentsBankAccountOwnershipVerification;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsDocumentsProofOfUltimateBeneficialOwnershipSigner =
    stripe_types_04::accounts::CreateAccountParamsDocumentsProofOfUltimateBeneficialOwnershipSigner;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsIndividualAddressKana = stripe_types_04::accounts::CreateAccountParamsCompanyAddressKana;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsIndividualRelationship =
    stripe_types_02::accounts::CreateAccountParamsIndividualRelationship;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsIndividualVerificationAdditionalDocument =
    stripe_types_02::accounts::CreateAccountParamsCompanyVerificationDocument;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsIndividualVerificationDocument =
    stripe_types_02::accounts::CreateAccountParamsCompanyVerificationDocument;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsSettingsCardPaymentsDeclineOn =
    stripe_types_02::accounts::CreateAccountParamsSettingsCardPaymentsDeclineOn;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsSettingsInvoicesHostedPaymentMethodSave =
    stripe_types_03::accounts::CreateAccountParamsSettingsInvoicesHostedPaymentMethodSave;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsSettingsPayments = stripe_types_01::accounts::CreateAccountParamsSettingsPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsSettingsPayoutsScheduleWeeklyAnchor =
    stripe_types_02::accounts::CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem =
    stripe_types_03::accounts::CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsSettingsPaypayPaymentsGoodsType =
    stripe_types_02::accounts::CreateAccountParamsSettingsPaypayPaymentsGoodsType;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsSettingsPaypayPaymentsSiteAccessible =
    stripe_types_03::accounts::CreateAccountParamsSettingsPaypayPaymentsSiteAccessible;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsSettingsPaypayPaymentsSiteInDevelopment =
    stripe_types_04::accounts::CreateAccountParamsSettingsPaypayPaymentsSiteInDevelopment;

#[cfg(feature = "accounts")]
pub type UpdateAccountParamsSettingsSepaDebitPayments =
    stripe_types_02::accounts::CreateAccountParamsSettingsSepaDebitPayments;

#[cfg(feature = "accounts")]
pub type UpdateAccountPersonParamsAddress = stripe_types_03::accounts::CreateAccountParamsBusinessProfileSupportAddress;

#[cfg(feature = "accounts")]
pub type UpdateAccountPersonParamsAddressKanji = stripe_types_04::accounts::CreateAccountParamsCompanyAddressKana;

#[cfg(feature = "accounts")]
pub type UpdateAccountPersonParamsDocumentsVisa =
    stripe_types_01::accounts::CreateAccountPersonParamsDocumentsCompanyAuthorization;

#[cfg(feature = "accounts")]
pub type UpdateAccountPersonParamsRegisteredAddress =
    stripe_types_03::accounts::CreateAccountParamsBusinessProfileSupportAddress;

#[cfg(feature = "accounts")]
pub type UpdateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem =
    stripe_types_04::accounts::CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem;

#[cfg(feature = "accounts")]
pub type UpdateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem =
    stripe_types_03::accounts::CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem;

#[cfg(feature = "accounts")]
pub type UpdateAccountPersonParamsVerificationAdditionalDocument =
    stripe_types_02::accounts::CreateAccountParamsCompanyVerificationDocument;
