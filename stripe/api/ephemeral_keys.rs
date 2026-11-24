#[cfg(feature = "ephemeral_keys")]
#[doc = "Create an ephemeral key"]
pub async fn create_ephemeral_key(
    client: &crate::Client,
    params: &stripe_types_02::ephemeral_keys::CreateEphemeralKeyParams,
) -> Result<stripe_types_05::ephemeral_keys::EphemeralKey, crate::Error> {
    let path = crate::params::format_path("/v1/ephemeral_keys", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "ephemeral_keys")]
#[doc = "Immediately invalidate an ephemeral key"]
pub async fn delete_ephemeral_key(
    client: &crate::Client,
    key: &str,
    params: &stripe_types_00::ephemeral_keys::DeleteEphemeralKeyParams,
) -> Result<stripe_types_05::ephemeral_keys::EphemeralKey, crate::Error> {
    let path = crate::params::format_path("/v1/ephemeral_keys/{key}", &[("key", key)]);
    client.delete_form(&path, params).await
}
