#[cfg(feature = "account_sessions")]
#[doc = "Create an Account Session"]
pub async fn create_account_session(
    client: &crate::Client,
    params: &stripe_types_12::account_sessions::CreateAccountSessionParams,
) -> Result<stripe_types_10::account_sessions::AccountSession, crate::Error> {
    let path = crate::params::format_path("/v1/account_sessions", &[]);
    client.post_form(&path, params).await
}
