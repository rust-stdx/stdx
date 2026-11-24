#[cfg(feature = "reporting")]
#[doc = "Create a Report Run"]
pub async fn create_reporting_report_run(
    client: &crate::Client,
    params: &stripe_types_08::reporting::CreateReportingReportRunParams,
) -> Result<stripe_types_08::reporting::ReportingReportRun, crate::Error> {
    let path = crate::params::format_path("/v1/reporting/report_runs", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "reporting")]
#[doc = "Retrieve a Report Run"]
pub async fn get_reporting_report_run(
    client: &crate::Client,
    report_run: &str,
    params: &stripe_types_01::reporting::GetReportingReportRunParams,
) -> Result<stripe_types_08::reporting::ReportingReportRun, crate::Error> {
    let path = crate::params::format_path("/v1/reporting/report_runs/{report_run}", &[("report_run", report_run)]);
    client.get(&path, params).await
}

#[cfg(feature = "reporting")]
#[doc = "Retrieve a Report Type"]
pub async fn get_reporting_report_type(
    client: &crate::Client,
    report_type: &str,
    params: &stripe_types_06::reporting::GetReportingReportTypeParams,
) -> Result<stripe_types_06::reporting::ReportingReportType, crate::Error> {
    let path = crate::params::format_path("/v1/reporting/report_types/{report_type}", &[("report_type", report_type)]);
    client.get(&path, params).await
}

#[cfg(feature = "reporting")]
#[doc = "List all Report Runs"]
pub async fn list_reporting_report_runs(
    client: &crate::Client,
    params: &stripe_types_06::reporting::ListReportingReportRunsParams,
) -> Result<crate::List<stripe_types_08::reporting::ReportingReportRun>, crate::Error> {
    let path = crate::params::format_path("/v1/reporting/report_runs", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "reporting")]
#[doc = "List all Report Types"]
pub async fn list_reporting_report_types(
    client: &crate::Client,
    params: &stripe_types_05::reporting::ListReportingReportTypesParams,
) -> Result<crate::List<stripe_types_06::reporting::ReportingReportType>, crate::Error> {
    let path = crate::params::format_path("/v1/reporting/report_types", &[]);
    client.get(&path, params).await
}
