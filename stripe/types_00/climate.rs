#[cfg(any(feature = "climate", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ClimateOrderCancellationReason {
    Expired,
    ProductUnavailable,
    Requested,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "climate", feature = "events"))]
impl ClimateOrderCancellationReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ClimateOrderCancellationReason::Expired => "expired",
            ClimateOrderCancellationReason::ProductUnavailable => "product_unavailable",
            ClimateOrderCancellationReason::Requested => "requested",
            ClimateOrderCancellationReason::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "climate", feature = "events"))]
impl<'json> json::Decode<'json> for ClimateOrderCancellationReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ClimateOrderCancellationReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "expired" => ClimateOrderCancellationReason::Expired,
            "product_unavailable" => ClimateOrderCancellationReason::ProductUnavailable,
            "requested" => ClimateOrderCancellationReason::Requested,
            other => ClimateOrderCancellationReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "climate", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ClimateProductObject {
    ClimateProduct,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "climate", feature = "events"))]
impl ClimateProductObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ClimateProductObject::ClimateProduct => "climate.product",
            ClimateProductObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "climate", feature = "events"))]
impl Default for ClimateProductObject {
    fn default() -> Self {
        ClimateProductObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "climate", feature = "events"))]
impl<'json> json::Decode<'json> for ClimateProductObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ClimateProductObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "climate.product" => ClimateProductObject::ClimateProduct,
            other => ClimateProductObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "climate")]
#[derive(Debug, Clone, Default)]
pub struct UpdateClimateOrderParamsBeneficiary {
    #[doc = "Publicly displayable name for the end beneficiary of carbon removal."]
    pub public_name: json::OwnedValue,
}

#[cfg(feature = "climate")]
impl crate::params::FormEncode for UpdateClimateOrderParamsBeneficiary {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}
