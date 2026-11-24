#[cfg(feature = "plans")]
#[derive(Debug, Clone, Default)]
pub struct CreatePlanParamsProductTaxDetails {
    #[doc = "A tax location ID."]
    pub performance_location: Option<String>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<json::OwnedValue>,
}

#[cfg(feature = "plans")]
impl crate::params::FormEncode for CreatePlanParamsProductTaxDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("performance_location", self.performance_location.as_ref());
    }
}

#[cfg(feature = "plans")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePlanParamsTransformUsageRound {
    Down,
    Up,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "plans")]
impl CreatePlanParamsTransformUsageRound {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePlanParamsTransformUsageRound::Down => "down",
            CreatePlanParamsTransformUsageRound::Up => "up",
            CreatePlanParamsTransformUsageRound::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "plans")]
impl<'json> json::Decode<'json> for CreatePlanParamsTransformUsageRound {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePlanParamsTransformUsageRound::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "down" => CreatePlanParamsTransformUsageRound::Down,
            "up" => CreatePlanParamsTransformUsageRound::Up,
            other => CreatePlanParamsTransformUsageRound::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "plans")]
impl crate::params::FormEncode for CreatePlanParamsTransformUsageRound {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "plans")]
#[derive(Debug, Clone, Default)]
pub struct ListPlansParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "plans")]
impl crate::params::FormEncode for ListPlansParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}
