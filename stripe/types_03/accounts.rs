#[cfg(any(feature = "accounts", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum CapabilityObject {
    Capability,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "accounts", feature = "events"))]
impl CapabilityObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CapabilityObject::Capability => "capability",
            CapabilityObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "accounts", feature = "events"))]
impl Default for CapabilityObject {
    fn default() -> Self {
        CapabilityObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "accounts", feature = "events"))]
impl<'json> json::Decode<'json> for CapabilityObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CapabilityObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "capability" => CapabilityObject::Capability,
            other => CapabilityObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsBusinessProfileAnnualRevenue {
    #[doc = "A non-negative integer representing the amount in the [smallest currency unit](/currencies#zero-decimal)."]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "The close-out date of the preceding fiscal year in ISO 8601 format."]
    pub fiscal_year_end: String,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsBusinessProfileAnnualRevenue {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("currency", &self.currency);
        form.field("fiscal_year_end", &self.fiscal_year_end);
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsBusinessProfileSupportAddress {
    #[doc = "City, district, suburb, town, or village."]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: Option<String>,
    #[doc = "Address line 1, such as the street, PO Box, or company name."]
    pub line1: Option<String>,
    #[doc = "Address line 2, such as the apartment, suite, unit, or building."]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    pub postal_code: Option<String>,
    #[doc = "State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    pub state: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsBusinessProfileSupportAddress {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("line1", self.line1.as_ref());
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountParamsBusinessType {
    Company,
    GovernmentEntity,
    Individual,
    NonProfit,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl CreateAccountParamsBusinessType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountParamsBusinessType::Company => "company",
            CreateAccountParamsBusinessType::GovernmentEntity => "government_entity",
            CreateAccountParamsBusinessType::Individual => "individual",
            CreateAccountParamsBusinessType::NonProfit => "non_profit",
            CreateAccountParamsBusinessType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for CreateAccountParamsBusinessType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountParamsBusinessType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "company" => CreateAccountParamsBusinessType::Company,
            "government_entity" => CreateAccountParamsBusinessType::GovernmentEntity,
            "individual" => CreateAccountParamsBusinessType::Individual,
            "non_profit" => CreateAccountParamsBusinessType::NonProfit,
            other => CreateAccountParamsBusinessType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsBusinessType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountParamsCompanyOwnershipExemptionReason {
    Unnamed,
    QualifiedEntityExceedsOwnershipThreshold,
    QualifiesAsFinancialInstitution,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl CreateAccountParamsCompanyOwnershipExemptionReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountParamsCompanyOwnershipExemptionReason::Unnamed => "",
            CreateAccountParamsCompanyOwnershipExemptionReason::QualifiedEntityExceedsOwnershipThreshold => {
                "qualified_entity_exceeds_ownership_threshold"
            }
            CreateAccountParamsCompanyOwnershipExemptionReason::QualifiesAsFinancialInstitution => {
                "qualifies_as_financial_institution"
            }
            CreateAccountParamsCompanyOwnershipExemptionReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for CreateAccountParamsCompanyOwnershipExemptionReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountParamsCompanyOwnershipExemptionReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateAccountParamsCompanyOwnershipExemptionReason::Unnamed,
            "qualified_entity_exceeds_ownership_threshold" => {
                CreateAccountParamsCompanyOwnershipExemptionReason::QualifiedEntityExceedsOwnershipThreshold
            }
            "qualifies_as_financial_institution" => {
                CreateAccountParamsCompanyOwnershipExemptionReason::QualifiesAsFinancialInstitution
            }
            other => CreateAccountParamsCompanyOwnershipExemptionReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsCompanyOwnershipExemptionReason {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsCompanyRegistrationDate {
    #[doc = "The day of registration, between 1 and 31."]
    pub day: i64,
    #[doc = "The month of registration, between 1 and 12."]
    pub month: i64,
    #[doc = "The four-digit year of registration."]
    pub year: i64,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsCompanyRegistrationDate {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("day", &self.day);
        form.field("month", &self.month);
        form.field("year", &self.year);
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountParamsControllerFeesPayer {
    Account,
    Application,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl CreateAccountParamsControllerFeesPayer {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountParamsControllerFeesPayer::Account => "account",
            CreateAccountParamsControllerFeesPayer::Application => "application",
            CreateAccountParamsControllerFeesPayer::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for CreateAccountParamsControllerFeesPayer {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountParamsControllerFeesPayer::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account" => CreateAccountParamsControllerFeesPayer::Account,
            "application" => CreateAccountParamsControllerFeesPayer::Application,
            other => CreateAccountParamsControllerFeesPayer::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsControllerFeesPayer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountParamsControllerStripeDashboardType {
    Express,
    Full,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl CreateAccountParamsControllerStripeDashboardType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountParamsControllerStripeDashboardType::Express => "express",
            CreateAccountParamsControllerStripeDashboardType::Full => "full",
            CreateAccountParamsControllerStripeDashboardType::None => "none",
            CreateAccountParamsControllerStripeDashboardType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for CreateAccountParamsControllerStripeDashboardType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountParamsControllerStripeDashboardType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "express" => CreateAccountParamsControllerStripeDashboardType::Express,
            "full" => CreateAccountParamsControllerStripeDashboardType::Full,
            "none" => CreateAccountParamsControllerStripeDashboardType::None,
            other => CreateAccountParamsControllerStripeDashboardType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsControllerStripeDashboardType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountParamsIndividualPoliticalExposure {
    Existing,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl CreateAccountParamsIndividualPoliticalExposure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountParamsIndividualPoliticalExposure::Existing => "existing",
            CreateAccountParamsIndividualPoliticalExposure::None => "none",
            CreateAccountParamsIndividualPoliticalExposure::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for CreateAccountParamsIndividualPoliticalExposure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountParamsIndividualPoliticalExposure::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "existing" => CreateAccountParamsIndividualPoliticalExposure::Existing,
            "none" => CreateAccountParamsIndividualPoliticalExposure::None,
            other => CreateAccountParamsIndividualPoliticalExposure::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsIndividualPoliticalExposure {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettingsBacsDebitPayments {
    #[doc = "The Bacs Direct Debit Display Name for this account."]
    pub display_name: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsBacsDebitPayments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_name", self.display_name.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettingsCardIssuingTosAcceptance {
    #[doc = "The Unix timestamp marking when the account representative accepted the service agreement."]
    pub date: Option<i64>,
    #[doc = "The IP address from which the account representative accepted the service agreement."]
    pub ip: Option<String>,
    #[doc = "The user agent of the browser from which the account representative accepted the service agreement."]
    pub user_agent: Option<json::OwnedValue>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsCardIssuingTosAcceptance {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("date", self.date.as_ref());
        form.field_opt("ip", self.ip.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountParamsSettingsInvoicesHostedPaymentMethodSave {
    Always,
    Never,
    Offer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl CreateAccountParamsSettingsInvoicesHostedPaymentMethodSave {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountParamsSettingsInvoicesHostedPaymentMethodSave::Always => "always",
            CreateAccountParamsSettingsInvoicesHostedPaymentMethodSave::Never => "never",
            CreateAccountParamsSettingsInvoicesHostedPaymentMethodSave::Offer => "offer",
            CreateAccountParamsSettingsInvoicesHostedPaymentMethodSave::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for CreateAccountParamsSettingsInvoicesHostedPaymentMethodSave {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountParamsSettingsInvoicesHostedPaymentMethodSave::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CreateAccountParamsSettingsInvoicesHostedPaymentMethodSave::Always,
            "never" => CreateAccountParamsSettingsInvoicesHostedPaymentMethodSave::Never,
            "offer" => CreateAccountParamsSettingsInvoicesHostedPaymentMethodSave::Offer,
            other => CreateAccountParamsSettingsInvoicesHostedPaymentMethodSave::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsInvoicesHostedPaymentMethodSave {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem {
    Friday,
    Monday,
    Thursday,
    Tuesday,
    Wednesday,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem::Friday => "friday",
            CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem::Monday => "monday",
            CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem::Thursday => "thursday",
            CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem::Tuesday => "tuesday",
            CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem::Wednesday => "wednesday",
            CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "friday" => CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem::Friday,
            "monday" => CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem::Monday,
            "thursday" => CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem::Thursday,
            "tuesday" => CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem::Tuesday,
            "wednesday" => CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem::Wednesday,
            other => CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettingsPaypayPaymentsSiteAccessible {}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsPaypayPaymentsSiteAccessible {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountParamsSettingsPaypayPaymentsSiteType {
    Accessible,
    InDevelopment,
    Restricted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl CreateAccountParamsSettingsPaypayPaymentsSiteType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountParamsSettingsPaypayPaymentsSiteType::Accessible => "accessible",
            CreateAccountParamsSettingsPaypayPaymentsSiteType::InDevelopment => "in_development",
            CreateAccountParamsSettingsPaypayPaymentsSiteType::Restricted => "restricted",
            CreateAccountParamsSettingsPaypayPaymentsSiteType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for CreateAccountParamsSettingsPaypayPaymentsSiteType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountParamsSettingsPaypayPaymentsSiteType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "accessible" => CreateAccountParamsSettingsPaypayPaymentsSiteType::Accessible,
            "in_development" => CreateAccountParamsSettingsPaypayPaymentsSiteType::InDevelopment,
            "restricted" => CreateAccountParamsSettingsPaypayPaymentsSiteType::Restricted,
            other => CreateAccountParamsSettingsPaypayPaymentsSiteType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsPaypayPaymentsSiteType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem {
    AfricanAmerican,
    AmericanIndianOrAlaskaNative,
    Asian,
    AsianIndian,
    BlackOrAfricanAmerican,
    Chinese,
    Ethiopian,
    Filipino,
    GuamanianOrChamorro,
    Haitian,
    Jamaican,
    Japanese,
    Korean,
    NativeHawaiian,
    NativeHawaiianOrOtherPacificIslander,
    Nigerian,
    OtherAsian,
    OtherBlackOrAfricanAmerican,
    OtherPacificIslander,
    PreferNotToAnswer,
    Samoan,
    Somali,
    Vietnamese,
    White,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::AfricanAmerican => "african_american",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::AmericanIndianOrAlaskaNative => {
                "american_indian_or_alaska_native"
            }
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Asian => "asian",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::AsianIndian => "asian_indian",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::BlackOrAfricanAmerican => {
                "black_or_african_american"
            }
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Chinese => "chinese",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Ethiopian => "ethiopian",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Filipino => "filipino",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::GuamanianOrChamorro => "guamanian_or_chamorro",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Haitian => "haitian",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Jamaican => "jamaican",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Japanese => "japanese",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Korean => "korean",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::NativeHawaiian => "native_hawaiian",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::NativeHawaiianOrOtherPacificIslander => {
                "native_hawaiian_or_other_pacific_islander"
            }
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Nigerian => "nigerian",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::OtherAsian => "other_asian",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::OtherBlackOrAfricanAmerican => {
                "other_black_or_african_american"
            }
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::OtherPacificIslander => "other_pacific_islander",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::PreferNotToAnswer => "prefer_not_to_answer",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Samoan => "samoan",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Somali => "somali",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Vietnamese => "vietnamese",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::White => "white",
            CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "african_american" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::AfricanAmerican,
            "american_indian_or_alaska_native" => {
                CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::AmericanIndianOrAlaskaNative
            }
            "asian" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Asian,
            "asian_indian" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::AsianIndian,
            "black_or_african_american" => {
                CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::BlackOrAfricanAmerican
            }
            "chinese" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Chinese,
            "ethiopian" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Ethiopian,
            "filipino" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Filipino,
            "guamanian_or_chamorro" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::GuamanianOrChamorro,
            "haitian" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Haitian,
            "jamaican" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Jamaican,
            "japanese" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Japanese,
            "korean" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Korean,
            "native_hawaiian" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::NativeHawaiian,
            "native_hawaiian_or_other_pacific_islander" => {
                CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::NativeHawaiianOrOtherPacificIslander
            }
            "nigerian" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Nigerian,
            "other_asian" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::OtherAsian,
            "other_black_or_african_american" => {
                CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::OtherBlackOrAfricanAmerican
            }
            "other_pacific_islander" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::OtherPacificIslander,
            "prefer_not_to_answer" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::PreferNotToAnswer,
            "samoan" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Samoan,
            "somali" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Somali,
            "vietnamese" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Vietnamese,
            "white" => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::White,
            other => CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct GetAccountParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for GetAccountParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
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
#[derive(Debug, Clone, PartialEq)]
pub enum PersonRaceDetailsRaceItem {
    AfricanAmerican,
    AmericanIndianOrAlaskaNative,
    Asian,
    AsianIndian,
    BlackOrAfricanAmerican,
    Chinese,
    Ethiopian,
    Filipino,
    GuamanianOrChamorro,
    Haitian,
    Jamaican,
    Japanese,
    Korean,
    NativeHawaiian,
    NativeHawaiianOrOtherPacificIslander,
    Nigerian,
    OtherAsian,
    OtherBlackOrAfricanAmerican,
    OtherPacificIslander,
    PreferNotToAnswer,
    Samoan,
    Somali,
    Vietnamese,
    White,
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
impl PersonRaceDetailsRaceItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PersonRaceDetailsRaceItem::AfricanAmerican => "african_american",
            PersonRaceDetailsRaceItem::AmericanIndianOrAlaskaNative => "american_indian_or_alaska_native",
            PersonRaceDetailsRaceItem::Asian => "asian",
            PersonRaceDetailsRaceItem::AsianIndian => "asian_indian",
            PersonRaceDetailsRaceItem::BlackOrAfricanAmerican => "black_or_african_american",
            PersonRaceDetailsRaceItem::Chinese => "chinese",
            PersonRaceDetailsRaceItem::Ethiopian => "ethiopian",
            PersonRaceDetailsRaceItem::Filipino => "filipino",
            PersonRaceDetailsRaceItem::GuamanianOrChamorro => "guamanian_or_chamorro",
            PersonRaceDetailsRaceItem::Haitian => "haitian",
            PersonRaceDetailsRaceItem::Jamaican => "jamaican",
            PersonRaceDetailsRaceItem::Japanese => "japanese",
            PersonRaceDetailsRaceItem::Korean => "korean",
            PersonRaceDetailsRaceItem::NativeHawaiian => "native_hawaiian",
            PersonRaceDetailsRaceItem::NativeHawaiianOrOtherPacificIslander => {
                "native_hawaiian_or_other_pacific_islander"
            }
            PersonRaceDetailsRaceItem::Nigerian => "nigerian",
            PersonRaceDetailsRaceItem::OtherAsian => "other_asian",
            PersonRaceDetailsRaceItem::OtherBlackOrAfricanAmerican => "other_black_or_african_american",
            PersonRaceDetailsRaceItem::OtherPacificIslander => "other_pacific_islander",
            PersonRaceDetailsRaceItem::PreferNotToAnswer => "prefer_not_to_answer",
            PersonRaceDetailsRaceItem::Samoan => "samoan",
            PersonRaceDetailsRaceItem::Somali => "somali",
            PersonRaceDetailsRaceItem::Vietnamese => "vietnamese",
            PersonRaceDetailsRaceItem::White => "white",
            PersonRaceDetailsRaceItem::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PersonRaceDetailsRaceItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PersonRaceDetailsRaceItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "african_american" => PersonRaceDetailsRaceItem::AfricanAmerican,
            "american_indian_or_alaska_native" => PersonRaceDetailsRaceItem::AmericanIndianOrAlaskaNative,
            "asian" => PersonRaceDetailsRaceItem::Asian,
            "asian_indian" => PersonRaceDetailsRaceItem::AsianIndian,
            "black_or_african_american" => PersonRaceDetailsRaceItem::BlackOrAfricanAmerican,
            "chinese" => PersonRaceDetailsRaceItem::Chinese,
            "ethiopian" => PersonRaceDetailsRaceItem::Ethiopian,
            "filipino" => PersonRaceDetailsRaceItem::Filipino,
            "guamanian_or_chamorro" => PersonRaceDetailsRaceItem::GuamanianOrChamorro,
            "haitian" => PersonRaceDetailsRaceItem::Haitian,
            "jamaican" => PersonRaceDetailsRaceItem::Jamaican,
            "japanese" => PersonRaceDetailsRaceItem::Japanese,
            "korean" => PersonRaceDetailsRaceItem::Korean,
            "native_hawaiian" => PersonRaceDetailsRaceItem::NativeHawaiian,
            "native_hawaiian_or_other_pacific_islander" => {
                PersonRaceDetailsRaceItem::NativeHawaiianOrOtherPacificIslander
            }
            "nigerian" => PersonRaceDetailsRaceItem::Nigerian,
            "other_asian" => PersonRaceDetailsRaceItem::OtherAsian,
            "other_black_or_african_american" => PersonRaceDetailsRaceItem::OtherBlackOrAfricanAmerican,
            "other_pacific_islander" => PersonRaceDetailsRaceItem::OtherPacificIslander,
            "prefer_not_to_answer" => PersonRaceDetailsRaceItem::PreferNotToAnswer,
            "samoan" => PersonRaceDetailsRaceItem::Samoan,
            "somali" => PersonRaceDetailsRaceItem::Somali,
            "vietnamese" => PersonRaceDetailsRaceItem::Vietnamese,
            "white" => PersonRaceDetailsRaceItem::White,
            other => PersonRaceDetailsRaceItem::Unknown(other.to_string()),
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
pub struct PersonRelationship {
    #[doc = "Whether the person is the authorizer of the account's representative."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub authorizer: Option<bool>,
    #[doc = "Whether the person is a director of the account's legal entity."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,
    #[doc = "Whether the person has significant responsibility to control, manage, or direct the organization."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,
    #[doc = "Whether the person is the legal guardian of the account's representative."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub legal_guardian: Option<bool>,
    #[doc = "Whether the person is an owner of the account’s legal entity."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    #[doc = "The percent owned by the person of the account's legal entity."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub percent_ownership: Option<f64>,
    #[doc = "Whether the person is authorized as the primary representative of the account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub representative: Option<bool>,
    #[doc = "The person's title (e.g., CEO, Support Engineer)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum RejectAccountParamsPayoutsAction {
    None,
    Pause,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl RejectAccountParamsPayoutsAction {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            RejectAccountParamsPayoutsAction::None => "none",
            RejectAccountParamsPayoutsAction::Pause => "pause",
            RejectAccountParamsPayoutsAction::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for RejectAccountParamsPayoutsAction {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(RejectAccountParamsPayoutsAction::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => RejectAccountParamsPayoutsAction::None,
            "pause" => RejectAccountParamsPayoutsAction::Pause,
            other => RejectAccountParamsPayoutsAction::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for RejectAccountParamsPayoutsAction {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateAccountExternalAccountParamsAccountHolderType {
    Unnamed,
    Company,
    Individual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl UpdateAccountExternalAccountParamsAccountHolderType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            UpdateAccountExternalAccountParamsAccountHolderType::Unnamed => "",
            UpdateAccountExternalAccountParamsAccountHolderType::Company => "company",
            UpdateAccountExternalAccountParamsAccountHolderType::Individual => "individual",
            UpdateAccountExternalAccountParamsAccountHolderType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for UpdateAccountExternalAccountParamsAccountHolderType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(UpdateAccountExternalAccountParamsAccountHolderType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => UpdateAccountExternalAccountParamsAccountHolderType::Unnamed,
            "company" => UpdateAccountExternalAccountParamsAccountHolderType::Company,
            "individual" => UpdateAccountExternalAccountParamsAccountHolderType::Individual,
            other => UpdateAccountExternalAccountParamsAccountHolderType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountExternalAccountParamsAccountHolderType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
