#[cfg(feature = "apps")]
#[doc = "Create an app install"]
pub async fn create_app_install(
    client: &crate::Client,
    params: &stripe_types_08::apps::CreateAppInstallParams,
) -> Result<stripe_types_05::apps::AppsInstall, crate::Error> {
    let path = crate::params::format_path("/v1/apps/installs", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "apps")]
#[doc = "Set a Secret"]
pub async fn create_app_secret(
    client: &crate::Client,
    params: &stripe_types_11::apps::CreateAppSecretParams,
) -> Result<stripe_types_08::apps::AppsSecret, crate::Error> {
    let path = crate::params::format_path("/v1/apps/secrets", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "apps")]
#[doc = "Delete a Secret"]
pub async fn delete_app_secret(
    client: &crate::Client,
    params: &stripe_types_11::apps::DeleteAppSecretParams,
) -> Result<stripe_types_08::apps::AppsSecret, crate::Error> {
    let path = crate::params::format_path("/v1/apps/secrets/delete", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "apps")]
#[doc = "Retrieve an app install"]
pub async fn get_app_install(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_00::apps::GetAppInstallParams,
) -> Result<stripe_types_05::apps::AppsInstall, crate::Error> {
    let path = crate::params::format_path("/v1/apps/installs/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "apps")]
#[doc = "Find a Secret"]
pub async fn get_app_secret_find(
    client: &crate::Client,
    params: &stripe_types_10::apps::GetAppSecretFindParams,
) -> Result<stripe_types_08::apps::AppsSecret, crate::Error> {
    let path = crate::params::format_path("/v1/apps/secrets/find", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "apps")]
#[doc = "List all app installs"]
pub async fn list_app_installs(
    client: &crate::Client,
    params: &stripe_types_06::apps::ListAppInstallsParams,
) -> Result<crate::List<stripe_types_05::apps::AppsInstall>, crate::Error> {
    let path = crate::params::format_path("/v1/apps/installs", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "apps")]
#[doc = "List secrets"]
pub async fn list_app_secrets(
    client: &crate::Client,
    params: &stripe_types_07::apps::ListAppSecretsParams,
) -> Result<crate::List<stripe_types_08::apps::AppsSecret>, crate::Error> {
    let path = crate::params::format_path("/v1/apps/secrets", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "apps")]
#[doc = "Uninstall an app install"]
pub async fn uninstall_app_install(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::apps::UninstallAppInstallParams,
) -> Result<stripe_types_05::apps::AppsInstall, crate::Error> {
    let path = crate::params::format_path("/v1/apps/installs/{id}/uninstall", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "apps")]
#[doc = "Update an app install"]
pub async fn update_app_install(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_05::apps::UpdateAppInstallParams,
) -> Result<stripe_types_05::apps::AppsInstall, crate::Error> {
    let path = crate::params::format_path("/v1/apps/installs/{id}", &[("id", id)]);
    client.post_form(&path, params).await
}
