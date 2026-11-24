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
pub enum ConnectCollectionTransferObject {
    ConnectCollectionTransfer,
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
impl ConnectCollectionTransferObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ConnectCollectionTransferObject::ConnectCollectionTransfer => "connect_collection_transfer",
            ConnectCollectionTransferObject::Unknown(value) => value,
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
impl Default for ConnectCollectionTransferObject {
    fn default() -> Self {
        ConnectCollectionTransferObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for ConnectCollectionTransferObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ConnectCollectionTransferObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "connect_collection_transfer" => ConnectCollectionTransferObject::ConnectCollectionTransfer,
            other => ConnectCollectionTransferObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedAccountFeaturesClaim {
    #[doc = "Whether Stripe user authentication is disabled."]
    #[json(default)]
    pub disable_stripe_user_authentication: bool,
    #[doc = "Whether external account collection is enabled."]
    #[json(default)]
    pub external_account_collection: bool,
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedPaymentDisputesFeatures {
    #[doc = "Whether connected accounts can manage destination charges that are created on behalf of them."]
    #[json(default)]
    pub destination_on_behalf_of_charge_management: bool,
    #[doc = "Whether responding to disputes is enabled, including submitting evidence and accepting disputes."]
    #[json(default)]
    pub dispute_management: bool,
    #[doc = "Whether sending refunds is enabled."]
    #[json(default)]
    pub refund_management: bool,
    #[doc = "Whether to allow connected accounts to submit disputes using Smart Disputes."]
    #[json(default)]
    pub smart_disputes_management: bool,
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedPayoutsFeatures {
    #[doc = "Whether Stripe user authentication is disabled."]
    #[json(default)]
    pub disable_stripe_user_authentication: bool,
    #[doc = "Whether to allow payout schedule to be changed."]
    #[json(default)]
    pub edit_payout_schedule: bool,
    #[doc = "Whether external account collection is enabled."]
    #[json(default)]
    pub external_account_collection: bool,
    #[doc = "Whether to allow creation of instant payouts."]
    #[json(default)]
    pub instant_payouts: bool,
    #[doc = "Whether to allow creation of standard payouts."]
    #[json(default)]
    pub standard_payouts: bool,
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettingsBranding {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account."]
    pub icon: Option<String>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Check..."]
    pub logo: Option<String>,
    #[doc = "A CSS hex color value representing the primary branding color for this account."]
    pub primary_color: Option<String>,
    #[doc = "A CSS hex color value representing the secondary branding color for this account."]
    pub secondary_color: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsBranding {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("icon", self.icon.as_ref());
        form.field_opt("logo", self.logo.as_ref());
        form.field_opt("primary_color", self.primary_color.as_ref());
        form.field_opt("secondary_color", self.secondary_color.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettingsPayments {
    #[doc = "The default text that appears on statements for non-card charges outside of Japan."]
    pub statement_descriptor: Option<String>,
    #[doc = "The Kana variation of `statement_descriptor` used for charges in Japan."]
    pub statement_descriptor_kana: Option<String>,
    #[doc = "The Kanji variation of `statement_descriptor` used for charges in Japan."]
    pub statement_descriptor_kanji: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsPayments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
        form.field_opt("statement_descriptor_kana", self.statement_descriptor_kana.as_ref());
        form.field_opt("statement_descriptor_kanji", self.statement_descriptor_kanji.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountParamsSettingsPayoutsScheduleInterval {
    Daily,
    Manual,
    Monthly,
    Weekly,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl CreateAccountParamsSettingsPayoutsScheduleInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountParamsSettingsPayoutsScheduleInterval::Daily => "daily",
            CreateAccountParamsSettingsPayoutsScheduleInterval::Manual => "manual",
            CreateAccountParamsSettingsPayoutsScheduleInterval::Monthly => "monthly",
            CreateAccountParamsSettingsPayoutsScheduleInterval::Weekly => "weekly",
            CreateAccountParamsSettingsPayoutsScheduleInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for CreateAccountParamsSettingsPayoutsScheduleInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountParamsSettingsPayoutsScheduleInterval::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "daily" => CreateAccountParamsSettingsPayoutsScheduleInterval::Daily,
            "manual" => CreateAccountParamsSettingsPayoutsScheduleInterval::Manual,
            "monthly" => CreateAccountParamsSettingsPayoutsScheduleInterval::Monthly,
            "weekly" => CreateAccountParamsSettingsPayoutsScheduleInterval::Weekly,
            other => CreateAccountParamsSettingsPayoutsScheduleInterval::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsPayoutsScheduleInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsTosAcceptance {
    #[doc = "The Unix timestamp marking when the account representative accepted their service agreement."]
    pub date: Option<i64>,
    #[doc = "The IP address from which the account representative accepted their service agreement."]
    pub ip: Option<String>,
    #[doc = "The user's service agreement type."]
    pub service_agreement: Option<String>,
    #[doc = "The user agent of the browser from which the account representative accepted their service agreement."]
    pub user_agent: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsTosAcceptance {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("date", self.date.as_ref());
        form.field_opt("ip", self.ip.as_ref());
        form.field_opt("service_agreement", self.service_agreement.as_ref());
        form.field_opt("user_agent", self.user_agent.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountPersonParamsDocumentsCompanyAuthorization {
    #[doc = "One or more document ids returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value ..."]
    pub files: Option<Vec<json::OwnedValue>>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountPersonParamsDocumentsCompanyAuthorization {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountPersonParamsRelationship {
    #[doc = "Whether the person is the authorizer of the account's representative."]
    pub authorizer: Option<bool>,
    #[doc = "Whether the person is a director of the account's legal entity."]
    pub director: Option<bool>,
    #[doc = "Whether the person has significant responsibility to control, manage, or direct the organization."]
    pub executive: Option<bool>,
    #[doc = "Whether the person is the legal guardian of the account's representative."]
    pub legal_guardian: Option<bool>,
    #[doc = "Whether the person is an owner of the account’s legal entity."]
    pub owner: Option<bool>,
    #[doc = "The percent owned by the person of the account's legal entity."]
    pub percent_ownership: Option<json::OwnedValue>,
    #[doc = "Whether the person is authorized as the primary representative of the account."]
    pub representative: Option<bool>,
    #[doc = "The person's title (e.g., CEO, Support Engineer)."]
    pub title: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountPersonParamsRelationship {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("authorizer", self.authorizer.as_ref());
        form.field_opt("director", self.director.as_ref());
        form.field_opt("executive", self.executive.as_ref());
        form.field_opt("legal_guardian", self.legal_guardian.as_ref());
        form.field_opt("owner", self.owner.as_ref());
        form.field_opt("representative", self.representative.as_ref());
        form.field_opt("title", self.title.as_ref());
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
#[derive(Debug, Clone, PartialEq)]
pub enum LegalEntityCompanyOwnershipExemptionReason {
    QualifiedEntityExceedsOwnershipThreshold,
    QualifiesAsFinancialInstitution,
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
impl LegalEntityCompanyOwnershipExemptionReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            LegalEntityCompanyOwnershipExemptionReason::QualifiedEntityExceedsOwnershipThreshold => {
                "qualified_entity_exceeds_ownership_threshold"
            }
            LegalEntityCompanyOwnershipExemptionReason::QualifiesAsFinancialInstitution => {
                "qualifies_as_financial_institution"
            }
            LegalEntityCompanyOwnershipExemptionReason::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for LegalEntityCompanyOwnershipExemptionReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(LegalEntityCompanyOwnershipExemptionReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "qualified_entity_exceeds_ownership_threshold" => {
                LegalEntityCompanyOwnershipExemptionReason::QualifiedEntityExceedsOwnershipThreshold
            }
            "qualifies_as_financial_institution" => {
                LegalEntityCompanyOwnershipExemptionReason::QualifiesAsFinancialInstitution
            }
            other => LegalEntityCompanyOwnershipExemptionReason::Unknown(other.to_string()),
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
#[derive(Debug, Clone, PartialEq)]
pub enum LegalEntityCompanyStructure {
    FreeZoneEstablishment,
    FreeZoneLlc,
    GovernmentInstrumentality,
    GovernmentalUnit,
    IncorporatedNonProfit,
    IncorporatedPartnership,
    LimitedLiabilityPartnership,
    Llc,
    MultiMemberLlc,
    PrivateCompany,
    PrivateCorporation,
    PrivatePartnership,
    PublicCompany,
    PublicCorporation,
    PublicPartnership,
    RegisteredCharity,
    SingleMemberLlc,
    SoleEstablishment,
    SoleProprietorship,
    TaxExemptGovernmentInstrumentality,
    UnincorporatedAssociation,
    UnincorporatedNonProfit,
    UnincorporatedPartnership,
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
impl LegalEntityCompanyStructure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            LegalEntityCompanyStructure::FreeZoneEstablishment => "free_zone_establishment",
            LegalEntityCompanyStructure::FreeZoneLlc => "free_zone_llc",
            LegalEntityCompanyStructure::GovernmentInstrumentality => "government_instrumentality",
            LegalEntityCompanyStructure::GovernmentalUnit => "governmental_unit",
            LegalEntityCompanyStructure::IncorporatedNonProfit => "incorporated_non_profit",
            LegalEntityCompanyStructure::IncorporatedPartnership => "incorporated_partnership",
            LegalEntityCompanyStructure::LimitedLiabilityPartnership => "limited_liability_partnership",
            LegalEntityCompanyStructure::Llc => "llc",
            LegalEntityCompanyStructure::MultiMemberLlc => "multi_member_llc",
            LegalEntityCompanyStructure::PrivateCompany => "private_company",
            LegalEntityCompanyStructure::PrivateCorporation => "private_corporation",
            LegalEntityCompanyStructure::PrivatePartnership => "private_partnership",
            LegalEntityCompanyStructure::PublicCompany => "public_company",
            LegalEntityCompanyStructure::PublicCorporation => "public_corporation",
            LegalEntityCompanyStructure::PublicPartnership => "public_partnership",
            LegalEntityCompanyStructure::RegisteredCharity => "registered_charity",
            LegalEntityCompanyStructure::SingleMemberLlc => "single_member_llc",
            LegalEntityCompanyStructure::SoleEstablishment => "sole_establishment",
            LegalEntityCompanyStructure::SoleProprietorship => "sole_proprietorship",
            LegalEntityCompanyStructure::TaxExemptGovernmentInstrumentality => "tax_exempt_government_instrumentality",
            LegalEntityCompanyStructure::UnincorporatedAssociation => "unincorporated_association",
            LegalEntityCompanyStructure::UnincorporatedNonProfit => "unincorporated_non_profit",
            LegalEntityCompanyStructure::UnincorporatedPartnership => "unincorporated_partnership",
            LegalEntityCompanyStructure::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for LegalEntityCompanyStructure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(LegalEntityCompanyStructure::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "free_zone_establishment" => LegalEntityCompanyStructure::FreeZoneEstablishment,
            "free_zone_llc" => LegalEntityCompanyStructure::FreeZoneLlc,
            "government_instrumentality" => LegalEntityCompanyStructure::GovernmentInstrumentality,
            "governmental_unit" => LegalEntityCompanyStructure::GovernmentalUnit,
            "incorporated_non_profit" => LegalEntityCompanyStructure::IncorporatedNonProfit,
            "incorporated_partnership" => LegalEntityCompanyStructure::IncorporatedPartnership,
            "limited_liability_partnership" => LegalEntityCompanyStructure::LimitedLiabilityPartnership,
            "llc" => LegalEntityCompanyStructure::Llc,
            "multi_member_llc" => LegalEntityCompanyStructure::MultiMemberLlc,
            "private_company" => LegalEntityCompanyStructure::PrivateCompany,
            "private_corporation" => LegalEntityCompanyStructure::PrivateCorporation,
            "private_partnership" => LegalEntityCompanyStructure::PrivatePartnership,
            "public_company" => LegalEntityCompanyStructure::PublicCompany,
            "public_corporation" => LegalEntityCompanyStructure::PublicCorporation,
            "public_partnership" => LegalEntityCompanyStructure::PublicPartnership,
            "registered_charity" => LegalEntityCompanyStructure::RegisteredCharity,
            "single_member_llc" => LegalEntityCompanyStructure::SingleMemberLlc,
            "sole_establishment" => LegalEntityCompanyStructure::SoleEstablishment,
            "sole_proprietorship" => LegalEntityCompanyStructure::SoleProprietorship,
            "tax_exempt_government_instrumentality" => LegalEntityCompanyStructure::TaxExemptGovernmentInstrumentality,
            "unincorporated_association" => LegalEntityCompanyStructure::UnincorporatedAssociation,
            "unincorporated_non_profit" => LegalEntityCompanyStructure::UnincorporatedNonProfit,
            "unincorporated_partnership" => LegalEntityCompanyStructure::UnincorporatedPartnership,
            other => LegalEntityCompanyStructure::Unknown(other.to_string()),
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
#[derive(Debug, Clone, PartialEq)]
pub enum PersonPoliticalExposure {
    Existing,
    None,
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
impl PersonPoliticalExposure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PersonPoliticalExposure::Existing => "existing",
            PersonPoliticalExposure::None => "none",
            PersonPoliticalExposure::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PersonPoliticalExposure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PersonPoliticalExposure::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "existing" => PersonPoliticalExposure::Existing,
            "none" => PersonPoliticalExposure::None,
            other => PersonPoliticalExposure::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountCapabilityParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "To request a new capability for an account, pass true."]
    pub requested: Option<bool>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountCapabilityParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("requested", self.requested.as_ref());
    }
}
