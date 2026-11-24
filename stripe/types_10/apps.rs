#[cfg(feature = "apps")]
#[derive(Debug, Clone)]
pub struct GetAppSecretFindParams {
    pub expand: Option<Vec<String>>,
    pub name: String,
    pub scope: stripe_types_09::apps::GetAppSecretFindParamsScope,
}

#[cfg(feature = "apps")]
impl crate::params::FormEncode for GetAppSecretFindParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("name", &self.name);
        form.field("scope", &self.scope);
    }
}
