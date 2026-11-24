#[cfg(any(feature = "climate", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ClimateOrderStatus {
    AwaitingFunds,
    Canceled,
    Confirmed,
    Delivered,
    Open,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "climate", feature = "events"))]
impl ClimateOrderStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ClimateOrderStatus::AwaitingFunds => "awaiting_funds",
            ClimateOrderStatus::Canceled => "canceled",
            ClimateOrderStatus::Confirmed => "confirmed",
            ClimateOrderStatus::Delivered => "delivered",
            ClimateOrderStatus::Open => "open",
            ClimateOrderStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "climate", feature = "events"))]
impl Default for ClimateOrderStatus {
    fn default() -> Self {
        ClimateOrderStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "climate", feature = "events"))]
impl<'json> json::Decode<'json> for ClimateOrderStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ClimateOrderStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "awaiting_funds" => ClimateOrderStatus::AwaitingFunds,
            "canceled" => ClimateOrderStatus::Canceled,
            "confirmed" => ClimateOrderStatus::Confirmed,
            "delivered" => ClimateOrderStatus::Delivered,
            "open" => ClimateOrderStatus::Open,
            other => ClimateOrderStatus::Unknown(other.to_string()),
        })
    }
}
