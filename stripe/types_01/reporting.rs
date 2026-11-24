#[cfg(any(feature = "events", feature = "reporting"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialReportingFinanceReportRunRunParameters {
    #[doc = "The set of output columns requested for inclusion in the report run."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    #[doc = "Connected account ID by which to filter the report run."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub connected_account: Option<String>,
    #[doc = "Currency of objects to be included in the report run."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "Ending timestamp of data to be included in the report run."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub interval_end: Option<i64>,
    #[doc = "Starting timestamp of data to be included in the report run."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub interval_start: Option<i64>,
    #[doc = "Payout ID by which to filter the report run."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payout: Option<String>,
    #[doc = "Category of balance transactions to be included in the report run."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reporting_category: Option<String>,
    #[doc = "Defaults to `Etc/UTC`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[cfg(feature = "reporting")]
#[derive(Debug, Clone, Default)]
pub struct GetReportingReportRunParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "reporting")]
impl crate::params::FormEncode for GetReportingReportRunParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
