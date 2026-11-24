#[cfg(feature = "apps")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct AppsSecret {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "If true, indicates that this secret has been deleted"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[doc = "The Unix timestamp for the expiry time of the secret, after which the secret deletes."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "A name for the secret that's unique within the scope."]
    #[json(default)]
    pub name: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::apps::AppsSecretObject,
    #[doc = "The plaintext secret value to be stored."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<stripe_types_05::apps::SecretServiceResourceScope>,
}

#[cfg(feature = "apps")]
impl crate::PagedItem for AppsSecret {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "apps")]
#[derive(Debug, Clone, Default)]
pub struct CreateAppInstallParams {
    #[doc = "The ID of the app to install."]
    pub app: String,
    #[doc = "The distribution channel to install from."]
    pub channel: Option<stripe_types_06::apps::CreateAppInstallParamsChannel>,
    #[doc = "For OAuth apps, the PKCE code challenge used to issue the `auth_code` returned on the install."]
    pub code_challenge: Option<String>,
    #[doc = "The method used to derive `code_challenge`."]
    pub code_challenge_method: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "apps")]
impl crate::params::FormEncode for CreateAppInstallParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("app", &self.app);
        form.field_opt("channel", self.channel.as_ref());
        form.field_opt("code_challenge", self.code_challenge.as_ref());
        form.field_opt("code_challenge_method", self.code_challenge_method.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
