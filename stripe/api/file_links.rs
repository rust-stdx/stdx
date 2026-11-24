#[cfg(feature = "file_links")]
#[doc = "Create a file link"]
pub async fn create_file_link(
    client: &crate::Client,
    params: &stripe_types_00::file_links::CreateFileLinkParams,
) -> Result<stripe_types_06::file_links::FileLink, crate::Error> {
    let path = crate::params::format_path("/v1/file_links", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "file_links")]
#[doc = "Retrieve a file link"]
pub async fn get_file_link(
    client: &crate::Client,
    link: &str,
    params: &stripe_types_02::file_links::GetFileLinkParams,
) -> Result<stripe_types_06::file_links::FileLink, crate::Error> {
    let path = crate::params::format_path("/v1/file_links/{link}", &[("link", link)]);
    client.get(&path, params).await
}

#[cfg(feature = "file_links")]
#[doc = "List all file links"]
pub async fn list_file_links(
    client: &crate::Client,
    params: &stripe_types_06::file_links::ListFileLinksParams,
) -> Result<crate::List<stripe_types_06::file_links::FileLink>, crate::Error> {
    let path = crate::params::format_path("/v1/file_links", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "file_links")]
#[doc = "Update a file link"]
pub async fn update_file_link(
    client: &crate::Client,
    link: &str,
    params: &stripe_types_00::file_links::UpdateFileLinkParams,
) -> Result<stripe_types_06::file_links::FileLink, crate::Error> {
    let path = crate::params::format_path("/v1/file_links/{link}", &[("link", link)]);
    client.post_form(&path, params).await
}
