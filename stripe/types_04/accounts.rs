#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedFinancialAccountTransactionsFeatures {
    #[doc = "Whether to allow card spend dispute management features."]
    #[json(default)]
    pub card_spend_dispute_management: bool,
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedInstantPayoutsPromotionFeatures {
    #[doc = "Whether Stripe user authentication is disabled."]
    #[json(default)]
    pub disable_stripe_user_authentication: bool,
    #[doc = "Whether external account collection is enabled."]
    #[json(default)]
    pub external_account_collection: bool,
    #[doc = "Whether to allow creation of instant payouts."]
    #[json(default)]
    pub instant_payouts: bool,
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedIssuingCardsListFeatures {
    #[doc = "Whether to allow card management features."]
    #[json(default)]
    pub card_management: bool,
    #[doc = "Whether to allow card spend dispute management features."]
    #[json(default)]
    pub card_spend_dispute_management: bool,
    #[doc = "Whether to allow cardholder management features."]
    #[json(default)]
    pub cardholder_management: bool,
    #[doc = "Whether Stripe user authentication is disabled."]
    #[json(default)]
    pub disable_stripe_user_authentication: bool,
    #[doc = "Whether to allow spend control management features."]
    #[json(default)]
    pub spend_control_management: bool,
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedPaymentMethodSettingsFeatures {
    #[doc = "Whether Stripe user authentication is disabled."]
    #[json(default)]
    pub disable_stripe_user_authentication: bool,
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsCapabilitiesAcssDebitPayments {
    #[doc = "Passing true requests the capability for the account, if it is not already requested."]
    pub requested: Option<bool>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsCapabilitiesAcssDebitPayments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("requested", self.requested.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsCompanyAddressKana {
    #[doc = "City or ward."]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: Option<String>,
    #[doc = "Block or building number."]
    pub line1: Option<String>,
    #[doc = "Building details."]
    pub line2: Option<String>,
    #[doc = "Postal code."]
    pub postal_code: Option<String>,
    #[doc = "Prefecture."]
    pub state: Option<String>,
    #[doc = "Town or cho-me."]
    pub town: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsCompanyAddressKana {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("line1", self.line1.as_ref());
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
        form.field_opt("town", self.town.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsCompanyDirectorshipDeclaration {
    #[doc = "The Unix timestamp marking when the directorship declaration attestation was made."]
    pub date: Option<i64>,
    #[doc = "The IP address from which the directorship declaration attestation was made."]
    pub ip: Option<String>,
    #[doc = "The user agent of the browser from which the directorship declaration attestation was made."]
    pub user_agent: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsCompanyDirectorshipDeclaration {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("date", self.date.as_ref());
        form.field_opt("ip", self.ip.as_ref());
        form.field_opt("user_agent", self.user_agent.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountParamsCompanyStructure {
    Unnamed,
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

#[cfg(feature = "accounts")]
impl CreateAccountParamsCompanyStructure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountParamsCompanyStructure::Unnamed => "",
            CreateAccountParamsCompanyStructure::FreeZoneEstablishment => "free_zone_establishment",
            CreateAccountParamsCompanyStructure::FreeZoneLlc => "free_zone_llc",
            CreateAccountParamsCompanyStructure::GovernmentInstrumentality => "government_instrumentality",
            CreateAccountParamsCompanyStructure::GovernmentalUnit => "governmental_unit",
            CreateAccountParamsCompanyStructure::IncorporatedNonProfit => "incorporated_non_profit",
            CreateAccountParamsCompanyStructure::IncorporatedPartnership => "incorporated_partnership",
            CreateAccountParamsCompanyStructure::LimitedLiabilityPartnership => "limited_liability_partnership",
            CreateAccountParamsCompanyStructure::Llc => "llc",
            CreateAccountParamsCompanyStructure::MultiMemberLlc => "multi_member_llc",
            CreateAccountParamsCompanyStructure::PrivateCompany => "private_company",
            CreateAccountParamsCompanyStructure::PrivateCorporation => "private_corporation",
            CreateAccountParamsCompanyStructure::PrivatePartnership => "private_partnership",
            CreateAccountParamsCompanyStructure::PublicCompany => "public_company",
            CreateAccountParamsCompanyStructure::PublicCorporation => "public_corporation",
            CreateAccountParamsCompanyStructure::PublicPartnership => "public_partnership",
            CreateAccountParamsCompanyStructure::RegisteredCharity => "registered_charity",
            CreateAccountParamsCompanyStructure::SingleMemberLlc => "single_member_llc",
            CreateAccountParamsCompanyStructure::SoleEstablishment => "sole_establishment",
            CreateAccountParamsCompanyStructure::SoleProprietorship => "sole_proprietorship",
            CreateAccountParamsCompanyStructure::TaxExemptGovernmentInstrumentality => {
                "tax_exempt_government_instrumentality"
            }
            CreateAccountParamsCompanyStructure::UnincorporatedAssociation => "unincorporated_association",
            CreateAccountParamsCompanyStructure::UnincorporatedNonProfit => "unincorporated_non_profit",
            CreateAccountParamsCompanyStructure::UnincorporatedPartnership => "unincorporated_partnership",
            CreateAccountParamsCompanyStructure::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for CreateAccountParamsCompanyStructure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountParamsCompanyStructure::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateAccountParamsCompanyStructure::Unnamed,
            "free_zone_establishment" => CreateAccountParamsCompanyStructure::FreeZoneEstablishment,
            "free_zone_llc" => CreateAccountParamsCompanyStructure::FreeZoneLlc,
            "government_instrumentality" => CreateAccountParamsCompanyStructure::GovernmentInstrumentality,
            "governmental_unit" => CreateAccountParamsCompanyStructure::GovernmentalUnit,
            "incorporated_non_profit" => CreateAccountParamsCompanyStructure::IncorporatedNonProfit,
            "incorporated_partnership" => CreateAccountParamsCompanyStructure::IncorporatedPartnership,
            "limited_liability_partnership" => CreateAccountParamsCompanyStructure::LimitedLiabilityPartnership,
            "llc" => CreateAccountParamsCompanyStructure::Llc,
            "multi_member_llc" => CreateAccountParamsCompanyStructure::MultiMemberLlc,
            "private_company" => CreateAccountParamsCompanyStructure::PrivateCompany,
            "private_corporation" => CreateAccountParamsCompanyStructure::PrivateCorporation,
            "private_partnership" => CreateAccountParamsCompanyStructure::PrivatePartnership,
            "public_company" => CreateAccountParamsCompanyStructure::PublicCompany,
            "public_corporation" => CreateAccountParamsCompanyStructure::PublicCorporation,
            "public_partnership" => CreateAccountParamsCompanyStructure::PublicPartnership,
            "registered_charity" => CreateAccountParamsCompanyStructure::RegisteredCharity,
            "single_member_llc" => CreateAccountParamsCompanyStructure::SingleMemberLlc,
            "sole_establishment" => CreateAccountParamsCompanyStructure::SoleEstablishment,
            "sole_proprietorship" => CreateAccountParamsCompanyStructure::SoleProprietorship,
            "tax_exempt_government_instrumentality" => {
                CreateAccountParamsCompanyStructure::TaxExemptGovernmentInstrumentality
            }
            "unincorporated_association" => CreateAccountParamsCompanyStructure::UnincorporatedAssociation,
            "unincorporated_non_profit" => CreateAccountParamsCompanyStructure::UnincorporatedNonProfit,
            "unincorporated_partnership" => CreateAccountParamsCompanyStructure::UnincorporatedPartnership,
            other => CreateAccountParamsCompanyStructure::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsCompanyStructure {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountParamsControllerLossesPayments {
    Application,
    Stripe,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl CreateAccountParamsControllerLossesPayments {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountParamsControllerLossesPayments::Application => "application",
            CreateAccountParamsControllerLossesPayments::Stripe => "stripe",
            CreateAccountParamsControllerLossesPayments::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for CreateAccountParamsControllerLossesPayments {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountParamsControllerLossesPayments::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "application" => CreateAccountParamsControllerLossesPayments::Application,
            "stripe" => CreateAccountParamsControllerLossesPayments::Stripe,
            other => CreateAccountParamsControllerLossesPayments::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsControllerLossesPayments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsDocumentsProofOfUltimateBeneficialOwnershipSigner {
    #[doc = "The token of the person signing the document, if applicable."]
    pub person: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsDocumentsProofOfUltimateBeneficialOwnershipSigner {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("person", self.person.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsGroups {
    #[doc = "The group the account is in to determine their payments pricing, and null if the account is on customized pricing."]
    pub payments_pricing: Option<json::OwnedValue>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsGroups {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettingsPaypayPaymentsSiteInDevelopment {
    #[doc = "The password needed to access your business's website."]
    pub password: String,
    #[doc = "The username needed to access your business's website."]
    pub username: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsPaypayPaymentsSiteInDevelopment {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("password", &self.password);
        form.field_opt("username", self.username.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettingsPaypayPaymentsSiteRestricted {
    #[doc = "The file explaining the payment flow for your business."]
    pub payment_flow_file: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsPaypayPaymentsSiteRestricted {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("payment_flow_file", self.payment_flow_file.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem {
    Cuban,
    HispanicOrLatino,
    Mexican,
    NotHispanicOrLatino,
    OtherHispanicOrLatino,
    PreferNotToAnswer,
    PuertoRican,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem::Cuban => "cuban",
            CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem::HispanicOrLatino => "hispanic_or_latino",
            CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem::Mexican => "mexican",
            CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem::NotHispanicOrLatino => {
                "not_hispanic_or_latino"
            }
            CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem::OtherHispanicOrLatino => {
                "other_hispanic_or_latino"
            }
            CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem::PreferNotToAnswer => {
                "prefer_not_to_answer"
            }
            CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem::PuertoRican => "puerto_rican",
            CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cuban" => CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem::Cuban,
            "hispanic_or_latino" => CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem::HispanicOrLatino,
            "mexican" => CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem::Mexican,
            "not_hispanic_or_latino" => {
                CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem::NotHispanicOrLatino
            }
            "other_hispanic_or_latino" => {
                CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem::OtherHispanicOrLatino
            }
            "prefer_not_to_answer" => {
                CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem::PreferNotToAnswer
            }
            "puerto_rican" => CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem::PuertoRican,
            other => CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
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
pub struct LegalEntityDirectorshipDeclaration {
    #[doc = "The Unix timestamp marking when the directorship declaration attestation was made."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<i64>,
    #[doc = "The IP address from which the directorship declaration attestation was made."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "The user-agent string from the browser where the directorship declaration attestation was made."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
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
pub struct LegalEntityDob {
    #[doc = "The day of birth, between 1 and 31."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<i64>,
    #[doc = "The month of birth, between 1 and 12."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,
    #[doc = "The four-digit year of birth."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
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
pub struct LegalEntityJapanAddress {
    #[doc = "City/Ward."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Block/Building number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[doc = "Building details."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "Prefecture."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Town/cho-me."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
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
pub struct LegalEntityRegistrationDate {
    #[doc = "The day of registration, between 1 and 31."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<i64>,
    #[doc = "The month of registration, between 1 and 12."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,
    #[doc = "The four-digit year of registration."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
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
pub struct LegalEntityRepresentativeDeclaration {
    #[doc = "The Unix timestamp marking when the representative declaration attestation was made."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<i64>,
    #[doc = "The IP address from which the representative declaration attestation was made."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "The user-agent string from the browser where the representative declaration attestation was made."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
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
pub struct LegalEntityUboDeclaration {
    #[doc = "The Unix timestamp marking when the beneficial owner attestation was made."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<i64>,
    #[doc = "The IP address from which the beneficial owner attestation was made."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "The user-agent string from the browser where the beneficial owner attestation was made."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListAccountExternalAccountsParamsObject {
    BankAccount,
    Card,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl ListAccountExternalAccountsParamsObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListAccountExternalAccountsParamsObject::BankAccount => "bank_account",
            ListAccountExternalAccountsParamsObject::Card => "card",
            ListAccountExternalAccountsParamsObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for ListAccountExternalAccountsParamsObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListAccountExternalAccountsParamsObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "bank_account" => ListAccountExternalAccountsParamsObject::BankAccount,
            "card" => ListAccountExternalAccountsParamsObject::Card,
            other => ListAccountExternalAccountsParamsObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for ListAccountExternalAccountsParamsObject {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct ListAccountsParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for ListAccountsParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum RejectAccountParamsReason {
    Credit,
    FraudNoIntentToFulfill,
    FraudOther,
    FraudPaymentMethodCasher,
    FraudPaymentMethodTester,
    Other,
    TermsOfService,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl RejectAccountParamsReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            RejectAccountParamsReason::Credit => "credit",
            RejectAccountParamsReason::FraudNoIntentToFulfill => "fraud_no_intent_to_fulfill",
            RejectAccountParamsReason::FraudOther => "fraud_other",
            RejectAccountParamsReason::FraudPaymentMethodCasher => "fraud_payment_method_casher",
            RejectAccountParamsReason::FraudPaymentMethodTester => "fraud_payment_method_tester",
            RejectAccountParamsReason::Other => "other",
            RejectAccountParamsReason::TermsOfService => "terms_of_service",
            RejectAccountParamsReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for RejectAccountParamsReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(RejectAccountParamsReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "credit" => RejectAccountParamsReason::Credit,
            "fraud_no_intent_to_fulfill" => RejectAccountParamsReason::FraudNoIntentToFulfill,
            "fraud_other" => RejectAccountParamsReason::FraudOther,
            "fraud_payment_method_casher" => RejectAccountParamsReason::FraudPaymentMethodCasher,
            "fraud_payment_method_tester" => RejectAccountParamsReason::FraudPaymentMethodTester,
            "other" => RejectAccountParamsReason::Other,
            "terms_of_service" => RejectAccountParamsReason::TermsOfService,
            other => RejectAccountParamsReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for RejectAccountParamsReason {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
