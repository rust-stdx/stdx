#[cfg(feature = "reporting")]
#[derive(Debug, Clone, Default)]
pub struct CreateReportingReportRunParamsParameters {
    #[doc = "The set of report columns to include in the report output."]
    pub columns: Option<Vec<String>>,
    #[doc = "Connected account ID to filter for in the report run."]
    pub connected_account: Option<String>,
    #[doc = "Currency of objects to be included in the report run."]
    pub currency: Option<String>,
    #[doc = "Ending timestamp of data to be included in the report run (exclusive)."]
    pub interval_end: Option<i64>,
    #[doc = "Starting timestamp of data to be included in the report run."]
    pub interval_start: Option<i64>,
    #[doc = "Payout ID by which to filter the report run."]
    pub payout: Option<String>,
    #[doc = "Category of balance transactions to be included in the report run."]
    pub reporting_category:
        Option<stripe_types_04::reporting::CreateReportingReportRunParamsParametersReportingCategory>,
    #[doc = "Defaults to `Etc/UTC`."]
    pub timezone: Option<stripe_types_00::reporting::CreateReportingReportRunParamsParametersTimezone>,
}

#[cfg(feature = "reporting")]
impl crate::params::FormEncode for CreateReportingReportRunParamsParameters {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.columns.as_ref() {
            form.field_array("columns", values.as_slice());
        }
        form.field_opt("connected_account", self.connected_account.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("interval_end", self.interval_end.as_ref());
        form.field_opt("interval_start", self.interval_start.as_ref());
        form.field_opt("payout", self.payout.as_ref());
        form.field_opt("reporting_category", self.reporting_category.as_ref());
        form.field_opt("timezone", self.timezone.as_ref());
    }
}

#[cfg(feature = "reporting")]
pub type ListReportingReportTypesParams = stripe_types_01::reporting::GetReportingReportRunParams;
