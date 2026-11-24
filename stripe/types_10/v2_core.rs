#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountLinkParamsUseCaseAccountUpdate {
    #[doc = "Specifies the requirements that Stripe collects from v2/core/accounts in the Onboarding flow."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub collection_options:
        Option<stripe_types_07::v2_core::CreateCoreAccountLinkParamsUseCaseAccountUpdateCollectionOptions>,
    #[doc = "The URL the user will be redirected to if the Account Link is expired, has been used, or is otherwise invalid."]
    pub refresh_url: String,
    #[doc = "The URL that the user will be redirected to upon completing the linked flow."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationCustomerBilling {
    #[doc = "Default invoice settings for the customer account."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<stripe_types_07::v2_core::CreateCoreAccountParamsConfigurationCustomerBillingInvoice>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationRecipientCapabilities {
    #[doc = "Capabilities that enable the recipient to manage their Stripe Balance (/v1/balance)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub stripe_balance:
        Option<stripe_types_08::v2_core::CreateCoreAccountParamsConfigurationRecipientCapabilitiesStripeBalance>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityAttestations {
    #[doc = "This hash is used to attest that the directors information provided to Stripe is both current and correct."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub directorship_declaration:
        Option<stripe_types_00::v2_core::CreateCoreAccountParamsIdentityAttestationsDirectorshipDeclaration>,
    #[doc = "This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsIdentityAttestationsOwnershipDeclaration>,
    #[doc = "Attestation that all Persons with a specific Relationship value have been provided."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub persons_provided: Option<stripe_types_07::v2_core::CreateCoreAccountParamsIdentityAttestationsPersonsProvided>,
    #[doc = "This hash is used to attest that the representative is authorized to act as the representative of their legal entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub representative_declaration:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsIdentityAttestationsRepresentativeDeclaration>,
    #[doc = "Attestations of accepted terms of service agreements."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<stripe_types_06::v2_core::CreateCoreAccountParamsIdentityAttestationsTermsOfService>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityIndividualDocuments {
    #[doc = "One or more documents that demonstrate proof that this person is authorized to represent the company."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_authorization:
        Option<stripe_types_08::v2_core::CreateCoreAccountParamsIdentityIndividualDocumentsCompanyAuthorization>,
    #[doc = "One or more documents showing the person's passport page with photo and personal data."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub passport: Option<stripe_types_08::v2_core::CreateCoreAccountParamsIdentityIndividualDocumentsPassport>,
    #[doc = "An identifying document showing the person's name, either a passport or local ID card."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub primary_verification:
        Option<stripe_types_07::v2_core::CreateCoreAccountParamsIdentityIndividualDocumentsPrimaryVerification>,
    #[doc = "A document showing address, either a passport, local ID card, or utility bill from a well-known utility company."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub secondary_verification:
        Option<stripe_types_09::v2_core::CreateCoreAccountParamsIdentityIndividualDocumentsSecondaryVerification>,
    #[doc = "One or more documents showing the person's visa required for living in the country where they are residing."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub visa: Option<stripe_types_07::v2_core::CreateCoreAccountParamsIdentityIndividualDocumentsVisa>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsDocuments {
    #[doc = "One or more documents that support the bank account ownership verification requirement."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification: Option<stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerification>,
    #[doc = "One or more documents that demonstrate proof of a company's license to operate."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_license: Option<stripe_types_08::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyLicense>,
    #[doc = "One or more documents showing the company's Memorandum of Association."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_memorandum_of_association: Option<stripe_types_09::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyMemorandumOfAssociation>,
    #[doc = "Certain countries only: One or more documents showing the ministerial decree legalizing the company's establishment."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_ministerial_decree: Option<stripe_types_09::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyMinisterialDecree>,
    #[doc = "One or more documents that demonstrate proof of a company's registration with the appropriate local authorities."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_registration_verification: Option<stripe_types_08::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyRegistrationVerification>,
    #[doc = "One or more documents that demonstrate proof of a company's tax ID."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_tax_id_verification: Option<stripe_types_08::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyTaxIdVerification>,
    #[doc = "A document verifying the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub primary_verification: Option<stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerification>,
    #[doc = "One or more documents that demonstrate proof of address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub proof_of_address: Option<stripe_types_07::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfAddress>,
    #[doc = "One or more documents that demonstrate proof of ultimate beneficial ownership."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub proof_of_registration: Option<stripe_types_07::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfRegistration>,
    #[doc = "One or more documents that demonstrate proof of ultimate beneficial ownership."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub proof_of_ultimate_beneficial_ownership: Option<stripe_types_08::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnership>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsConfigurationCustomerBillingInvoice {
    #[doc = "The list of up to 4 default custom fields to be displayed on invoices for this customer."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<
        Vec<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationCustomerBillingInvoiceCustomFieldsItem>,
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
        Option<stripe_types_07::v2_core::UpdateCoreAccountParamsConfigurationCustomerBillingInvoiceRendering>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsDefaults {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The Account's preferred locales (languages), ordered by preference."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub locales: Option<Vec<stripe_types_06::v2_core::UpdateCoreAccountParamsDefaultsLocalesItem>>,
    #[doc = "Account profile information."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub profile: Option<stripe_types_06::v2_core::UpdateCoreAccountParamsDefaultsProfile>,
    #[doc = "Default responsibilities held by either Stripe or the platform."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub responsibilities: Option<stripe_types_08::v2_core::UpdateCoreAccountParamsDefaultsResponsibilities>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityBusinessDetailsDocuments {
    #[doc = "One or more documents that support the bank account ownership verification requirement."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification: Option<stripe_types_07::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerification>,
    #[doc = "One or more documents that demonstrate proof of a company's license to operate."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_license: Option<stripe_types_07::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyLicense>,
    #[doc = "One or more documents showing the company's Memorandum of Association."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_memorandum_of_association: Option<stripe_types_07::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyMemorandumOfAssociation>,
    #[doc = "Certain countries only: One or more documents showing the ministerial decree legalizing the company's establishment."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_ministerial_decree: Option<stripe_types_09::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyMinisterialDecree>,
    #[doc = "One or more documents that demonstrate proof of a company's registration with the appropriate local authorities."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_registration_verification: Option<stripe_types_07::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyRegistrationVerification>,
    #[doc = "One or more documents that demonstrate proof of a company's tax ID."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_tax_id_verification: Option<stripe_types_07::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyTaxIdVerification>,
    #[doc = "A document verifying the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub primary_verification: Option<stripe_types_08::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsPrimaryVerification>,
    #[doc = "One or more documents that demonstrate proof of address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub proof_of_address: Option<stripe_types_09::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfAddress>,
    #[doc = "One or more documents that demonstrate proof of ultimate beneficial ownership."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub proof_of_registration: Option<stripe_types_09::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfRegistration>,
    #[doc = "One or more documents that demonstrate proof of ultimate beneficial ownership."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub proof_of_ultimate_beneficial_ownership: Option<stripe_types_09::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnership>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountPersonParamsDocuments {
    #[doc = "One or more documents that demonstrate proof that this person is authorized to represent the company."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub company_authorization:
        Option<stripe_types_07::v2_core::UpdateCoreAccountPersonParamsDocumentsCompanyAuthorization>,
    #[doc = "One or more documents showing the person's passport page with photo and personal data."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub passport: Option<stripe_types_09::v2_core::UpdateCoreAccountPersonParamsDocumentsPassport>,
    #[doc = "An identifying document showing the person's name, either a passport or local ID card."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub primary_verification:
        Option<stripe_types_08::v2_core::UpdateCoreAccountPersonParamsDocumentsPrimaryVerification>,
    #[doc = "A document showing address, either a passport, local ID card, or utility bill from a well-known utility company."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub secondary_verification:
        Option<stripe_types_09::v2_core::UpdateCoreAccountPersonParamsDocumentsSecondaryVerification>,
    #[doc = "One or more documents showing the person's visa required for living in the country where they are residing."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub visa: Option<stripe_types_08::v2_core::UpdateCoreAccountPersonParamsDocumentsVisa>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationCustomerBilling {
    #[doc = "The ID of a `PaymentMethod` attached to this Account's `customer` configuration, used as the default payment method f..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,
    #[doc = "Default invoice settings for the customer account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerBillingInvoice>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationCustomerCapabilities {
    #[doc = "Generates requirements for enabling automatic indirect tax calculation on this customer's invoices or subscriptions."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub automatic_indirect_tax:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationMerchantCapabilitiesStripeBalance {
    #[doc = "Enables this Account to complete payouts from their Stripe Balance (/v1/balance)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payouts: Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationRecipientCapabilitiesStripeBalance {
    #[doc = "Enables this Account to complete payouts from their Stripe Balance (/v1/balance)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payouts: Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Enables this Account to receive /v1/transfers into their Stripe Balance (/v1/balance)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub stripe_transfers:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityBusinessDetails {
    #[doc = "Additional addresses associated with the business."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_addresses:
        Option<Vec<stripe_types_05::v2_core::V2CoreAccountIdentityBusinessDetailsAdditionalAddressesItem>>,
    #[doc = "The company's primary address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::v2_core::V2CoreAccountConfigurationMerchantSupportAddress>,
    #[doc = "The business gross annual revenue for its preceding fiscal year."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub annual_revenue: Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsAnnualRevenue>,
    #[doc = "Documents that may be submitted to satisfy various informational requests."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub documents: Option<stripe_types_08::v2_core::V2CoreAccountIdentityBusinessDetailsDocuments>,
    #[doc = "Estimated maximum number of workers currently engaged by the business (including employees, contractors, and vendors)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub estimated_worker_count: Option<i64>,
    #[doc = "The provided ID numbers of a business entity."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id_numbers: Option<Vec<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsIdNumbersItem>>,
    #[doc = "An estimate of the monthly revenue of the business."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub monthly_estimated_revenue:
        Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsMonthlyEstimatedRevenue>,
    #[doc = "The company's phone number (used for verification)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The business legal name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registered_name: Option<String>,
    #[doc = "When the business was incorporated or registered."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<stripe_types_04::v2_core::V2CoreAccountIdentityBusinessDetailsRegistrationDate>,
    #[doc = "The business registration address of the business entity in non latin script."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub script_addresses: Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsScriptAddresses>,
    #[doc = "The business legal name in non latin script."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub script_names: Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsScriptNames>,
    #[doc = "The category identifying the legal structure of the business."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub structure: Option<stripe_types_02::v2_core::V2CoreAccountIdentityBusinessDetailsStructure>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountLinkUseCase {
    #[doc = "Hash containing configuration options for an Account Link object that onboards a new account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_onboarding: Option<stripe_types_08::v2_core::V2CoreAccountLinkUseCaseAccountOnboarding>,
    #[doc = "Hash containing configuration options for an Account Link that updates an existing account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_update: Option<stripe_types_08::v2_core::V2CoreAccountLinkUseCaseAccountOnboarding>,
    #[doc = "Open Enum."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::v2_core::V2CoreAccountLinkUseCaseType,
}
