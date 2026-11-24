#[cfg(feature = "forwarding")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ForwardingRequest {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::forwarding::ForwardingRequestObject,
    #[doc = "The PaymentMethod to insert into the forwarded request."]
    #[json(default)]
    pub payment_method: String,
    #[doc = "The field kinds to be replaced in the forwarded request."]
    #[json(default)]
    pub replacements: Vec<stripe_types_04::forwarding::ForwardingRequestReplacementsItem>,
    #[doc = "Context about the request from Stripe's servers to the destination endpoint."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request_context: Option<stripe_types_03::forwarding::ForwardedRequestContext>,
    #[doc = "The request that was sent to the destination endpoint."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request_details: Option<stripe_types_05::forwarding::ForwardedRequestDetails>,
    #[doc = "The response that the destination endpoint returned to us."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub response_details: Option<stripe_types_06::forwarding::ForwardedResponseDetails>,
    #[doc = "The destination URL for the forwarded request."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg(feature = "forwarding")]
impl crate::PagedItem for ForwardingRequest {
    fn id(&self) -> &str {
        &self.id
    }
}
