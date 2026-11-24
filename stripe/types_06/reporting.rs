#[cfg(feature = "reporting")]
pub type GetReportingReportTypeParams = stripe_types_01::reporting::GetReportingReportRunParams;

#[cfg(feature = "reporting")]
#[derive(Debug, Clone, Default)]
pub struct ListReportingReportRunsParams {
    pub created: Option<stripe_types_00::reporting::ListReportingReportRunsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "reporting")]
impl crate::params::FormEncode for ListReportingReportRunsParams {
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

#[cfg(any(feature = "events", feature = "reporting"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReportingReportType {
    #[doc = "Most recent time for which this Report Type is available."]
    #[json(default)]
    pub data_available_end: i64,
    #[doc = "Earliest time for which this Report Type is available."]
    #[json(default)]
    pub data_available_start: i64,
    #[doc = "List of column names that are included by default when this Report Type gets run."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_columns: Option<Vec<String>>,
    #[doc = "The [ID of the Report Type](https://docs.stripe.com/reporting/statements/api#available-report-types), such as `balanc..."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Human-readable name of the Report Type"]
    #[json(default)]
    pub name: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::reporting::ReportingReportTypeObject,
    #[doc = "When this Report Type was latest updated."]
    #[json(default)]
    pub updated: i64,
    #[doc = "Version of the Report Type."]
    #[json(default)]
    pub version: i64,
}

#[cfg(any(feature = "events", feature = "reporting"))]
impl crate::PagedItem for ReportingReportType {
    fn id(&self) -> &str {
        &self.id
    }
}
