#[cfg(feature = "sources")]
#[doc = "Create a source"]
pub async fn create_source(
    client: &crate::Client,
    params: &stripe_types_11::sources::CreateSourceParams,
) -> Result<stripe_types_10::sources::Source, crate::Error> {
    let path = crate::params::format_path("/v1/sources", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "sources")]
#[doc = "Retrieve a source"]
pub async fn get_source(
    client: &crate::Client,
    source: &str,
    params: &stripe_types_03::sources::GetSourceParams,
) -> Result<stripe_types_10::sources::Source, crate::Error> {
    let path = crate::params::format_path("/v1/sources/{source}", &[("source", source)]);
    client.get(&path, params).await
}

#[cfg(feature = "sources")]
#[doc = "<p>List source transactions for a given source.</p>"]
pub async fn list_source_source_transactions(
    client: &crate::Client,
    source: &str,
    params: &stripe_types_02::sources::ListSourceSourceTransactionsParams,
) -> Result<crate::List<stripe_types_06::sources::SourceTransaction>, crate::Error> {
    let path = crate::params::format_path("/v1/sources/{source}/source_transactions", &[("source", source)]);
    client.get(&path, params).await
}

#[cfg(feature = "sources")]
#[doc = "Update a source"]
pub async fn update_source(
    client: &crate::Client,
    source: &str,
    params: &stripe_types_12::sources::UpdateSourceParams,
) -> Result<stripe_types_10::sources::Source, crate::Error> {
    let path = crate::params::format_path("/v1/sources/{source}", &[("source", source)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "sources")]
#[doc = "<p>Verify a given source.</p>"]
pub async fn verify_source(
    client: &crate::Client,
    source: &str,
    params: &stripe_types_01::sources::VerifySourceParams,
) -> Result<stripe_types_10::sources::Source, crate::Error> {
    let path = crate::params::format_path("/v1/sources/{source}/verify", &[("source", source)]);
    client.post_form(&path, params).await
}
