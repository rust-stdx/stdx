#[cfg(feature = "v2_billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateBillingMeterEventAdjustmentParamsType {
    Cancel,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_billing")]
impl CreateBillingMeterEventAdjustmentParamsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateBillingMeterEventAdjustmentParamsType::Cancel => "cancel",
            CreateBillingMeterEventAdjustmentParamsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_billing")]
impl json::Encode for CreateBillingMeterEventAdjustmentParamsType {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_billing")]
impl<'json> json::Decode<'json> for CreateBillingMeterEventAdjustmentParamsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateBillingMeterEventAdjustmentParamsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cancel" => CreateBillingMeterEventAdjustmentParamsType::Cancel,
            other => CreateBillingMeterEventAdjustmentParamsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_billing")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct MeterEventStreamResponse {}
