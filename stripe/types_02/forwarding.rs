#[cfg(feature = "forwarding")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ForwardedRequestHeader {
    #[doc = "The header name."]
    #[json(default)]
    pub name: String,
    #[doc = "The header value."]
    #[json(default)]
    pub value: String,
}
