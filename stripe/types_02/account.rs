#[cfg(any(feature = "accounts", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum AccountCapabilityFutureRequirementsDisabledReason {
    Other,
    PausedInactivity,
    PendingOnboarding,
    PendingReview,
    PlatformDisabled,
    PlatformPaused,
    RejectedInactivity,
    RejectedOther,
    RejectedUnsupportedBusiness,
    RequirementsFieldsNeeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "accounts", feature = "events"))]
impl AccountCapabilityFutureRequirementsDisabledReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            AccountCapabilityFutureRequirementsDisabledReason::Other => "other",
            AccountCapabilityFutureRequirementsDisabledReason::PausedInactivity => "paused.inactivity",
            AccountCapabilityFutureRequirementsDisabledReason::PendingOnboarding => "pending.onboarding",
            AccountCapabilityFutureRequirementsDisabledReason::PendingReview => "pending.review",
            AccountCapabilityFutureRequirementsDisabledReason::PlatformDisabled => "platform_disabled",
            AccountCapabilityFutureRequirementsDisabledReason::PlatformPaused => "platform_paused",
            AccountCapabilityFutureRequirementsDisabledReason::RejectedInactivity => "rejected.inactivity",
            AccountCapabilityFutureRequirementsDisabledReason::RejectedOther => "rejected.other",
            AccountCapabilityFutureRequirementsDisabledReason::RejectedUnsupportedBusiness => {
                "rejected.unsupported_business"
            }
            AccountCapabilityFutureRequirementsDisabledReason::RequirementsFieldsNeeded => "requirements.fields_needed",
            AccountCapabilityFutureRequirementsDisabledReason::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "accounts", feature = "events"))]
impl<'json> json::Decode<'json> for AccountCapabilityFutureRequirementsDisabledReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(AccountCapabilityFutureRequirementsDisabledReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "other" => AccountCapabilityFutureRequirementsDisabledReason::Other,
            "paused.inactivity" => AccountCapabilityFutureRequirementsDisabledReason::PausedInactivity,
            "pending.onboarding" => AccountCapabilityFutureRequirementsDisabledReason::PendingOnboarding,
            "pending.review" => AccountCapabilityFutureRequirementsDisabledReason::PendingReview,
            "platform_disabled" => AccountCapabilityFutureRequirementsDisabledReason::PlatformDisabled,
            "platform_paused" => AccountCapabilityFutureRequirementsDisabledReason::PlatformPaused,
            "rejected.inactivity" => AccountCapabilityFutureRequirementsDisabledReason::RejectedInactivity,
            "rejected.other" => AccountCapabilityFutureRequirementsDisabledReason::RejectedOther,
            "rejected.unsupported_business" => {
                AccountCapabilityFutureRequirementsDisabledReason::RejectedUnsupportedBusiness
            }
            "requirements.fields_needed" => AccountCapabilityFutureRequirementsDisabledReason::RequirementsFieldsNeeded,
            other => AccountCapabilityFutureRequirementsDisabledReason::Unknown(other.to_string()),
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
pub struct AccountMonthlyEstimatedRevenue {
    #[doc = "A non-negative integer representing how much to charge in the [smallest currency unit](/currencies#zero-decimal)."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
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
#[derive(Debug, Clone, PartialEq)]
pub enum AccountRequirementsErrorCode {
    ExternalRequest,
    InformationMissing,
    InvalidAddressCityStatePostalCode,
    InvalidAddressCmraAddress,
    InvalidAddressHighwayContractBox,
    InvalidAddressPrivateMailbox,
    InvalidAddressRegisteredAgentAddress,
    InvalidBusinessProfileName,
    InvalidBusinessProfileNameDenylisted,
    InvalidCompanyNameDenylisted,
    InvalidDobAgeOverMaximum,
    InvalidDobAgeUnder18,
    InvalidDobAgeUnderMinimum,
    InvalidProductDescriptionLength,
    InvalidProductDescriptionUrlMatch,
    InvalidRepresentativeCountry,
    InvalidSignator,
    InvalidStatementDescriptorBusinessMismatch,
    InvalidStatementDescriptorDenylisted,
    InvalidStatementDescriptorLength,
    InvalidStatementDescriptorPrefixDenylisted,
    InvalidStatementDescriptorPrefixMismatch,
    InvalidStreetAddress,
    InvalidTaxId,
    InvalidTaxIdFormat,
    InvalidTosAcceptance,
    InvalidUrlDenylisted,
    InvalidUrlFormat,
    InvalidUrlLength,
    InvalidUrlWebPresenceDetected,
    InvalidUrlWebsiteBusinessInformationMismatch,
    InvalidUrlWebsiteEmpty,
    InvalidUrlWebsiteInaccessible,
    InvalidUrlWebsiteInaccessibleGeoblocked,
    InvalidUrlWebsiteInaccessiblePasswordProtected,
    InvalidUrlWebsiteIncomplete,
    InvalidUrlWebsiteIncompleteCancellationPolicy,
    InvalidUrlWebsiteIncompleteCustomerServiceDetails,
    InvalidUrlWebsiteIncompleteLegalRestrictions,
    InvalidUrlWebsiteIncompleteRefundPolicy,
    InvalidUrlWebsiteIncompleteReturnPolicy,
    InvalidUrlWebsiteIncompleteTermsAndConditions,
    InvalidUrlWebsiteIncompleteUnderConstruction,
    InvalidUrlWebsiteOther,
    InvalidValueOther,
    UnsupportedBusinessType,
    VerificationDirectorsMismatch,
    VerificationDocumentAddressMismatch,
    VerificationDocumentAddressMissing,
    VerificationDocumentCorrupt,
    VerificationDocumentCountryNotSupported,
    VerificationDocumentDirectorsMismatch,
    VerificationDocumentDobMismatch,
    VerificationDocumentDuplicateType,
    VerificationDocumentExpired,
    VerificationDocumentFailedCopy,
    VerificationDocumentFailedGreyscale,
    VerificationDocumentFailedOther,
    VerificationDocumentFailedTestMode,
    VerificationDocumentFraudulent,
    VerificationDocumentIdNumberMismatch,
    VerificationDocumentIdNumberMissing,
    VerificationDocumentIncomplete,
    VerificationDocumentInvalid,
    VerificationDocumentIssueOrExpiryDateMissing,
    VerificationDocumentManipulated,
    VerificationDocumentMissingBack,
    VerificationDocumentMissingFront,
    VerificationDocumentNameMismatch,
    VerificationDocumentNameMissing,
    VerificationDocumentNationalityMismatch,
    VerificationDocumentNotReadable,
    VerificationDocumentNotSigned,
    VerificationDocumentNotUploaded,
    VerificationDocumentPhotoMismatch,
    VerificationDocumentTooLarge,
    VerificationDocumentTypeNotSupported,
    VerificationExtraneousDirectors,
    VerificationFailedAddressMatch,
    VerificationFailedAuthorizerAuthority,
    VerificationFailedBusinessIecNumber,
    VerificationFailedDocumentMatch,
    VerificationFailedIdNumberMatch,
    VerificationFailedKeyedIdentity,
    VerificationFailedKeyedMatch,
    VerificationFailedNameMatch,
    VerificationFailedOther,
    VerificationFailedRepresentativeAuthority,
    VerificationFailedResidentialAddress,
    VerificationFailedTaxIdMatch,
    VerificationFailedTaxIdNotIssued,
    VerificationLegalEntityStructureMismatch,
    VerificationMissingDirectors,
    VerificationMissingExecutives,
    VerificationMissingOwners,
    VerificationRejectedOwnershipExemptionReason,
    VerificationRequiresAdditionalMemorandumOfAssociations,
    VerificationRequiresAdditionalProofOfRegistration,
    VerificationSupportability,
    /// A value not known to this version of the crate.
    Unknown(String),
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
impl AccountRequirementsErrorCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            AccountRequirementsErrorCode::ExternalRequest => "external_request",
            AccountRequirementsErrorCode::InformationMissing => "information_missing",
            AccountRequirementsErrorCode::InvalidAddressCityStatePostalCode => "invalid_address_city_state_postal_code",
            AccountRequirementsErrorCode::InvalidAddressCmraAddress => "invalid_address_cmra_address",
            AccountRequirementsErrorCode::InvalidAddressHighwayContractBox => "invalid_address_highway_contract_box",
            AccountRequirementsErrorCode::InvalidAddressPrivateMailbox => "invalid_address_private_mailbox",
            AccountRequirementsErrorCode::InvalidAddressRegisteredAgentAddress => {
                "invalid_address_registered_agent_address"
            }
            AccountRequirementsErrorCode::InvalidBusinessProfileName => "invalid_business_profile_name",
            AccountRequirementsErrorCode::InvalidBusinessProfileNameDenylisted => {
                "invalid_business_profile_name_denylisted"
            }
            AccountRequirementsErrorCode::InvalidCompanyNameDenylisted => "invalid_company_name_denylisted",
            AccountRequirementsErrorCode::InvalidDobAgeOverMaximum => "invalid_dob_age_over_maximum",
            AccountRequirementsErrorCode::InvalidDobAgeUnder18 => "invalid_dob_age_under_18",
            AccountRequirementsErrorCode::InvalidDobAgeUnderMinimum => "invalid_dob_age_under_minimum",
            AccountRequirementsErrorCode::InvalidProductDescriptionLength => "invalid_product_description_length",
            AccountRequirementsErrorCode::InvalidProductDescriptionUrlMatch => "invalid_product_description_url_match",
            AccountRequirementsErrorCode::InvalidRepresentativeCountry => "invalid_representative_country",
            AccountRequirementsErrorCode::InvalidSignator => "invalid_signator",
            AccountRequirementsErrorCode::InvalidStatementDescriptorBusinessMismatch => {
                "invalid_statement_descriptor_business_mismatch"
            }
            AccountRequirementsErrorCode::InvalidStatementDescriptorDenylisted => {
                "invalid_statement_descriptor_denylisted"
            }
            AccountRequirementsErrorCode::InvalidStatementDescriptorLength => "invalid_statement_descriptor_length",
            AccountRequirementsErrorCode::InvalidStatementDescriptorPrefixDenylisted => {
                "invalid_statement_descriptor_prefix_denylisted"
            }
            AccountRequirementsErrorCode::InvalidStatementDescriptorPrefixMismatch => {
                "invalid_statement_descriptor_prefix_mismatch"
            }
            AccountRequirementsErrorCode::InvalidStreetAddress => "invalid_street_address",
            AccountRequirementsErrorCode::InvalidTaxId => "invalid_tax_id",
            AccountRequirementsErrorCode::InvalidTaxIdFormat => "invalid_tax_id_format",
            AccountRequirementsErrorCode::InvalidTosAcceptance => "invalid_tos_acceptance",
            AccountRequirementsErrorCode::InvalidUrlDenylisted => "invalid_url_denylisted",
            AccountRequirementsErrorCode::InvalidUrlFormat => "invalid_url_format",
            AccountRequirementsErrorCode::InvalidUrlLength => "invalid_url_length",
            AccountRequirementsErrorCode::InvalidUrlWebPresenceDetected => "invalid_url_web_presence_detected",
            AccountRequirementsErrorCode::InvalidUrlWebsiteBusinessInformationMismatch => {
                "invalid_url_website_business_information_mismatch"
            }
            AccountRequirementsErrorCode::InvalidUrlWebsiteEmpty => "invalid_url_website_empty",
            AccountRequirementsErrorCode::InvalidUrlWebsiteInaccessible => "invalid_url_website_inaccessible",
            AccountRequirementsErrorCode::InvalidUrlWebsiteInaccessibleGeoblocked => {
                "invalid_url_website_inaccessible_geoblocked"
            }
            AccountRequirementsErrorCode::InvalidUrlWebsiteInaccessiblePasswordProtected => {
                "invalid_url_website_inaccessible_password_protected"
            }
            AccountRequirementsErrorCode::InvalidUrlWebsiteIncomplete => "invalid_url_website_incomplete",
            AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteCancellationPolicy => {
                "invalid_url_website_incomplete_cancellation_policy"
            }
            AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteCustomerServiceDetails => {
                "invalid_url_website_incomplete_customer_service_details"
            }
            AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteLegalRestrictions => {
                "invalid_url_website_incomplete_legal_restrictions"
            }
            AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteRefundPolicy => {
                "invalid_url_website_incomplete_refund_policy"
            }
            AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteReturnPolicy => {
                "invalid_url_website_incomplete_return_policy"
            }
            AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteTermsAndConditions => {
                "invalid_url_website_incomplete_terms_and_conditions"
            }
            AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteUnderConstruction => {
                "invalid_url_website_incomplete_under_construction"
            }
            AccountRequirementsErrorCode::InvalidUrlWebsiteOther => "invalid_url_website_other",
            AccountRequirementsErrorCode::InvalidValueOther => "invalid_value_other",
            AccountRequirementsErrorCode::UnsupportedBusinessType => "unsupported_business_type",
            AccountRequirementsErrorCode::VerificationDirectorsMismatch => "verification_directors_mismatch",
            AccountRequirementsErrorCode::VerificationDocumentAddressMismatch => {
                "verification_document_address_mismatch"
            }
            AccountRequirementsErrorCode::VerificationDocumentAddressMissing => "verification_document_address_missing",
            AccountRequirementsErrorCode::VerificationDocumentCorrupt => "verification_document_corrupt",
            AccountRequirementsErrorCode::VerificationDocumentCountryNotSupported => {
                "verification_document_country_not_supported"
            }
            AccountRequirementsErrorCode::VerificationDocumentDirectorsMismatch => {
                "verification_document_directors_mismatch"
            }
            AccountRequirementsErrorCode::VerificationDocumentDobMismatch => "verification_document_dob_mismatch",
            AccountRequirementsErrorCode::VerificationDocumentDuplicateType => "verification_document_duplicate_type",
            AccountRequirementsErrorCode::VerificationDocumentExpired => "verification_document_expired",
            AccountRequirementsErrorCode::VerificationDocumentFailedCopy => "verification_document_failed_copy",
            AccountRequirementsErrorCode::VerificationDocumentFailedGreyscale => {
                "verification_document_failed_greyscale"
            }
            AccountRequirementsErrorCode::VerificationDocumentFailedOther => "verification_document_failed_other",
            AccountRequirementsErrorCode::VerificationDocumentFailedTestMode => {
                "verification_document_failed_test_mode"
            }
            AccountRequirementsErrorCode::VerificationDocumentFraudulent => "verification_document_fraudulent",
            AccountRequirementsErrorCode::VerificationDocumentIdNumberMismatch => {
                "verification_document_id_number_mismatch"
            }
            AccountRequirementsErrorCode::VerificationDocumentIdNumberMissing => {
                "verification_document_id_number_missing"
            }
            AccountRequirementsErrorCode::VerificationDocumentIncomplete => "verification_document_incomplete",
            AccountRequirementsErrorCode::VerificationDocumentInvalid => "verification_document_invalid",
            AccountRequirementsErrorCode::VerificationDocumentIssueOrExpiryDateMissing => {
                "verification_document_issue_or_expiry_date_missing"
            }
            AccountRequirementsErrorCode::VerificationDocumentManipulated => "verification_document_manipulated",
            AccountRequirementsErrorCode::VerificationDocumentMissingBack => "verification_document_missing_back",
            AccountRequirementsErrorCode::VerificationDocumentMissingFront => "verification_document_missing_front",
            AccountRequirementsErrorCode::VerificationDocumentNameMismatch => "verification_document_name_mismatch",
            AccountRequirementsErrorCode::VerificationDocumentNameMissing => "verification_document_name_missing",
            AccountRequirementsErrorCode::VerificationDocumentNationalityMismatch => {
                "verification_document_nationality_mismatch"
            }
            AccountRequirementsErrorCode::VerificationDocumentNotReadable => "verification_document_not_readable",
            AccountRequirementsErrorCode::VerificationDocumentNotSigned => "verification_document_not_signed",
            AccountRequirementsErrorCode::VerificationDocumentNotUploaded => "verification_document_not_uploaded",
            AccountRequirementsErrorCode::VerificationDocumentPhotoMismatch => "verification_document_photo_mismatch",
            AccountRequirementsErrorCode::VerificationDocumentTooLarge => "verification_document_too_large",
            AccountRequirementsErrorCode::VerificationDocumentTypeNotSupported => {
                "verification_document_type_not_supported"
            }
            AccountRequirementsErrorCode::VerificationExtraneousDirectors => "verification_extraneous_directors",
            AccountRequirementsErrorCode::VerificationFailedAddressMatch => "verification_failed_address_match",
            AccountRequirementsErrorCode::VerificationFailedAuthorizerAuthority => {
                "verification_failed_authorizer_authority"
            }
            AccountRequirementsErrorCode::VerificationFailedBusinessIecNumber => {
                "verification_failed_business_iec_number"
            }
            AccountRequirementsErrorCode::VerificationFailedDocumentMatch => "verification_failed_document_match",
            AccountRequirementsErrorCode::VerificationFailedIdNumberMatch => "verification_failed_id_number_match",
            AccountRequirementsErrorCode::VerificationFailedKeyedIdentity => "verification_failed_keyed_identity",
            AccountRequirementsErrorCode::VerificationFailedKeyedMatch => "verification_failed_keyed_match",
            AccountRequirementsErrorCode::VerificationFailedNameMatch => "verification_failed_name_match",
            AccountRequirementsErrorCode::VerificationFailedOther => "verification_failed_other",
            AccountRequirementsErrorCode::VerificationFailedRepresentativeAuthority => {
                "verification_failed_representative_authority"
            }
            AccountRequirementsErrorCode::VerificationFailedResidentialAddress => {
                "verification_failed_residential_address"
            }
            AccountRequirementsErrorCode::VerificationFailedTaxIdMatch => "verification_failed_tax_id_match",
            AccountRequirementsErrorCode::VerificationFailedTaxIdNotIssued => "verification_failed_tax_id_not_issued",
            AccountRequirementsErrorCode::VerificationLegalEntityStructureMismatch => {
                "verification_legal_entity_structure_mismatch"
            }
            AccountRequirementsErrorCode::VerificationMissingDirectors => "verification_missing_directors",
            AccountRequirementsErrorCode::VerificationMissingExecutives => "verification_missing_executives",
            AccountRequirementsErrorCode::VerificationMissingOwners => "verification_missing_owners",
            AccountRequirementsErrorCode::VerificationRejectedOwnershipExemptionReason => {
                "verification_rejected_ownership_exemption_reason"
            }
            AccountRequirementsErrorCode::VerificationRequiresAdditionalMemorandumOfAssociations => {
                "verification_requires_additional_memorandum_of_associations"
            }
            AccountRequirementsErrorCode::VerificationRequiresAdditionalProofOfRegistration => {
                "verification_requires_additional_proof_of_registration"
            }
            AccountRequirementsErrorCode::VerificationSupportability => "verification_supportability",
            AccountRequirementsErrorCode::Unknown(value) => value,
        }
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
impl Default for AccountRequirementsErrorCode {
    fn default() -> Self {
        AccountRequirementsErrorCode::Unknown(String::new())
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
impl<'json> json::Decode<'json> for AccountRequirementsErrorCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(AccountRequirementsErrorCode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "external_request" => AccountRequirementsErrorCode::ExternalRequest,
            "information_missing" => AccountRequirementsErrorCode::InformationMissing,
            "invalid_address_city_state_postal_code" => AccountRequirementsErrorCode::InvalidAddressCityStatePostalCode,
            "invalid_address_cmra_address" => AccountRequirementsErrorCode::InvalidAddressCmraAddress,
            "invalid_address_highway_contract_box" => AccountRequirementsErrorCode::InvalidAddressHighwayContractBox,
            "invalid_address_private_mailbox" => AccountRequirementsErrorCode::InvalidAddressPrivateMailbox,
            "invalid_address_registered_agent_address" => {
                AccountRequirementsErrorCode::InvalidAddressRegisteredAgentAddress
            }
            "invalid_business_profile_name" => AccountRequirementsErrorCode::InvalidBusinessProfileName,
            "invalid_business_profile_name_denylisted" => {
                AccountRequirementsErrorCode::InvalidBusinessProfileNameDenylisted
            }
            "invalid_company_name_denylisted" => AccountRequirementsErrorCode::InvalidCompanyNameDenylisted,
            "invalid_dob_age_over_maximum" => AccountRequirementsErrorCode::InvalidDobAgeOverMaximum,
            "invalid_dob_age_under_18" => AccountRequirementsErrorCode::InvalidDobAgeUnder18,
            "invalid_dob_age_under_minimum" => AccountRequirementsErrorCode::InvalidDobAgeUnderMinimum,
            "invalid_product_description_length" => AccountRequirementsErrorCode::InvalidProductDescriptionLength,
            "invalid_product_description_url_match" => AccountRequirementsErrorCode::InvalidProductDescriptionUrlMatch,
            "invalid_representative_country" => AccountRequirementsErrorCode::InvalidRepresentativeCountry,
            "invalid_signator" => AccountRequirementsErrorCode::InvalidSignator,
            "invalid_statement_descriptor_business_mismatch" => {
                AccountRequirementsErrorCode::InvalidStatementDescriptorBusinessMismatch
            }
            "invalid_statement_descriptor_denylisted" => {
                AccountRequirementsErrorCode::InvalidStatementDescriptorDenylisted
            }
            "invalid_statement_descriptor_length" => AccountRequirementsErrorCode::InvalidStatementDescriptorLength,
            "invalid_statement_descriptor_prefix_denylisted" => {
                AccountRequirementsErrorCode::InvalidStatementDescriptorPrefixDenylisted
            }
            "invalid_statement_descriptor_prefix_mismatch" => {
                AccountRequirementsErrorCode::InvalidStatementDescriptorPrefixMismatch
            }
            "invalid_street_address" => AccountRequirementsErrorCode::InvalidStreetAddress,
            "invalid_tax_id" => AccountRequirementsErrorCode::InvalidTaxId,
            "invalid_tax_id_format" => AccountRequirementsErrorCode::InvalidTaxIdFormat,
            "invalid_tos_acceptance" => AccountRequirementsErrorCode::InvalidTosAcceptance,
            "invalid_url_denylisted" => AccountRequirementsErrorCode::InvalidUrlDenylisted,
            "invalid_url_format" => AccountRequirementsErrorCode::InvalidUrlFormat,
            "invalid_url_length" => AccountRequirementsErrorCode::InvalidUrlLength,
            "invalid_url_web_presence_detected" => AccountRequirementsErrorCode::InvalidUrlWebPresenceDetected,
            "invalid_url_website_business_information_mismatch" => {
                AccountRequirementsErrorCode::InvalidUrlWebsiteBusinessInformationMismatch
            }
            "invalid_url_website_empty" => AccountRequirementsErrorCode::InvalidUrlWebsiteEmpty,
            "invalid_url_website_inaccessible" => AccountRequirementsErrorCode::InvalidUrlWebsiteInaccessible,
            "invalid_url_website_inaccessible_geoblocked" => {
                AccountRequirementsErrorCode::InvalidUrlWebsiteInaccessibleGeoblocked
            }
            "invalid_url_website_inaccessible_password_protected" => {
                AccountRequirementsErrorCode::InvalidUrlWebsiteInaccessiblePasswordProtected
            }
            "invalid_url_website_incomplete" => AccountRequirementsErrorCode::InvalidUrlWebsiteIncomplete,
            "invalid_url_website_incomplete_cancellation_policy" => {
                AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteCancellationPolicy
            }
            "invalid_url_website_incomplete_customer_service_details" => {
                AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteCustomerServiceDetails
            }
            "invalid_url_website_incomplete_legal_restrictions" => {
                AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteLegalRestrictions
            }
            "invalid_url_website_incomplete_refund_policy" => {
                AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteRefundPolicy
            }
            "invalid_url_website_incomplete_return_policy" => {
                AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteReturnPolicy
            }
            "invalid_url_website_incomplete_terms_and_conditions" => {
                AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteTermsAndConditions
            }
            "invalid_url_website_incomplete_under_construction" => {
                AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteUnderConstruction
            }
            "invalid_url_website_other" => AccountRequirementsErrorCode::InvalidUrlWebsiteOther,
            "invalid_value_other" => AccountRequirementsErrorCode::InvalidValueOther,
            "unsupported_business_type" => AccountRequirementsErrorCode::UnsupportedBusinessType,
            "verification_directors_mismatch" => AccountRequirementsErrorCode::VerificationDirectorsMismatch,
            "verification_document_address_mismatch" => {
                AccountRequirementsErrorCode::VerificationDocumentAddressMismatch
            }
            "verification_document_address_missing" => AccountRequirementsErrorCode::VerificationDocumentAddressMissing,
            "verification_document_corrupt" => AccountRequirementsErrorCode::VerificationDocumentCorrupt,
            "verification_document_country_not_supported" => {
                AccountRequirementsErrorCode::VerificationDocumentCountryNotSupported
            }
            "verification_document_directors_mismatch" => {
                AccountRequirementsErrorCode::VerificationDocumentDirectorsMismatch
            }
            "verification_document_dob_mismatch" => AccountRequirementsErrorCode::VerificationDocumentDobMismatch,
            "verification_document_duplicate_type" => AccountRequirementsErrorCode::VerificationDocumentDuplicateType,
            "verification_document_expired" => AccountRequirementsErrorCode::VerificationDocumentExpired,
            "verification_document_failed_copy" => AccountRequirementsErrorCode::VerificationDocumentFailedCopy,
            "verification_document_failed_greyscale" => {
                AccountRequirementsErrorCode::VerificationDocumentFailedGreyscale
            }
            "verification_document_failed_other" => AccountRequirementsErrorCode::VerificationDocumentFailedOther,
            "verification_document_failed_test_mode" => {
                AccountRequirementsErrorCode::VerificationDocumentFailedTestMode
            }
            "verification_document_fraudulent" => AccountRequirementsErrorCode::VerificationDocumentFraudulent,
            "verification_document_id_number_mismatch" => {
                AccountRequirementsErrorCode::VerificationDocumentIdNumberMismatch
            }
            "verification_document_id_number_missing" => {
                AccountRequirementsErrorCode::VerificationDocumentIdNumberMissing
            }
            "verification_document_incomplete" => AccountRequirementsErrorCode::VerificationDocumentIncomplete,
            "verification_document_invalid" => AccountRequirementsErrorCode::VerificationDocumentInvalid,
            "verification_document_issue_or_expiry_date_missing" => {
                AccountRequirementsErrorCode::VerificationDocumentIssueOrExpiryDateMissing
            }
            "verification_document_manipulated" => AccountRequirementsErrorCode::VerificationDocumentManipulated,
            "verification_document_missing_back" => AccountRequirementsErrorCode::VerificationDocumentMissingBack,
            "verification_document_missing_front" => AccountRequirementsErrorCode::VerificationDocumentMissingFront,
            "verification_document_name_mismatch" => AccountRequirementsErrorCode::VerificationDocumentNameMismatch,
            "verification_document_name_missing" => AccountRequirementsErrorCode::VerificationDocumentNameMissing,
            "verification_document_nationality_mismatch" => {
                AccountRequirementsErrorCode::VerificationDocumentNationalityMismatch
            }
            "verification_document_not_readable" => AccountRequirementsErrorCode::VerificationDocumentNotReadable,
            "verification_document_not_signed" => AccountRequirementsErrorCode::VerificationDocumentNotSigned,
            "verification_document_not_uploaded" => AccountRequirementsErrorCode::VerificationDocumentNotUploaded,
            "verification_document_photo_mismatch" => AccountRequirementsErrorCode::VerificationDocumentPhotoMismatch,
            "verification_document_too_large" => AccountRequirementsErrorCode::VerificationDocumentTooLarge,
            "verification_document_type_not_supported" => {
                AccountRequirementsErrorCode::VerificationDocumentTypeNotSupported
            }
            "verification_extraneous_directors" => AccountRequirementsErrorCode::VerificationExtraneousDirectors,
            "verification_failed_address_match" => AccountRequirementsErrorCode::VerificationFailedAddressMatch,
            "verification_failed_authorizer_authority" => {
                AccountRequirementsErrorCode::VerificationFailedAuthorizerAuthority
            }
            "verification_failed_business_iec_number" => {
                AccountRequirementsErrorCode::VerificationFailedBusinessIecNumber
            }
            "verification_failed_document_match" => AccountRequirementsErrorCode::VerificationFailedDocumentMatch,
            "verification_failed_id_number_match" => AccountRequirementsErrorCode::VerificationFailedIdNumberMatch,
            "verification_failed_keyed_identity" => AccountRequirementsErrorCode::VerificationFailedKeyedIdentity,
            "verification_failed_keyed_match" => AccountRequirementsErrorCode::VerificationFailedKeyedMatch,
            "verification_failed_name_match" => AccountRequirementsErrorCode::VerificationFailedNameMatch,
            "verification_failed_other" => AccountRequirementsErrorCode::VerificationFailedOther,
            "verification_failed_representative_authority" => {
                AccountRequirementsErrorCode::VerificationFailedRepresentativeAuthority
            }
            "verification_failed_residential_address" => {
                AccountRequirementsErrorCode::VerificationFailedResidentialAddress
            }
            "verification_failed_tax_id_match" => AccountRequirementsErrorCode::VerificationFailedTaxIdMatch,
            "verification_failed_tax_id_not_issued" => AccountRequirementsErrorCode::VerificationFailedTaxIdNotIssued,
            "verification_legal_entity_structure_mismatch" => {
                AccountRequirementsErrorCode::VerificationLegalEntityStructureMismatch
            }
            "verification_missing_directors" => AccountRequirementsErrorCode::VerificationMissingDirectors,
            "verification_missing_executives" => AccountRequirementsErrorCode::VerificationMissingExecutives,
            "verification_missing_owners" => AccountRequirementsErrorCode::VerificationMissingOwners,
            "verification_rejected_ownership_exemption_reason" => {
                AccountRequirementsErrorCode::VerificationRejectedOwnershipExemptionReason
            }
            "verification_requires_additional_memorandum_of_associations" => {
                AccountRequirementsErrorCode::VerificationRequiresAdditionalMemorandumOfAssociations
            }
            "verification_requires_additional_proof_of_registration" => {
                AccountRequirementsErrorCode::VerificationRequiresAdditionalProofOfRegistration
            }
            "verification_supportability" => AccountRequirementsErrorCode::VerificationSupportability,
            other => AccountRequirementsErrorCode::Unknown(other.to_string()),
        })
    }
}
