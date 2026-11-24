#[cfg(feature = "apps")]
#[derive(Debug, Clone)]
pub struct CreateAppSecretParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The Unix timestamp for the expiry time of the secret, after which the secret deletes."]
    pub expires_at: Option<i64>,
    #[doc = "A name for the secret that's unique within the scope."]
    pub name: String,
    #[doc = "The plaintext secret value to be stored."]
    pub payload: String,
    #[doc = "Specifies the scoping of the secret."]
    pub scope: stripe_types_09::apps::CreateAppSecretParamsScope,
}

#[cfg(feature = "apps")]
impl crate::params::FormEncode for CreateAppSecretParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("expires_at", self.expires_at.as_ref());
        form.field("name", &self.name);
        form.field("payload", &self.payload);
        form.field("scope", &self.scope);
    }
}

#[cfg(feature = "apps")]
#[derive(Debug, Clone)]
pub struct DeleteAppSecretParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A name for the secret that's unique within the scope."]
    pub name: String,
    #[doc = "Specifies the scoping of the secret."]
    pub scope: stripe_types_07::apps::DeleteAppSecretParamsScope,
}

#[cfg(feature = "apps")]
impl crate::params::FormEncode for DeleteAppSecretParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("name", &self.name);
        form.field("scope", &self.scope);
    }
}
