#[cfg(feature = "webhook_endpoints")]
#[derive(Debug, Clone, Default)]
pub struct ListWebhookEndpointsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "webhook_endpoints")]
impl crate::params::FormEncode for ListWebhookEndpointsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "webhook_endpoints")]
#[derive(Debug, Clone, PartialEq)]
pub enum WebhookEndpointObject {
    WebhookEndpoint,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "webhook_endpoints")]
impl WebhookEndpointObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            WebhookEndpointObject::WebhookEndpoint => "webhook_endpoint",
            WebhookEndpointObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "webhook_endpoints")]
impl Default for WebhookEndpointObject {
    fn default() -> Self {
        WebhookEndpointObject::Unknown(String::new())
    }
}

#[cfg(feature = "webhook_endpoints")]
impl<'json> json::Decode<'json> for WebhookEndpointObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(WebhookEndpointObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "webhook_endpoint" => WebhookEndpointObject::WebhookEndpoint,
            other => WebhookEndpointObject::Unknown(other.to_string()),
        })
    }
}
