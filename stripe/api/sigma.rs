#[cfg(feature = "sigma")]
#[doc = "Retrieve a scheduled query run"]
pub async fn get_sigma_scheduled_query_run(
    client: &crate::Client,
    scheduled_query_run: &str,
    params: &stripe_types_03::sigma::GetSigmaScheduledQueryRunParams,
) -> Result<stripe_types_09::sigma::ScheduledQueryRun, crate::Error> {
    let path = crate::params::format_path(
        "/v1/sigma/scheduled_query_runs/{scheduled_query_run}",
        &[("scheduled_query_run", scheduled_query_run)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "sigma")]
#[doc = "List all scheduled query runs"]
pub async fn list_sigma_scheduled_query_runs(
    client: &crate::Client,
    params: &stripe_types_03::sigma::ListSigmaScheduledQueryRunsParams,
) -> Result<crate::List<stripe_types_09::sigma::ScheduledQueryRun>, crate::Error> {
    let path = crate::params::format_path("/v1/sigma/scheduled_query_runs", &[]);
    client.get(&path, params).await
}
