#[cfg(feature = "v2_billing")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateBillingMeterEventAdjustmentParamsCancel {
    #[doc = "The identifier that was originally assigned to the meter event."]
    pub identifier: String,
}

#[cfg(feature = "v2_billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2BillingMeterEventAdjustmentObject {
    V2BillingMeterEventAdjustment,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_billing")]
impl V2BillingMeterEventAdjustmentObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2BillingMeterEventAdjustmentObject::V2BillingMeterEventAdjustment => "v2.billing.meter_event_adjustment",
            V2BillingMeterEventAdjustmentObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_billing")]
impl Default for V2BillingMeterEventAdjustmentObject {
    fn default() -> Self {
        V2BillingMeterEventAdjustmentObject::Unknown(String::new())
    }
}

#[cfg(feature = "v2_billing")]
impl<'json> json::Decode<'json> for V2BillingMeterEventAdjustmentObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2BillingMeterEventAdjustmentObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.billing.meter_event_adjustment" => V2BillingMeterEventAdjustmentObject::V2BillingMeterEventAdjustment,
            other => V2BillingMeterEventAdjustmentObject::Unknown(other.to_string()),
        })
    }
}
