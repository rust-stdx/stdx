#[cfg(feature = "confirmation_tokens")]
#[doc = "Retrieve a ConfirmationToken"]
pub async fn get_confirmation_token(
    client: &crate::Client,
    confirmation_token: &str,
    params: &stripe_types_02::confirmation_tokens::GetConfirmationTokenParams,
) -> Result<stripe_types_15::confirmation_tokens::ConfirmationToken, crate::Error> {
    let path = crate::params::format_path(
        "/v1/confirmation_tokens/{confirmation_token}",
        &[("confirmation_token", confirmation_token)],
    );
    client.get(&path, params).await
}
