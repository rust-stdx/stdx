#[cfg(feature = "tokens")]
#[doc = "Create a bank account token"]
pub async fn create_token(
    client: &crate::Client,
    params: &stripe_types_13::tokens::CreateTokenParams,
) -> Result<stripe_types_15::tokens::Token, crate::Error> {
    let path = crate::params::format_path("/v1/tokens", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "tokens")]
#[doc = "Retrieve a token"]
pub async fn get_token(
    client: &crate::Client,
    token: &str,
    params: &stripe_types_00::tokens::GetTokenParams,
) -> Result<stripe_types_15::tokens::Token, crate::Error> {
    let path = crate::params::format_path("/v1/tokens/{token}", &[("token", token)]);
    client.get(&path, params).await
}
