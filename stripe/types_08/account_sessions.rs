#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsNotificationBanner {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "The list of features enabled in the embedded component."]
    pub features:
        Option<stripe_types_05::account_sessions::CreateAccountSessionParamsComponentsNotificationBannerFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsNotificationBanner {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsPayouts {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "The list of features enabled in the embedded component."]
    pub features: Option<stripe_types_06::account_sessions::CreateAccountSessionParamsComponentsPayoutsFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsPayouts {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}
