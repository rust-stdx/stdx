#[cfg(feature = "forwarding")]
#[derive(Debug, Clone, Default)]
pub struct CreateForwardingRequestParamsRequest {
    #[doc = "The body payload to send to the destination endpoint."]
    pub body: Option<String>,
    #[doc = "The headers to include in the forwarded request."]
    pub headers: Option<Vec<stripe_types_00::forwarding::CreateForwardingRequestParamsRequestHeadersItem>>,
}

#[cfg(feature = "forwarding")]
impl crate::params::FormEncode for CreateForwardingRequestParamsRequest {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("body", self.body.as_ref());
        if let Some(values) = self.headers.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("headers", index, item);
            }
        }
    }
}

#[cfg(feature = "forwarding")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ForwardedRequestDetails {
    #[doc = "The body payload to send to the destination endpoint."]
    #[json(default)]
    pub body: String,
    #[doc = "The headers to include in the forwarded request."]
    #[json(default)]
    pub headers: Vec<stripe_types_02::forwarding::ForwardedRequestHeader>,
    #[doc = "The HTTP method used to call the destination endpoint."]
    #[json(default)]
    pub http_method: stripe_types_01::forwarding::ForwardedRequestDetailsHttpMethod,
}

#[cfg(feature = "forwarding")]
#[derive(Debug, Clone, Default)]
pub struct ListForwardingRequestsParams {
    pub created: Option<stripe_types_01::forwarding::ListForwardingRequestsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "forwarding")]
impl crate::params::FormEncode for ListForwardingRequestsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
