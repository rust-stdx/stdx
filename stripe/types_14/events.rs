#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingTokenCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_13::issuing::IssuingToken>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingTokenUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_13::issuing::IssuingToken>,
}
