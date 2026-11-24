#[cfg(feature = "v2_billing")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct MeterEventStreamParamsEventsItem {
    #[doc = "The name of the meter event."]
    pub event_name: String,
    #[doc = "A unique identifier for the event."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[doc = "The payload of the event."]
    pub payload: std::collections::HashMap<String, String>,
    #[doc = "The time of the event."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[cfg(feature = "v2_billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2BillingMeterEventAdjustmentType {
    Cancel,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_billing")]
impl V2BillingMeterEventAdjustmentType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2BillingMeterEventAdjustmentType::Cancel => "cancel",
            V2BillingMeterEventAdjustmentType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_billing")]
impl Default for V2BillingMeterEventAdjustmentType {
    fn default() -> Self {
        V2BillingMeterEventAdjustmentType::Unknown(String::new())
    }
}

#[cfg(feature = "v2_billing")]
impl<'json> json::Decode<'json> for V2BillingMeterEventAdjustmentType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2BillingMeterEventAdjustmentType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cancel" => V2BillingMeterEventAdjustmentType::Cancel,
            other => V2BillingMeterEventAdjustmentType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2BillingMeterEventObject {
    V2BillingMeterEvent,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_billing")]
impl V2BillingMeterEventObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2BillingMeterEventObject::V2BillingMeterEvent => "v2.billing.meter_event",
            V2BillingMeterEventObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_billing")]
impl Default for V2BillingMeterEventObject {
    fn default() -> Self {
        V2BillingMeterEventObject::Unknown(String::new())
    }
}

#[cfg(feature = "v2_billing")]
impl<'json> json::Decode<'json> for V2BillingMeterEventObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2BillingMeterEventObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.billing.meter_event" => V2BillingMeterEventObject::V2BillingMeterEvent,
            other => V2BillingMeterEventObject::Unknown(other.to_string()),
        })
    }
}
