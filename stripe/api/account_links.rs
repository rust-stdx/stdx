#[cfg(feature = "account_links")]
#[doc = "Create an account link"]
pub async fn create_account_link(
    client: &crate::Client,
    params: &stripe_types_10::account_links::CreateAccountLinkParams,
) -> Result<stripe_types_06::account_links::AccountLink, crate::Error> {
    let path = crate::params::format_path("/v1/account_links", &[]);
    client.post_form(&path, params).await
}
