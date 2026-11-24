#[cfg(feature = "reporting")]
#[derive(Debug, Clone, Default)]
pub struct CreateReportingReportRunParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Parameters specifying how the report should be run."]
    pub parameters: Option<stripe_types_05::reporting::CreateReportingReportRunParamsParameters>,
    #[doc = "The ID of the [report type](https://docs.stripe.com/reporting/statements/api#report-types) to run, such as `'balance...."]
    pub report_type: String,
}

#[cfg(feature = "reporting")]
impl crate::params::FormEncode for CreateReportingReportRunParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("parameters", self.parameters.as_ref());
        form.field("report_type", &self.report_type);
    }
}

#[cfg(any(feature = "events", feature = "reporting"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReportingReportRun {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "If something should go wrong during the run, a message about the failure (populated when  `status=failed`)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "`true` if the report is run on live mode data and `false` if it is run on test mode data."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::reporting::ReportingReportRunObject,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<stripe_types_01::reporting::FinancialReportingFinanceReportRunRunParameters>,
    #[doc = "The ID of the [report type](https://docs.stripe.com/reports/report-types) to run, such as `'balance.summary.1'`."]
    #[json(default)]
    pub report_type: String,
    #[doc = "The file object representing the result of the report run (populated when  `status=succeeded`)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<stripe_types_06::files::File>,
    #[doc = "Status of this report run."]
    #[json(default)]
    pub status: String,
    #[doc = "Timestamp at which this run successfully finished (populated when  `status=succeeded`)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub succeeded_at: Option<i64>,
}

#[cfg(any(feature = "events", feature = "reporting"))]
impl crate::PagedItem for ReportingReportRun {
    fn id(&self) -> &str {
        &self.id
    }
}
