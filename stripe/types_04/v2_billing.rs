#[cfg(feature = "v2_billing")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2BillingMeterEventAdjustmentCancel {
    #[doc = "The identifier that was originally assigned to the meter event."]
    #[json(default)]
    pub identifier: String,
}

#[cfg(feature = "v2_billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2BillingMeterEventAdjustmentStatus {
    Complete,
    Pending,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_billing")]
impl V2BillingMeterEventAdjustmentStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2BillingMeterEventAdjustmentStatus::Complete => "complete",
            V2BillingMeterEventAdjustmentStatus::Pending => "pending",
            V2BillingMeterEventAdjustmentStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_billing")]
impl Default for V2BillingMeterEventAdjustmentStatus {
    fn default() -> Self {
        V2BillingMeterEventAdjustmentStatus::Unknown(String::new())
    }
}

#[cfg(feature = "v2_billing")]
impl<'json> json::Decode<'json> for V2BillingMeterEventAdjustmentStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2BillingMeterEventAdjustmentStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "complete" => V2BillingMeterEventAdjustmentStatus::Complete,
            "pending" => V2BillingMeterEventAdjustmentStatus::Pending,
            other => V2BillingMeterEventAdjustmentStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2BillingMeterEventSessionObject {
    V2BillingMeterEventSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_billing")]
impl V2BillingMeterEventSessionObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2BillingMeterEventSessionObject::V2BillingMeterEventSession => "v2.billing.meter_event_session",
            V2BillingMeterEventSessionObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_billing")]
impl Default for V2BillingMeterEventSessionObject {
    fn default() -> Self {
        V2BillingMeterEventSessionObject::Unknown(String::new())
    }
}

#[cfg(feature = "v2_billing")]
impl<'json> json::Decode<'json> for V2BillingMeterEventSessionObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2BillingMeterEventSessionObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.billing.meter_event_session" => V2BillingMeterEventSessionObject::V2BillingMeterEventSession,
            other => V2BillingMeterEventSessionObject::Unknown(other.to_string()),
        })
    }
}
