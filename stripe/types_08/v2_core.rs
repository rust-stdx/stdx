#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationMerchantScriptStatementDescriptor {
    #[doc = "The Kana variation of statement_descriptor used for charges in Japan."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana:
        Option<stripe_types_04::v2_core::CreateCoreAccountParamsConfigurationMerchantScriptStatementDescriptorKana>,
    #[doc = "The Kanji variation of statement_descriptor used for charges in Japan."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantScriptStatementDescriptorKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationRecipientCapabilitiesStripeBalance {
    #[doc = "Enables this Account to receive /v1/transfers into their Stripe Balance (/v1/balance)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub stripe_transfers: Option<
        stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationRecipientCapabilitiesStripeBalanceStripeTransfers,
    >,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerification {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyMinisterialDecree {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type:
        stripe_types_05::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyMinisterialDecreeType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfAddress {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfAddressType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfRegistration {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "Person that is signing the document."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub signer: Option<
        stripe_types_06::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfRegistrationSigner,
    >,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type:
        stripe_types_06::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfRegistrationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnership {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "Person that is signing the document."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub signer: Option<stripe_types_05::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnershipSigner>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnershipType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsIdNumbersItem {
    #[doc = "The registrar of the ID number (Only valid for DE ID number types)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub registrar: Option<String>,
    #[doc = "Open Enum."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsIdNumbersItemType,
    #[doc = "The value of the ID number."]
    pub value: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsMonthlyEstimatedRevenue {
    #[doc = "A non-negative integer representing the amount in the smallest currency unit."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub amount:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsMonthlyEstimatedRevenueAmount>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityIndividualAdditionalAddressesItem {
    #[doc = "City, district, suburb, town, or village."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: String,
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
    pub purpose: stripe_types_05::v2_core::CreateCoreAccountParamsIdentityIndividualAdditionalAddressesItemPurpose,
    #[doc = "State, county, province, or region."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Town or district."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityIndividualDocumentsCompanyAuthorization {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::CreateCoreAccountParamsIdentityIndividualDocumentsCompanyAuthorizationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityIndividualDocumentsPassport {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::CreateCoreAccountParamsIdentityIndividualDocumentsPassportType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityIndividualScriptAddresses {
    #[doc = "Kana Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_05::v2_core::CreateCoreAccountParamsIdentityIndividualScriptAddressesKana>,
    #[doc = "Kanji Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_05::v2_core::CreateCoreAccountParamsIdentityIndividualScriptAddressesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonParamsAdditionalTermsOfService {
    #[doc = "Stripe terms of service agreement."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub account: Option<stripe_types_06::v2_core::CreateCoreAccountPersonParamsAdditionalTermsOfServiceAccount>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonParamsDocumentsCompanyAuthorization {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::CreateCoreAccountPersonParamsDocumentsCompanyAuthorizationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonParamsDocumentsPassport {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::CreateCoreAccountPersonParamsDocumentsPassportType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonParamsDocumentsPrimaryVerification {
    #[doc = "The [file upload](https://docs.stripe.com/api/persons/update#create_file) tokens referring to each side of the document."]
    pub front_back: stripe_types_06::v2_core::CreateCoreAccountPersonParamsDocumentsPrimaryVerificationFrontBack,
    #[doc = "The format of the verification document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::CreateCoreAccountPersonParamsDocumentsPrimaryVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonTokenParamsAdditionalTermsOfService {
    #[doc = "Details on the Person's acceptance of the Stripe Services Agreement; IP, date, and User Agent are expanded by Stripe."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub account: Option<stripe_types_05::v2_core::CreateCoreAccountPersonTokenParamsAdditionalTermsOfServiceAccount>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonTokenParamsScriptAddresses {
    #[doc = "Kana Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_05::v2_core::CreateCoreAccountPersonTokenParamsScriptAddressesKana>,
    #[doc = "Kanji Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_05::v2_core::CreateCoreAccountPersonTokenParamsScriptAddressesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyLicense {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type:
        stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyLicenseType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyRegistrationVerification {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyRegistrationVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyTaxIdVerification {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyTaxIdVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnership {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "Person that is signing the document."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub signer: Option<stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnershipSigner>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnershipType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsScriptNames {
    #[doc = "Kana name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsScriptNamesKana>,
    #[doc = "Kanji name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsScriptNamesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityIndividualDocumentsPassport {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityIndividualDocumentsPassportType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsConfigurationCustomerShipping {
    #[doc = "Customer shipping address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationCustomerShippingAddress>,
    #[doc = "Customer name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Customer phone (including extension)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsDefaultsResponsibilities {
    #[doc = "A value indicating the party responsible for collecting fees from this account."]
    pub fees_collector: stripe_types_06::v2_core::UpdateCoreAccountParamsDefaultsResponsibilitiesFeesCollector,
    #[doc = "A value indicating who is responsible for losses when this Account can’t pay back negative balances from payments."]
    pub losses_collector: stripe_types_05::v2_core::UpdateCoreAccountParamsDefaultsResponsibilitiesLossesCollector,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityAttestationsPersonsProvided {
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
    pub ownership_exemption_reason: Option<
        stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason,
    >,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityBusinessDetailsAdditionalAddressesItem {
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
    pub purpose: stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose,
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
pub struct UpdateCoreAccountParamsIdentityBusinessDetailsAnnualRevenue {
    #[doc = "A non-negative integer representing the amount in the smallest currency unit."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub amount: Option<stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsAnnualRevenueAmount>,
    #[doc = "The close-out date of the preceding fiscal year in ISO 8601 format."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub fiscal_year_end: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsPrimaryVerification {
    #[doc = "The [file upload](https://docs.stripe.com/api/persons/update#create_file) tokens referring to each side of the document."]
    pub front_back:
        stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack,
    #[doc = "The format of the verification document."]
    #[json(rename = "type")]
    pub r#type:
        stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityBusinessDetailsScriptAddresses {
    #[doc = "Kana Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsScriptAddressesKana>,
    #[doc = "Kanji Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsScriptAddressesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityBusinessDetailsScriptNames {
    #[doc = "Kana name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsScriptNamesKana>,
    #[doc = "Kanji name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsScriptNamesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityIndividualAdditionalNamesItem {
    #[doc = "The person's full name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[doc = "The person's first or given name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The purpose or type of the additional name."]
    pub purpose: stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityIndividualAdditionalNamesItemPurpose,
    #[doc = "The person's last or family name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityIndividualDocumentsPrimaryVerification {
    #[doc = "The [file upload](https://docs.stripe.com/api/persons/update#create_file) tokens referring to each side of the document."]
    pub front_back:
        stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityIndividualDocumentsPrimaryVerificationFrontBack,
    #[doc = "The format of the verification document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityIndividualDocumentsPrimaryVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityIndividualDocumentsVisa {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityIndividualDocumentsVisaType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityIndividualIdNumbersItem {
    #[doc = "The ID number type of an individual."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityIndividualIdNumbersItemType,
    #[doc = "The value of the ID number."]
    pub value: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityIndividualScriptAddresses {
    #[doc = "Kana Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityIndividualScriptAddressesKana>,
    #[doc = "Kanji Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityIndividualScriptAddressesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountPersonParamsAdditionalNamesItem {
    #[doc = "The person's full name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[doc = "The person's first or given name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The purpose or type of the additional name."]
    pub purpose: stripe_types_05::v2_core::UpdateCoreAccountPersonParamsAdditionalNamesItemPurpose,
    #[doc = "The person's last or family name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountPersonParamsDocumentsPrimaryVerification {
    #[doc = "The [file upload](https://docs.stripe.com/api/persons/update#create_file) tokens referring to each side of the document."]
    pub front_back: stripe_types_06::v2_core::UpdateCoreAccountPersonParamsDocumentsPrimaryVerificationFrontBack,
    #[doc = "The format of the verification document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::UpdateCoreAccountPersonParamsDocumentsPrimaryVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountPersonParamsDocumentsVisa {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::UpdateCoreAccountPersonParamsDocumentsVisaType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountPersonParamsScriptAddresses {
    #[doc = "Kana Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_05::v2_core::UpdateCoreAccountPersonParamsScriptAddressesKana>,
    #[doc = "Kanji Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_05::v2_core::UpdateCoreAccountPersonParamsScriptAddressesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountPersonParamsScriptNames {
    #[doc = "Persons name in kana script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_06::v2_core::UpdateCoreAccountPersonParamsScriptNamesKana>,
    #[doc = "Persons name in kanji script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_06::v2_core::UpdateCoreAccountPersonParamsScriptNamesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountDefaults {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The Account's preferred locales (languages), ordered by preference."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub locales: Option<Vec<stripe_types_01::v2_core::V2CoreAccountDefaultsLocalesItem>>,
    #[doc = "Account profile information."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<stripe_types_04::v2_core::V2CoreAccountDefaultsProfile>,
    #[doc = "Default responsibilities held by either Stripe or the platform."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub responsibilities: Option<stripe_types_06::v2_core::V2CoreAccountDefaultsResponsibilities>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountFutureRequirementsSummary {
    #[doc = "The soonest date and time a requirement on the Account will become `past due`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub minimum_deadline: Option<stripe_types_06::v2_core::V2CoreAccountFutureRequirementsSummaryMinimumDeadline>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityAttestations {
    #[doc = "This hash is used to attest that the directors information provided to Stripe is both current and correct."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub directorship_declaration:
        Option<stripe_types_02::v2_core::V2CoreAccountIdentityAttestationsDirectorshipDeclaration>,
    #[doc = "This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ownership_declaration:
        Option<stripe_types_02::v2_core::V2CoreAccountIdentityAttestationsDirectorshipDeclaration>,
    #[doc = "Attestation that all Persons with a specific Relationship value have been provided."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub persons_provided: Option<stripe_types_06::v2_core::V2CoreAccountIdentityAttestationsPersonsProvided>,
    #[doc = "This hash is used to attest that the representative is authorized to act as the representative of their legal entity."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub representative_declaration:
        Option<stripe_types_02::v2_core::V2CoreAccountIdentityAttestationsDirectorshipDeclaration>,
    #[doc = "Attestations of accepted terms of service agreements."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<stripe_types_05::v2_core::V2CoreAccountIdentityAttestationsTermsOfService>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityBusinessDetailsDocuments {
    #[doc = "One or more documents that support the Bank account ownership verification requirement."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification:
        Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerification>,
    #[doc = "One or more documents that demonstrate proof of a company's license to operate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub company_license:
        Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerification>,
    #[doc = "One or more documents showing the company's Memorandum of Association."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub company_memorandum_of_association:
        Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerification>,
    #[doc = "Certain countries only: One or more documents showing the ministerial decree legalizing the company's establishment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub company_ministerial_decree:
        Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerification>,
    #[doc = "One or more documents that demonstrate proof of a company's registration with the appropriate local authorities."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub company_registration_verification:
        Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerification>,
    #[doc = "One or more documents that demonstrate proof of a company's tax ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub company_tax_id_verification:
        Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerification>,
    #[doc = "A document verifying the business."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub primary_verification:
        Option<stripe_types_05::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsPrimaryVerification>,
    #[doc = "One or more documents that demonstrate proof of address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub proof_of_address:
        Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerification>,
    #[doc = "One or more documents showing the company's proof of registration with the national business registry."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub proof_of_registration:
        Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsProofOfRegistration>,
    #[doc = "One or more documents that demonstrate proof of ultimate beneficial ownership."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub proof_of_ultimate_beneficial_ownership:
        Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsProofOfRegistration>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountLinkUseCaseAccountOnboarding {
    #[doc = "Specifies the requirements that Stripe collects from v2/core/accounts in the Onboarding flow."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub collection_options:
        Option<stripe_types_05::v2_core::V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptions>,
    #[doc = "The URL the user will be redirected to if the AccountLink is expired, has been used, or is otherwise invalid."]
    #[json(default)]
    pub refresh_url: String,
    #[doc = "The URL that the user will be redirected to upon completing the linked flow."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}
