#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsAccountCompanyAddressKana {
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

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsAccountCompanyAddressKana {
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

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTokenParamsAccountCompanyOwnershipExemptionReason {
    Unnamed,
    QualifiedEntityExceedsOwnershipThreshold,
    QualifiesAsFinancialInstitution,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tokens")]
impl CreateTokenParamsAccountCompanyOwnershipExemptionReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTokenParamsAccountCompanyOwnershipExemptionReason::Unnamed => "",
            CreateTokenParamsAccountCompanyOwnershipExemptionReason::QualifiedEntityExceedsOwnershipThreshold => {
                "qualified_entity_exceeds_ownership_threshold"
            }
            CreateTokenParamsAccountCompanyOwnershipExemptionReason::QualifiesAsFinancialInstitution => {
                "qualifies_as_financial_institution"
            }
            CreateTokenParamsAccountCompanyOwnershipExemptionReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tokens")]
impl<'json> json::Decode<'json> for CreateTokenParamsAccountCompanyOwnershipExemptionReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTokenParamsAccountCompanyOwnershipExemptionReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateTokenParamsAccountCompanyOwnershipExemptionReason::Unnamed,
            "qualified_entity_exceeds_ownership_threshold" => {
                CreateTokenParamsAccountCompanyOwnershipExemptionReason::QualifiedEntityExceedsOwnershipThreshold
            }
            "qualifies_as_financial_institution" => {
                CreateTokenParamsAccountCompanyOwnershipExemptionReason::QualifiesAsFinancialInstitution
            }
            other => CreateTokenParamsAccountCompanyOwnershipExemptionReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsAccountCompanyOwnershipExemptionReason {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTokenParamsAccountIndividualPoliticalExposure {
    Existing,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tokens")]
impl CreateTokenParamsAccountIndividualPoliticalExposure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTokenParamsAccountIndividualPoliticalExposure::Existing => "existing",
            CreateTokenParamsAccountIndividualPoliticalExposure::None => "none",
            CreateTokenParamsAccountIndividualPoliticalExposure::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tokens")]
impl<'json> json::Decode<'json> for CreateTokenParamsAccountIndividualPoliticalExposure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTokenParamsAccountIndividualPoliticalExposure::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "existing" => CreateTokenParamsAccountIndividualPoliticalExposure::Existing,
            "none" => CreateTokenParamsAccountIndividualPoliticalExposure::None,
            other => CreateTokenParamsAccountIndividualPoliticalExposure::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsAccountIndividualPoliticalExposure {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsAccountIndividualRelationship {
    #[doc = "Whether the person is a director of the account's legal entity."]
    pub director: Option<bool>,
    #[doc = "Whether the person has significant responsibility to control, manage, or direct the organization."]
    pub executive: Option<bool>,
    #[doc = "Whether the person is an owner of the account’s legal entity."]
    pub owner: Option<bool>,
    #[doc = "The percent owned by the person of the account's legal entity."]
    pub percent_ownership: Option<json::OwnedValue>,
    #[doc = "The person's title (e.g., CEO, Support Engineer)."]
    pub title: Option<String>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsAccountIndividualRelationship {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("director", self.director.as_ref());
        form.field_opt("executive", self.executive.as_ref());
        form.field_opt("owner", self.owner.as_ref());
        form.field_opt("title", self.title.as_ref());
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTokenParamsBankAccountAccountHolderType {
    Company,
    Individual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tokens")]
impl CreateTokenParamsBankAccountAccountHolderType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTokenParamsBankAccountAccountHolderType::Company => "company",
            CreateTokenParamsBankAccountAccountHolderType::Individual => "individual",
            CreateTokenParamsBankAccountAccountHolderType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tokens")]
impl<'json> json::Decode<'json> for CreateTokenParamsBankAccountAccountHolderType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTokenParamsBankAccountAccountHolderType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "company" => CreateTokenParamsBankAccountAccountHolderType::Company,
            "individual" => CreateTokenParamsBankAccountAccountHolderType::Individual,
            other => CreateTokenParamsBankAccountAccountHolderType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsBankAccountAccountHolderType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTokenParamsBankAccountAccountType {
    Checking,
    Futsu,
    Savings,
    Toza,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tokens")]
impl CreateTokenParamsBankAccountAccountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTokenParamsBankAccountAccountType::Checking => "checking",
            CreateTokenParamsBankAccountAccountType::Futsu => "futsu",
            CreateTokenParamsBankAccountAccountType::Savings => "savings",
            CreateTokenParamsBankAccountAccountType::Toza => "toza",
            CreateTokenParamsBankAccountAccountType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tokens")]
impl<'json> json::Decode<'json> for CreateTokenParamsBankAccountAccountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTokenParamsBankAccountAccountType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "checking" => CreateTokenParamsBankAccountAccountType::Checking,
            "futsu" => CreateTokenParamsBankAccountAccountType::Futsu,
            "savings" => CreateTokenParamsBankAccountAccountType::Savings,
            "toza" => CreateTokenParamsBankAccountAccountType::Toza,
            other => CreateTokenParamsBankAccountAccountType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsBankAccountAccountType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem {
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

#[cfg(feature = "tokens")]
impl CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::AfricanAmerican => "african_american",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::AmericanIndianOrAlaskaNative => {
                "american_indian_or_alaska_native"
            }
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Asian => "asian",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::AsianIndian => "asian_indian",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::BlackOrAfricanAmerican => "black_or_african_american",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Chinese => "chinese",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Ethiopian => "ethiopian",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Filipino => "filipino",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::GuamanianOrChamorro => "guamanian_or_chamorro",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Haitian => "haitian",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Jamaican => "jamaican",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Japanese => "japanese",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Korean => "korean",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::NativeHawaiian => "native_hawaiian",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::NativeHawaiianOrOtherPacificIslander => {
                "native_hawaiian_or_other_pacific_islander"
            }
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Nigerian => "nigerian",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::OtherAsian => "other_asian",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::OtherBlackOrAfricanAmerican => {
                "other_black_or_african_american"
            }
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::OtherPacificIslander => "other_pacific_islander",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::PreferNotToAnswer => "prefer_not_to_answer",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Samoan => "samoan",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Somali => "somali",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Vietnamese => "vietnamese",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::White => "white",
            CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tokens")]
impl<'json> json::Decode<'json> for CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "african_american" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::AfricanAmerican,
            "american_indian_or_alaska_native" => {
                CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::AmericanIndianOrAlaskaNative
            }
            "asian" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Asian,
            "asian_indian" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::AsianIndian,
            "black_or_african_american" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::BlackOrAfricanAmerican,
            "chinese" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Chinese,
            "ethiopian" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Ethiopian,
            "filipino" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Filipino,
            "guamanian_or_chamorro" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::GuamanianOrChamorro,
            "haitian" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Haitian,
            "jamaican" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Jamaican,
            "japanese" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Japanese,
            "korean" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Korean,
            "native_hawaiian" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::NativeHawaiian,
            "native_hawaiian_or_other_pacific_islander" => {
                CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::NativeHawaiianOrOtherPacificIslander
            }
            "nigerian" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Nigerian,
            "other_asian" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::OtherAsian,
            "other_black_or_african_american" => {
                CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::OtherBlackOrAfricanAmerican
            }
            "other_pacific_islander" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::OtherPacificIslander,
            "prefer_not_to_answer" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::PreferNotToAnswer,
            "samoan" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Samoan,
            "somali" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Somali,
            "vietnamese" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Vietnamese,
            "white" => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::White,
            other => CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem {
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
pub struct TokenCardNetworks {
    #[doc = "The preferred network for co-branded cards."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferred: Option<String>,
}
