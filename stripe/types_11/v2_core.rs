#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationMerchant {
    #[doc = "Settings used for Bacs debit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments:
        Option<stripe_types_02::v2_core::CreateCoreAccountParamsConfigurationMerchantBacsDebitPayments>,
    #[doc = "Settings used to apply the merchant's branding to email receipts, invoices, Checkout, and other products."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub branding: Option<stripe_types_04::v2_core::CreateCoreAccountParamsConfigurationMerchantBranding>,
    #[doc = "Capabilities to request on the Merchant Configuration."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<stripe_types_09::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilities>,
    #[doc = "Card payments settings."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCardPayments>,
    #[doc = "Settings specific to Konbini payments on the account."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub konbini_payments: Option<stripe_types_07::v2_core::CreateCoreAccountParamsConfigurationMerchantKonbiniPayments>,
    #[doc = "The Merchant Category Code (MCC) for the Merchant Configuration."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    #[doc = "Settings for the default text that appears on statements for language variations."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_statement_descriptor:
        Option<stripe_types_08::v2_core::CreateCoreAccountParamsConfigurationMerchantScriptStatementDescriptor>,
    #[doc = "Settings used for SEPA Direct Debit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments:
        Option<stripe_types_01::v2_core::CreateCoreAccountParamsConfigurationMerchantSepaDebitPayments>,
    #[doc = "Statement descriptor."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantStatementDescriptor>,
    #[doc = "Publicly available contact information for sending support issues to."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub support: Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantSupport>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsDocuments {
    #[doc = "One or more documents that support the bank account ownership verification requirement."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification: Option<stripe_types_08::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerification>,
    #[doc = "One or more documents that demonstrate proof of a company's license to operate."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_license: Option<stripe_types_09::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyLicense>,
    #[doc = "One or more documents showing the company's Memorandum of Association."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_memorandum_of_association: Option<stripe_types_09::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyMemorandumOfAssociation>,
    #[doc = "Certain countries only: One or more documents showing the ministerial decree legalizing the company's establishment."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_ministerial_decree: Option<stripe_types_08::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyMinisterialDecree>,
    #[doc = "One or more documents that demonstrate proof of a company's registration with the appropriate local authorities."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_registration_verification: Option<stripe_types_09::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyRegistrationVerification>,
    #[doc = "One or more documents that demonstrate proof of a company's tax ID."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_tax_id_verification: Option<stripe_types_09::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyTaxIdVerification>,
    #[doc = "A document verifying the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub primary_verification: Option<stripe_types_09::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsPrimaryVerification>,
    #[doc = "One or more documents that demonstrate proof of address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub proof_of_address: Option<stripe_types_08::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfAddress>,
    #[doc = "One or more documents showing the company's proof of registration with the national business registry."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub proof_of_registration: Option<stripe_types_08::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfRegistration>,
    #[doc = "One or more documents that demonstrate proof of ultimate beneficial ownership."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub proof_of_ultimate_beneficial_ownership: Option<stripe_types_08::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnership>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonParamsDocuments {
    #[doc = "One or more documents that demonstrate proof that this person is authorized to represent the company."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_authorization:
        Option<stripe_types_08::v2_core::CreateCoreAccountPersonParamsDocumentsCompanyAuthorization>,
    #[doc = "One or more documents showing the person's passport page with photo and personal data."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub passport: Option<stripe_types_08::v2_core::CreateCoreAccountPersonParamsDocumentsPassport>,
    #[doc = "An identifying document showing the person's name, either a passport or local ID card."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub primary_verification:
        Option<stripe_types_08::v2_core::CreateCoreAccountPersonParamsDocumentsPrimaryVerification>,
    #[doc = "A document showing address, either a passport, local ID card, or utility bill from a well-known utility company."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub secondary_verification:
        Option<stripe_types_07::v2_core::CreateCoreAccountPersonParamsDocumentsSecondaryVerification>,
    #[doc = "One or more documents showing the person's visa required for living in the country where they are residing."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub visa: Option<stripe_types_09::v2_core::CreateCoreAccountPersonParamsDocumentsVisa>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonTokenParamsDocuments {
    #[doc = "One or more documents that demonstrate proof that this person is authorized to represent the company."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_authorization:
        Option<stripe_types_09::v2_core::CreateCoreAccountPersonTokenParamsDocumentsCompanyAuthorization>,
    #[doc = "One or more documents showing the person's passport page with photo and personal data."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub passport: Option<stripe_types_09::v2_core::CreateCoreAccountPersonTokenParamsDocumentsPassport>,
    #[doc = "An identifying document showing the person's name, either a passport or local ID card."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub primary_verification:
        Option<stripe_types_09::v2_core::CreateCoreAccountPersonTokenParamsDocumentsPrimaryVerification>,
    #[doc = "A document showing address, either a passport, local ID card, or utility bill from a well-known utility company."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub secondary_verification:
        Option<stripe_types_07::v2_core::CreateCoreAccountPersonTokenParamsDocumentsSecondaryVerification>,
    #[doc = "One or more documents showing the person's visa required for living in the country where they are residing."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub visa: Option<stripe_types_09::v2_core::CreateCoreAccountPersonTokenParamsDocumentsVisa>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityIndividualDocuments {
    #[doc = "One or more documents that demonstrate proof that this person is authorized to represent the company."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_authorization:
        Option<stripe_types_07::v2_core::CreateCoreAccountTokenParamsIdentityIndividualDocumentsCompanyAuthorization>,
    #[doc = "One or more documents showing the person's passport page with photo and personal data."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub passport: Option<stripe_types_08::v2_core::CreateCoreAccountTokenParamsIdentityIndividualDocumentsPassport>,
    #[doc = "An identifying document showing the person's name, either a passport or local ID card."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub primary_verification:
        Option<stripe_types_07::v2_core::CreateCoreAccountTokenParamsIdentityIndividualDocumentsPrimaryVerification>,
    #[doc = "A document showing address, either a passport, local ID card, or utility bill from a well-known utility company."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub secondary_verification:
        Option<stripe_types_09::v2_core::CreateCoreAccountTokenParamsIdentityIndividualDocumentsSecondaryVerification>,
    #[doc = "One or more documents showing the person's visa required for living in the country where they are residing."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub visa: Option<stripe_types_07::v2_core::CreateCoreAccountTokenParamsIdentityIndividualDocumentsVisa>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsConfigurationMerchantKonbiniPayments {
    #[doc = "Support for Konbini payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub support: Option<stripe_types_07::v2_core::UpdateCoreAccountParamsConfigurationMerchantKonbiniPaymentsSupport>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsConfigurationRecipientCapabilities {
    #[doc = "Capabilities that enable the recipient to manage their Stripe Balance (/v1/balance)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub stripe_balance:
        Option<stripe_types_09::v2_core::UpdateCoreAccountParamsConfigurationRecipientCapabilitiesStripeBalance>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityAttestations {
    #[doc = "This hash is used to attest that the directors information provided to Stripe is both current and correct."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub directorship_declaration:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityAttestationsDirectorshipDeclaration>,
    #[doc = "This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityAttestationsOwnershipDeclaration>,
    #[doc = "Attestation that all Persons with a specific Relationship value have been provided."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub persons_provided: Option<stripe_types_08::v2_core::UpdateCoreAccountParamsIdentityAttestationsPersonsProvided>,
    #[doc = "This hash is used to attest that the representative is authorized to act as the representative of their legal entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub representative_declaration:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityAttestationsRepresentativeDeclaration>,
    #[doc = "Attestations of accepted terms of service agreements."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<stripe_types_09::v2_core::UpdateCoreAccountParamsIdentityAttestationsTermsOfService>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityIndividualDocuments {
    #[doc = "One or more documents that demonstrate proof that this person is authorized to represent the company."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_authorization:
        Option<stripe_types_09::v2_core::UpdateCoreAccountParamsIdentityIndividualDocumentsCompanyAuthorization>,
    #[doc = "One or more documents showing the person's passport page with photo and personal data."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub passport: Option<stripe_types_09::v2_core::UpdateCoreAccountParamsIdentityIndividualDocumentsPassport>,
    #[doc = "An identifying document showing the person's name, either a passport or local ID card."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub primary_verification:
        Option<stripe_types_08::v2_core::UpdateCoreAccountParamsIdentityIndividualDocumentsPrimaryVerification>,
    #[doc = "A document showing address, either a passport, local ID card, or utility bill from a well-known utility company."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub secondary_verification:
        Option<stripe_types_07::v2_core::UpdateCoreAccountParamsIdentityIndividualDocumentsSecondaryVerification>,
    #[doc = "One or more documents showing the person's visa required for living in the country where they are residing."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub visa: Option<stripe_types_08::v2_core::UpdateCoreAccountParamsIdentityIndividualDocumentsVisa>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountFutureRequirementsEntriesItemImpact {
    #[doc = "The Capabilities that will be restricted if the requirement is not collected and satisfactory to Stripe."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub restricts_capabilities: Option<
        Vec<stripe_types_09::v2_core::V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItem>,
    >,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityIndividual {
    #[doc = "The account ID which the individual belongs to."]
    #[json(default)]
    pub account: String,
    #[doc = "Additional addresses associated with the individual."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_addresses:
        Option<Vec<stripe_types_05::v2_core::V2CoreAccountIdentityIndividualAdditionalAddressesItem>>,
    #[doc = "Additional names (e.g."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_names: Option<Vec<stripe_types_05::v2_core::V2CoreAccountIdentityIndividualAdditionalNamesItem>>,
    #[doc = "Terms of service acceptances."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_terms_of_service: Option<stripe_types_05::v2_core::V2CoreAccountIdentityAttestationsTermsOfService>,
    #[doc = "The individual's residential address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::v2_core::V2CoreAccountConfigurationMerchantSupportAddress>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: String,
    #[doc = "The individual's date of birth."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<stripe_types_04::v2_core::V2CoreAccountIdentityBusinessDetailsRegistrationDate>,
    #[doc = "Documents that may be submitted to satisfy various informational requests."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub documents: Option<stripe_types_09::v2_core::V2CoreAccountIdentityIndividualDocuments>,
    #[doc = "The individual's email address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The individual's first name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The identification numbers (e.g., SSN) associated with the individual."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id_numbers: Option<Vec<stripe_types_05::v2_core::V2CoreAccountIdentityIndividualIdNumbersItem>>,
    #[doc = "The individual's gender (International regulations require either 'male' or 'female')."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub legal_gender: Option<stripe_types_01::v2_core::V2CoreAccountIdentityIndividualLegalGender>,
    #[doc = "Set of key-value pairs that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The countries where the individual is a national."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nationalities: Option<Vec<String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: String,
    #[doc = "The individual's phone number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "Indicates if the individual or any of their representatives, family members, or other closely related persons, declar..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<stripe_types_00::v2_core::V2CoreAccountIdentityIndividualPoliticalExposure>,
    #[doc = "The relationship that this individual has with the Account's identity."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<stripe_types_02::v2_core::V2CoreAccountIdentityIndividualRelationship>,
    #[doc = "The script addresses (e.g., non-Latin characters) associated with the individual."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub script_addresses: Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsScriptAddresses>,
    #[doc = "The script names (e.g."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub script_names: Option<stripe_types_05::v2_core::V2CoreAccountIdentityIndividualScriptNames>,
    #[doc = "The individual's last name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[doc = "Time at which the object was last updated."]
    #[json(default)]
    pub updated: String,
}

#[cfg(feature = "v2_core")]
impl crate::PagedItem for V2CoreAccountIdentityIndividual {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountPerson {
    #[doc = "The account ID which the individual belongs to."]
    #[json(default)]
    pub account: String,
    #[doc = "Additional addresses associated with the person."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_addresses:
        Option<Vec<stripe_types_05::v2_core::V2CoreAccountIdentityIndividualAdditionalAddressesItem>>,
    #[doc = "Additional names (e.g."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_names: Option<Vec<stripe_types_05::v2_core::V2CoreAccountIdentityIndividualAdditionalNamesItem>>,
    #[doc = "Attestations of accepted terms of service agreements."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_terms_of_service: Option<stripe_types_05::v2_core::V2CoreAccountIdentityAttestationsTermsOfService>,
    #[doc = "The person's residential address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::v2_core::V2CoreAccountConfigurationMerchantSupportAddress>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: String,
    #[doc = "The person's date of birth."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<stripe_types_04::v2_core::V2CoreAccountIdentityBusinessDetailsRegistrationDate>,
    #[doc = "Documents that may be submitted to satisfy various informational requests."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub documents: Option<stripe_types_09::v2_core::V2CoreAccountIdentityIndividualDocuments>,
    #[doc = "The person's email address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The person's first name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "Unique identifier for the Person."]
    #[json(default)]
    pub id: String,
    #[doc = "The identification numbers (e.g., SSN) associated with the person."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id_numbers: Option<Vec<stripe_types_05::v2_core::V2CoreAccountIdentityIndividualIdNumbersItem>>,
    #[doc = "The person's gender (International regulations require either 'male' or 'female')."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub legal_gender: Option<stripe_types_01::v2_core::V2CoreAccountIdentityIndividualLegalGender>,
    #[doc = "Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of key-value pairs that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The countries where the person is a national."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nationalities: Option<Vec<String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::v2_core::V2CoreAccountPersonObject,
    #[doc = "The person's phone number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The person's political exposure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<stripe_types_00::v2_core::V2CoreAccountIdentityIndividualPoliticalExposure>,
    #[doc = "The relationship that this person has with the Account's business or legal entity."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<stripe_types_02::v2_core::V2CoreAccountIdentityIndividualRelationship>,
    #[doc = "The script addresses (e.g., non-Latin characters) associated with the person."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub script_addresses: Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsScriptAddresses>,
    #[doc = "The script names (e.g."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub script_names: Option<stripe_types_05::v2_core::V2CoreAccountIdentityIndividualScriptNames>,
    #[doc = "The person's last name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[doc = "Time at which the object was last updated."]
    #[json(default)]
    pub updated: String,
}

#[cfg(feature = "v2_core")]
impl crate::PagedItem for V2CoreAccountPerson {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreEventDestination {
    #[doc = "Configuration for delivering events through an Amazon EventBridge partner event source."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amazon_eventbridge: Option<stripe_types_05::v2_core::V2CoreEventDestinationAmazonEventbridge>,
    #[doc = "Configuration for delivering events through an Azure Event Grid partner topic."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub azure_event_grid: Option<stripe_types_06::v2_core::V2CoreEventDestinationAzureEventGrid>,
    #[doc = "The time when the destination was created."]
    #[json(default)]
    pub created: String,
    #[doc = "An optional user-defined description of the destination's purpose."]
    #[json(default)]
    pub description: String,
    #[doc = "The list of event types enabled for delivery to this destination."]
    #[json(default)]
    pub enabled_events: Vec<String>,
    #[doc = "Whether to deliver as snapshot or thin events."]
    #[json(default)]
    pub event_payload: stripe_types_00::v2_core::V2CoreEventDestinationEventPayload,
    #[doc = "Specifies which accounts' events route to this destination."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub events_from: Option<Vec<String>>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "User-defined key/value data for the destination; it has no effect on event matching or delivery."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "A user-defined label for identifying the destination in Stripe."]
    #[json(default)]
    pub name: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::v2_core::V2CoreEventDestinationObject,
    #[doc = "For snapshot events only, the Stripe API version used to render event objects."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_api_version: Option<String>,
    #[doc = "Whether Stripe currently attempts delivery."]
    #[json(default)]
    pub status: stripe_types_01::v2_core::V2CoreEventDestinationStatus,
    #[doc = "Additional lifecycle context for the destination status, when available."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<stripe_types_09::v2_core::V2CoreEventDestinationStatusDetails>,
    #[doc = "The delivery transport."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::v2_core::V2CoreEventDestinationType,
    #[doc = "The time when the destination object was last updated."]
    #[json(default)]
    pub updated: String,
    #[doc = "Configuration for delivering events to a webhook endpoint."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub webhook_endpoint: Option<stripe_types_00::v2_core::V2CoreEventDestinationWebhookEndpoint>,
}

#[cfg(feature = "v2_core")]
impl crate::PagedItem for V2CoreEventDestination {
    fn id(&self) -> &str {
        &self.id
    }
}
