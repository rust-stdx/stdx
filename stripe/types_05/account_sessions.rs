#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsAccountManagement {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "The list of features enabled in the embedded component."]
    pub features:
        Option<stripe_types_02::account_sessions::CreateAccountSessionParamsComponentsAccountManagementFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsAccountManagement {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
pub type CreateAccountSessionParamsComponentsAccountOnboardingFeatures =
    stripe_types_02::account_sessions::CreateAccountSessionParamsComponentsAccountManagementFeatures;

#[cfg(feature = "account_sessions")]
pub type CreateAccountSessionParamsComponentsDocumentsFeatures =
    stripe_types_03::account_sessions::CreateAccountSessionParamsComponentsBalanceReportFeatures;

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsIssuingCardsList {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "The list of features enabled in the embedded component."]
    pub features:
        Option<stripe_types_00::account_sessions::CreateAccountSessionParamsComponentsIssuingCardsListFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsIssuingCardsList {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
pub type CreateAccountSessionParamsComponentsNotificationBannerFeatures =
    stripe_types_02::account_sessions::CreateAccountSessionParamsComponentsAccountManagementFeatures;

#[cfg(feature = "account_sessions")]
pub type CreateAccountSessionParamsComponentsPaymentDetailsFeatures =
    stripe_types_02::account_sessions::CreateAccountSessionParamsComponentsDisputesListFeatures;

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsPaymentMethodSettings {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "The list of features enabled in the embedded component."]
    pub features:
        Option<stripe_types_01::account_sessions::CreateAccountSessionParamsComponentsPaymentMethodSettingsFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsPaymentMethodSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
pub type CreateAccountSessionParamsComponentsTaxRegistrationsFeatures =
    stripe_types_03::account_sessions::CreateAccountSessionParamsComponentsBalanceReportFeatures;

#[cfg(feature = "account_sessions")]
pub type CreateAccountSessionParamsComponentsTaxSettingsFeatures =
    stripe_types_03::account_sessions::CreateAccountSessionParamsComponentsBalanceReportFeatures;
