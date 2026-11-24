#[cfg(feature = "tokens")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTokenParamsAccountBusinessType {
    Company,
    GovernmentEntity,
    Individual,
    NonProfit,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tokens")]
impl CreateTokenParamsAccountBusinessType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTokenParamsAccountBusinessType::Company => "company",
            CreateTokenParamsAccountBusinessType::GovernmentEntity => "government_entity",
            CreateTokenParamsAccountBusinessType::Individual => "individual",
            CreateTokenParamsAccountBusinessType::NonProfit => "non_profit",
            CreateTokenParamsAccountBusinessType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tokens")]
impl<'json> json::Decode<'json> for CreateTokenParamsAccountBusinessType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTokenParamsAccountBusinessType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "company" => CreateTokenParamsAccountBusinessType::Company,
            "government_entity" => CreateTokenParamsAccountBusinessType::GovernmentEntity,
            "individual" => CreateTokenParamsAccountBusinessType::Individual,
            "non_profit" => CreateTokenParamsAccountBusinessType::NonProfit,
            other => CreateTokenParamsAccountBusinessType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsAccountBusinessType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsAccountCompanyRegistrationDate {
    #[doc = "The day of registration, between 1 and 31."]
    pub day: i64,
    #[doc = "The month of registration, between 1 and 12."]
    pub month: i64,
    #[doc = "The four-digit year of registration."]
    pub year: i64,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsAccountCompanyRegistrationDate {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("day", &self.day);
        form.field("month", &self.month);
        form.field("year", &self.year);
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTokenParamsAccountCompanyStructure {
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

#[cfg(feature = "tokens")]
impl CreateTokenParamsAccountCompanyStructure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTokenParamsAccountCompanyStructure::Unnamed => "",
            CreateTokenParamsAccountCompanyStructure::FreeZoneEstablishment => "free_zone_establishment",
            CreateTokenParamsAccountCompanyStructure::FreeZoneLlc => "free_zone_llc",
            CreateTokenParamsAccountCompanyStructure::GovernmentInstrumentality => "government_instrumentality",
            CreateTokenParamsAccountCompanyStructure::GovernmentalUnit => "governmental_unit",
            CreateTokenParamsAccountCompanyStructure::IncorporatedNonProfit => "incorporated_non_profit",
            CreateTokenParamsAccountCompanyStructure::IncorporatedPartnership => "incorporated_partnership",
            CreateTokenParamsAccountCompanyStructure::LimitedLiabilityPartnership => "limited_liability_partnership",
            CreateTokenParamsAccountCompanyStructure::Llc => "llc",
            CreateTokenParamsAccountCompanyStructure::MultiMemberLlc => "multi_member_llc",
            CreateTokenParamsAccountCompanyStructure::PrivateCompany => "private_company",
            CreateTokenParamsAccountCompanyStructure::PrivateCorporation => "private_corporation",
            CreateTokenParamsAccountCompanyStructure::PrivatePartnership => "private_partnership",
            CreateTokenParamsAccountCompanyStructure::PublicCompany => "public_company",
            CreateTokenParamsAccountCompanyStructure::PublicCorporation => "public_corporation",
            CreateTokenParamsAccountCompanyStructure::PublicPartnership => "public_partnership",
            CreateTokenParamsAccountCompanyStructure::RegisteredCharity => "registered_charity",
            CreateTokenParamsAccountCompanyStructure::SingleMemberLlc => "single_member_llc",
            CreateTokenParamsAccountCompanyStructure::SoleEstablishment => "sole_establishment",
            CreateTokenParamsAccountCompanyStructure::SoleProprietorship => "sole_proprietorship",
            CreateTokenParamsAccountCompanyStructure::TaxExemptGovernmentInstrumentality => {
                "tax_exempt_government_instrumentality"
            }
            CreateTokenParamsAccountCompanyStructure::UnincorporatedAssociation => "unincorporated_association",
            CreateTokenParamsAccountCompanyStructure::UnincorporatedNonProfit => "unincorporated_non_profit",
            CreateTokenParamsAccountCompanyStructure::UnincorporatedPartnership => "unincorporated_partnership",
            CreateTokenParamsAccountCompanyStructure::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tokens")]
impl<'json> json::Decode<'json> for CreateTokenParamsAccountCompanyStructure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTokenParamsAccountCompanyStructure::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateTokenParamsAccountCompanyStructure::Unnamed,
            "free_zone_establishment" => CreateTokenParamsAccountCompanyStructure::FreeZoneEstablishment,
            "free_zone_llc" => CreateTokenParamsAccountCompanyStructure::FreeZoneLlc,
            "government_instrumentality" => CreateTokenParamsAccountCompanyStructure::GovernmentInstrumentality,
            "governmental_unit" => CreateTokenParamsAccountCompanyStructure::GovernmentalUnit,
            "incorporated_non_profit" => CreateTokenParamsAccountCompanyStructure::IncorporatedNonProfit,
            "incorporated_partnership" => CreateTokenParamsAccountCompanyStructure::IncorporatedPartnership,
            "limited_liability_partnership" => CreateTokenParamsAccountCompanyStructure::LimitedLiabilityPartnership,
            "llc" => CreateTokenParamsAccountCompanyStructure::Llc,
            "multi_member_llc" => CreateTokenParamsAccountCompanyStructure::MultiMemberLlc,
            "private_company" => CreateTokenParamsAccountCompanyStructure::PrivateCompany,
            "private_corporation" => CreateTokenParamsAccountCompanyStructure::PrivateCorporation,
            "private_partnership" => CreateTokenParamsAccountCompanyStructure::PrivatePartnership,
            "public_company" => CreateTokenParamsAccountCompanyStructure::PublicCompany,
            "public_corporation" => CreateTokenParamsAccountCompanyStructure::PublicCorporation,
            "public_partnership" => CreateTokenParamsAccountCompanyStructure::PublicPartnership,
            "registered_charity" => CreateTokenParamsAccountCompanyStructure::RegisteredCharity,
            "single_member_llc" => CreateTokenParamsAccountCompanyStructure::SingleMemberLlc,
            "sole_establishment" => CreateTokenParamsAccountCompanyStructure::SoleEstablishment,
            "sole_proprietorship" => CreateTokenParamsAccountCompanyStructure::SoleProprietorship,
            "tax_exempt_government_instrumentality" => {
                CreateTokenParamsAccountCompanyStructure::TaxExemptGovernmentInstrumentality
            }
            "unincorporated_association" => CreateTokenParamsAccountCompanyStructure::UnincorporatedAssociation,
            "unincorporated_non_profit" => CreateTokenParamsAccountCompanyStructure::UnincorporatedNonProfit,
            "unincorporated_partnership" => CreateTokenParamsAccountCompanyStructure::UnincorporatedPartnership,
            other => CreateTokenParamsAccountCompanyStructure::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsAccountCompanyStructure {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsCvcUpdate {
    #[doc = "The CVC value, in string form."]
    pub cvc: String,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsCvcUpdate {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("cvc", &self.cvc);
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsPersonAdditionalTosAcceptancesAccount {
    #[doc = "The Unix timestamp marking when the account representative accepted the service agreement."]
    pub date: Option<i64>,
    #[doc = "The IP address from which the account representative accepted the service agreement."]
    pub ip: Option<String>,
    #[doc = "The user agent of the browser from which the account representative accepted the service agreement."]
    pub user_agent: Option<json::OwnedValue>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsPersonAdditionalTosAcceptancesAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("date", self.date.as_ref());
        form.field_opt("ip", self.ip.as_ref());
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsPersonRelationship {
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

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsPersonRelationship {
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

#[cfg(any(feature = "financial_connections", feature = "tokens"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenObject {
    Token,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "financial_connections", feature = "tokens"))]
impl TokenObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TokenObject::Token => "token",
            TokenObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "financial_connections", feature = "tokens"))]
impl Default for TokenObject {
    fn default() -> Self {
        TokenObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "financial_connections", feature = "tokens"))]
impl<'json> json::Decode<'json> for TokenObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TokenObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "token" => TokenObject::Token,
            other => TokenObject::Unknown(other.to_string()),
        })
    }
}
