#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CloseCoreAccountParams {
    #[doc = "Configurations on the Account to be closed."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub applied_configurations: Option<Vec<stripe_types_03::v2_core::CloseCoreAccountParamsAppliedConfigurationsItem>>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptions {
    #[doc = "Specifies whether the platform collects only currently_due requirements (`currently_due`) or both currently_due and e..."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub fields: Option<stripe_types_04::v2_core::CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFields>,
    #[doc = "Specifies whether the platform collects future_requirements in addition to requirements in Connect Onboarding."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<stripe_types_03::v2_core::CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFutureRequirements>,
}

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountLinkParamsUseCaseAccountUpdateCollectionOptionsFields =
    stripe_types_04::v2_core::CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFields;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationCustomerBillingInvoiceRendering {
    #[doc = "Indicates whether displayed line item prices and amounts on invoice PDFs include inclusive tax amounts."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<
        stripe_types_04::v2_core::CreateCoreAccountParamsConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay,
    >,
    #[doc = "ID of the invoice rendering template to use for future invoices."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesAchDebitPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesAcssDebitPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesAffirmPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesBacsDebitPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesBancontactPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesBlikPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesBlikRecurringPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesCartesBancairesPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesCashappPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesFpxPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesIdealPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesJpBankTransferPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesKlarnaPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesKonbiniPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesLinkPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesNaverPayPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesP24Payments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesPayByBankPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesPaynowPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesRevolutPayPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesSepaBankTransferPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesSepaDebitPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesSunbitPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesSwishPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesUsBankTransferPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantStatementDescriptor =
    stripe_types_04::v2_core::CreateCoreAccountParamsConfigurationMerchantScriptStatementDescriptorKana;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationMerchantSupport {
    #[doc = "A publicly available mailing address for sending support issues to."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationMerchantSupportAddress>,
    #[doc = "A publicly available email address for sending support issues to."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "A publicly available phone number to call with support issues."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "A publicly available website for handling support issues."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsDefaultsResponsibilities {
    #[doc = "A value indicating the party responsible for collecting fees from this account."]
    pub fees_collector: stripe_types_00::v2_core::CreateCoreAccountParamsDefaultsResponsibilitiesFeesCollector,
    #[doc = "A value indicating who is responsible for losses when this Account can’t pay back negative balances from payments."]
    pub losses_collector: stripe_types_04::v2_core::CreateCoreAccountParamsDefaultsResponsibilitiesLossesCollector,
}

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose =
    stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsAddress =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationMerchantSupportAddress;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsAnnualRevenueAmount =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAnnualRevenueAmount;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyLicenseType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyMemorandumOfAssociationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyMinisterialDecreeType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnershipSigner =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfRegistrationSigner;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnershipType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsMonthlyEstimatedRevenueAmount =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAnnualRevenueAmount;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsRegistrationDate =
    stripe_types_01::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsRegistrationDate;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsScriptAddressesKanji =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationMerchantSupportAddress;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualAdditionalAddressesItemPurpose =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualAdditionalAddressesItemPurpose;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualAdditionalNamesItemPurpose =
    stripe_types_04::v2_core::CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItemPurpose;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualAddress =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationMerchantSupportAddress;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualDocumentsCompanyAuthorizationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualDocumentsSecondaryVerificationFrontBack =
    stripe_types_01::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualDocumentsSecondaryVerificationType =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualIdNumbersItemType =
    stripe_types_04::v2_core::CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualLegalGender =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualLegalGender;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualScriptAddressesKana =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationMerchantSupportAddress;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualScriptAddressesKanji =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationMerchantSupportAddress;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsAdditionalNamesItemPurpose =
    stripe_types_04::v2_core::CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItemPurpose;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsDocumentsPrimaryVerificationType =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsDocumentsSecondaryVerificationFrontBack =
    stripe_types_01::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsDocumentsSecondaryVerificationType =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsLegalGender =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualLegalGender;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsPoliticalExposure =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualPoliticalExposure;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsRelationship =
    stripe_types_00::v2_core::CreateCoreAccountPersonTokenParamsRelationship;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsScriptAddressesKanji =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationMerchantSupportAddress;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsScriptNamesKana =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityIndividualScriptNamesKana;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsScriptNamesKanji =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityIndividualScriptNamesKana;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsAdditionalTermsOfServiceAccount =
    stripe_types_01::v2_core::CreateCoreAccountTokenParamsIdentityAttestationsTermsOfServiceAccount;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsDocumentsPrimaryVerificationType =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsDocumentsSecondaryVerificationFrontBack =
    stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsDocumentsSecondaryVerificationType =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsLegalGender =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualLegalGender;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsPoliticalExposure =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualPoliticalExposure;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsScriptAddressesKana =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsScriptAddressesKanji =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsScriptNamesKana =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityIndividualScriptNamesKana;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityAttestationsPersonsProvided {
    #[doc = "Whether the company’s directors have been provided."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub directors: Option<bool>,
    #[doc = "Whether the company’s executives have been provided."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub executives: Option<bool>,
    #[doc = "Whether the company’s owners have been provided."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub owners: Option<bool>,
    #[doc = "Reason for why the company is exempt from providing ownership information."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub ownership_exemption_reason: Option<stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsAdditionalAddressesItem {
    #[doc = "City, district, suburb, town, or village."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Address line 1 (e.g., street, PO Box, or company name)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[doc = "Address line 2 (e.g., apartment, suite, unit, or building)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "Purpose of additional address."]
    pub purpose:
        stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose,
    #[doc = "State, county, province, or region."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Town or district."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsAnnualRevenue {
    #[doc = "A non-negative integer representing the amount in the smallest currency unit."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub amount:
        Option<stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAnnualRevenueAmount>,
    #[doc = "The close-out date of the preceding fiscal year in ISO 8601 format."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub fiscal_year_end: Option<String>,
}

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyLicenseType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyMemorandumOfAssociationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyMinisterialDecreeType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfRegistrationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnershipType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItem {
    #[doc = "The registrar of the ID number (Only valid for DE ID number types)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub registrar: Option<String>,
    #[doc = "Open Enum."]
    #[json(rename = "type")]
    pub r#type: stripe_types_01::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType,
    #[doc = "The value of the ID number."]
    pub value: String,
}

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityBusinessDetailsMonthlyEstimatedRevenueAmount =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAnnualRevenueAmount;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityBusinessDetailsScriptAddressesKana =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItem {
    #[doc = "The person's full name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[doc = "The person's first or given name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The purpose or type of the additional name."]
    pub purpose: stripe_types_04::v2_core::CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItemPurpose,
    #[doc = "The person's last or family name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
}

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityIndividualAddress =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityIndividualDateOfBirth =
    stripe_types_01::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsRegistrationDate;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityIndividualDocumentsPassportType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityIndividualDocumentsPrimaryVerificationFrontBack =
    stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityIndividualDocumentsSecondaryVerificationFrontBack =
    stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityIndividualDocumentsVisaType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityIndividualScriptAddressesKana =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, Default)]
pub struct GetCoreAccountParams {
    pub include: Option<Vec<stripe_types_04::v2_core::GetCoreAccountParamsIncludeItem>>,
}

#[cfg(feature = "v2_core")]
impl crate::params::FormEncode for GetCoreAccountParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.include.as_ref() {
            form.field_repeated("include", values.as_slice());
        }
    }
}

#[cfg(feature = "v2_core")]
pub type GetCoreEventDestinationParamsIncludeItem =
    stripe_types_03::v2_core::ListCoreEventDestinationsParamsIncludeItem;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, Default)]
pub struct ListCoreAccountsParams {
    pub applied_configurations: Option<Vec<stripe_types_00::v2_core::ListCoreAccountsParamsAppliedConfigurationsItem>>,
    pub closed: Option<bool>,
    pub limit: Option<i64>,
}

#[cfg(feature = "v2_core")]
impl crate::params::FormEncode for ListCoreAccountsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.applied_configurations.as_ref() {
            form.field_repeated("applied_configurations", values.as_slice());
        }
        form.field_opt("closed", self.closed.as_ref());
        form.field_opt("limit", self.limit.as_ref());
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, Default)]
pub struct ListCoreEventDestinationsParams {
    pub include: Option<Vec<stripe_types_03::v2_core::ListCoreEventDestinationsParamsIncludeItem>>,
    pub limit: Option<i64>,
}

#[cfg(feature = "v2_core")]
impl crate::params::FormEncode for ListCoreEventDestinationsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.include.as_ref() {
            form.field_repeated("include", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
    }
}

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxExempt =
    stripe_types_04::v2_core::CreateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxExempt;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationCustomerBillingInvoiceCustomFieldsItem =
    stripe_types_04::v2_core::CreateCoreAccountParamsConfigurationCustomerBillingInvoiceCustomFieldsItem;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsConfigurationCustomerCapabilities {
    #[doc = "Generates requirements for enabling automatic indirect tax calculation on this customer's invoices or subscriptions."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub automatic_indirect_tax:
        Option<stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
}

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationCustomerShippingAddress =
    stripe_types_03::v2_core::CreateCoreAccountParamsConfigurationCustomerShippingAddress;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantBranding =
    stripe_types_04::v2_core::CreateCoreAccountParamsConfigurationMerchantBranding;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesAfterpayClearpayPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesAlmaPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesAmazonPayPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesBoletoPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesCardPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesCashappPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesEpsPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesGbBankTransferPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesJcbPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesKakaoPayPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesKonbiniPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesKrCardPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesMobilepayPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesMultibancoPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesMxBankTransferPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesOxxoPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesPaycoPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesPromptpayPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesSamsungPayPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesSatispayPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesSequraPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesTwintPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesZipPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCardPaymentsDeclineOn =
    stripe_types_04::v2_core::CreateCoreAccountParamsConfigurationMerchantCardPaymentsDeclineOn;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantKonbiniPaymentsSupportHours =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationMerchantKonbiniPaymentsSupportHours;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantScriptStatementDescriptorKana =
    stripe_types_04::v2_core::CreateCoreAccountParamsConfigurationMerchantScriptStatementDescriptorKana;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantSepaDebitPayments =
    stripe_types_01::v2_core::CreateCoreAccountParamsConfigurationMerchantSepaDebitPayments;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantSupportAddress =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationRecipientCapabilitiesStripeBalanceStripeTransfers =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsDefaultsResponsibilitiesLossesCollector =
    stripe_types_04::v2_core::CreateCoreAccountParamsDefaultsResponsibilitiesLossesCollector;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityAttestationsDirectorshipDeclaration =
    stripe_types_00::v2_core::CreateCoreAccountParamsIdentityAttestationsDirectorshipDeclaration;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityAttestationsOwnershipDeclaration =
    stripe_types_00::v2_core::CreateCoreAccountParamsIdentityAttestationsDirectorshipDeclaration;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason =
    stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityAttestationsTermsOfServiceMoneyManager =
    stripe_types_00::v2_core::CreateCoreAccountParamsIdentityAttestationsDirectorshipDeclaration;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyMemorandumOfAssociationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyRegistrationVerificationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyTaxIdVerificationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfAddressType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfRegistrationSigner =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfRegistrationSigner;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfRegistrationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsIdNumbersItemType =
    stripe_types_01::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsScriptAddressesKana =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsStructure =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualAdditionalAddressesItemPurpose =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualAdditionalAddressesItemPurpose;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualAdditionalNamesItemPurpose =
    stripe_types_04::v2_core::CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItemPurpose;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualDocumentsPassportType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualDocumentsPrimaryVerificationFrontBack =
    stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualDocumentsPrimaryVerificationType =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualDocumentsSecondaryVerificationFrontBack =
    stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualDocumentsVisaType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualIdNumbersItemType =
    stripe_types_04::v2_core::CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualRelationship =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualRelationship;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualScriptAddressesKanji =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualScriptNamesKana =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityIndividualScriptNamesKana;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualScriptNamesKanji =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityIndividualScriptNamesKana;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsAdditionalAddressesItemPurpose =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualAdditionalAddressesItemPurpose;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsAdditionalNamesItemPurpose =
    stripe_types_04::v2_core::CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItemPurpose;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsAddress =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsDocumentsCompanyAuthorizationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsDocumentsSecondaryVerificationType =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsIdNumbersItemType =
    stripe_types_04::v2_core::CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsRelationship =
    stripe_types_00::v2_core::CreateCoreAccountPersonTokenParamsRelationship;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsScriptAddressesKana =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsScriptAddressesKanji =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationMerchantKonbiniPaymentsSupport {
    #[doc = "Support email address for Konbini payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Support hours for Konbini payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<stripe_types_03::v2_core::V2CoreAccountConfigurationMerchantKonbiniPaymentsSupportHours>,
    #[doc = "Support phone number for Konbini payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationMerchantSupport {
    #[doc = "A publicly available mailing address for sending support issues to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::v2_core::V2CoreAccountConfigurationMerchantSupportAddress>,
    #[doc = "A publicly available email address for sending support issues to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "A publicly available phone number to call with support issues."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "A publicly available website for handling support issues."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountFutureRequirementsEntriesItemErrorsItem {
    #[doc = "Machine-readable code describing the error."]
    #[json(default)]
    pub code: stripe_types_03::v2_core::V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode,
    #[doc = "Human-readable description of the error."]
    #[json(default)]
    pub description: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountFutureRequirementsEntriesItemRequestedReasonsItem {
    #[doc = "Machine-readable description of Stripe's reason for collecting the requirement."]
    #[json(default)]
    pub code: stripe_types_04::v2_core::V2CoreAccountFutureRequirementsEntriesItemRequestedReasonsItemCode,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityAttestationsTermsOfService {
    #[doc = "Details on the Account's acceptance of the Stripe Services Agreement..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<stripe_types_02::v2_core::V2CoreAccountIdentityAttestationsDirectorshipDeclaration>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityBusinessDetailsAdditionalAddressesItem {
    #[doc = "City, district, suburb, town, or village."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Address line 1 (e.g., street, PO Box, or company name)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[doc = "Address line 2 (e.g., apartment, suite, unit, or building)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "Purpose of additional address."]
    #[json(default)]
    pub purpose: stripe_types_03::v2_core::V2CoreAccountIdentityBusinessDetailsAdditionalAddressesItemPurpose,
    #[doc = "State, county, province, or region."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Town or district."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityBusinessDetailsDocumentsPrimaryVerification {
    #[doc = "The [file upload](https://docs.stripe.com/api/persons/update#create_file) tokens for the front and back of the verifi..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub front_back:
        Option<stripe_types_02::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack>,
    #[doc = "The format of the verification document."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsPrimaryVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityIndividualAdditionalAddressesItem {
    #[doc = "City, district, suburb, town, or village."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Address line 1 (e.g., street, PO Box, or company name)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[doc = "Address line 2 (e.g., apartment, suite, unit, or building)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "Purpose of additional address."]
    #[json(default)]
    pub purpose: stripe_types_02::v2_core::V2CoreAccountIdentityIndividualAdditionalAddressesItemPurpose,
    #[doc = "State, county, province, or region."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Town or district."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityIndividualAdditionalNamesItem {
    #[doc = "The individual's full name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[doc = "The individual's first or given name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The purpose or type of the additional name."]
    #[json(default)]
    pub purpose: stripe_types_02::v2_core::V2CoreAccountIdentityIndividualAdditionalNamesItemPurpose,
    #[doc = "The individual's last or family name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityIndividualIdNumbersItem {
    #[doc = "The ID number type of an individual."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::v2_core::V2CoreAccountIdentityIndividualIdNumbersItemType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityIndividualScriptNames {
    #[doc = "Persons name in kana script."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_02::v2_core::V2CoreAccountIdentityIndividualScriptNamesKana>,
    #[doc = "Persons name in kanji script."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_02::v2_core::V2CoreAccountIdentityIndividualScriptNamesKana>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptions {
    #[doc = "Specifies whether the platform collects only currently_due requirements (`currently_due`) or both currently_due and e..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<stripe_types_03::v2_core::V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFields>,
    #[doc = "Specifies whether the platform collects future_requirements in addition to requirements in Connect Onboarding."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub future_requirements:
        Option<stripe_types_01::v2_core::V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFutureRequirements>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountPersonToken {
    #[doc = "Time at which the token was created."]
    #[json(default)]
    pub created: String,
    #[doc = "Time at which the token will expire."]
    #[json(default)]
    pub expires_at: String,
    #[doc = "Unique identifier for the token."]
    #[json(default)]
    pub id: String,
    #[doc = "Has the value `true` if the token exists in live mode or the value `false` if the object exists in test mode."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::v2_core::V2CoreAccountPersonTokenObject,
    #[doc = "Determines if the token has already been used (tokens can only be used once)."]
    #[json(default)]
    pub used: bool,
}

#[cfg(feature = "v2_core")]
impl crate::PagedItem for V2CoreAccountPersonToken {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountToken {
    #[doc = "Time at which the token was created."]
    #[json(default)]
    pub created: String,
    #[doc = "Time at which the token will expire."]
    #[json(default)]
    pub expires_at: String,
    #[doc = "Unique identifier for the token."]
    #[json(default)]
    pub id: String,
    #[doc = "Has the value `true` if the token exists in live mode or the value `false` if the object exists in test mode."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::v2_core::V2CoreAccountTokenObject,
    #[doc = "Determines if the token has already been used (tokens can only be used once)."]
    #[json(default)]
    pub used: bool,
}

#[cfg(feature = "v2_core")]
impl crate::PagedItem for V2CoreAccountToken {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreEventDestinationAmazonEventbridge {
    #[doc = "The AWS account ID that owns the event bus receiving events."]
    #[json(default)]
    pub aws_account_id: String,
    #[doc = "The ARN of the Stripe-created partner event source in your AWS account."]
    #[json(default)]
    pub aws_event_source_arn: String,
    #[doc = "The AWS-reported lifecycle state of the partner event source."]
    #[json(default)]
    pub aws_event_source_status: stripe_types_02::v2_core::V2CoreEventDestinationAmazonEventbridgeAwsEventSourceStatus,
}
