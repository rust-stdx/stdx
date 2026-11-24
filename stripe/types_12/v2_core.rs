#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountLinkParamsUseCase {
    #[doc = "Hash containing configuration options for an Account Link object that onboards a new account."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub account_onboarding: Option<stripe_types_09::v2_core::CreateCoreAccountLinkParamsUseCaseAccountOnboarding>,
    #[doc = "Hash containing configuration options for an Account Link that updates an existing account."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub account_update: Option<stripe_types_10::v2_core::CreateCoreAccountLinkParamsUseCaseAccountUpdate>,
    #[doc = "Open Enum."]
    #[json(rename = "type")]
    pub r#type: stripe_types_03::v2_core::CreateCoreAccountLinkParamsUseCaseType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationCustomer {
    #[doc = "Automatic indirect tax settings to be used when automatic tax calculation is enabled on the customer's invoices, subs..."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub automatic_indirect_tax:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationCustomerAutomaticIndirectTax>,
    #[doc = "Billing settings - default settings used for this customer in Billing flows such as Invoices and Subscriptions."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub billing: Option<stripe_types_10::v2_core::CreateCoreAccountParamsConfigurationCustomerBilling>,
    #[doc = "Capabilities that have been requested on the Customer Configuration."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationCustomerCapabilities>,
    #[doc = "The customer's shipping information."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationCustomerShipping>,
    #[doc = "ID of the test clock to attach to the customer."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationRecipient {
    #[doc = "Capabilities to be requested on the Recipient Configuration."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<stripe_types_10::v2_core::CreateCoreAccountParamsConfigurationRecipientCapabilities>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetails {
    #[doc = "Additional addresses associated with the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_addresses:
        Option<Vec<stripe_types_09::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsAdditionalAddressesItem>>,
    #[doc = "The business registration address of the business entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_05::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsAddress>,
    #[doc = "The business gross annual revenue for its preceding fiscal year."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub annual_revenue: Option<stripe_types_09::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsAnnualRevenue>,
    #[doc = "A document verifying the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub documents: Option<stripe_types_11::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocuments>,
    #[doc = "Estimated maximum number of workers currently engaged by the business (including employees, contractors, and vendors)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub estimated_worker_count: Option<i64>,
    #[doc = "The ID numbers of a business entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub id_numbers: Option<Vec<stripe_types_08::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsIdNumbersItem>>,
    #[doc = "An estimate of the monthly revenue of the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub monthly_estimated_revenue:
        Option<stripe_types_08::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsMonthlyEstimatedRevenue>,
    #[doc = "The phone number of the Business Entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The business legal name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub registered_name: Option<String>,
    #[doc = "When the business was incorporated or registered."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub registration_date:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsRegistrationDate>,
    #[doc = "The business registration address of the business entity in non latin script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_addresses:
        Option<stripe_types_07::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsScriptAddresses>,
    #[doc = "The business legal name in non latin script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_names: Option<stripe_types_09::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsScriptNames>,
    #[doc = "The category identifying the legal structure of the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub structure: Option<stripe_types_06::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsStructure>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityIndividual {
    #[doc = "Additional addresses associated with the individual."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_addresses:
        Option<Vec<stripe_types_08::v2_core::CreateCoreAccountParamsIdentityIndividualAdditionalAddressesItem>>,
    #[doc = "Additional names (e.g."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_names:
        Option<Vec<stripe_types_07::v2_core::CreateCoreAccountParamsIdentityIndividualAdditionalNamesItem>>,
    #[doc = "The individual's residential address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_05::v2_core::CreateCoreAccountParamsIdentityIndividualAddress>,
    #[doc = "The individual's date of birth."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<stripe_types_06::v2_core::CreateCoreAccountParamsIdentityIndividualDateOfBirth>,
    #[doc = "Documents that may be submitted to satisfy various informational requests."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub documents: Option<stripe_types_10::v2_core::CreateCoreAccountParamsIdentityIndividualDocuments>,
    #[doc = "The individual's email address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The individual's first name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The identification numbers (e.g., SSN) associated with the individual."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub id_numbers: Option<Vec<stripe_types_07::v2_core::CreateCoreAccountParamsIdentityIndividualIdNumbersItem>>,
    #[doc = "The individual's gender (International regulations require either 'male' or 'female')."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub legal_gender: Option<stripe_types_05::v2_core::CreateCoreAccountParamsIdentityIndividualLegalGender>,
    #[doc = "Set of key-value pairs that you can attach to an object."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The countries where the individual is a national."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub nationalities: Option<Vec<String>>,
    #[doc = "The individual's phone number."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The individual's political exposure."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub political_exposure:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsIdentityIndividualPoliticalExposure>,
    #[doc = "The relationship that this individual has with the account's identity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<stripe_types_06::v2_core::CreateCoreAccountParamsIdentityIndividualRelationship>,
    #[doc = "The script addresses (e.g., non-Latin characters) associated with the individual."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_addresses: Option<stripe_types_08::v2_core::CreateCoreAccountParamsIdentityIndividualScriptAddresses>,
    #[doc = "The individuals primary name in non latin script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_names: Option<stripe_types_07::v2_core::CreateCoreAccountParamsIdentityIndividualScriptNames>,
    #[doc = "The individual's last name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonParams {
    #[doc = "Additional addresses associated with the person."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_addresses:
        Option<Vec<stripe_types_07::v2_core::CreateCoreAccountPersonParamsAdditionalAddressesItem>>,
    #[doc = "Additional names (e.g."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_names: Option<Vec<stripe_types_09::v2_core::CreateCoreAccountPersonParamsAdditionalNamesItem>>,
    #[doc = "Attestations of accepted terms of service agreements."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_terms_of_service:
        Option<stripe_types_08::v2_core::CreateCoreAccountPersonParamsAdditionalTermsOfService>,
    #[doc = "The person's residential address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_06::v2_core::CreateCoreAccountPersonParamsAddress>,
    #[doc = "The person's date of birth."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<stripe_types_06::v2_core::CreateCoreAccountPersonParamsDateOfBirth>,
    #[doc = "Documents that may be submitted to satisfy various informational requests."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub documents: Option<stripe_types_11::v2_core::CreateCoreAccountPersonParamsDocuments>,
    #[doc = "Email."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The person's first name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The identification numbers (e.g., SSN) associated with the person."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub id_numbers: Option<Vec<stripe_types_07::v2_core::CreateCoreAccountPersonParamsIdNumbersItem>>,
    #[doc = "The person's gender (International regulations require either 'male' or 'female')."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub legal_gender: Option<stripe_types_05::v2_core::CreateCoreAccountPersonParamsLegalGender>,
    #[doc = "Set of key-value pairs that you can attach to an object."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The nationalities (countries) this person is associated with."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub nationalities: Option<Vec<String>>,
    #[doc = "The person token generated by the person token api."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub person_token: Option<String>,
    #[doc = "The phone number for this person."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The person's political exposure."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<stripe_types_05::v2_core::CreateCoreAccountPersonParamsPoliticalExposure>,
    #[doc = "The relationship that this person has with the Account's business or legal entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<stripe_types_05::v2_core::CreateCoreAccountPersonParamsRelationship>,
    #[doc = "The script addresses (e.g., non-Latin characters) associated with the person."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_addresses: Option<stripe_types_07::v2_core::CreateCoreAccountPersonParamsScriptAddresses>,
    #[doc = "The script names (e.g."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_names: Option<stripe_types_07::v2_core::CreateCoreAccountPersonParamsScriptNames>,
    #[doc = "The person's last name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonTokenParams {
    #[doc = "Additional addresses associated with the person."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_addresses:
        Option<Vec<stripe_types_09::v2_core::CreateCoreAccountPersonTokenParamsAdditionalAddressesItem>>,
    #[doc = "Additional names (e.g."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_names: Option<Vec<stripe_types_09::v2_core::CreateCoreAccountPersonTokenParamsAdditionalNamesItem>>,
    #[doc = "Attestations of accepted terms of service agreements."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_terms_of_service:
        Option<stripe_types_08::v2_core::CreateCoreAccountPersonTokenParamsAdditionalTermsOfService>,
    #[doc = "The person's residential address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_06::v2_core::CreateCoreAccountPersonTokenParamsAddress>,
    #[doc = "The person's date of birth."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<stripe_types_06::v2_core::CreateCoreAccountPersonTokenParamsDateOfBirth>,
    #[doc = "Documents that may be submitted to satisfy various informational requests."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub documents: Option<stripe_types_11::v2_core::CreateCoreAccountPersonTokenParamsDocuments>,
    #[doc = "Email."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The person's first name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The identification numbers (e.g., SSN) associated with the person."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub id_numbers: Option<Vec<stripe_types_09::v2_core::CreateCoreAccountPersonTokenParamsIdNumbersItem>>,
    #[doc = "The person's gender (International regulations require either 'male' or 'female')."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub legal_gender: Option<stripe_types_05::v2_core::CreateCoreAccountPersonTokenParamsLegalGender>,
    #[doc = "Set of key-value pairs that you can attach to an object."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The nationalities (countries) this person is associated with."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub nationalities: Option<Vec<String>>,
    #[doc = "The phone number for this person."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The person's political exposure."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<stripe_types_05::v2_core::CreateCoreAccountPersonTokenParamsPoliticalExposure>,
    #[doc = "The relationship that this person has with the Account's business or legal entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<stripe_types_00::v2_core::CreateCoreAccountPersonTokenParamsRelationship>,
    #[doc = "The script addresses (e.g., non-Latin characters) associated with the person."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_addresses: Option<stripe_types_08::v2_core::CreateCoreAccountPersonTokenParamsScriptAddresses>,
    #[doc = "The script names (e.g."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_names: Option<stripe_types_07::v2_core::CreateCoreAccountPersonTokenParamsScriptNames>,
    #[doc = "The person's last name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetails {
    #[doc = "Additional addresses associated with the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_addresses: Option<
        Vec<stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAdditionalAddressesItem>,
    >,
    #[doc = "The business registration address of the business entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress>,
    #[doc = "The business gross annual revenue for its preceding fiscal year."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub annual_revenue:
        Option<stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsAnnualRevenue>,
    #[doc = "A document verifying the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub documents: Option<stripe_types_10::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocuments>,
    #[doc = "Estimated maximum number of workers currently engaged by the business (including employees, contractors, and vendors)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub estimated_worker_count: Option<i64>,
    #[doc = "The ID numbers of a business entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub id_numbers:
        Option<Vec<stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItem>>,
    #[doc = "An estimate of the monthly revenue of the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub monthly_estimated_revenue:
        Option<stripe_types_09::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsMonthlyEstimatedRevenue>,
    #[doc = "The phone number of the Business Entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The business legal name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub registered_name: Option<String>,
    #[doc = "When the business was incorporated or registered."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub registration_date:
        Option<stripe_types_01::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsRegistrationDate>,
    #[doc = "The business registration address of the business entity in non latin script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_addresses:
        Option<stripe_types_07::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsScriptAddresses>,
    #[doc = "The business legal name in non latin script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_names: Option<stripe_types_08::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsScriptNames>,
    #[doc = "The category identifying the legal structure of the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub structure: Option<stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityIndividual {
    #[doc = "Additional addresses associated with the individual."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_addresses:
        Option<Vec<stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityIndividualAdditionalAddressesItem>>,
    #[doc = "Additional names (e.g."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_names:
        Option<Vec<stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItem>>,
    #[doc = "The individual's residential address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityIndividualAddress>,
    #[doc = "The individual's date of birth."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityIndividualDateOfBirth>,
    #[doc = "Documents that may be submitted to satisfy various informational requests."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub documents: Option<stripe_types_11::v2_core::CreateCoreAccountTokenParamsIdentityIndividualDocuments>,
    #[doc = "The individual's email address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The individual's first name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The identification numbers (e.g., SSN) associated with the individual."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub id_numbers: Option<Vec<stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItem>>,
    #[doc = "The individual's gender (International regulations require either 'male' or 'female')."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub legal_gender: Option<stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualLegalGender>,
    #[doc = "Set of key-value pairs that you can attach to an object."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The countries where the individual is a national."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub nationalities: Option<Vec<String>>,
    #[doc = "The individual's phone number."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The individual's political exposure."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub political_exposure:
        Option<stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualPoliticalExposure>,
    #[doc = "The relationship that this individual has with the account's identity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<stripe_types_03::v2_core::CreateCoreAccountTokenParamsIdentityIndividualRelationship>,
    #[doc = "The script addresses (e.g., non-Latin characters) associated with the individual."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_addresses:
        Option<stripe_types_09::v2_core::CreateCoreAccountTokenParamsIdentityIndividualScriptAddresses>,
    #[doc = "The individuals primary name in non latin script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_names: Option<stripe_types_09::v2_core::CreateCoreAccountTokenParamsIdentityIndividualScriptNames>,
    #[doc = "The individual's last name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsConfigurationCustomerBilling {
    #[doc = "The ID of a `PaymentMethod` attached to this Account's `customer` configuration, used as the default payment method f..."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,
    #[doc = "Default invoice settings for the customer account."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<stripe_types_10::v2_core::UpdateCoreAccountParamsConfigurationCustomerBillingInvoice>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsConfigurationMerchant {
    #[doc = "Represents the state of the configuration, and can be updated to deactivate or re-apply a configuration."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub applied: Option<bool>,
    #[doc = "Settings for Bacs Direct Debit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantBacsDebitPayments>,
    #[doc = "Settings used to apply the merchant's branding to email receipts, invoices, Checkout, and other products."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub branding: Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantBranding>,
    #[doc = "Capabilities to request on the Merchant Configuration."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<stripe_types_09::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilities>,
    #[doc = "Card payments settings."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<stripe_types_09::v2_core::UpdateCoreAccountParamsConfigurationMerchantCardPayments>,
    #[doc = "Settings specific to Konbini payments on the account."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub konbini_payments: Option<stripe_types_11::v2_core::UpdateCoreAccountParamsConfigurationMerchantKonbiniPayments>,
    #[doc = "The Merchant Category Code (MCC) for the merchant."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    #[doc = "Settings for the default text that appears on statements for language variations."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_statement_descriptor:
        Option<stripe_types_09::v2_core::UpdateCoreAccountParamsConfigurationMerchantScriptStatementDescriptor>,
    #[doc = "Settings for SEPA Direct Debit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantSepaDebitPayments>,
    #[doc = "Settings for the default [statement descriptor](/connect/statement-descriptors) text."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantStatementDescriptor>,
    #[doc = "Publicly available contact information for sending support issues to."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub support: Option<stripe_types_07::v2_core::UpdateCoreAccountParamsConfigurationMerchantSupport>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsConfigurationRecipient {
    #[doc = "Represents the state of the configuration, and can be updated to deactivate or re-apply a configuration."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub applied: Option<bool>,
    #[doc = "Capabilities to request on the Recipient Configuration."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<stripe_types_11::v2_core::UpdateCoreAccountParamsConfigurationRecipientCapabilities>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityBusinessDetails {
    #[doc = "Additional addresses associated with the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_addresses:
        Option<Vec<stripe_types_08::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsAdditionalAddressesItem>>,
    #[doc = "The business registration address of the business entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsAddress>,
    #[doc = "The business gross annual revenue for its preceding fiscal year."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub annual_revenue: Option<stripe_types_08::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsAnnualRevenue>,
    #[doc = "A document verifying the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub documents: Option<stripe_types_10::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocuments>,
    #[doc = "Estimated maximum number of workers currently engaged by the business (including employees, contractors, and vendors)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub estimated_worker_count: Option<i64>,
    #[doc = "The ID numbers of a business entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub id_numbers: Option<Vec<stripe_types_09::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsIdNumbersItem>>,
    #[doc = "An estimate of the monthly revenue of the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub monthly_estimated_revenue:
        Option<stripe_types_07::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsMonthlyEstimatedRevenue>,
    #[doc = "The phone number of the Business Entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The business legal name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub registered_name: Option<String>,
    #[doc = "When the business was incorporated or registered."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub registration_date:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsRegistrationDate>,
    #[doc = "The business registration address of the business entity in non latin script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_addresses:
        Option<stripe_types_08::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsScriptAddresses>,
    #[doc = "The business legal name in non latin script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_names: Option<stripe_types_08::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsScriptNames>,
    #[doc = "The category identifying the legal structure of the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub structure: Option<stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsStructure>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityIndividual {
    #[doc = "Additional addresses associated with the individual."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_addresses:
        Option<Vec<stripe_types_07::v2_core::UpdateCoreAccountParamsIdentityIndividualAdditionalAddressesItem>>,
    #[doc = "Additional names (e.g."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_names:
        Option<Vec<stripe_types_08::v2_core::UpdateCoreAccountParamsIdentityIndividualAdditionalNamesItem>>,
    #[doc = "The individual's residential address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityIndividualAddress>,
    #[doc = "The individual's date of birth."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityIndividualDateOfBirth>,
    #[doc = "Documents that may be submitted to satisfy various informational requests."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub documents: Option<stripe_types_11::v2_core::UpdateCoreAccountParamsIdentityIndividualDocuments>,
    #[doc = "The individual's email address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The individual's first name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The identification numbers (e.g., SSN) associated with the individual."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub id_numbers: Option<Vec<stripe_types_08::v2_core::UpdateCoreAccountParamsIdentityIndividualIdNumbersItem>>,
    #[doc = "The individual's gender (International regulations require either 'male' or 'female')."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub legal_gender: Option<stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityIndividualLegalGender>,
    #[doc = "Set of key-value pairs that you can attach to an object."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The countries where the individual is a national."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub nationalities: Option<Vec<String>>,
    #[doc = "The individual's phone number."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The individual's political exposure."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub political_exposure:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityIndividualPoliticalExposure>,
    #[doc = "The relationship that this individual has with the account's identity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityIndividualRelationship>,
    #[doc = "The script addresses (e.g., non-Latin characters) associated with the individual."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_addresses: Option<stripe_types_08::v2_core::UpdateCoreAccountParamsIdentityIndividualScriptAddresses>,
    #[doc = "The individuals primary name in non latin script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_names: Option<stripe_types_09::v2_core::UpdateCoreAccountParamsIdentityIndividualScriptNames>,
    #[doc = "The individual's last name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountPersonParams {
    #[doc = "Additional addresses associated with the person."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_addresses:
        Option<Vec<stripe_types_07::v2_core::UpdateCoreAccountPersonParamsAdditionalAddressesItem>>,
    #[doc = "Additional names (e.g."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_names: Option<Vec<stripe_types_08::v2_core::UpdateCoreAccountPersonParamsAdditionalNamesItem>>,
    #[doc = "Attestations of accepted terms of service agreements."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub additional_terms_of_service:
        Option<stripe_types_07::v2_core::UpdateCoreAccountPersonParamsAdditionalTermsOfService>,
    #[doc = "The primary address associated with the person."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_05::v2_core::UpdateCoreAccountPersonParamsAddress>,
    #[doc = "The person's date of birth."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<stripe_types_06::v2_core::UpdateCoreAccountPersonParamsDateOfBirth>,
    #[doc = "Documents that may be submitted to satisfy various informational requests."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub documents: Option<stripe_types_10::v2_core::UpdateCoreAccountPersonParamsDocuments>,
    #[doc = "Email."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The person's first name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The identification numbers (e.g., SSN) associated with the person."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub id_numbers: Option<Vec<stripe_types_09::v2_core::UpdateCoreAccountPersonParamsIdNumbersItem>>,
    #[doc = "The person's gender (International regulations require either 'male' or 'female')."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub legal_gender: Option<stripe_types_06::v2_core::UpdateCoreAccountPersonParamsLegalGender>,
    #[doc = "Set of key-value pairs that you can attach to an object."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The nationalities (countries) this person is associated with."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub nationalities: Option<Vec<String>>,
    #[doc = "The person token generated by the person token api."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub person_token: Option<String>,
    #[doc = "The phone number for this person."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The person's political exposure."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<stripe_types_06::v2_core::UpdateCoreAccountPersonParamsPoliticalExposure>,
    #[doc = "The relationship that this person has with the Account's business or legal entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<stripe_types_05::v2_core::UpdateCoreAccountPersonParamsRelationship>,
    #[doc = "The script addresses (e.g., non-Latin characters) associated with the person."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_addresses: Option<stripe_types_08::v2_core::UpdateCoreAccountPersonParamsScriptAddresses>,
    #[doc = "The script names (e.g."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub script_names: Option<stripe_types_08::v2_core::UpdateCoreAccountPersonParamsScriptNames>,
    #[doc = "The person's last name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationCustomer {
    #[doc = "Indicates whether the customer configuration is active."]
    #[json(default)]
    pub applied: bool,
    #[doc = "Settings for automatic indirect tax calculation on the customer's invoices, subscriptions, Checkout Sessions, and Pay..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub automatic_indirect_tax:
        Option<stripe_types_06::v2_core::V2CoreAccountConfigurationCustomerAutomaticIndirectTax>,
    #[doc = "Default Billing settings for the customer account, used in Invoices and Subscriptions."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing: Option<stripe_types_10::v2_core::V2CoreAccountConfigurationCustomerBilling>,
    #[doc = "Capabilities that have been requested on the Customer Configuration."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<stripe_types_10::v2_core::V2CoreAccountConfigurationCustomerCapabilities>,
    #[doc = "The customer's shipping information."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping: Option<stripe_types_06::v2_core::V2CoreAccountConfigurationCustomerShipping>,
    #[doc = "ID of the test clock to attach to the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationMerchantCapabilities {
    #[doc = "Allow the merchant to process ACH debit payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ach_debit_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process ACSS debit payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Affirm payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub affirm_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Afterpay/Clearpay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Alma payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alma_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Amazon Pay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amazon_pay_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Australian BECS Direct Debit payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process BACS Direct Debit payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Bancontact payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bancontact_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process BLIK payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process recurring BLIK payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik_recurring_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Boleto payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub boleto_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to collect card payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Cartes Bancaires payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Cash App payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cashapp_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process EPS payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eps_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process FPX payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fpx_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process UK bank transfer payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub gb_bank_transfer_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process GrabPay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub grabpay_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process iDEAL payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ideal_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process JCB card payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub jcb_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Japanese bank transfer payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub jp_bank_transfer_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Kakao Pay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kakao_pay_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Klarna payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Konbini convenience store payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub konbini_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Korean card payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kr_card_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Link payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process MobilePay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mobilepay_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Multibanco payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub multibanco_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Mexican bank transfer payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mx_bank_transfer_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Naver Pay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub naver_pay_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process OXXO payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub oxxo_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Przelewy24 (P24) payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub p24_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Pay by Bank payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_by_bank_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process PAYCO payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payco_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process PayNow payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paynow_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process PromptPay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promptpay_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Revolut Pay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub revolut_pay_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Samsung Pay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub samsung_pay_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Satispay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub satispay_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process SEPA bank transfer payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_bank_transfer_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process SEPA Direct Debit payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process SeQura payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sequra_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Capabilities that enable the merchant to manage their Stripe Balance (/v1/balance)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub stripe_balance: Option<stripe_types_10::v2_core::V2CoreAccountConfigurationMerchantCapabilitiesStripeBalance>,
    #[doc = "Allow the merchant to process Sunbit payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sunbit_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Swish payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub swish_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process TWINT payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub twint_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process US bank transfer payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_transfer_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
    #[doc = "Allow the merchant to process Zip payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub zip_payments:
        Option<stripe_types_07::v2_core::V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTax>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationRecipientCapabilities {
    #[doc = "Capabilities that enable the recipient to manage their Stripe Balance (/v1/balance)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub stripe_balance: Option<stripe_types_10::v2_core::V2CoreAccountConfigurationRecipientCapabilitiesStripeBalance>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountFutureRequirementsEntriesItem {
    #[doc = "Indicates whether the platform or Stripe is currently responsible for taking action on the requirement."]
    #[json(default)]
    pub awaiting_action_from: stripe_types_01::v2_core::V2CoreAccountFutureRequirementsEntriesItemAwaitingActionFrom,
    #[doc = "Machine-readable string describing the requirement."]
    #[json(default)]
    pub description: String,
    #[doc = "Descriptions of why the requirement must be collected, or why the collected information isn't satisfactory to Stripe."]
    #[json(default)]
    pub errors: Vec<stripe_types_05::v2_core::V2CoreAccountFutureRequirementsEntriesItemErrorsItem>,
    #[doc = "A hash describing the impact of not collecting the requirement, or Stripe not being able to verify the collected info..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub impact: Option<stripe_types_11::v2_core::V2CoreAccountFutureRequirementsEntriesItemImpact>,
    #[doc = "The soonest point when the account will be impacted by not providing the requirement."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub minimum_deadline: Option<
        stripe_types_06::v2_core::V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadline,
    >,
    #[doc = "A reference to the location of the requirement."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<stripe_types_06::v2_core::V2CoreAccountFutureRequirementsEntriesItemReference>,
    #[doc = "A list of reasons why Stripe is collecting the requirement."]
    #[json(default)]
    pub requested_reasons:
        Vec<stripe_types_05::v2_core::V2CoreAccountFutureRequirementsEntriesItemRequestedReasonsItem>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentity {
    #[doc = "Attestations from the identity's key people, e.g."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub attestations: Option<stripe_types_08::v2_core::V2CoreAccountIdentityAttestations>,
    #[doc = "Information about the company or business."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub business_details: Option<stripe_types_10::v2_core::V2CoreAccountIdentityBusinessDetails>,
    #[doc = "The country in which the account holder resides, or in which the business is legally established."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "The entity type represented by the Account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<stripe_types_02::v2_core::V2CoreAccountIdentityEntityType>,
    #[doc = "Information about the individual represented by the Account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub individual: Option<stripe_types_11::v2_core::V2CoreAccountIdentityIndividual>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountLink {
    #[doc = "The ID of the connected account this Account Link applies to."]
    #[json(default)]
    pub account: String,
    #[doc = "The timestamp at which this Account Link was created."]
    #[json(default)]
    pub created: String,
    #[doc = "The timestamp at which this Account Link will expire."]
    #[json(default)]
    pub expires_at: String,
    #[doc = "Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::v2_core::V2CoreAccountLinkObject,
    #[doc = "The URL at which the account can access the Stripe-hosted flow."]
    #[json(default)]
    pub url: String,
    #[doc = "Describes the Stripe-hosted flow for this Account Link, including its type and flow-specific configuration."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub use_case: Option<stripe_types_10::v2_core::V2CoreAccountLinkUseCase>,
}
