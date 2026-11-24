#[cfg(any(feature = "climate", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ClimateOrderObject {
    ClimateOrder,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "climate", feature = "events"))]
impl ClimateOrderObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ClimateOrderObject::ClimateOrder => "climate.order",
            ClimateOrderObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "climate", feature = "events"))]
impl Default for ClimateOrderObject {
    fn default() -> Self {
        ClimateOrderObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "climate", feature = "events"))]
impl<'json> json::Decode<'json> for ClimateOrderObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ClimateOrderObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "climate.order" => ClimateOrderObject::ClimateOrder,
            other => ClimateOrderObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "climate", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ClimateRemovalsLocation {
    #[doc = "The city where the supplier is located."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Two-letter ISO code representing the country where the supplier is located."]
    #[json(default)]
    pub country: String,
    #[doc = "The geographic latitude where the supplier is located."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[doc = "The geographic longitude where the supplier is located."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[doc = "The state/county/province/region where the supplier is located."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[cfg(feature = "climate")]
#[derive(Debug, Clone, Default)]
pub struct GetClimateOrderParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "climate")]
impl crate::params::FormEncode for GetClimateOrderParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
