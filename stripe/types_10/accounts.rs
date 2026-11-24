#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettings {
    #[doc = "Settings specific to Bacs Direct Debit."]
    pub bacs_debit_payments: Option<stripe_types_03::accounts::CreateAccountParamsSettingsBacsDebitPayments>,
    #[doc = "Settings used to apply the account's branding to email receipts, invoices, Checkout, and other products."]
    pub branding: Option<stripe_types_01::accounts::CreateAccountParamsSettingsBranding>,
    #[doc = "Settings specific to the account's use of the Card Issuing product."]
    pub card_issuing: Option<stripe_types_06::accounts::CreateAccountParamsSettingsCardIssuing>,
    #[doc = "Settings specific to card charging on the account."]
    pub card_payments: Option<stripe_types_05::accounts::CreateAccountParamsSettingsCardPayments>,
    #[doc = "Settings specific to the account’s use of Invoices."]
    pub invoices: Option<stripe_types_06::accounts::CreateAccountParamsSettingsInvoices>,
    #[doc = "Settings that apply across payment methods for charging on the account."]
    pub payments: Option<stripe_types_01::accounts::CreateAccountParamsSettingsPayments>,
    #[doc = "Settings specific to the account's payouts."]
    pub payouts: Option<stripe_types_08::accounts::CreateAccountParamsSettingsPayouts>,
    #[doc = "Settings specific to the PayPay payments method."]
    pub paypay_payments: Option<stripe_types_09::accounts::CreateAccountParamsSettingsPaypayPayments>,
    #[doc = "Settings specific to SEPA Direct Debit payments."]
    pub sepa_debit_payments: Option<stripe_types_02::accounts::CreateAccountParamsSettingsSepaDebitPayments>,
    #[doc = "Settings specific to the account's Treasury FinancialAccounts."]
    pub treasury: Option<stripe_types_07::accounts::CreateAccountParamsSettingsTreasury>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettings {
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
pub struct CreateAccountPersonParams {
    #[doc = "Details on the legal guardian's or authorizer's acceptance of the required Stripe agreements."]
    pub additional_tos_acceptances:
        Option<stripe_types_07::accounts::CreateAccountPersonParamsAdditionalTosAcceptances>,
    #[doc = "The person's address."]
    pub address: Option<stripe_types_05::accounts::CreateAccountPersonParamsAddress>,
    #[doc = "The Kana variation of the person's address (Japan only)."]
    pub address_kana: Option<stripe_types_05::accounts::CreateAccountPersonParamsAddressKana>,
    #[doc = "The Kanji variation of the person's address (Japan only)."]
    pub address_kanji: Option<stripe_types_05::accounts::CreateAccountPersonParamsAddressKanji>,
    #[doc = "The person's date of birth."]
    pub dob: Option<stripe_types_05::accounts::CreateAccountPersonParamsDob>,
    #[doc = "Documents that may be submitted to satisfy various informational requests."]
    pub documents: Option<stripe_types_07::accounts::CreateAccountPersonParamsDocuments>,
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
    pub political_exposure: Option<stripe_types_05::accounts::CreateAccountPersonParamsPoliticalExposure>,
    #[doc = "The person's registered address."]
    pub registered_address: Option<stripe_types_05::accounts::CreateAccountPersonParamsRegisteredAddress>,
    #[doc = "The relationship that this person has with the account's legal entity."]
    pub relationship: Option<stripe_types_01::accounts::CreateAccountPersonParamsRelationship>,
    #[doc = "The last four digits of the person's Social Security number (U.S."]
    pub ssn_last_4: Option<String>,
    #[doc = "Demographic data related to the person."]
    pub us_cfpb_data: Option<stripe_types_08::accounts::CreateAccountPersonParamsUsCfpbData>,
    #[doc = "The person's verification status."]
    pub verification: Option<stripe_types_08::accounts::CreateAccountPersonParamsVerification>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountPersonParams {
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
pub struct LegalEntityCompanyVerification {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<stripe_types_09::accounts::LegalEntityCompanyVerificationDocument>,
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
pub struct LegalEntityPersonVerification {
    #[doc = "A document showing address, either a passport, local ID card, or utility bill from a well-known utility company."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<stripe_types_08::accounts::LegalEntityPersonVerificationDocument>,
    #[doc = "A user-displayable string describing the verification state for the person."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[doc = "One of `document_address_mismatch`, `document_dob_mismatch`, `document_duplicate_type`, `document_id_number_mismatch`..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub details_code: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<stripe_types_08::accounts::LegalEntityPersonVerificationDocument>,
    #[doc = "The state of verification for the person."]
    #[json(default)]
    pub status: String,
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsDocuments {
    #[doc = "One or more documents that support the Bank account ownership verification..."]
    pub bank_account_ownership_verification:
        Option<stripe_types_06::accounts::UpdateAccountParamsDocumentsBankAccountOwnershipVerification>,
    #[doc = "One or more documents that demonstrate proof of a company's license to operate."]
    pub company_license: Option<stripe_types_05::accounts::UpdateAccountParamsDocumentsCompanyLicense>,
    #[doc = "One or more documents showing the company's governing document (for example, a memorandum of association, constitutio..."]
    pub company_memorandum_of_association:
        Option<stripe_types_05::accounts::UpdateAccountParamsDocumentsCompanyMemorandumOfAssociation>,
    #[doc = "(Certain countries only) One or more documents showing the ministerial decree legalizing the company's establishment."]
    pub company_ministerial_decree:
        Option<stripe_types_06::accounts::UpdateAccountParamsDocumentsCompanyMinisterialDecree>,
    #[doc = "One or more documents that demonstrate proof of a company's registration with the appropriate local authorities."]
    pub company_registration_verification:
        Option<stripe_types_06::accounts::UpdateAccountParamsDocumentsCompanyRegistrationVerification>,
    #[doc = "One or more documents that demonstrate proof of a company's tax ID."]
    pub company_tax_id_verification:
        Option<stripe_types_05::accounts::UpdateAccountParamsDocumentsCompanyTaxIdVerification>,
    #[doc = "One or more documents that demonstrate proof of address."]
    pub proof_of_address: Option<stripe_types_06::accounts::UpdateAccountParamsDocumentsProofOfAddress>,
    #[doc = "One or more documents showing the company’s proof of registration with the national business registry."]
    pub proof_of_registration: Option<stripe_types_08::accounts::UpdateAccountParamsDocumentsProofOfRegistration>,
    #[doc = "One or more documents that demonstrate proof of ultimate beneficial ownership."]
    pub proof_of_ultimate_beneficial_ownership:
        Option<stripe_types_09::accounts::UpdateAccountParamsDocumentsProofOfUltimateBeneficialOwnership>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsDocuments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt(
            "bank_account_ownership_verification",
            self.bank_account_ownership_verification.as_ref(),
        );
        form.field_opt("company_license", self.company_license.as_ref());
        form.field_opt(
            "company_memorandum_of_association",
            self.company_memorandum_of_association.as_ref(),
        );
        form.field_opt("company_ministerial_decree", self.company_ministerial_decree.as_ref());
        form.field_opt(
            "company_registration_verification",
            self.company_registration_verification.as_ref(),
        );
        form.field_opt("company_tax_id_verification", self.company_tax_id_verification.as_ref());
        form.field_opt("proof_of_address", self.proof_of_address.as_ref());
        form.field_opt("proof_of_registration", self.proof_of_registration.as_ref());
        form.field_opt(
            "proof_of_ultimate_beneficial_ownership",
            self.proof_of_ultimate_beneficial_ownership.as_ref(),
        );
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsIndividual {
    #[doc = "The individual's primary address."]
    pub address: Option<stripe_types_05::accounts::UpdateAccountParamsIndividualAddress>,
    #[doc = "The Kana variation of the individual's primary address (Japan only)."]
    pub address_kana: Option<stripe_types_06::accounts::UpdateAccountParamsIndividualAddressKana>,
    #[doc = "The Kanji variation of the individual's primary address (Japan only)."]
    pub address_kanji: Option<stripe_types_05::accounts::UpdateAccountParamsIndividualAddressKanji>,
    #[doc = "The individual's date of birth."]
    pub dob: Option<stripe_types_05::accounts::UpdateAccountParamsIndividualDob>,
    #[doc = "The individual's email address."]
    pub email: Option<String>,
    #[doc = "The individual's first name."]
    pub first_name: Option<String>,
    #[doc = "The Kana variation of the individual's first name (Japan only)."]
    pub first_name_kana: Option<String>,
    #[doc = "The Kanji variation of the individual's first name (Japan only)."]
    pub first_name_kanji: Option<String>,
    #[doc = "A list of alternate names or aliases that the individual is known by."]
    pub full_name_aliases: Option<json::OwnedValue>,
    #[doc = "The individual's gender"]
    pub gender: Option<String>,
    #[doc = "The government-issued ID number of the individual, as appropriate for the representative's country."]
    pub id_number: Option<String>,
    #[doc = "The government-issued secondary ID number of the individual, as appropriate for the representative's country, will be..."]
    pub id_number_secondary: Option<String>,
    #[doc = "The individual's last name."]
    pub last_name: Option<String>,
    #[doc = "The Kana variation of the individual's last name (Japan only)."]
    pub last_name_kana: Option<String>,
    #[doc = "The Kanji variation of the individual's last name (Japan only)."]
    pub last_name_kanji: Option<String>,
    #[doc = "The individual's maiden name."]
    pub maiden_name: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The individual's phone number."]
    pub phone: Option<String>,
    #[doc = "Indicates if the person or any of their representatives, family members, or other closely related persons, declares t..."]
    pub political_exposure: Option<stripe_types_05::accounts::UpdateAccountParamsIndividualPoliticalExposure>,
    #[doc = "The individual's registered address."]
    pub registered_address: Option<stripe_types_05::accounts::UpdateAccountParamsIndividualRegisteredAddress>,
    #[doc = "Describes the person’s relationship to the account."]
    pub relationship: Option<stripe_types_06::accounts::UpdateAccountParamsIndividualRelationship>,
    #[doc = "The last four digits of the individual's Social Security Number (U.S."]
    pub ssn_last_4: Option<String>,
    #[doc = "The individual's verification document information."]
    pub verification: Option<stripe_types_08::accounts::UpdateAccountParamsIndividualVerification>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsIndividual {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("address_kana", self.address_kana.as_ref());
        form.field_opt("address_kanji", self.address_kanji.as_ref());
        form.field_opt("dob", self.dob.as_ref());
        form.field_opt("email", self.email.as_ref());
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
        form.field_opt("phone", self.phone.as_ref());
        form.field_opt("political_exposure", self.political_exposure.as_ref());
        form.field_opt("registered_address", self.registered_address.as_ref());
        form.field_opt("relationship", self.relationship.as_ref());
        form.field_opt("ssn_last_4", self.ssn_last_4.as_ref());
        form.field_opt("verification", self.verification.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsSettingsPayouts {
    #[doc = "A Boolean indicating whether Stripe should try to reclaim negative balances from an attached bank account."]
    pub debit_negative_balances: Option<bool>,
    #[doc = "Details on when funds from charges are available, and when they are paid out to an external account."]
    pub schedule: Option<stripe_types_08::accounts::UpdateAccountParamsSettingsPayoutsSchedule>,
    #[doc = "The text that appears on the bank account statement for payouts."]
    pub statement_descriptor: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsSettingsPayouts {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("debit_negative_balances", self.debit_negative_balances.as_ref());
        form.field_opt("schedule", self.schedule.as_ref());
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
    }
}
