#[cfg(feature = "v2_core")]
pub type CreateCoreAccountLinkParamsUseCaseAccountUpdateCollectionOptionsFutureRequirements =
    stripe_types_03::v2_core::CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFutureRequirements;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationCustomerAutomaticIndirectTax {
    #[doc = "Describes the customer's tax exemption status, which is `none`, `exempt`, or `reverse`."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub exempt:
        Option<stripe_types_04::v2_core::CreateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxExempt>,
    #[doc = "A recent IP address of the customer used for tax reporting and tax location inference."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationCustomerCapabilities {
    #[doc = "Generates requirements for enabling automatic indirect tax calculation on this customer's invoices or subscriptions."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub automatic_indirect_tax:
        Option<stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationCustomerShipping {
    #[doc = "Customer shipping address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_03::v2_core::CreateCoreAccountParamsConfigurationCustomerShippingAddress>,
    #[doc = "Customer name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Customer phone (including extension)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesAfterpayClearpayPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesAlmaPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesAmazonPayPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesAuBecsDebitPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesBoletoPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesCardPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesEpsPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesGbBankTransferPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesGrabpayPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesJcbPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesKakaoPayPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesKrCardPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesMobilepayPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesMultibancoPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesMxBankTransferPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesOxxoPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesPaycoPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesPromptpayPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesSamsungPayPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesSatispayPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesSequraPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesTwintPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantCapabilitiesZipPayments =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationMerchantCardPayments {
    #[doc = "Automatically declines certain charge types regardless of whether the card issuer accepted or declined the charge."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub decline_on: Option<stripe_types_04::v2_core::CreateCoreAccountParamsConfigurationMerchantCardPaymentsDeclineOn>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationMerchantKonbiniPaymentsSupport {
    #[doc = "Support email address for Konbini payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Support hours for Konbini payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub hours:
        Option<stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationMerchantKonbiniPaymentsSupportHours>,
    #[doc = "Support phone number for Konbini payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationMerchantScriptStatementDescriptorKanji =
    stripe_types_04::v2_core::CreateCoreAccountParamsConfigurationMerchantScriptStatementDescriptorKana;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsConfigurationRecipientCapabilitiesStripeBalanceStripeTransfers =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityAttestationsOwnershipDeclaration =
    stripe_types_00::v2_core::CreateCoreAccountParamsIdentityAttestationsDirectorshipDeclaration;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason =
    stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityAttestationsRepresentativeDeclaration =
    stripe_types_00::v2_core::CreateCoreAccountParamsIdentityAttestationsDirectorshipDeclaration;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityAttestationsTermsOfService {
    #[doc = "Details on the Account's acceptance of the Stripe Services Agreement..."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub account: Option<stripe_types_01::v2_core::CreateCoreAccountParamsIdentityAttestationsTermsOfServiceAccount>,
}

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyRegistrationVerificationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyTaxIdVerificationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfAddressType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfRegistrationSigner =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfRegistrationSigner;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfRegistrationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsIdNumbersItemType =
    stripe_types_01::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsScriptAddressesKana =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationMerchantSupportAddress;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsScriptNamesKana =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsScriptNamesKana;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsScriptNamesKanji =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsScriptNamesKana;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityBusinessDetailsStructure =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityEntityType =
    stripe_types_01::v2_core::CreateCoreAccountTokenParamsIdentityEntityType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualDateOfBirth =
    stripe_types_01::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsRegistrationDate;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualDocumentsPassportType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualDocumentsPrimaryVerificationFrontBack =
    stripe_types_01::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualDocumentsPrimaryVerificationType =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualDocumentsVisaType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualPoliticalExposure =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualPoliticalExposure;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualRelationship =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualRelationship;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualScriptNamesKana =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityIndividualScriptNamesKana;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountParamsIdentityIndividualScriptNamesKanji =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityIndividualScriptNamesKana;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsAdditionalAddressesItemPurpose =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualAdditionalAddressesItemPurpose;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsAdditionalTermsOfServiceAccount =
    stripe_types_01::v2_core::CreateCoreAccountParamsIdentityAttestationsTermsOfServiceAccount;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsAddress =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationMerchantSupportAddress;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsDateOfBirth =
    stripe_types_01::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsRegistrationDate;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsDocumentsCompanyAuthorizationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsDocumentsPassportType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsDocumentsPrimaryVerificationFrontBack =
    stripe_types_01::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsDocumentsVisaType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsIdNumbersItemType =
    stripe_types_04::v2_core::CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonParamsScriptAddressesKana =
    stripe_types_00::v2_core::CreateCoreAccountParamsConfigurationMerchantSupportAddress;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsAdditionalAddressesItemPurpose =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualAdditionalAddressesItemPurpose;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsAdditionalNamesItemPurpose =
    stripe_types_04::v2_core::CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItemPurpose;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsAddress =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsDateOfBirth =
    stripe_types_01::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsRegistrationDate;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsDocumentsCompanyAuthorizationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsDocumentsPassportType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsDocumentsPrimaryVerificationFrontBack =
    stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsDocumentsVisaType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsIdNumbersItemType =
    stripe_types_04::v2_core::CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountPersonTokenParamsScriptNamesKanji =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityIndividualScriptNamesKana;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityAttestationsOwnershipDeclaration =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityAttestationsDirectorshipDeclaration;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityAttestationsRepresentativeDeclaration =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityAttestationsDirectorshipDeclaration;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityAttestationsTermsOfService {
    #[doc = "Details on the Account's acceptance of the Stripe Services Agreement; IP, date, and User Agent are expanded by Stripe."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub account:
        Option<stripe_types_01::v2_core::CreateCoreAccountTokenParamsIdentityAttestationsTermsOfServiceAccount>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerification {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType,
}

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyRegistrationVerificationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyTaxIdVerificationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerification {
    #[doc = "The [file upload](https://docs.stripe.com/api/persons/update#create_file) tokens referring to each side of the document."]
    pub front_back: stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack,
    #[doc = "The format of the verification document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType,
}

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfAddressType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnershipSigner =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfRegistrationSigner;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityBusinessDetailsScriptAddressesKanji =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityBusinessDetailsScriptNamesKanji =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsScriptNamesKana;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityIndividualAdditionalAddressesItem {
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
    pub purpose: stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualAdditionalAddressesItemPurpose,
    #[doc = "State, county, province, or region."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Town or district."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityIndividualDocumentsCompanyAuthorizationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityIndividualDocumentsPrimaryVerificationType =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityIndividualDocumentsSecondaryVerificationType =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItem {
    #[doc = "The ID number type of an individual."]
    #[json(rename = "type")]
    pub r#type: stripe_types_04::v2_core::CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType,
    #[doc = "The value of the ID number."]
    pub value: String,
}

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityIndividualScriptAddressesKanji =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
pub type CreateCoreAccountTokenParamsIdentityIndividualScriptNamesKanji =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityIndividualScriptNamesKana;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreEventDestinationParams {
    #[doc = "AWS account and region where Stripe creates the EventBridge partner event source."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub amazon_eventbridge: Option<stripe_types_00::v2_core::CreateCoreEventDestinationParamsAmazonEventbridge>,
    #[doc = "Azure subscription, resource group, and region where Stripe creates the partner topic."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub azure_event_grid: Option<stripe_types_04::v2_core::CreateCoreEventDestinationParamsAzureEventGrid>,
    #[doc = "An optional user-defined description of the destination's purpose."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The list of event types enabled for delivery to this destination."]
    pub enabled_events: Vec<String>,
    #[doc = "Whether to deliver as snapshot or thin events."]
    pub event_payload: stripe_types_01::v2_core::CreateCoreEventDestinationParamsEventPayload,
    #[doc = "The account or organization scopes that can supply events."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub events_from: Option<Vec<String>>,
    #[doc = "Include normally redacted webhook fields in the create response."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<stripe_types_04::v2_core::CreateCoreEventDestinationParamsIncludeItem>>,
    #[doc = "User-defined key/value data for the destination."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "A user-defined label for identifying the destination."]
    pub name: String,
    #[doc = "For snapshot events only, the Stripe API version used to render event objects; do not provide this for thin events."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub snapshot_api_version: Option<String>,
    #[doc = "The delivery transport."]
    #[json(rename = "type")]
    pub r#type: stripe_types_01::v2_core::CreateCoreEventDestinationParamsType,
    #[doc = "Delivery target for the webhook endpoint."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub webhook_endpoint: Option<stripe_types_04::v2_core::CreateCoreEventDestinationParamsWebhookEndpoint>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, Default)]
pub struct ListCoreEventsParams {
    pub created: Option<stripe_types_01::v2_core::ListCoreEventsParamsCreated>,
    pub limit: Option<i64>,
    pub object_id: Option<String>,
    pub types: Option<Vec<String>>,
}

#[cfg(feature = "v2_core")]
impl crate::params::FormEncode for ListCoreEventsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("object_id", self.object_id.as_ref());
        if let Some(values) = self.types.as_ref() {
            form.field_repeated("types", values.as_slice());
        }
    }
}

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay =
    stripe_types_04::v2_core::CreateCoreAccountParamsConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantBacsDebitPayments =
    stripe_types_02::v2_core::CreateCoreAccountParamsConfigurationMerchantBacsDebitPayments;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesAchDebitPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesAcssDebitPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesAffirmPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesAuBecsDebitPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesBacsDebitPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesBancontactPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesBlikPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesBlikRecurringPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesCartesBancairesPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesFpxPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesGrabpayPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesIdealPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesJpBankTransferPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesKlarnaPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesLinkPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesNaverPayPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesP24Payments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesPayByBankPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesPaynowPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesRevolutPayPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesSepaBankTransferPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesSepaDebitPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesSunbitPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesSwishPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantCapabilitiesUsBankTransferPayments =
    stripe_types_02::v2_core::UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantScriptStatementDescriptorKanji =
    stripe_types_04::v2_core::CreateCoreAccountParamsConfigurationMerchantScriptStatementDescriptorKana;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsConfigurationMerchantStatementDescriptor =
    stripe_types_04::v2_core::CreateCoreAccountParamsConfigurationMerchantScriptStatementDescriptorKana;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsDashboard = stripe_types_02::v2_core::CreateCoreAccountParamsDashboard;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsDefaultsLocalesItem =
    stripe_types_01::v2_core::CreateCoreAccountParamsDefaultsLocalesItem;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsDefaultsProfile = stripe_types_01::v2_core::CreateCoreAccountParamsDefaultsProfile;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsDefaultsResponsibilitiesFeesCollector =
    stripe_types_00::v2_core::CreateCoreAccountParamsDefaultsResponsibilitiesFeesCollector;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityAttestationsRepresentativeDeclaration =
    stripe_types_00::v2_core::CreateCoreAccountParamsIdentityAttestationsDirectorshipDeclaration;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityAttestationsTermsOfServiceAccount =
    stripe_types_00::v2_core::CreateCoreAccountParamsIdentityAttestationsDirectorshipDeclaration;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityAttestationsTermsOfServiceCryptoMoneyManager =
    stripe_types_00::v2_core::CreateCoreAccountParamsIdentityAttestationsDirectorshipDeclaration;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose =
    stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsAddress =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsAnnualRevenueAmount =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAnnualRevenueAmount;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyLicenseType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyMinisterialDecreeType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack =
    stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnershipSigner =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfRegistrationSigner;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnershipType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsMonthlyEstimatedRevenueAmount =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAnnualRevenueAmount;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsRegistrationDate =
    stripe_types_01::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsRegistrationDate;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsScriptAddressesKanji =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsScriptNamesKana =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsScriptNamesKana;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityBusinessDetailsScriptNamesKanji =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsScriptNamesKana;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityEntityType =
    stripe_types_01::v2_core::CreateCoreAccountTokenParamsIdentityEntityType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualAddress =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualDateOfBirth =
    stripe_types_01::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsRegistrationDate;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualDocumentsCompanyAuthorizationType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualDocumentsSecondaryVerificationType =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualLegalGender =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualLegalGender;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualPoliticalExposure =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualPoliticalExposure;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIdentityIndividualScriptAddressesKana =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountParamsIncludeItem = stripe_types_01::v2_core::CreateCoreAccountParamsIncludeItem;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsAdditionalTermsOfServiceAccount =
    stripe_types_00::v2_core::CreateCoreAccountParamsIdentityAttestationsDirectorshipDeclaration;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsDateOfBirth =
    stripe_types_01::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsRegistrationDate;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsDocumentsPassportType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsDocumentsPrimaryVerificationFrontBack =
    stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsDocumentsPrimaryVerificationType =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsDocumentsSecondaryVerificationFrontBack =
    stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsDocumentsVisaType = stripe_types_02::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsLegalGender =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualLegalGender;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsPoliticalExposure =
    stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualPoliticalExposure;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsScriptNamesKana =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityIndividualScriptNamesKana;

#[cfg(feature = "v2_core")]
pub type UpdateCoreAccountPersonParamsScriptNamesKanji =
    stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityIndividualScriptNamesKana;

#[cfg(feature = "v2_core")]
pub type UpdateCoreEventDestinationParamsWebhookEndpoint =
    stripe_types_04::v2_core::CreateCoreEventDestinationParamsWebhookEndpoint;

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationCustomerAutomaticIndirectTax {
    #[doc = "The customer account's tax exemption status: `none`, `exempt`, or `reverse`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub exempt: Option<stripe_types_01::v2_core::V2CoreAccountConfigurationCustomerAutomaticIndirectTaxExempt>,
    #[doc = "A recent IP address of the customer used for tax reporting and tax location inference."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The customer account's identified tax location, derived from `location_source`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<stripe_types_01::v2_core::V2CoreAccountConfigurationCustomerAutomaticIndirectTaxLocation>,
    #[doc = "Data source used to identify the customer account's tax location."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub location_source:
        Option<stripe_types_00::v2_core::V2CoreAccountConfigurationCustomerAutomaticIndirectTaxLocationSource>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationCustomerBillingInvoiceRendering {
    #[doc = "Indicates whether displayed line item prices and amounts on invoice PDFs include inclusive tax amounts."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_tax_display:
        Option<stripe_types_01::v2_core::V2CoreAccountConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay>,
    #[doc = "ID of the invoice rendering template to use for future invoices."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItem {
    #[doc = "Machine-readable code explaining the reason for the Capability to be in its current status."]
    #[json(default)]
    pub code: stripe_types_03::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode,
    #[doc = "Machine-readable code explaining how to make the Capability active."]
    #[json(default)]
    pub resolution: stripe_types_03::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemResolution,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationCustomerShipping {
    #[doc = "Customer shipping address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_04::v2_core::V2CoreAccountConfigurationCustomerShippingAddress>,
    #[doc = "Customer name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Customer phone (including extension)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationMerchantCardPayments {
    #[doc = "Automatically declines certain charge types regardless of whether the card issuer accepted or declined the charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub decline_on: Option<stripe_types_01::v2_core::V2CoreAccountConfigurationMerchantCardPaymentsDeclineOn>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationMerchantScriptStatementDescriptor {
    #[doc = "The Kana variation of statement_descriptor used for charges in Japan."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_00::v2_core::V2CoreAccountConfigurationMerchantScriptStatementDescriptorKana>,
    #[doc = "The Kanji variation of statement_descriptor used for charges in Japan."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_00::v2_core::V2CoreAccountConfigurationMerchantScriptStatementDescriptorKana>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountDefaultsResponsibilities {
    #[doc = "Indicates whether the platform or connected account is responsible for paying Stripe fees for pricing-control-eligibl..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fees_collector: Option<stripe_types_04::v2_core::V2CoreAccountDefaultsResponsibilitiesFeesCollector>,
    #[doc = "A value indicating the responsibility for losses on this account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub losses_collector: Option<stripe_types_02::v2_core::V2CoreAccountDefaultsResponsibilitiesLossesCollector>,
    #[doc = "A value indicating responsibility for collecting requirements on this account."]
    #[json(default)]
    pub requirements_collector: stripe_types_02::v2_core::V2CoreAccountDefaultsResponsibilitiesLossesCollector,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadline {
    #[doc = "The current status of the requirement's impact."]
    #[json(default)]
    pub status: stripe_types_03::v2_core::V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadlineStatus,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountFutureRequirementsEntriesItemReference {
    #[doc = "If `inquiry` is the type, the inquiry token."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub inquiry: Option<String>,
    #[doc = "If `resource` is the type, the resource token."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The type of the reference."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::v2_core::V2CoreAccountFutureRequirementsEntriesItemReferenceType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountFutureRequirementsSummaryMinimumDeadline {
    #[doc = "The current strictest status of all requirements on the Account."]
    #[json(default)]
    pub status: stripe_types_03::v2_core::V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadlineStatus,
    #[doc = "The soonest RFC3339 date & time UTC value a requirement can impact the Account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityAttestationsPersonsProvided {
    #[doc = "Whether the company’s directors have been provided."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub directors: Option<bool>,
    #[doc = "Whether the company’s executives have been provided."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub executives: Option<bool>,
    #[doc = "Whether the company’s owners have been provided."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub owners: Option<bool>,
    #[doc = "Reason for why the company is exempt from providing ownership information."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ownership_exemption_reason:
        Option<stripe_types_00::v2_core::V2CoreAccountIdentityAttestationsPersonsProvidedOwnershipExemptionReason>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityBusinessDetailsAnnualRevenue {
    #[doc = "Annual revenue amount in minor currency units (for example, '123' for 1.23 USD)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<stripe_types_04::v2_core::V2CoreAccountIdentityBusinessDetailsAnnualRevenueAmount>,
    #[doc = "The close-out date of the preceding fiscal year in ISO 8601 format."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fiscal_year_end: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerification {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    #[json(default)]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type", default)]
    pub r#type:
        stripe_types_00::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityBusinessDetailsDocumentsProofOfRegistration {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    #[json(default)]
    pub files: Vec<String>,
    #[doc = "Person that is signing the document."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub signer:
        Option<stripe_types_04::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsProofOfRegistrationSigner>,
    #[doc = "The format of the document."]
    #[json(rename = "type", default)]
    pub r#type:
        stripe_types_00::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityBusinessDetailsIdNumbersItem {
    #[doc = "The registrar of the ID number (Only valid for DE ID number types)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registrar: Option<String>,
    #[doc = "Open Enum."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::v2_core::V2CoreAccountIdentityBusinessDetailsIdNumbersItemType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityBusinessDetailsMonthlyEstimatedRevenue {
    #[doc = "Estimated monthly revenue amount in minor currency units (for example, '123' for 1.23 USD)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<stripe_types_04::v2_core::V2CoreAccountIdentityBusinessDetailsAnnualRevenueAmount>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityBusinessDetailsScriptAddresses {
    #[doc = "Kana Address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_01::v2_core::V2CoreAccountConfigurationMerchantSupportAddress>,
    #[doc = "Kanji Address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_01::v2_core::V2CoreAccountConfigurationMerchantSupportAddress>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityBusinessDetailsScriptNames {
    #[doc = "Kana name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_03::v2_core::V2CoreAccountIdentityBusinessDetailsScriptNamesKana>,
    #[doc = "Kanji name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_03::v2_core::V2CoreAccountIdentityBusinessDetailsScriptNamesKana>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreEventDestinationAzureEventGrid {
    #[doc = "The name of the Stripe-created partner topic that receives events."]
    #[json(default)]
    pub azure_partner_topic_name: String,
    #[doc = "The Azure-reported lifecycle state of the partner topic."]
    #[json(default)]
    pub azure_partner_topic_status:
        stripe_types_02::v2_core::V2CoreEventDestinationAzureEventGridAzurePartnerTopicStatus,
    #[doc = "The Azure region where the partner topic is located."]
    #[json(default)]
    pub azure_region: String,
    #[doc = "The Azure resource group containing the partner topic."]
    #[json(default)]
    pub azure_resource_group_name: String,
    #[doc = "The Azure subscription containing the resource group and partner topic."]
    #[json(default)]
    pub azure_subscription_id: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreEventDestinationStatusDetailsDisabled {
    #[doc = "Reason event destination has been disabled."]
    #[json(default)]
    pub reason: stripe_types_04::v2_core::V2CoreEventDestinationStatusDetailsDisabledReason,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreEventReason {
    #[doc = "Information on the API request that instigated the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<stripe_types_03::v2_core::V2CoreEventReasonRequest>,
    #[doc = "Event reason type."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::v2_core::V2CoreEventReasonType,
}
