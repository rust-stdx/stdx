#[cfg(feature = "forwarding")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ForwardedRequestContext {
    #[doc = "The time it took in milliseconds for the destination endpoint to respond."]
    #[json(default)]
    pub destination_duration: i64,
    #[doc = "The IP address of the destination."]
    #[json(default)]
    pub destination_ip_address: String,
}

#[cfg(feature = "forwarding")]
#[derive(Debug, Clone, PartialEq)]
pub enum ForwardingRequestObject {
    ForwardingRequest,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "forwarding")]
impl ForwardingRequestObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ForwardingRequestObject::ForwardingRequest => "forwarding.request",
            ForwardingRequestObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "forwarding")]
impl Default for ForwardingRequestObject {
    fn default() -> Self {
        ForwardingRequestObject::Unknown(String::new())
    }
}

#[cfg(feature = "forwarding")]
impl<'json> json::Decode<'json> for ForwardingRequestObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ForwardingRequestObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "forwarding.request" => ForwardingRequestObject::ForwardingRequest,
            other => ForwardingRequestObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "forwarding")]
#[derive(Debug, Clone, Default)]
pub struct GetForwardingRequestParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "forwarding")]
impl crate::params::FormEncode for GetForwardingRequestParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
