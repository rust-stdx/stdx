#[cfg(feature = "files")]
#[doc = "Create a file"]
///
/// # Unsupported encoding
///
/// This endpoint takes a `multipart/form-data` body, which `stripe` cannot encode
/// yet, so this method always returns [`Error::UnsupportedEncoding`](crate::Error::UnsupportedEncoding)
/// without sending a request.
pub async fn create_file(
    client: &crate::Client,
    params: &stripe_types_05::files::CreateFileParams,
) -> Result<stripe_types_06::files::File, crate::Error> {
    let _ = (client, params);
    Err(crate::Error::UnsupportedEncoding {
        endpoint: "/v1/files",
        encoding: "multipart/form-data",
    })
}

#[cfg(feature = "files")]
#[doc = "Retrieve a file"]
pub async fn get_file(
    client: &crate::Client,
    file: &str,
    params: &stripe_types_00::files::GetFileParams,
) -> Result<stripe_types_06::files::File, crate::Error> {
    let path = crate::params::format_path("/v1/files/{file}", &[("file", file)]);
    client.get(&path, params).await
}

#[cfg(feature = "files")]
#[doc = "List all files"]
pub async fn list_files(
    client: &crate::Client,
    params: &stripe_types_06::files::ListFilesParams,
) -> Result<crate::List<stripe_types_06::files::File>, crate::Error> {
    let path = crate::params::format_path("/v1/files", &[]);
    client.get(&path, params).await
}
