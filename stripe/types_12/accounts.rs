#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParams {
    #[doc = "An [account token](https://docs.stripe.com/api#create_account_token), used to securely provide details to the account."]
    pub account_token: Option<String>,
    #[doc = "Business information about the account."]
    pub business_profile: Option<stripe_types_05::accounts::CreateAccountParamsBusinessProfile>,
    #[doc = "The business type."]
    pub business_type: Option<stripe_types_03::accounts::CreateAccountParamsBusinessType>,
    #[doc = "Each key of the dictionary represents a capability, and each capability maps to its settings (for example, whether it..."]
    pub capabilities: Option<stripe_types_07::accounts::CreateAccountParamsCapabilities>,
    #[doc = "Information about the company or business."]
    pub company: Option<stripe_types_07::accounts::CreateAccountParamsCompany>,
    #[doc = "A hash of configuration describing the account controller's attributes."]
    pub controller: Option<stripe_types_08::accounts::CreateAccountParamsController>,
    #[doc = "The country in which the account holder resides, or in which the business is legally established."]
    pub country: Option<String>,
    #[doc = "Three-letter ISO currency code representing the default currency for the account."]
    pub default_currency: Option<String>,
    #[doc = "Documents that may be submitted to satisfy various informational requests."]
    pub documents: Option<stripe_types_09::accounts::CreateAccountParamsDocuments>,
    #[doc = "The email address of the account holder."]
    pub email: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A card or bank account to attach to the account for receiving [payouts](/connect/bank-debit-card-payouts) (you won’t ..."]
    pub external_account: Option<String>,
    #[doc = "A hash of account group type to tokens."]
    pub groups: Option<stripe_types_04::accounts::CreateAccountParamsGroups>,
    #[doc = "Information about the person represented by the account."]
    pub individual: Option<stripe_types_11::accounts::CreateAccountParamsIndividual>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Options for customizing how the account functions within Stripe."]
    pub settings: Option<stripe_types_10::accounts::CreateAccountParamsSettings>,
    #[doc = "Details on the account's acceptance of the [Stripe Services Agreement](/connect/updating-accounts#tos-acceptance)."]
    pub tos_acceptance: Option<stripe_types_01::accounts::CreateAccountParamsTosAcceptance>,
    #[doc = "The `type` parameter is deprecated."]
    pub r#type: Option<stripe_types_00::accounts::CreateAccountParamsType>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_token", self.account_token.as_ref());
        form.field_opt("business_profile", self.business_profile.as_ref());
        form.field_opt("business_type", self.business_type.as_ref());
        form.field_opt("capabilities", self.capabilities.as_ref());
        form.field_opt("company", self.company.as_ref());
        form.field_opt("controller", self.controller.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("default_currency", self.default_currency.as_ref());
        form.field_opt("documents", self.documents.as_ref());
        form.field_opt("email", self.email.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("external_account", self.external_account.as_ref());
        form.field_opt("groups", self.groups.as_ref());
        form.field_opt("individual", self.individual.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("settings", self.settings.as_ref());
        form.field_opt("tos_acceptance", self.tos_acceptance.as_ref());
        form.field_opt("type", self.r#type.as_ref());
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
pub struct LegalEntityCompany {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::common::Address>,
    #[doc = "The Kana variation of the company's primary address (Japan only)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<stripe_types_04::accounts::LegalEntityJapanAddress>,
    #[doc = "The Kanji variation of the company's primary address (Japan only)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<stripe_types_04::accounts::LegalEntityJapanAddress>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub administrative_address: Option<stripe_types_01::common::Address>,
    #[doc = "Whether the company's directors have been provided."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub directors_provided: Option<bool>,
    #[doc = "This hash is used to attest that the director information provided to Stripe is both current and correct."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub directorship_declaration: Option<stripe_types_04::accounts::LegalEntityDirectorshipDeclaration>,
    #[doc = "Whether the company's executives have been provided."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub executives_provided: Option<bool>,
    #[doc = "The export license ID number of the company, also referred as Import Export Code (India only)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub export_license_id: Option<String>,
    #[doc = "The purpose code to use for export transactions (India only)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub export_purpose_code: Option<String>,
    #[doc = "The company's legal name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The Kana variation of the company's legal name (Japan only)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name_kana: Option<String>,
    #[doc = "The Kanji variation of the company's legal name (Japan only)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name_kanji: Option<String>,
    #[doc = "Whether the company's owners have been provided."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<bool>,
    #[doc = "This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ownership_declaration: Option<stripe_types_04::accounts::LegalEntityUboDeclaration>,
    #[doc = "This value is used to determine if a business is exempt from providing ultimate beneficial owners."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ownership_exemption_reason: Option<stripe_types_01::accounts::LegalEntityCompanyOwnershipExemptionReason>,
    #[doc = "The company's phone number (used for verification)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub principal_place_of_business: Option<stripe_types_01::common::Address>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<stripe_types_04::accounts::LegalEntityRegistrationDate>,
    #[doc = "This hash is used to attest that the representative is authorized to act as the representative of their legal entity."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub representative_declaration: Option<stripe_types_04::accounts::LegalEntityRepresentativeDeclaration>,
    #[doc = "The category identifying the legal structure of the company or legal entity."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub structure: Option<stripe_types_01::accounts::LegalEntityCompanyStructure>,
    #[doc = "Whether the company's business ID number was provided."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_id_provided: Option<bool>,
    #[doc = "The jurisdiction in which the `tax_id` is registered (Germany-based companies only)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_id_registrar: Option<String>,
    #[doc = "Whether the company's business VAT number was provided."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub vat_id_provided: Option<bool>,
    #[doc = "Information on the verification state of the company."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<stripe_types_10::accounts::LegalEntityCompanyVerification>,
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
pub struct Person {
    #[doc = "The account the person is associated with."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_tos_acceptances: Option<stripe_types_05::accounts::PersonAdditionalTosAcceptances>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::common::Address>,
    #[doc = "The Kana variation of the person's address (Japan only)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<stripe_types_04::accounts::LegalEntityJapanAddress>,
    #[doc = "The Kanji variation of the person's address (Japan only)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<stripe_types_04::accounts::LegalEntityJapanAddress>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dob: Option<stripe_types_04::accounts::LegalEntityDob>,
    #[doc = "The person's email address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The person's first name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "The Kana variation of the person's first name (Japan only)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<String>,
    #[doc = "The Kanji variation of the person's first name (Japan only)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<String>,
    #[doc = "A list of alternate names or aliases that the person is known by."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<Vec<String>>,
    #[doc = "Information about the upcoming new requirements for this person..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<stripe_types_09::accounts::PersonFutureRequirements>,
    #[doc = "The person's gender."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "Whether the person's `id_number` was provided."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id_number_provided: Option<bool>,
    #[doc = "Whether the person's `id_number_secondary` was provided."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id_number_secondary_provided: Option<bool>,
    #[doc = "The person's last name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "The Kana variation of the person's last name (Japan only)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<String>,
    #[doc = "The Kanji variation of the person's last name (Japan only)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<String>,
    #[doc = "The person's maiden name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The country where the person is a national."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::accounts::PersonObject,
    #[doc = "The person's phone number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "Indicates if the person or any of their representatives, family members, or other closely related persons, declares t..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<stripe_types_01::accounts::PersonPoliticalExposure>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<stripe_types_01::common::Address>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<stripe_types_03::accounts::PersonRelationship>,
    #[doc = "Information about the requirements for this person, including what information needs to be collected, and by when."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub requirements: Option<stripe_types_07::accounts::PersonRequirements>,
    #[doc = "Whether the last four digits of the person's Social Security number have been provided (U.S."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ssn_last_4_provided: Option<bool>,
    #[doc = "Demographic data related to the person."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_cfpb_data: Option<stripe_types_07::accounts::PersonUsCfpbData>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<stripe_types_10::accounts::LegalEntityPersonVerification>,
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
impl crate::PagedItem for Person {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsSettings {
    #[doc = "Settings specific to Bacs Direct Debit payments."]
    pub bacs_debit_payments: Option<stripe_types_05::accounts::UpdateAccountParamsSettingsBacsDebitPayments>,
    #[doc = "Settings used to apply the account's branding to email receipts, invoices, Checkout, and other products."]
    pub branding: Option<stripe_types_05::accounts::UpdateAccountParamsSettingsBranding>,
    #[doc = "Settings specific to the account's use of the Card Issuing product."]
    pub card_issuing: Option<stripe_types_09::accounts::UpdateAccountParamsSettingsCardIssuing>,
    #[doc = "Settings specific to card charging on the account."]
    pub card_payments: Option<stripe_types_09::accounts::UpdateAccountParamsSettingsCardPayments>,
    #[doc = "Settings specific to the account's use of Invoices."]
    pub invoices: Option<stripe_types_07::accounts::UpdateAccountParamsSettingsInvoices>,
    #[doc = "Settings that apply across payment methods for charging on the account."]
    pub payments: Option<stripe_types_06::accounts::UpdateAccountParamsSettingsPayments>,
    #[doc = "Settings specific to the account's payouts."]
    pub payouts: Option<stripe_types_10::accounts::UpdateAccountParamsSettingsPayouts>,
    #[doc = "Settings specific to the PayPay payments method."]
    pub paypay_payments: Option<stripe_types_11::accounts::UpdateAccountParamsSettingsPaypayPayments>,
    #[doc = "Settings specific to SEPA Direct Debit payments."]
    pub sepa_debit_payments: Option<stripe_types_06::accounts::UpdateAccountParamsSettingsSepaDebitPayments>,
    #[doc = "Settings specific to the account's Treasury FinancialAccounts."]
    pub treasury: Option<stripe_types_09::accounts::UpdateAccountParamsSettingsTreasury>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bacs_debit_payments", self.bacs_debit_payments.as_ref());
        form.field_opt("branding", self.branding.as_ref());
        form.field_opt("card_issuing", self.card_issuing.as_ref());
        form.field_opt("card_payments", self.card_payments.as_ref());
        form.field_opt("invoices", self.invoices.as_ref());
        form.field_opt("payments", self.payments.as_ref());
        form.field_opt("payouts", self.payouts.as_ref());
        form.field_opt("paypay_payments", self.paypay_payments.as_ref());
        form.field_opt("sepa_debit_payments", self.sepa_debit_payments.as_ref());
        form.field_opt("treasury", self.treasury.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountPersonParams {
    #[doc = "Details on the legal guardian's or authorizer's acceptance of the required Stripe agreements."]
    pub additional_tos_acceptances:
        Option<stripe_types_08::accounts::UpdateAccountPersonParamsAdditionalTosAcceptances>,
    #[doc = "The person's address."]
    pub address: Option<stripe_types_06::accounts::UpdateAccountPersonParamsAddress>,
    #[doc = "The Kana variation of the person's address (Japan only)."]
    pub address_kana: Option<stripe_types_05::accounts::UpdateAccountPersonParamsAddressKana>,
    #[doc = "The Kanji variation of the person's address (Japan only)."]
    pub address_kanji: Option<stripe_types_06::accounts::UpdateAccountPersonParamsAddressKanji>,
    #[doc = "The person's date of birth."]
    pub dob: Option<stripe_types_05::accounts::UpdateAccountPersonParamsDob>,
    #[doc = "Documents that may be submitted to satisfy various informational requests."]
    pub documents: Option<stripe_types_09::accounts::UpdateAccountPersonParamsDocuments>,
    #[doc = "The person's email address."]
    pub email: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The person's first name."]
    pub first_name: Option<String>,
    #[doc = "The Kana variation of the person's first name (Japan only)."]
    pub first_name_kana: Option<String>,
    #[doc = "The Kanji variation of the person's first name (Japan only)."]
    pub first_name_kanji: Option<String>,
    #[doc = "A list of alternate names or aliases that the person is known by."]
    pub full_name_aliases: Option<json::OwnedValue>,
    #[doc = "The person's gender (International regulations require either 'male' or 'female')."]
    pub gender: Option<String>,
    #[doc = "The person's ID number, as appropriate for their country."]
    pub id_number: Option<String>,
    #[doc = "The person's secondary ID number, as appropriate for their country, will be used for enhanced verification checks."]
    pub id_number_secondary: Option<String>,
    #[doc = "The person's last name."]
    pub last_name: Option<String>,
    #[doc = "The Kana variation of the person's last name (Japan only)."]
    pub last_name_kana: Option<String>,
    #[doc = "The Kanji variation of the person's last name (Japan only)."]
    pub last_name_kanji: Option<String>,
    #[doc = "The person's maiden name."]
    pub maiden_name: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The country where the person is a national."]
    pub nationality: Option<String>,
    #[doc = "A [person token](https://docs.stripe.com/connect/account-tokens), used to securely provide details to the person."]
    pub person_token: Option<String>,
    #[doc = "The person's phone number."]
    pub phone: Option<String>,
    #[doc = "Indicates if the person or any of their representatives, family members, or other closely related persons, declares t..."]
    pub political_exposure: Option<stripe_types_05::accounts::UpdateAccountPersonParamsPoliticalExposure>,
    #[doc = "The person's registered address."]
    pub registered_address: Option<stripe_types_06::accounts::UpdateAccountPersonParamsRegisteredAddress>,
    #[doc = "The relationship that this person has with the account's legal entity."]
    pub relationship: Option<stripe_types_05::accounts::UpdateAccountPersonParamsRelationship>,
    #[doc = "The last four digits of the person's Social Security number (U.S."]
    pub ssn_last_4: Option<String>,
    #[doc = "Demographic data related to the person."]
    pub us_cfpb_data: Option<stripe_types_11::accounts::UpdateAccountPersonParamsUsCfpbData>,
    #[doc = "The person's verification status."]
    pub verification: Option<stripe_types_09::accounts::UpdateAccountPersonParamsVerification>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountPersonParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("additional_tos_acceptances", self.additional_tos_acceptances.as_ref());
        form.field_opt("address", self.address.as_ref());
        form.field_opt("address_kana", self.address_kana.as_ref());
        form.field_opt("address_kanji", self.address_kanji.as_ref());
        form.field_opt("dob", self.dob.as_ref());
        form.field_opt("documents", self.documents.as_ref());
        form.field_opt("email", self.email.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("first_name", self.first_name.as_ref());
        form.field_opt("first_name_kana", self.first_name_kana.as_ref());
        form.field_opt("first_name_kanji", self.first_name_kanji.as_ref());
        form.field_opt("gender", self.gender.as_ref());
        form.field_opt("id_number", self.id_number.as_ref());
        form.field_opt("id_number_secondary", self.id_number_secondary.as_ref());
        form.field_opt("last_name", self.last_name.as_ref());
        form.field_opt("last_name_kana", self.last_name_kana.as_ref());
        form.field_opt("last_name_kanji", self.last_name_kanji.as_ref());
        form.field_opt("maiden_name", self.maiden_name.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("nationality", self.nationality.as_ref());
        form.field_opt("person_token", self.person_token.as_ref());
        form.field_opt("phone", self.phone.as_ref());
        form.field_opt("political_exposure", self.political_exposure.as_ref());
        form.field_opt("registered_address", self.registered_address.as_ref());
        form.field_opt("relationship", self.relationship.as_ref());
        form.field_opt("ssn_last_4", self.ssn_last_4.as_ref());
        form.field_opt("us_cfpb_data", self.us_cfpb_data.as_ref());
        form.field_opt("verification", self.verification.as_ref());
    }
}
