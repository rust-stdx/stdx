#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CloseCoreAccountParamsAppliedConfigurationsItem {
    Customer,
    Merchant,
    Recipient,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CloseCoreAccountParamsAppliedConfigurationsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CloseCoreAccountParamsAppliedConfigurationsItem::Customer => "customer",
            CloseCoreAccountParamsAppliedConfigurationsItem::Merchant => "merchant",
            CloseCoreAccountParamsAppliedConfigurationsItem::Recipient => "recipient",
            CloseCoreAccountParamsAppliedConfigurationsItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CloseCoreAccountParamsAppliedConfigurationsItem {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CloseCoreAccountParamsAppliedConfigurationsItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CloseCoreAccountParamsAppliedConfigurationsItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer" => CloseCoreAccountParamsAppliedConfigurationsItem::Customer,
            "merchant" => CloseCoreAccountParamsAppliedConfigurationsItem::Merchant,
            "recipient" => CloseCoreAccountParamsAppliedConfigurationsItem::Recipient,
            other => CloseCoreAccountParamsAppliedConfigurationsItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFutureRequirements {
    Include,
    Omit,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFutureRequirements {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFutureRequirements::Include => {
                "include"
            }
            CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFutureRequirements::Omit => "omit",
            CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFutureRequirements::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFutureRequirements {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json>
    for CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFutureRequirements
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFutureRequirements::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "include" => {
                CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFutureRequirements::Include
            }
            "omit" => CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFutureRequirements::Omit,
            other => CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFutureRequirements::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountLinkParamsUseCaseType {
    AccountOnboarding,
    AccountUpdate,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountLinkParamsUseCaseType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountLinkParamsUseCaseType::AccountOnboarding => "account_onboarding",
            CreateCoreAccountLinkParamsUseCaseType::AccountUpdate => "account_update",
            CreateCoreAccountLinkParamsUseCaseType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountLinkParamsUseCaseType {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreAccountLinkParamsUseCaseType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCoreAccountLinkParamsUseCaseType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_onboarding" => CreateCoreAccountLinkParamsUseCaseType::AccountOnboarding,
            "account_update" => CreateCoreAccountLinkParamsUseCaseType::AccountUpdate,
            other => CreateCoreAccountLinkParamsUseCaseType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationCustomerShippingAddress {
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
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsAnnualRevenueAmount {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "A non-negative integer representing how much to charge in the smallest currency unit..."]
    pub value: i64,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType {
    FrontBack,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType::FrontBack => {
                "front_back"
            }
            CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json>
    for CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "front_back" => {
                CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType::FrontBack
            }
            other => CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountTokenParamsIdentityIndividualAdditionalAddressesItemPurpose {
    Registered,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountTokenParamsIdentityIndividualAdditionalAddressesItemPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountTokenParamsIdentityIndividualAdditionalAddressesItemPurpose::Registered => "registered",
            CreateCoreAccountTokenParamsIdentityIndividualAdditionalAddressesItemPurpose::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountTokenParamsIdentityIndividualAdditionalAddressesItemPurpose {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreAccountTokenParamsIdentityIndividualAdditionalAddressesItemPurpose {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCoreAccountTokenParamsIdentityIndividualAdditionalAddressesItemPurpose::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "registered" => CreateCoreAccountTokenParamsIdentityIndividualAdditionalAddressesItemPurpose::Registered,
            other => {
                CreateCoreAccountTokenParamsIdentityIndividualAdditionalAddressesItemPurpose::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountTokenParamsIdentityIndividualLegalGender {
    Female,
    Male,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountTokenParamsIdentityIndividualLegalGender {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountTokenParamsIdentityIndividualLegalGender::Female => "female",
            CreateCoreAccountTokenParamsIdentityIndividualLegalGender::Male => "male",
            CreateCoreAccountTokenParamsIdentityIndividualLegalGender::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountTokenParamsIdentityIndividualLegalGender {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreAccountTokenParamsIdentityIndividualLegalGender {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCoreAccountTokenParamsIdentityIndividualLegalGender::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "female" => CreateCoreAccountTokenParamsIdentityIndividualLegalGender::Female,
            "male" => CreateCoreAccountTokenParamsIdentityIndividualLegalGender::Male,
            other => CreateCoreAccountTokenParamsIdentityIndividualLegalGender::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountTokenParamsIdentityIndividualPoliticalExposure {
    Existing,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountTokenParamsIdentityIndividualPoliticalExposure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountTokenParamsIdentityIndividualPoliticalExposure::Existing => "existing",
            CreateCoreAccountTokenParamsIdentityIndividualPoliticalExposure::None => "none",
            CreateCoreAccountTokenParamsIdentityIndividualPoliticalExposure::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountTokenParamsIdentityIndividualPoliticalExposure {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreAccountTokenParamsIdentityIndividualPoliticalExposure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCoreAccountTokenParamsIdentityIndividualPoliticalExposure::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "existing" => CreateCoreAccountTokenParamsIdentityIndividualPoliticalExposure::Existing,
            "none" => CreateCoreAccountTokenParamsIdentityIndividualPoliticalExposure::None,
            other => CreateCoreAccountTokenParamsIdentityIndividualPoliticalExposure::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityIndividualRelationship {
    #[doc = "Whether the person is a director of the account's identity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,
    #[doc = "Whether the person has significant responsibility to control, manage, or direct the organization."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,
    #[doc = "Whether the person is an owner of the account's identity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    #[doc = "The percent owned by the person of the account's legal entity."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub percent_ownership: Option<String>,
    #[doc = "The person's title (e.g., CEO, Support Engineer)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, Default)]
pub struct ListCoreAccountPersonsParams {
    pub limit: Option<i64>,
}

#[cfg(feature = "v2_core")]
impl crate::params::FormEncode for ListCoreAccountPersonsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("limit", self.limit.as_ref());
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListCoreEventDestinationsParamsIncludeItem {
    WebhookEndpointUrl,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl ListCoreEventDestinationsParamsIncludeItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListCoreEventDestinationsParamsIncludeItem::WebhookEndpointUrl => "webhook_endpoint.url",
            ListCoreEventDestinationsParamsIncludeItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for ListCoreEventDestinationsParamsIncludeItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListCoreEventDestinationsParamsIncludeItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "webhook_endpoint.url" => ListCoreEventDestinationsParamsIncludeItem::WebhookEndpointUrl,
            other => ListCoreEventDestinationsParamsIncludeItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
impl crate::params::FormEncode for ListCoreEventDestinationsParamsIncludeItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus {
    Active,
    Pending,
    Rejected,
    Restricted,
    Unsupported,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus::Active => "active",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus::Pending => "pending",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus::Rejected => "rejected",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus::Restricted => "restricted",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus::Unsupported => "unsupported",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus {
    fn default() -> Self {
        V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus::Active,
            "pending" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus::Pending,
            "rejected" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus::Rejected,
            "restricted" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus::Restricted,
            "unsupported" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus::Unsupported,
            other => {
                V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatus::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode {
    DeterminingStatus,
    RejectedFraud,
    RejectedIncompleteVerification,
    RejectedListed,
    RejectedOther,
    RejectedPlatformFraud,
    RejectedPlatformOther,
    RejectedPlatformTermsOfService,
    RejectedTermsOfService,
    RequirementsPastDue,
    RequirementsPendingVerification,
    RestrictedOther,
    UnsupportedBusiness,
    UnsupportedCountry,
    UnsupportedEntityType,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::DeterminingStatus => "determining_status",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RejectedFraud => "rejected_fraud",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RejectedIncompleteVerification => "rejected_incomplete_verification",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RejectedListed => "rejected_listed",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RejectedOther => "rejected_other",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RejectedPlatformFraud => "rejected_platform_fraud",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RejectedPlatformOther => "rejected_platform_other",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RejectedPlatformTermsOfService => "rejected_platform_terms_of_service",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RejectedTermsOfService => "rejected_terms_of_service",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RequirementsPastDue => "requirements_past_due",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RequirementsPendingVerification => "requirements_pending_verification",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RestrictedOther => "restricted_other",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::UnsupportedBusiness => "unsupported_business",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::UnsupportedCountry => "unsupported_country",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::UnsupportedEntityType => "unsupported_entity_type",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode {
    fn default() -> Self {
        V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json>
    for V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "determining_status" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::DeterminingStatus,
            "rejected_fraud" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RejectedFraud,
            "rejected_incomplete_verification" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RejectedIncompleteVerification,
            "rejected_listed" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RejectedListed,
            "rejected_other" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RejectedOther,
            "rejected_platform_fraud" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RejectedPlatformFraud,
            "rejected_platform_other" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RejectedPlatformOther,
            "rejected_platform_terms_of_service" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RejectedPlatformTermsOfService,
            "rejected_terms_of_service" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RejectedTermsOfService,
            "requirements_past_due" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RequirementsPastDue,
            "requirements_pending_verification" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RequirementsPendingVerification,
            "restricted_other" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::RestrictedOther,
            "unsupported_business" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::UnsupportedBusiness,
            "unsupported_country" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::UnsupportedCountry,
            "unsupported_entity_type" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::UnsupportedEntityType,
            other => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemCode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemResolution {
    ContactStripe,
    NoResolution,
    ProvideInfo,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemResolution {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemResolution::ContactStripe => "contact_stripe",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemResolution::NoResolution => "no_resolution",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemResolution::ProvideInfo => "provide_info",
            V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemResolution::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemResolution {
    fn default() -> Self {
        V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemResolution::Unknown(
            String::new(),
        )
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json>
    for V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemResolution
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemResolution::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "contact_stripe" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemResolution::ContactStripe,
            "no_resolution" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemResolution::NoResolution,
            "provide_info" => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemResolution::ProvideInfo,
            other => V2CoreAccountConfigurationCustomerCapabilitiesAutomaticIndirectTaxStatusDetailsItemResolution::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationMerchantBranding {
    #[doc = "ID of a [file upload](https://docs.stripe.com/api/persons/update#create_file): An icon for the merchant."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[doc = "ID of a [file upload](https://docs.stripe.com/api/persons/update#create_file): A logo for the merchant that will be u..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[doc = "A CSS hex color value representing the primary branding color for the merchant."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    #[doc = "A CSS hex color value representing the secondary branding color for the merchant."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationMerchantKonbiniPaymentsSupportHours {
    #[doc = "Support hours end time (JST time of day) for in `HH:MM` format."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "Support hours start time (JST time of day) for in `HH:MM` format."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationMerchantSepaDebitPayments {
    #[doc = "Creditor ID for SEPA Direct Debit payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub creditor_id: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode {
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
    InvalidUrlWebPresenceDetected,
    InvalidValueOther,
    UnresolvableIpAddress,
    UnresolvablePostalCode,
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
    VerificationMissingDirectors,
    VerificationMissingExecutives,
    VerificationMissingOwners,
    VerificationRequiresAdditionalMemorandumOfAssociations,
    VerificationRequiresAdditionalProofOfRegistration,
    VerificationSelfieDocumentMissingPhoto,
    VerificationSelfieFaceMismatch,
    VerificationSelfieManipulated,
    VerificationSelfieUnverifiedOther,
    VerificationSupportability,
    VerificationTokenStale,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidAddressCityStatePostalCode => "invalid_address_city_state_postal_code",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidAddressCmraAddress => "invalid_address_cmra_address",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidAddressHighwayContractBox => "invalid_address_highway_contract_box",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidAddressPrivateMailbox => "invalid_address_private_mailbox",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidAddressRegisteredAgentAddress => "invalid_address_registered_agent_address",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidBusinessProfileName => "invalid_business_profile_name",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidBusinessProfileNameDenylisted => "invalid_business_profile_name_denylisted",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidCompanyNameDenylisted => "invalid_company_name_denylisted",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidDobAgeOverMaximum => "invalid_dob_age_over_maximum",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidDobAgeUnder18 => "invalid_dob_age_under_18",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidDobAgeUnderMinimum => "invalid_dob_age_under_minimum",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidProductDescriptionLength => "invalid_product_description_length",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidProductDescriptionUrlMatch => "invalid_product_description_url_match",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidRepresentativeCountry => "invalid_representative_country",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidStatementDescriptorBusinessMismatch => "invalid_statement_descriptor_business_mismatch",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidStatementDescriptorDenylisted => "invalid_statement_descriptor_denylisted",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidStatementDescriptorLength => "invalid_statement_descriptor_length",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidStatementDescriptorPrefixDenylisted => "invalid_statement_descriptor_prefix_denylisted",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidStatementDescriptorPrefixMismatch => "invalid_statement_descriptor_prefix_mismatch",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidStreetAddress => "invalid_street_address",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidTaxId => "invalid_tax_id",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidTaxIdFormat => "invalid_tax_id_format",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidTosAcceptance => "invalid_tos_acceptance",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlDenylisted => "invalid_url_denylisted",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlFormat => "invalid_url_format",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteBusinessInformationMismatch => "invalid_url_website_business_information_mismatch",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteEmpty => "invalid_url_website_empty",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteInaccessible => "invalid_url_website_inaccessible",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteInaccessibleGeoblocked => "invalid_url_website_inaccessible_geoblocked",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteInaccessiblePasswordProtected => "invalid_url_website_inaccessible_password_protected",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteIncomplete => "invalid_url_website_incomplete",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteIncompleteCancellationPolicy => "invalid_url_website_incomplete_cancellation_policy",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteIncompleteCustomerServiceDetails => "invalid_url_website_incomplete_customer_service_details",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteIncompleteLegalRestrictions => "invalid_url_website_incomplete_legal_restrictions",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteIncompleteRefundPolicy => "invalid_url_website_incomplete_refund_policy",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteIncompleteReturnPolicy => "invalid_url_website_incomplete_return_policy",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteIncompleteTermsAndConditions => "invalid_url_website_incomplete_terms_and_conditions",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteIncompleteUnderConstruction => "invalid_url_website_incomplete_under_construction",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteOther => "invalid_url_website_other",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebPresenceDetected => "invalid_url_web_presence_detected",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidValueOther => "invalid_value_other",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::UnresolvableIpAddress => "unresolvable_ip_address",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::UnresolvablePostalCode => "unresolvable_postal_code",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDirectorsMismatch => "verification_directors_mismatch",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentAddressMismatch => "verification_document_address_mismatch",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentAddressMissing => "verification_document_address_missing",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentCorrupt => "verification_document_corrupt",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentCountryNotSupported => "verification_document_country_not_supported",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentDirectorsMismatch => "verification_document_directors_mismatch",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentDobMismatch => "verification_document_dob_mismatch",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentDuplicateType => "verification_document_duplicate_type",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentExpired => "verification_document_expired",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentFailedCopy => "verification_document_failed_copy",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentFailedGreyscale => "verification_document_failed_greyscale",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentFailedOther => "verification_document_failed_other",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentFailedTestMode => "verification_document_failed_test_mode",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentFraudulent => "verification_document_fraudulent",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentIdNumberMismatch => "verification_document_id_number_mismatch",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentIdNumberMissing => "verification_document_id_number_missing",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentIncomplete => "verification_document_incomplete",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentInvalid => "verification_document_invalid",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentIssueOrExpiryDateMissing => "verification_document_issue_or_expiry_date_missing",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentManipulated => "verification_document_manipulated",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentMissingBack => "verification_document_missing_back",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentMissingFront => "verification_document_missing_front",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentNameMismatch => "verification_document_name_mismatch",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentNameMissing => "verification_document_name_missing",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentNationalityMismatch => "verification_document_nationality_mismatch",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentNotReadable => "verification_document_not_readable",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentNotSigned => "verification_document_not_signed",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentNotUploaded => "verification_document_not_uploaded",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentPhotoMismatch => "verification_document_photo_mismatch",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentTooLarge => "verification_document_too_large",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentTypeNotSupported => "verification_document_type_not_supported",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationExtraneousDirectors => "verification_extraneous_directors",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedAddressMatch => "verification_failed_address_match",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedBusinessIecNumber => "verification_failed_business_iec_number",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedDocumentMatch => "verification_failed_document_match",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedIdNumberMatch => "verification_failed_id_number_match",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedKeyedIdentity => "verification_failed_keyed_identity",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedKeyedMatch => "verification_failed_keyed_match",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedNameMatch => "verification_failed_name_match",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedOther => "verification_failed_other",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedRepresentativeAuthority => "verification_failed_representative_authority",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedResidentialAddress => "verification_failed_residential_address",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedTaxIdMatch => "verification_failed_tax_id_match",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedTaxIdNotIssued => "verification_failed_tax_id_not_issued",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationMissingDirectors => "verification_missing_directors",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationMissingExecutives => "verification_missing_executives",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationMissingOwners => "verification_missing_owners",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationRequiresAdditionalMemorandumOfAssociations => "verification_requires_additional_memorandum_of_associations",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationRequiresAdditionalProofOfRegistration => "verification_requires_additional_proof_of_registration",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationSelfieDocumentMissingPhoto => "verification_selfie_document_missing_photo",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationSelfieFaceMismatch => "verification_selfie_face_mismatch",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationSelfieManipulated => "verification_selfie_manipulated",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationSelfieUnverifiedOther => "verification_selfie_unverified_other",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationSupportability => "verification_supportability",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationTokenStale => "verification_token_stale",
            V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode {
    fn default() -> Self {
        V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "invalid_address_city_state_postal_code" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidAddressCityStatePostalCode,
            "invalid_address_cmra_address" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidAddressCmraAddress,
            "invalid_address_highway_contract_box" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidAddressHighwayContractBox,
            "invalid_address_private_mailbox" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidAddressPrivateMailbox,
            "invalid_address_registered_agent_address" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidAddressRegisteredAgentAddress,
            "invalid_business_profile_name" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidBusinessProfileName,
            "invalid_business_profile_name_denylisted" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidBusinessProfileNameDenylisted,
            "invalid_company_name_denylisted" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidCompanyNameDenylisted,
            "invalid_dob_age_over_maximum" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidDobAgeOverMaximum,
            "invalid_dob_age_under_18" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidDobAgeUnder18,
            "invalid_dob_age_under_minimum" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidDobAgeUnderMinimum,
            "invalid_product_description_length" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidProductDescriptionLength,
            "invalid_product_description_url_match" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidProductDescriptionUrlMatch,
            "invalid_representative_country" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidRepresentativeCountry,
            "invalid_statement_descriptor_business_mismatch" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidStatementDescriptorBusinessMismatch,
            "invalid_statement_descriptor_denylisted" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidStatementDescriptorDenylisted,
            "invalid_statement_descriptor_length" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidStatementDescriptorLength,
            "invalid_statement_descriptor_prefix_denylisted" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidStatementDescriptorPrefixDenylisted,
            "invalid_statement_descriptor_prefix_mismatch" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidStatementDescriptorPrefixMismatch,
            "invalid_street_address" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidStreetAddress,
            "invalid_tax_id" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidTaxId,
            "invalid_tax_id_format" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidTaxIdFormat,
            "invalid_tos_acceptance" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidTosAcceptance,
            "invalid_url_denylisted" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlDenylisted,
            "invalid_url_format" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlFormat,
            "invalid_url_website_business_information_mismatch" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteBusinessInformationMismatch,
            "invalid_url_website_empty" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteEmpty,
            "invalid_url_website_inaccessible" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteInaccessible,
            "invalid_url_website_inaccessible_geoblocked" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteInaccessibleGeoblocked,
            "invalid_url_website_inaccessible_password_protected" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteInaccessiblePasswordProtected,
            "invalid_url_website_incomplete" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteIncomplete,
            "invalid_url_website_incomplete_cancellation_policy" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteIncompleteCancellationPolicy,
            "invalid_url_website_incomplete_customer_service_details" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteIncompleteCustomerServiceDetails,
            "invalid_url_website_incomplete_legal_restrictions" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteIncompleteLegalRestrictions,
            "invalid_url_website_incomplete_refund_policy" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteIncompleteRefundPolicy,
            "invalid_url_website_incomplete_return_policy" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteIncompleteReturnPolicy,
            "invalid_url_website_incomplete_terms_and_conditions" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteIncompleteTermsAndConditions,
            "invalid_url_website_incomplete_under_construction" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteIncompleteUnderConstruction,
            "invalid_url_website_other" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebsiteOther,
            "invalid_url_web_presence_detected" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidUrlWebPresenceDetected,
            "invalid_value_other" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::InvalidValueOther,
            "unresolvable_ip_address" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::UnresolvableIpAddress,
            "unresolvable_postal_code" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::UnresolvablePostalCode,
            "verification_directors_mismatch" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDirectorsMismatch,
            "verification_document_address_mismatch" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentAddressMismatch,
            "verification_document_address_missing" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentAddressMissing,
            "verification_document_corrupt" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentCorrupt,
            "verification_document_country_not_supported" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentCountryNotSupported,
            "verification_document_directors_mismatch" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentDirectorsMismatch,
            "verification_document_dob_mismatch" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentDobMismatch,
            "verification_document_duplicate_type" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentDuplicateType,
            "verification_document_expired" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentExpired,
            "verification_document_failed_copy" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentFailedCopy,
            "verification_document_failed_greyscale" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentFailedGreyscale,
            "verification_document_failed_other" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentFailedOther,
            "verification_document_failed_test_mode" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentFailedTestMode,
            "verification_document_fraudulent" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentFraudulent,
            "verification_document_id_number_mismatch" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentIdNumberMismatch,
            "verification_document_id_number_missing" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentIdNumberMissing,
            "verification_document_incomplete" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentIncomplete,
            "verification_document_invalid" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentInvalid,
            "verification_document_issue_or_expiry_date_missing" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentIssueOrExpiryDateMissing,
            "verification_document_manipulated" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentManipulated,
            "verification_document_missing_back" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentMissingBack,
            "verification_document_missing_front" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentMissingFront,
            "verification_document_name_mismatch" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentNameMismatch,
            "verification_document_name_missing" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentNameMissing,
            "verification_document_nationality_mismatch" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentNationalityMismatch,
            "verification_document_not_readable" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentNotReadable,
            "verification_document_not_signed" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentNotSigned,
            "verification_document_not_uploaded" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentNotUploaded,
            "verification_document_photo_mismatch" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentPhotoMismatch,
            "verification_document_too_large" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentTooLarge,
            "verification_document_type_not_supported" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationDocumentTypeNotSupported,
            "verification_extraneous_directors" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationExtraneousDirectors,
            "verification_failed_address_match" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedAddressMatch,
            "verification_failed_business_iec_number" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedBusinessIecNumber,
            "verification_failed_document_match" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedDocumentMatch,
            "verification_failed_id_number_match" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedIdNumberMatch,
            "verification_failed_keyed_identity" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedKeyedIdentity,
            "verification_failed_keyed_match" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedKeyedMatch,
            "verification_failed_name_match" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedNameMatch,
            "verification_failed_other" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedOther,
            "verification_failed_representative_authority" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedRepresentativeAuthority,
            "verification_failed_residential_address" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedResidentialAddress,
            "verification_failed_tax_id_match" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedTaxIdMatch,
            "verification_failed_tax_id_not_issued" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationFailedTaxIdNotIssued,
            "verification_missing_directors" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationMissingDirectors,
            "verification_missing_executives" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationMissingExecutives,
            "verification_missing_owners" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationMissingOwners,
            "verification_requires_additional_memorandum_of_associations" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationRequiresAdditionalMemorandumOfAssociations,
            "verification_requires_additional_proof_of_registration" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationRequiresAdditionalProofOfRegistration,
            "verification_selfie_document_missing_photo" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationSelfieDocumentMissingPhoto,
            "verification_selfie_face_mismatch" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationSelfieFaceMismatch,
            "verification_selfie_manipulated" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationSelfieManipulated,
            "verification_selfie_unverified_other" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationSelfieUnverifiedOther,
            "verification_supportability" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationSupportability,
            "verification_token_stale" => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::VerificationTokenStale,
            other => V2CoreAccountFutureRequirementsEntriesItemErrorsItemCode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability {
    AchDebitPayments,
    AcssDebitPayments,
    AffirmPayments,
    AfterpayClearpayPayments,
    AlmaPayments,
    AmazonPayPayments,
    AutomaticIndirectTax,
    AuBecsDebitPayments,
    BacsDebitPayments,
    BancontactPayments,
    BankAccountsLocal,
    BankAccountsWire,
    BlikPayments,
    BlikRecurringPayments,
    BoletoPayments,
    Cards,
    CardPayments,
    CartesBancairesPayments,
    CashappPayments,
    EpsPayments,
    FpxPayments,
    GbBankTransferPayments,
    GrabpayPayments,
    IdealPayments,
    JcbPayments,
    JpBankTransferPayments,
    KakaoPayPayments,
    KlarnaPayments,
    KonbiniPayments,
    KrCardPayments,
    LinkPayments,
    MobilepayPayments,
    MultibancoPayments,
    MxBankTransferPayments,
    NaverPayPayments,
    OxxoPayments,
    P24Payments,
    PaycoPayments,
    PaynowPayments,
    PayByBankPayments,
    PromptpayPayments,
    RevolutPayPayments,
    SamsungPayPayments,
    SatispayPayments,
    SepaBankTransferPayments,
    SepaDebitPayments,
    StripeBalancePayouts,
    StripeBalanceStripeTransfers,
    SwishPayments,
    TwintPayments,
    UsBankTransferPayments,
    ZipPayments,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::AchDebitPayments => "ach_debit_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::AcssDebitPayments => "acss_debit_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::AffirmPayments => "affirm_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::AfterpayClearpayPayments => "afterpay_clearpay_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::AlmaPayments => "alma_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::AmazonPayPayments => "amazon_pay_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::AutomaticIndirectTax => "automatic_indirect_tax",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::AuBecsDebitPayments => "au_becs_debit_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::BacsDebitPayments => "bacs_debit_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::BancontactPayments => "bancontact_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::BankAccountsLocal => "bank_accounts.local",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::BankAccountsWire => "bank_accounts.wire",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::BlikPayments => "blik_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::BlikRecurringPayments => "blik_recurring_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::BoletoPayments => "boleto_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::Cards => "cards",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::CardPayments => "card_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::CartesBancairesPayments => "cartes_bancaires_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::CashappPayments => "cashapp_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::EpsPayments => "eps_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::FpxPayments => "fpx_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::GbBankTransferPayments => "gb_bank_transfer_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::GrabpayPayments => "grabpay_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::IdealPayments => "ideal_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::JcbPayments => "jcb_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::JpBankTransferPayments => "jp_bank_transfer_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::KakaoPayPayments => "kakao_pay_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::KlarnaPayments => "klarna_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::KonbiniPayments => "konbini_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::KrCardPayments => "kr_card_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::LinkPayments => "link_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::MobilepayPayments => "mobilepay_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::MultibancoPayments => "multibanco_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::MxBankTransferPayments => "mx_bank_transfer_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::NaverPayPayments => "naver_pay_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::OxxoPayments => "oxxo_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::P24Payments => "p24_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::PaycoPayments => "payco_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::PaynowPayments => "paynow_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::PayByBankPayments => "pay_by_bank_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::PromptpayPayments => "promptpay_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::RevolutPayPayments => "revolut_pay_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::SamsungPayPayments => "samsung_pay_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::SatispayPayments => "satispay_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::SepaBankTransferPayments => "sepa_bank_transfer_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::SepaDebitPayments => "sepa_debit_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::StripeBalancePayouts => "stripe_balance.payouts",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::StripeBalanceStripeTransfers => "stripe_balance.stripe_transfers",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::SwishPayments => "swish_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::TwintPayments => "twint_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::UsBankTransferPayments => "us_bank_transfer_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::ZipPayments => "zip_payments",
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability {
    fn default() -> Self {
        V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json>
    for V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach_debit_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::AchDebitPayments,
            "acss_debit_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::AcssDebitPayments,
            "affirm_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::AffirmPayments,
            "afterpay_clearpay_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::AfterpayClearpayPayments,
            "alma_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::AlmaPayments,
            "amazon_pay_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::AmazonPayPayments,
            "automatic_indirect_tax" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::AutomaticIndirectTax,
            "au_becs_debit_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::AuBecsDebitPayments,
            "bacs_debit_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::BacsDebitPayments,
            "bancontact_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::BancontactPayments,
            "bank_accounts.local" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::BankAccountsLocal,
            "bank_accounts.wire" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::BankAccountsWire,
            "blik_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::BlikPayments,
            "blik_recurring_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::BlikRecurringPayments,
            "boleto_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::BoletoPayments,
            "cards" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::Cards,
            "card_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::CardPayments,
            "cartes_bancaires_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::CartesBancairesPayments,
            "cashapp_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::CashappPayments,
            "eps_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::EpsPayments,
            "fpx_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::FpxPayments,
            "gb_bank_transfer_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::GbBankTransferPayments,
            "grabpay_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::GrabpayPayments,
            "ideal_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::IdealPayments,
            "jcb_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::JcbPayments,
            "jp_bank_transfer_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::JpBankTransferPayments,
            "kakao_pay_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::KakaoPayPayments,
            "klarna_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::KlarnaPayments,
            "konbini_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::KonbiniPayments,
            "kr_card_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::KrCardPayments,
            "link_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::LinkPayments,
            "mobilepay_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::MobilepayPayments,
            "multibanco_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::MultibancoPayments,
            "mx_bank_transfer_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::MxBankTransferPayments,
            "naver_pay_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::NaverPayPayments,
            "oxxo_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::OxxoPayments,
            "p24_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::P24Payments,
            "payco_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::PaycoPayments,
            "paynow_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::PaynowPayments,
            "pay_by_bank_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::PayByBankPayments,
            "promptpay_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::PromptpayPayments,
            "revolut_pay_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::RevolutPayPayments,
            "samsung_pay_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::SamsungPayPayments,
            "satispay_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::SatispayPayments,
            "sepa_bank_transfer_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::SepaBankTransferPayments,
            "sepa_debit_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::SepaDebitPayments,
            "stripe_balance.payouts" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::StripeBalancePayouts,
            "stripe_balance.stripe_transfers" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::StripeBalanceStripeTransfers,
            "swish_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::SwishPayments,
            "twint_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::TwintPayments,
            "us_bank_transfer_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::UsBankTransferPayments,
            "zip_payments" => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::ZipPayments,
            other => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadlineStatus {
    CurrentlyDue,
    EventuallyDue,
    PastDue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadlineStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadlineStatus::CurrentlyDue => {
                "currently_due"
            }
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadlineStatus::EventuallyDue => {
                "eventually_due"
            }
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadlineStatus::PastDue => {
                "past_due"
            }
            V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadlineStatus::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadlineStatus {
    fn default() -> Self {
        V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadlineStatus::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json>
    for V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadlineStatus
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadlineStatus::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "currently_due" => {
                V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadlineStatus::CurrentlyDue
            }
            "eventually_due" => {
                V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadlineStatus::EventuallyDue
            }
            "past_due" => {
                V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadlineStatus::PastDue
            }
            other => V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadlineStatus::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountIdentityBusinessDetailsAdditionalAddressesItemPurpose {
    Administrative,
    PrincipalPlaceOfBusiness,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountIdentityBusinessDetailsAdditionalAddressesItemPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountIdentityBusinessDetailsAdditionalAddressesItemPurpose::Administrative => "administrative",
            V2CoreAccountIdentityBusinessDetailsAdditionalAddressesItemPurpose::PrincipalPlaceOfBusiness => {
                "principal_place_of_business"
            }
            V2CoreAccountIdentityBusinessDetailsAdditionalAddressesItemPurpose::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountIdentityBusinessDetailsAdditionalAddressesItemPurpose {
    fn default() -> Self {
        V2CoreAccountIdentityBusinessDetailsAdditionalAddressesItemPurpose::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountIdentityBusinessDetailsAdditionalAddressesItemPurpose {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountIdentityBusinessDetailsAdditionalAddressesItemPurpose::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "administrative" => V2CoreAccountIdentityBusinessDetailsAdditionalAddressesItemPurpose::Administrative,
            "principal_place_of_business" => {
                V2CoreAccountIdentityBusinessDetailsAdditionalAddressesItemPurpose::PrincipalPlaceOfBusiness
            }
            other => V2CoreAccountIdentityBusinessDetailsAdditionalAddressesItemPurpose::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityBusinessDetailsScriptNamesKana {
    #[doc = "Registered name of the business."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registered_name: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFields {
    CurrentlyDue,
    EventuallyDue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFields {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFields::CurrentlyDue => "currently_due",
            V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFields::EventuallyDue => "eventually_due",
            V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFields::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFields {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFields::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "currently_due" => V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFields::CurrentlyDue,
            "eventually_due" => V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFields::EventuallyDue,
            other => V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFields::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountLinkUseCaseType {
    AccountOnboarding,
    AccountUpdate,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountLinkUseCaseType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountLinkUseCaseType::AccountOnboarding => "account_onboarding",
            V2CoreAccountLinkUseCaseType::AccountUpdate => "account_update",
            V2CoreAccountLinkUseCaseType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountLinkUseCaseType {
    fn default() -> Self {
        V2CoreAccountLinkUseCaseType::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountLinkUseCaseType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountLinkUseCaseType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_onboarding" => V2CoreAccountLinkUseCaseType::AccountOnboarding,
            "account_update" => V2CoreAccountLinkUseCaseType::AccountUpdate,
            other => V2CoreAccountLinkUseCaseType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreEventObject {
    V2CoreEvent,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreEventObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreEventObject::V2CoreEvent => "v2.core.event",
            V2CoreEventObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreEventObject {
    fn default() -> Self {
        V2CoreEventObject::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreEventObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreEventObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.event" => V2CoreEventObject::V2CoreEvent,
            other => V2CoreEventObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreEventReasonRequest {
    #[doc = "ID of the API request that caused the event."]
    #[json(default)]
    pub id: String,
    #[doc = "The idempotency key transmitted during the request."]
    #[json(default)]
    pub idempotency_key: String,
}

#[cfg(feature = "v2_core")]
impl crate::PagedItem for V2CoreEventReasonRequest {
    fn id(&self) -> &str {
        &self.id
    }
}
