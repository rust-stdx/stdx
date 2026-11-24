#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentMethodConfigurationParamsAcssDebitDisplayPreferencePreference {
    None,
    Off,
    On,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_method_configurations")]
impl CreatePaymentMethodConfigurationParamsAcssDebitDisplayPreferencePreference {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentMethodConfigurationParamsAcssDebitDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationParamsAcssDebitDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationParamsAcssDebitDisplayPreferencePreference::On => "on",
            CreatePaymentMethodConfigurationParamsAcssDebitDisplayPreferencePreference::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_method_configurations")]
impl<'json> json::Decode<'json> for CreatePaymentMethodConfigurationParamsAcssDebitDisplayPreferencePreference {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentMethodConfigurationParamsAcssDebitDisplayPreferencePreference::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => CreatePaymentMethodConfigurationParamsAcssDebitDisplayPreferencePreference::None,
            "off" => CreatePaymentMethodConfigurationParamsAcssDebitDisplayPreferencePreference::Off,
            "on" => CreatePaymentMethodConfigurationParamsAcssDebitDisplayPreferencePreference::On,
            other => {
                CreatePaymentMethodConfigurationParamsAcssDebitDisplayPreferencePreference::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsAcssDebitDisplayPreferencePreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
