#[cfg(feature = "forwarding")]
#[derive(Debug, Clone, PartialEq)]
pub enum ForwardedRequestDetailsHttpMethod {
    POST,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "forwarding")]
impl ForwardedRequestDetailsHttpMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ForwardedRequestDetailsHttpMethod::POST => "POST",
            ForwardedRequestDetailsHttpMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "forwarding")]
impl Default for ForwardedRequestDetailsHttpMethod {
    fn default() -> Self {
        ForwardedRequestDetailsHttpMethod::Unknown(String::new())
    }
}

#[cfg(feature = "forwarding")]
impl<'json> json::Decode<'json> for ForwardedRequestDetailsHttpMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ForwardedRequestDetailsHttpMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "POST" => ForwardedRequestDetailsHttpMethod::POST,
            other => ForwardedRequestDetailsHttpMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "forwarding")]
#[derive(Debug, Clone, Default)]
pub struct ListForwardingRequestsParamsCreated {
    #[doc = "Return results where the `created` field is greater than this value."]
    pub gt: Option<i64>,
    #[doc = "Return results where the `created` field is greater than or equal to this value."]
    pub gte: Option<i64>,
    #[doc = "Return results where the `created` field is less than this value."]
    pub lt: Option<i64>,
    #[doc = "Return results where the `created` field is less than or equal to this value."]
    pub lte: Option<i64>,
}

#[cfg(feature = "forwarding")]
impl crate::params::FormEncode for ListForwardingRequestsParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}
