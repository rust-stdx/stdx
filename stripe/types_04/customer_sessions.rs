#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodSaveUsage {
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "customer_sessions")]
impl CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodSaveUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodSaveUsage::OffSession => {
                "off_session"
            }
            CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodSaveUsage::OnSession => {
                "on_session"
            }
            CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodSaveUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "customer_sessions")]
impl<'json> json::Decode<'json> for CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodSaveUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodSaveUsage::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "off_session" => {
                CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodSaveUsage::OffSession
            }
            "on_session" => {
                CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodSaveUsage::OnSession
            }
            other => CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodSaveUsage::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "customer_sessions")]
impl crate::params::FormEncode for CreateCustomerSessionParamsComponentsPaymentElementFeaturesPaymentMethodSaveUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
