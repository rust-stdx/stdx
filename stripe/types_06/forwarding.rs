#[cfg(feature = "forwarding")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ForwardedResponseDetails {
    #[doc = "The response body from the destination endpoint to Stripe."]
    #[json(default)]
    pub body: String,
    #[doc = "HTTP headers that the destination endpoint returned."]
    #[json(default)]
    pub headers: Vec<stripe_types_02::forwarding::ForwardedRequestHeader>,
    #[doc = "The HTTP status code that the destination endpoint returned."]
    #[json(default)]
    pub status: i64,
}
