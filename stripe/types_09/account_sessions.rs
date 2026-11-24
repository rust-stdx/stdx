#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsPayments {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "The list of features enabled in the embedded component."]
    pub features: Option<stripe_types_06::account_sessions::CreateAccountSessionParamsComponentsPaymentsFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsPayments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsPayoutDetails {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "An empty list, because this embedded component has no features."]
    pub features: Option<stripe_types_06::account_sessions::CreateAccountSessionParamsComponentsPayoutDetailsFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsPayoutDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsPayoutReconciliationReport {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "An empty list, because this embedded component has no features."]
    pub features: Option<
        stripe_types_06::account_sessions::CreateAccountSessionParamsComponentsPayoutReconciliationReportFeatures,
    >,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsPayoutReconciliationReport {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsTaxRegistrations {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "An empty list, because this embedded component has no features."]
    pub features:
        Option<stripe_types_05::account_sessions::CreateAccountSessionParamsComponentsTaxRegistrationsFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsTaxRegistrations {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsTaxSettings {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "An empty list, because this embedded component has no features."]
    pub features: Option<stripe_types_05::account_sessions::CreateAccountSessionParamsComponentsTaxSettingsFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsTaxSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}
