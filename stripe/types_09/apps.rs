#[cfg(feature = "apps")]
#[derive(Debug, Clone)]
pub struct CreateAppSecretParamsScope {
    #[doc = "The secret scope type."]
    pub r#type: stripe_types_05::apps::CreateAppSecretParamsScopeType,
    #[doc = "The user ID."]
    pub user: Option<String>,
}

#[cfg(feature = "apps")]
impl crate::params::FormEncode for CreateAppSecretParamsScope {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
        form.field_opt("user", self.user.as_ref());
    }
}

#[cfg(feature = "apps")]
#[derive(Debug, Clone)]
pub struct GetAppSecretFindParamsScope {
    #[doc = "The secret scope type."]
    pub r#type: stripe_types_05::apps::GetAppSecretFindParamsScopeType,
    #[doc = "The user ID."]
    pub user: Option<String>,
}

#[cfg(feature = "apps")]
impl crate::params::FormEncode for GetAppSecretFindParamsScope {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
        form.field_opt("user", self.user.as_ref());
    }
}
