#[cfg(any(feature = "events", feature = "sigma"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SigmaScheduledQueryRunError {
    #[doc = "Information about the run failure."]
    #[json(default)]
    pub message: String,
}
