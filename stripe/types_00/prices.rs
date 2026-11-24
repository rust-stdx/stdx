#[cfg(feature = "prices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePriceParamsTiersMode {
    Graduated,
    Volume,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "prices")]
impl CreatePriceParamsTiersMode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePriceParamsTiersMode::Graduated => "graduated",
            CreatePriceParamsTiersMode::Volume => "volume",
            CreatePriceParamsTiersMode::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "prices")]
impl<'json> json::Decode<'json> for CreatePriceParamsTiersMode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePriceParamsTiersMode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "graduated" => CreatePriceParamsTiersMode::Graduated,
            "volume" => CreatePriceParamsTiersMode::Volume,
            other => CreatePriceParamsTiersMode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for CreatePriceParamsTiersMode {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "prices")]
#[derive(Debug, Clone, Default)]
pub struct ListPricesParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for ListPricesParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}
