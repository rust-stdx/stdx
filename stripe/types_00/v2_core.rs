#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax {
    #[doc = "To request a new Capability for an account, pass true."]
    pub requested: bool,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationMerchantKonbiniPaymentsSupportHours {
    #[doc = "Support hours end time (JST time of day) for in `HH:MM` format."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "Support hours start time (JST time of day) for in `HH:MM` format."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationMerchantSupportAddress {
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
    #[doc = "State, county, province, or region."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Town or district."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountParamsDefaultsResponsibilitiesFeesCollector {
    Application,
    ApplicationCustom,
    ApplicationExpress,
    Stripe,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountParamsDefaultsResponsibilitiesFeesCollector {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountParamsDefaultsResponsibilitiesFeesCollector::Application => "application",
            CreateCoreAccountParamsDefaultsResponsibilitiesFeesCollector::ApplicationCustom => "application_custom",
            CreateCoreAccountParamsDefaultsResponsibilitiesFeesCollector::ApplicationExpress => "application_express",
            CreateCoreAccountParamsDefaultsResponsibilitiesFeesCollector::Stripe => "stripe",
            CreateCoreAccountParamsDefaultsResponsibilitiesFeesCollector::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountParamsDefaultsResponsibilitiesFeesCollector {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreAccountParamsDefaultsResponsibilitiesFeesCollector {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCoreAccountParamsDefaultsResponsibilitiesFeesCollector::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "application" => CreateCoreAccountParamsDefaultsResponsibilitiesFeesCollector::Application,
            "application_custom" => CreateCoreAccountParamsDefaultsResponsibilitiesFeesCollector::ApplicationCustom,
            "application_express" => CreateCoreAccountParamsDefaultsResponsibilitiesFeesCollector::ApplicationExpress,
            "stripe" => CreateCoreAccountParamsDefaultsResponsibilitiesFeesCollector::Stripe,
            other => CreateCoreAccountParamsDefaultsResponsibilitiesFeesCollector::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityAttestationsDirectorshipDeclaration {
    #[doc = "The time marking when the director attestation was made."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[doc = "The IP address from which the director attestation was made."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "The user agent of the browser from which the director attestation was made."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonTokenParamsRelationship {
    #[doc = "Whether the individual is an authorizer of the Account's identity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub authorizer: Option<bool>,
    #[doc = "Indicates whether the person is a director of the associated legal entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,
    #[doc = "Indicates whether the person is an executive of the associated legal entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,
    #[doc = "Indicates whether the person is a legal guardian of the associated legal entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub legal_guardian: Option<bool>,
    #[doc = "Indicates whether the person is an owner of the associated legal entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    #[doc = "The percentage of ownership the person has in the associated legal entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub percent_ownership: Option<String>,
    #[doc = "Indicates whether the person is a representative of the associated legal entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub representative: Option<bool>,
    #[doc = "The title or position the person holds in the associated legal entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityAttestationsDirectorshipDeclaration {
    #[doc = "A boolean indicating if the directors information has been attested."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub attested: Option<bool>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsAddress {
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
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsProofOfRegistrationSigner {
    #[doc = "Person signing the document."]
    pub person: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsScriptNamesKana {
    #[doc = "Registered name of the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub registered_name: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure {
    Cooperative,
    FreeZoneEstablishment,
    FreeZoneLlc,
    GovernmentalUnit,
    GovernmentInstrumentality,
    IncorporatedAssociation,
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
    PublicListedCorporation,
    PublicPartnership,
    RegisteredCharity,
    SingleMemberLlc,
    SoleEstablishment,
    SoleProprietorship,
    TaxExemptGovernmentInstrumentality,
    Trust,
    UnincorporatedAssociation,
    UnincorporatedNonProfit,
    UnincorporatedPartnership,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::Cooperative => "cooperative",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::FreeZoneEstablishment => {
                "free_zone_establishment"
            }
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::FreeZoneLlc => "free_zone_llc",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::GovernmentalUnit => "governmental_unit",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::GovernmentInstrumentality => {
                "government_instrumentality"
            }
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::IncorporatedAssociation => {
                "incorporated_association"
            }
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::IncorporatedNonProfit => {
                "incorporated_non_profit"
            }
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::IncorporatedPartnership => {
                "incorporated_partnership"
            }
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::LimitedLiabilityPartnership => {
                "limited_liability_partnership"
            }
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::Llc => "llc",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::MultiMemberLlc => "multi_member_llc",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::PrivateCompany => "private_company",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::PrivateCorporation => "private_corporation",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::PrivatePartnership => "private_partnership",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::PublicCompany => "public_company",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::PublicCorporation => "public_corporation",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::PublicListedCorporation => {
                "public_listed_corporation"
            }
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::PublicPartnership => "public_partnership",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::RegisteredCharity => "registered_charity",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::SingleMemberLlc => "single_member_llc",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::SoleEstablishment => "sole_establishment",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::SoleProprietorship => "sole_proprietorship",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::TaxExemptGovernmentInstrumentality => {
                "tax_exempt_government_instrumentality"
            }
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::Trust => "trust",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::UnincorporatedAssociation => {
                "unincorporated_association"
            }
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::UnincorporatedNonProfit => {
                "unincorporated_non_profit"
            }
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::UnincorporatedPartnership => {
                "unincorporated_partnership"
            }
            CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cooperative" => CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::Cooperative,
            "free_zone_establishment" => {
                CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::FreeZoneEstablishment
            }
            "free_zone_llc" => CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::FreeZoneLlc,
            "governmental_unit" => CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::GovernmentalUnit,
            "government_instrumentality" => {
                CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::GovernmentInstrumentality
            }
            "incorporated_association" => {
                CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::IncorporatedAssociation
            }
            "incorporated_non_profit" => {
                CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::IncorporatedNonProfit
            }
            "incorporated_partnership" => {
                CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::IncorporatedPartnership
            }
            "limited_liability_partnership" => {
                CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::LimitedLiabilityPartnership
            }
            "llc" => CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::Llc,
            "multi_member_llc" => CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::MultiMemberLlc,
            "private_company" => CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::PrivateCompany,
            "private_corporation" => CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::PrivateCorporation,
            "private_partnership" => CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::PrivatePartnership,
            "public_company" => CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::PublicCompany,
            "public_corporation" => CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::PublicCorporation,
            "public_listed_corporation" => {
                CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::PublicListedCorporation
            }
            "public_partnership" => CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::PublicPartnership,
            "registered_charity" => CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::RegisteredCharity,
            "single_member_llc" => CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::SingleMemberLlc,
            "sole_establishment" => CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::SoleEstablishment,
            "sole_proprietorship" => CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::SoleProprietorship,
            "tax_exempt_government_instrumentality" => {
                CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::TaxExemptGovernmentInstrumentality
            }
            "trust" => CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::Trust,
            "unincorporated_association" => {
                CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::UnincorporatedAssociation
            }
            "unincorporated_non_profit" => {
                CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::UnincorporatedNonProfit
            }
            "unincorporated_partnership" => {
                CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::UnincorporatedPartnership
            }
            other => CreateCoreAccountTokenParamsIdentityBusinessDetailsStructure::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityIndividualScriptNamesKana {
    #[doc = "The person's first or given name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The person's last or family name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreEventDestinationParamsAmazonEventbridge {
    #[doc = "Your AWS account where Stripe creates the partner event source."]
    pub aws_account_id: String,
    #[doc = "The AWS region where Stripe creates the partner event source."]
    pub aws_region: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListCoreAccountsParamsAppliedConfigurationsItem {
    Customer,
    Merchant,
    Recipient,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl ListCoreAccountsParamsAppliedConfigurationsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListCoreAccountsParamsAppliedConfigurationsItem::Customer => "customer",
            ListCoreAccountsParamsAppliedConfigurationsItem::Merchant => "merchant",
            ListCoreAccountsParamsAppliedConfigurationsItem::Recipient => "recipient",
            ListCoreAccountsParamsAppliedConfigurationsItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for ListCoreAccountsParamsAppliedConfigurationsItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListCoreAccountsParamsAppliedConfigurationsItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer" => ListCoreAccountsParamsAppliedConfigurationsItem::Customer,
            "merchant" => ListCoreAccountsParamsAppliedConfigurationsItem::Merchant,
            "recipient" => ListCoreAccountsParamsAppliedConfigurationsItem::Recipient,
            other => ListCoreAccountsParamsAppliedConfigurationsItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
impl crate::params::FormEncode for ListCoreAccountsParamsAppliedConfigurationsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateCoreEventDestinationParamsIncludeItem {
    WebhookEndpointUrl,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl UpdateCoreEventDestinationParamsIncludeItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            UpdateCoreEventDestinationParamsIncludeItem::WebhookEndpointUrl => "webhook_endpoint.url",
            UpdateCoreEventDestinationParamsIncludeItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for UpdateCoreEventDestinationParamsIncludeItem {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for UpdateCoreEventDestinationParamsIncludeItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(UpdateCoreEventDestinationParamsIncludeItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "webhook_endpoint.url" => UpdateCoreEventDestinationParamsIncludeItem::WebhookEndpointUrl,
            other => UpdateCoreEventDestinationParamsIncludeItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountConfigurationCustomerAutomaticIndirectTaxLocationSource {
    IdentityAddress,
    IpAddress,
    PaymentMethod,
    ShippingAddress,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountConfigurationCustomerAutomaticIndirectTaxLocationSource {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountConfigurationCustomerAutomaticIndirectTaxLocationSource::IdentityAddress => "identity_address",
            V2CoreAccountConfigurationCustomerAutomaticIndirectTaxLocationSource::IpAddress => "ip_address",
            V2CoreAccountConfigurationCustomerAutomaticIndirectTaxLocationSource::PaymentMethod => "payment_method",
            V2CoreAccountConfigurationCustomerAutomaticIndirectTaxLocationSource::ShippingAddress => "shipping_address",
            V2CoreAccountConfigurationCustomerAutomaticIndirectTaxLocationSource::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountConfigurationCustomerAutomaticIndirectTaxLocationSource {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountConfigurationCustomerAutomaticIndirectTaxLocationSource::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "identity_address" => V2CoreAccountConfigurationCustomerAutomaticIndirectTaxLocationSource::IdentityAddress,
            "ip_address" => V2CoreAccountConfigurationCustomerAutomaticIndirectTaxLocationSource::IpAddress,
            "payment_method" => V2CoreAccountConfigurationCustomerAutomaticIndirectTaxLocationSource::PaymentMethod,
            "shipping_address" => V2CoreAccountConfigurationCustomerAutomaticIndirectTaxLocationSource::ShippingAddress,
            other => V2CoreAccountConfigurationCustomerAutomaticIndirectTaxLocationSource::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationCustomerBillingInvoiceCustomFieldsItem {
    #[doc = "The name of the custom field."]
    #[json(default)]
    pub name: String,
    #[doc = "The value of the custom field."]
    #[json(default)]
    pub value: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationMerchantBacsDebitPayments {
    #[doc = "Display name for Bacs Direct Debit payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Service User Number (SUN) for Bacs Direct Debit payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub service_user_number: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationMerchantScriptStatementDescriptorKana {
    #[doc = "The default text that appears on statements for non-card charges outside of Japan."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    #[doc = "Default text that appears on statements for card charges outside of Japan, prefixing any dynamic statement_descriptor..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountIdentityAttestationsPersonsProvidedOwnershipExemptionReason {
    QualifiedEntityExceedsOwnershipThreshold,
    QualifiesAsFinancialInstitution,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountIdentityAttestationsPersonsProvidedOwnershipExemptionReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountIdentityAttestationsPersonsProvidedOwnershipExemptionReason::QualifiedEntityExceedsOwnershipThreshold => "qualified_entity_exceeds_ownership_threshold",
            V2CoreAccountIdentityAttestationsPersonsProvidedOwnershipExemptionReason::QualifiesAsFinancialInstitution => "qualifies_as_financial_institution",
            V2CoreAccountIdentityAttestationsPersonsProvidedOwnershipExemptionReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountIdentityAttestationsPersonsProvidedOwnershipExemptionReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                V2CoreAccountIdentityAttestationsPersonsProvidedOwnershipExemptionReason::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "qualified_entity_exceeds_ownership_threshold" => V2CoreAccountIdentityAttestationsPersonsProvidedOwnershipExemptionReason::QualifiedEntityExceedsOwnershipThreshold,
            "qualifies_as_financial_institution" => V2CoreAccountIdentityAttestationsPersonsProvidedOwnershipExemptionReason::QualifiesAsFinancialInstitution,
            other => V2CoreAccountIdentityAttestationsPersonsProvidedOwnershipExemptionReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType {
    Files,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType::Files => "files",
            V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType {
    fn default() -> Self {
        V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "files" => V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType::Files,
            other => V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountIdentityIndividualPoliticalExposure {
    Existing,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountIdentityIndividualPoliticalExposure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountIdentityIndividualPoliticalExposure::Existing => "existing",
            V2CoreAccountIdentityIndividualPoliticalExposure::None => "none",
            V2CoreAccountIdentityIndividualPoliticalExposure::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountIdentityIndividualPoliticalExposure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountIdentityIndividualPoliticalExposure::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "existing" => V2CoreAccountIdentityIndividualPoliticalExposure::Existing,
            "none" => V2CoreAccountIdentityIndividualPoliticalExposure::None,
            other => V2CoreAccountIdentityIndividualPoliticalExposure::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreEventDestinationEventPayload {
    Snapshot,
    Thin,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreEventDestinationEventPayload {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreEventDestinationEventPayload::Snapshot => "snapshot",
            V2CoreEventDestinationEventPayload::Thin => "thin",
            V2CoreEventDestinationEventPayload::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreEventDestinationEventPayload {
    fn default() -> Self {
        V2CoreEventDestinationEventPayload::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreEventDestinationEventPayload {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreEventDestinationEventPayload::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "snapshot" => V2CoreEventDestinationEventPayload::Snapshot,
            "thin" => V2CoreEventDestinationEventPayload::Thin,
            other => V2CoreEventDestinationEventPayload::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreEventDestinationType {
    AmazonEventbridge,
    AzureEventGrid,
    WebhookEndpoint,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreEventDestinationType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreEventDestinationType::AmazonEventbridge => "amazon_eventbridge",
            V2CoreEventDestinationType::AzureEventGrid => "azure_event_grid",
            V2CoreEventDestinationType::WebhookEndpoint => "webhook_endpoint",
            V2CoreEventDestinationType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreEventDestinationType {
    fn default() -> Self {
        V2CoreEventDestinationType::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreEventDestinationType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreEventDestinationType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "amazon_eventbridge" => V2CoreEventDestinationType::AmazonEventbridge,
            "azure_event_grid" => V2CoreEventDestinationType::AzureEventGrid,
            "webhook_endpoint" => V2CoreEventDestinationType::WebhookEndpoint,
            other => V2CoreEventDestinationType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreEventDestinationWebhookEndpoint {
    #[doc = "The secret used to verify Stripe signatures on delivered events."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub signing_secret: Option<String>,
    #[doc = "The URL where Stripe sends matching events."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
