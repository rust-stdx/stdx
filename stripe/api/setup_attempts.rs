#[cfg(feature = "setup_attempts")]
#[doc = "List all SetupAttempts"]
pub async fn list_setup_attempts(
    client: &crate::Client,
    params: &stripe_types_05::setup_attempts::ListSetupAttemptsParams,
) -> Result<crate::List<stripe_types_14::setup_attempts::SetupAttempt>, crate::Error> {
    let path = crate::params::format_path("/v1/setup_attempts", &[]);
    client.get(&path, params).await
}
