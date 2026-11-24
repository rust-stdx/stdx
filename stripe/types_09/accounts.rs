#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsDocuments {
    #[doc = "One or more documents that support the Bank account ownership verification..."]
    pub bank_account_ownership_verification:
        Option<stripe_types_00::accounts::CreateAccountParamsDocumentsBankAccountOwnershipVerification>,
    #[doc = "One or more documents that demonstrate proof of a company's license to operate."]
    pub company_license: Option<stripe_types_06::accounts::CreateAccountParamsDocumentsCompanyLicense>,
    #[doc = "One or more documents showing the company's governing document (for example, a memorandum of association, constitutio..."]
    pub company_memorandum_of_association:
        Option<stripe_types_05::accounts::CreateAccountParamsDocumentsCompanyMemorandumOfAssociation>,
    #[doc = "(Certain countries only) One or more documents showing the ministerial decree legalizing the company's establishment."]
    pub company_ministerial_decree:
        Option<stripe_types_06::accounts::CreateAccountParamsDocumentsCompanyMinisterialDecree>,
    #[doc = "One or more documents that demonstrate proof of a company's registration with the appropriate local authorities."]
    pub company_registration_verification:
        Option<stripe_types_05::accounts::CreateAccountParamsDocumentsCompanyRegistrationVerification>,
    #[doc = "One or more documents that demonstrate proof of a company's tax ID."]
    pub company_tax_id_verification:
        Option<stripe_types_05::accounts::CreateAccountParamsDocumentsCompanyTaxIdVerification>,
    #[doc = "One or more documents that demonstrate proof of address."]
    pub proof_of_address: Option<stripe_types_05::accounts::CreateAccountParamsDocumentsProofOfAddress>,
    #[doc = "One or more documents that demonstrate proof of ultimate beneficial ownership."]
    pub proof_of_ultimate_beneficial_ownership:
        Option<stripe_types_06::accounts::CreateAccountParamsDocumentsProofOfUltimateBeneficialOwnership>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsDocuments {
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
        form.field_opt(
            "proof_of_ultimate_beneficial_ownership",
            self.proof_of_ultimate_beneficial_ownership.as_ref(),
        );
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettingsPaypayPayments {
    #[doc = "Additional files that are required to support the onboarding process of your business."]
    pub additional_files: Option<Vec<String>>,
    #[doc = "The type of goods your business sells."]
    pub goods_type: Option<stripe_types_02::accounts::CreateAccountParamsSettingsPaypayPaymentsGoodsType>,
    #[doc = "Details regarding your business's website."]
    pub site: Option<stripe_types_06::accounts::CreateAccountParamsSettingsPaypayPaymentsSite>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsPaypayPayments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.additional_files.as_ref() {
            form.field_array("additional_files", values.as_slice());
        }
        form.field_opt("goods_type", self.goods_type.as_ref());
        form.field_opt("site", self.site.as_ref());
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
#[derive(Debug, Clone)]
pub enum DeletedExternalAccount {
    /// Expanded `bank_account`.
    BankAccount(Box<stripe_types_06::payment_methods::DeletedBankAccount>),
    /// Expanded `card`.
    Card(Box<stripe_types_05::payment_methods::DeletedCard>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
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
impl<'json> json::Decode<'json> for DeletedExternalAccount {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "bank_account" => {
                DeletedExternalAccount::BankAccount(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            "card" => DeletedExternalAccount::Card(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            _ => DeletedExternalAccount::Other(value),
        })
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
pub struct LegalEntityCompanyVerificationDocument {
    #[doc = "The back of a document returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value of..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub back: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "A user-displayable string describing the verification state of this document."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[doc = "One of `document_corrupt`, `document_expired`, `document_failed_copy`, `document_failed_greyscale`, `document_failed_..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub details_code: Option<String>,
    #[doc = "The front of a document returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value o..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub front: Option<crate::Expandable<stripe_types_06::files::File>>,
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
pub struct PersonFutureRequirements {
    #[doc = "Fields that are due and can be resolved by providing the corresponding alternative fields instead."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alternatives: Option<Vec<stripe_types_01::account::AccountRequirementsAlternative>>,
    #[doc = "Fields that need to be resolved to keep the person's account enabled."]
    #[json(default)]
    pub currently_due: Vec<String>,
    #[doc = "Fields that are `currently_due` and need to be collected again because validation or verification failed."]
    #[json(default)]
    pub errors: Vec<stripe_types_06::account::AccountRequirementsError>,
    #[doc = "Fields you must collect when all thresholds are reached."]
    #[json(default)]
    pub eventually_due: Vec<String>,
    #[doc = "Fields that haven't been resolved by the account's `requirements.current_deadline`."]
    #[json(default)]
    pub past_due: Vec<String>,
    #[doc = "Fields that are being reviewed, or might become required depending on the results of a review."]
    #[json(default)]
    pub pending_verification: Vec<String>,
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsDocumentsProofOfUltimateBeneficialOwnership {
    #[doc = "One or more document ids returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value ..."]
    pub files: Option<Vec<String>>,
    #[doc = "Information regarding the person signing the document if applicable."]
    pub signer: Option<stripe_types_06::accounts::UpdateAccountParamsDocumentsProofOfUltimateBeneficialOwnershipSigner>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsDocumentsProofOfUltimateBeneficialOwnership {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.files.as_ref() {
            form.field_array("files", values.as_slice());
        }
        form.field_opt("signer", self.signer.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsSettingsCardIssuing {
    #[doc = "Details on the account's acceptance of the [Stripe Issuing Terms and Disclosures](/issuing/connect/tos_acceptance)."]
    pub tos_acceptance: Option<stripe_types_05::accounts::UpdateAccountParamsSettingsCardIssuingTosAcceptance>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsSettingsCardIssuing {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("tos_acceptance", self.tos_acceptance.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsSettingsCardPayments {
    #[doc = "Automatically declines certain charge types regardless of whether the card issuer accepted or declined the charge."]
    pub decline_on: Option<stripe_types_06::accounts::UpdateAccountParamsSettingsCardPaymentsDeclineOn>,
    #[doc = "The default text that appears on credit card statements when a charge is made."]
    pub statement_descriptor_prefix: Option<String>,
    #[doc = "The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only)."]
    pub statement_descriptor_prefix_kana: Option<json::OwnedValue>,
    #[doc = "The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only)."]
    pub statement_descriptor_prefix_kanji: Option<json::OwnedValue>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsSettingsCardPayments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("decline_on", self.decline_on.as_ref());
        form.field_opt("statement_descriptor_prefix", self.statement_descriptor_prefix.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsSettingsTreasury {
    #[doc = "Details on the account's acceptance of the Stripe Treasury Services Agreement."]
    pub tos_acceptance: Option<stripe_types_05::accounts::UpdateAccountParamsSettingsTreasuryTosAcceptance>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsSettingsTreasury {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("tos_acceptance", self.tos_acceptance.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountPersonParamsDocuments {
    #[doc = "One or more documents that demonstrate proof that this person is authorized to represent the company."]
    pub company_authorization:
        Option<stripe_types_05::accounts::UpdateAccountPersonParamsDocumentsCompanyAuthorization>,
    #[doc = "One or more documents showing the person's passport page with photo and personal data."]
    pub passport: Option<stripe_types_05::accounts::UpdateAccountPersonParamsDocumentsPassport>,
    #[doc = "One or more documents showing the person's visa required for living in the country where they are residing."]
    pub visa: Option<stripe_types_06::accounts::UpdateAccountPersonParamsDocumentsVisa>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountPersonParamsDocuments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("company_authorization", self.company_authorization.as_ref());
        form.field_opt("passport", self.passport.as_ref());
        form.field_opt("visa", self.visa.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountPersonParamsVerification {
    #[doc = "A document showing address, either a passport, local ID card, or utility bill from a well-known utility company."]
    pub additional_document: Option<stripe_types_06::accounts::UpdateAccountPersonParamsVerificationAdditionalDocument>,
    #[doc = "An identifying document, either a passport or local ID card."]
    pub document: Option<stripe_types_05::accounts::UpdateAccountPersonParamsVerificationDocument>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountPersonParamsVerification {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("additional_document", self.additional_document.as_ref());
        form.field_opt("document", self.document.as_ref());
    }
}
