#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountLinkParamsUseCaseAccountUpdateCollectionOptions {
    #[doc = "Specifies whether the platform collects only currently_due requirements (`currently_due`) or both currently_due and e..."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub fields:
        Option<stripe_types_05::v2_core::CreateCoreAccountLinkParamsUseCaseAccountUpdateCollectionOptionsFields>,
    #[doc = "Specifies whether the platform collects future_requirements in addition to requirements in Connect Onboarding."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<
        stripe_types_06::v2_core::CreateCoreAccountLinkParamsUseCaseAccountUpdateCollectionOptionsFutureRequirements,
    >,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationCustomerBillingInvoice {
    #[doc = "The list of up to 4 default custom fields to be displayed on invoices for this customer."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<
        Vec<stripe_types_04::v2_core::CreateCoreAccountParamsConfigurationCustomerBillingInvoiceCustomFieldsItem>,
    >,
    #[doc = "Default invoice footer."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[doc = "Sequence number to use on the customer account's next invoice."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub next_sequence: Option<i64>,
    #[doc = "Prefix used to generate unique invoice numbers."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[doc = "Default invoice PDF rendering options."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub rendering:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationCustomerBillingInvoiceRendering>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationMerchantKonbiniPayments {
    #[doc = "Support for Konbini payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub support: Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantKonbiniPaymentsSupport>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsDefaults {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The Account's preferred locales (languages), ordered by preference."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub locales: Option<Vec<stripe_types_01::v2_core::CreateCoreAccountParamsDefaultsLocalesItem>>,
    #[doc = "Account profile information."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub profile: Option<stripe_types_01::v2_core::CreateCoreAccountParamsDefaultsProfile>,
    #[doc = "Default responsibilities held by either Stripe or the platform."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub responsibilities: Option<stripe_types_05::v2_core::CreateCoreAccountParamsDefaultsResponsibilities>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityAttestationsPersonsProvided {
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
        stripe_types_06::v2_core::CreateCoreAccountParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason,
    >,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsScriptAddresses {
    #[doc = "Kana Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_06::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsScriptAddressesKana>,
    #[doc = "Kanji Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_05::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsScriptAddressesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityIndividualAdditionalNamesItem {
    #[doc = "The person's full name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[doc = "The person's first or given name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The purpose or type of the additional name."]
    pub purpose: stripe_types_05::v2_core::CreateCoreAccountParamsIdentityIndividualAdditionalNamesItemPurpose,
    #[doc = "The person's last or family name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityIndividualDocumentsPrimaryVerification {
    #[doc = "The [file upload](https://docs.stripe.com/api/persons/update#create_file) tokens referring to each side of the document."]
    pub front_back:
        stripe_types_06::v2_core::CreateCoreAccountParamsIdentityIndividualDocumentsPrimaryVerificationFrontBack,
    #[doc = "The format of the verification document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::CreateCoreAccountParamsIdentityIndividualDocumentsPrimaryVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityIndividualDocumentsVisa {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::CreateCoreAccountParamsIdentityIndividualDocumentsVisaType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityIndividualIdNumbersItem {
    #[doc = "The ID number type of an individual."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::CreateCoreAccountParamsIdentityIndividualIdNumbersItemType,
    #[doc = "The value of the ID number."]
    pub value: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityIndividualScriptNames {
    #[doc = "Persons name in kana script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_06::v2_core::CreateCoreAccountParamsIdentityIndividualScriptNamesKana>,
    #[doc = "Persons name in kanji script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_06::v2_core::CreateCoreAccountParamsIdentityIndividualScriptNamesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonParamsAdditionalAddressesItem {
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
    pub purpose: stripe_types_06::v2_core::CreateCoreAccountPersonParamsAdditionalAddressesItemPurpose,
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
pub struct CreateCoreAccountPersonParamsDocumentsSecondaryVerification {
    #[doc = "The [file upload](https://docs.stripe.com/api/persons/update#create_file) tokens referring to each side of the document."]
    pub front_back: stripe_types_05::v2_core::CreateCoreAccountPersonParamsDocumentsSecondaryVerificationFrontBack,
    #[doc = "The format of the verification document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::CreateCoreAccountPersonParamsDocumentsSecondaryVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonParamsIdNumbersItem {
    #[doc = "The ID number type of an individual."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::CreateCoreAccountPersonParamsIdNumbersItemType,
    #[doc = "The value of the ID number."]
    pub value: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonParamsScriptAddresses {
    #[doc = "Kana Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_06::v2_core::CreateCoreAccountPersonParamsScriptAddressesKana>,
    #[doc = "Kanji Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_05::v2_core::CreateCoreAccountPersonParamsScriptAddressesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonParamsScriptNames {
    #[doc = "Persons name in kana script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_05::v2_core::CreateCoreAccountPersonParamsScriptNamesKana>,
    #[doc = "Persons name in kanji script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_05::v2_core::CreateCoreAccountPersonParamsScriptNamesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonTokenParamsDocumentsSecondaryVerification {
    #[doc = "The [file upload](https://docs.stripe.com/api/persons/update#create_file) tokens referring to each side of the document."]
    pub front_back: stripe_types_05::v2_core::CreateCoreAccountPersonTokenParamsDocumentsSecondaryVerificationFrontBack,
    #[doc = "The format of the verification document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::CreateCoreAccountPersonTokenParamsDocumentsSecondaryVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonTokenParamsScriptNames {
    #[doc = "Persons name in kana script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_05::v2_core::CreateCoreAccountPersonTokenParamsScriptNamesKana>,
    #[doc = "Persons name in kanji script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_06::v2_core::CreateCoreAccountPersonTokenParamsScriptNamesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityAttestations {
    #[doc = "This hash is used to attest that the directors information provided to Stripe is both current and correct; IP, date, ..."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub directorship_declaration:
        Option<stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityAttestationsDirectorshipDeclaration>,
    #[doc = "This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct; IP,..."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration:
        Option<stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityAttestationsOwnershipDeclaration>,
    #[doc = "Attestation that all Persons with a specific Relationship value have been provided."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub persons_provided:
        Option<stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityAttestationsPersonsProvided>,
    #[doc = "This hash is used to attest that the representative is authorized to act as the representative of their legal entity;..."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub representative_declaration:
        Option<stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityAttestationsRepresentativeDeclaration>,
    #[doc = "Attestations of accepted terms of service agreements."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub terms_of_service:
        Option<stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityAttestationsTermsOfService>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfAddress {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type:
        stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfAddressType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfRegistration {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "Person that is signing the document."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub signer: Option<
        stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfRegistrationSigner,
    >,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type:
        stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfRegistrationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsScriptAddresses {
    #[doc = "Kana Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsScriptAddressesKana>,
    #[doc = "Kanji Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji:
        Option<stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsScriptAddressesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityIndividualDocumentsCompanyAuthorization {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type:
        stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityIndividualDocumentsCompanyAuthorizationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityIndividualDocumentsPrimaryVerification {
    #[doc = "The [file upload](https://docs.stripe.com/api/persons/update#create_file) tokens referring to each side of the document."]
    pub front_back:
        stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityIndividualDocumentsPrimaryVerificationFrontBack,
    #[doc = "The format of the verification document."]
    #[json(rename = "type")]
    pub r#type:
        stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityIndividualDocumentsPrimaryVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityIndividualDocumentsVisa {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityIndividualDocumentsVisaType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, Default)]
pub struct GetCoreEventDestinationParams {
    pub include: Option<Vec<stripe_types_05::v2_core::GetCoreEventDestinationParamsIncludeItem>>,
}

#[cfg(feature = "v2_core")]
impl crate::params::FormEncode for GetCoreEventDestinationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.include.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("include", index, item);
            }
        }
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsConfigurationCustomerBillingInvoiceRendering {
    #[doc = "Indicates whether displayed line item prices and amounts on invoice PDFs include inclusive tax amounts."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<
        stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay,
    >,
    #[doc = "ID of the invoice rendering template to use for future invoices."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsConfigurationMerchantKonbiniPaymentsSupport {
    #[doc = "Support email address for Konbini payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Support hours for Konbini payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub hours:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantKonbiniPaymentsSupportHours>,
    #[doc = "Support phone number for Konbini payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsConfigurationMerchantSupport {
    #[doc = "A publicly available mailing address for sending support issues to."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantSupportAddress>,
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
pub struct UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerification {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyLicense {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyLicenseType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyMemorandumOfAssociation {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyMemorandumOfAssociationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyRegistrationVerification {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyRegistrationVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyTaxIdVerification {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type:
        stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyTaxIdVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityBusinessDetailsMonthlyEstimatedRevenue {
    #[doc = "A non-negative integer representing the amount in the smallest currency unit."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub amount:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsMonthlyEstimatedRevenueAmount>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityIndividualAdditionalAddressesItem {
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
    pub purpose: stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityIndividualAdditionalAddressesItemPurpose,
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
pub struct UpdateCoreAccountParamsIdentityIndividualDocumentsSecondaryVerification {
    #[doc = "The [file upload](https://docs.stripe.com/api/persons/update#create_file) tokens referring to each side of the document."]
    pub front_back:
        stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityIndividualDocumentsSecondaryVerificationFrontBack,
    #[doc = "The format of the verification document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityIndividualDocumentsSecondaryVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountPersonParamsAdditionalAddressesItem {
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
    pub purpose: stripe_types_05::v2_core::UpdateCoreAccountPersonParamsAdditionalAddressesItemPurpose,
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
pub struct UpdateCoreAccountPersonParamsAdditionalTermsOfService {
    #[doc = "Stripe terms of service agreement."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub account: Option<stripe_types_06::v2_core::UpdateCoreAccountPersonParamsAdditionalTermsOfServiceAccount>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountPersonParamsDocumentsCompanyAuthorization {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::UpdateCoreAccountPersonParamsDocumentsCompanyAuthorizationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreEventDestinationParams {
    #[doc = "An optional user-defined description of the destination's purpose; it does not control routing."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The list of event types enabled for delivery to this destination."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub enabled_events: Option<Vec<String>>,
    #[doc = "Include the normally redacted `webhook_endpoint.url` in the response."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<stripe_types_00::v2_core::UpdateCoreEventDestinationParamsIncludeItem>>,
    #[doc = "Metadata."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "A user-defined label for identifying the destination; it does not control routing."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "New delivery target for the webhook endpoint."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub webhook_endpoint: Option<stripe_types_06::v2_core::UpdateCoreEventDestinationParamsWebhookEndpoint>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationCustomerBillingInvoice {
    #[doc = "The list of up to 4 default custom fields to be displayed on invoices for this customer."]
    #[json(default)]
    pub custom_fields: Vec<stripe_types_00::v2_core::V2CoreAccountConfigurationCustomerBillingInvoiceCustomFieldsItem>,
    #[doc = "Default invoice footer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[doc = "Sequence number to use on the customer account's next invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub next_sequence: Option<i64>,
    #[doc = "Prefix used to generate unique invoice numbers."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[doc = "Default invoice PDF rendering options."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub rendering: Option<stripe_types_06::v2_core::V2CoreAccountConfigurationCustomerBillingInvoiceRendering>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax {
    #[doc = "The status of the Capability."]
    #[json(default)]
    pub status: stripe_types_03::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus,
    #[doc = "Additional details about the capability's status."]
    #[json(default)]
    pub status_details: Vec<
        stripe_types_06::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItem,
    >,
}
