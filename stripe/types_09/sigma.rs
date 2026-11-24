#[cfg(any(feature = "events", feature = "sigma"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ScheduledQueryRun {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "When the query was run, Sigma contained a snapshot of your Stripe data at this time."]
    #[json(default)]
    pub data_load_time: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<stripe_types_02::sigma::SigmaScheduledQueryRunError>,
    #[doc = "The file object representing the results of the query."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<stripe_types_06::files::File>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::sigma::ScheduledQueryRunObject,
    #[doc = "Time at which the result expires and is no longer available for download."]
    #[json(default)]
    pub result_available_until: i64,
    #[doc = "SQL for the query."]
    #[json(default)]
    pub sql: String,
    #[doc = "The query's execution status, which will be `completed` for successful runs, and `canceled`, `failed`, or `timed_out`..."]
    #[json(default)]
    pub status: String,
    #[doc = "Title of the query."]
    #[json(default)]
    pub title: String,
}

#[cfg(any(feature = "events", feature = "sigma"))]
impl crate::PagedItem for ScheduledQueryRun {
    fn id(&self) -> &str {
        &self.id
    }
}
