#[cfg(feature = "tokens")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem {
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

#[cfg(feature = "tokens")]
impl CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem::Cuban => "cuban",
            CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem::HispanicOrLatino => "hispanic_or_latino",
            CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem::Mexican => "mexican",
            CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem::NotHispanicOrLatino => {
                "not_hispanic_or_latino"
            }
            CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem::OtherHispanicOrLatino => {
                "other_hispanic_or_latino"
            }
            CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem::PreferNotToAnswer => "prefer_not_to_answer",
            CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem::PuertoRican => "puerto_rican",
            CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tokens")]
impl<'json> json::Decode<'json> for CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cuban" => CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem::Cuban,
            "hispanic_or_latino" => CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem::HispanicOrLatino,
            "mexican" => CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem::Mexican,
            "not_hispanic_or_latino" => {
                CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem::NotHispanicOrLatino
            }
            "other_hispanic_or_latino" => {
                CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem::OtherHispanicOrLatino
            }
            "prefer_not_to_answer" => CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem::PreferNotToAnswer,
            "puerto_rican" => CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem::PuertoRican,
            other => CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
