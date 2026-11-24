#[cfg(feature = "apps")]
pub type CreateAppInstallParamsChannel = stripe_types_04::apps::ListAppInstallsParamsChannel;

#[cfg(feature = "apps")]
pub type DeleteAppSecretParamsScopeType = stripe_types_01::apps::ListAppSecretsParamsScopeType;

#[cfg(feature = "apps")]
#[derive(Debug, Clone, Default)]
pub struct ListAppInstallsParams {
    pub account: Option<String>,
    pub app: Option<String>,
    pub approval_required: Option<bool>,
    pub channel: Option<stripe_types_04::apps::ListAppInstallsParamsChannel>,
    pub created: Option<stripe_types_02::apps::ListAppInstallsParamsCreated>,
    pub created_by: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_01::apps::ListAppInstallsParamsStatus>,
}

#[cfg(feature = "apps")]
impl crate::params::FormEncode for ListAppInstallsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field_opt("app", self.app.as_ref());
        form.field_opt("approval_required", self.approval_required.as_ref());
        form.field_opt("channel", self.channel.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("created_by", self.created_by.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}

#[cfg(feature = "apps")]
#[derive(Debug, Clone)]
pub struct ListAppSecretsParamsScope {
    #[doc = "The secret scope type."]
    pub r#type: stripe_types_01::apps::ListAppSecretsParamsScopeType,
    #[doc = "The user ID."]
    pub user: Option<String>,
}

#[cfg(feature = "apps")]
impl crate::params::FormEncode for ListAppSecretsParamsScope {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
        form.field_opt("user", self.user.as_ref());
    }
}

#[cfg(feature = "apps")]
pub type UninstallAppInstallParams = stripe_types_00::apps::GetAppInstallParams;
