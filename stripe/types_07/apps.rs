#[cfg(feature = "apps")]
#[derive(Debug, Clone)]
pub struct DeleteAppSecretParamsScope {
    #[doc = "The secret scope type."]
    pub r#type: stripe_types_06::apps::DeleteAppSecretParamsScopeType,
    #[doc = "The user ID."]
    pub user: Option<String>,
}

#[cfg(feature = "apps")]
impl crate::params::FormEncode for DeleteAppSecretParamsScope {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
        form.field_opt("user", self.user.as_ref());
    }
}

#[cfg(feature = "apps")]
#[derive(Debug, Clone)]
pub struct ListAppSecretsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub scope: stripe_types_06::apps::ListAppSecretsParamsScope,
    pub starting_after: Option<String>,
}

#[cfg(feature = "apps")]
impl crate::params::FormEncode for ListAppSecretsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field("scope", &self.scope);
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
