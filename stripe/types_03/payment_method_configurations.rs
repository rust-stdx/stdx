#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethodConfigurationObject {
    PaymentMethodConfiguration,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_method_configurations")]
impl PaymentMethodConfigurationObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentMethodConfigurationObject::PaymentMethodConfiguration => "payment_method_configuration",
            PaymentMethodConfigurationObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_method_configurations")]
impl Default for PaymentMethodConfigurationObject {
    fn default() -> Self {
        PaymentMethodConfigurationObject::Unknown(String::new())
    }
}

#[cfg(feature = "payment_method_configurations")]
impl<'json> json::Decode<'json> for PaymentMethodConfigurationObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentMethodConfigurationObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "payment_method_configuration" => PaymentMethodConfigurationObject::PaymentMethodConfiguration,
            other => PaymentMethodConfigurationObject::Unknown(other.to_string()),
        })
    }
}
