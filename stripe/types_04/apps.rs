#[cfg(feature = "apps")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListAppInstallsParamsChannel {
    PrivateLive,
    PrivateTest,
    Public,
    Testing,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "apps")]
impl ListAppInstallsParamsChannel {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListAppInstallsParamsChannel::PrivateLive => "private_live",
            ListAppInstallsParamsChannel::PrivateTest => "private_test",
            ListAppInstallsParamsChannel::Public => "public",
            ListAppInstallsParamsChannel::Testing => "testing",
            ListAppInstallsParamsChannel::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "apps")]
impl<'json> json::Decode<'json> for ListAppInstallsParamsChannel {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListAppInstallsParamsChannel::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "private_live" => ListAppInstallsParamsChannel::PrivateLive,
            "private_test" => ListAppInstallsParamsChannel::PrivateTest,
            "public" => ListAppInstallsParamsChannel::Public,
            "testing" => ListAppInstallsParamsChannel::Testing,
            other => ListAppInstallsParamsChannel::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "apps")]
impl crate::params::FormEncode for ListAppInstallsParamsChannel {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
