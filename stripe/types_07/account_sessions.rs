#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsAccountOnboarding {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "The list of features enabled in the embedded component."]
    pub features:
        Option<stripe_types_05::account_sessions::CreateAccountSessionParamsComponentsAccountOnboardingFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsAccountOnboarding {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsDocuments {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "An empty list, because this embedded component has no features."]
    pub features: Option<stripe_types_05::account_sessions::CreateAccountSessionParamsComponentsDocumentsFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsDocuments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsPaymentDetails {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "The list of features enabled in the embedded component."]
    pub features: Option<stripe_types_05::account_sessions::CreateAccountSessionParamsComponentsPaymentDetailsFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsPaymentDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsPayoutsList {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "An empty list, because this embedded component has no features."]
    pub features: Option<stripe_types_06::account_sessions::CreateAccountSessionParamsComponentsPayoutsListFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsPayoutsList {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}
