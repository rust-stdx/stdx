#[cfg(any(feature = "apps", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct AppsInstall {
    #[doc = "The ID of the account that the app install belongs to."]
    #[json(default)]
    pub account: String,
    #[doc = "The ID of the app installed."]
    #[json(default)]
    pub app: String,
    #[doc = "Whether the installer must authorize pending permissions, content security policy entries, or endpoints."]
    #[json(default)]
    pub approval_required: bool,
    #[doc = "The authorization code for an oauth app install."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    #[doc = "The distribution channel associated with the app install."]
    #[json(default)]
    pub channel: stripe_types_00::apps::AppsInstallChannel,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub content_security_policy_granted: Option<stripe_types_02::apps::AppServiceResourceInstallContentSecurityPolicy>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub content_security_policy_pending: Option<stripe_types_02::apps::AppServiceResourceInstallContentSecurityPolicy>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The ID of the embedding platform that created the install, if applicable."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The endpoint URLs authorized by the installer."]
    #[json(default)]
    pub endpoints_granted: Vec<String>,
    #[doc = "The endpoint URLs requested by the latest app version that the installer has not authorized."]
    #[json(default)]
    pub endpoints_pending: Vec<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::apps::AppsInstallObject,
    #[doc = "The permissions authorized by the installer."]
    #[json(default)]
    pub permissions_granted: Vec<String>,
    #[doc = "The permissions requested by the latest app version that the installer has not authorized."]
    #[json(default)]
    pub permissions_pending: Vec<String>,
    #[doc = "The status of the app install."]
    #[json(default)]
    pub status: stripe_types_03::apps::AppsInstallStatus,
}

#[cfg(any(feature = "apps", feature = "events"))]
impl crate::PagedItem for AppsInstall {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "apps")]
pub type CreateAppSecretParamsScopeType = stripe_types_01::apps::ListAppSecretsParamsScopeType;

#[cfg(feature = "apps")]
pub type GetAppSecretFindParamsScopeType = stripe_types_01::apps::ListAppSecretsParamsScopeType;

#[cfg(feature = "apps")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SecretServiceResourceScope {
    #[doc = "The secret scope type."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::apps::SecretServiceResourceScopeType,
    #[doc = "The user ID, if type is set to 'user'"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[cfg(feature = "apps")]
pub type UpdateAppInstallParams = stripe_types_00::apps::GetAppInstallParams;
