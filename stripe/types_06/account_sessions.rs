#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsBalanceReport {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "An empty list, because this embedded component has no features."]
    pub features: Option<stripe_types_03::account_sessions::CreateAccountSessionParamsComponentsBalanceReportFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsBalanceReport {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsBalances {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "The list of features enabled in the embedded component."]
    pub features: Option<stripe_types_00::account_sessions::CreateAccountSessionParamsComponentsBalancesFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsBalances {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsDisputesList {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "The list of features enabled in the embedded component."]
    pub features: Option<stripe_types_02::account_sessions::CreateAccountSessionParamsComponentsDisputesListFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsDisputesList {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsFinancialAccount {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "The list of features enabled in the embedded component."]
    pub features:
        Option<stripe_types_01::account_sessions::CreateAccountSessionParamsComponentsFinancialAccountFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsFinancialAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsFinancialAccountTransactions {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "The list of features enabled in the embedded component."]
    pub features: Option<
        stripe_types_03::account_sessions::CreateAccountSessionParamsComponentsFinancialAccountTransactionsFeatures,
    >,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsFinancialAccountTransactions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsInstantPayoutsPromotion {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "The list of features enabled in the embedded component."]
    pub features:
        Option<stripe_types_03::account_sessions::CreateAccountSessionParamsComponentsInstantPayoutsPromotionFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsInstantPayoutsPromotion {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsIssuingCard {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "The list of features enabled in the embedded component."]
    pub features: Option<stripe_types_02::account_sessions::CreateAccountSessionParamsComponentsIssuingCardFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsIssuingCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsPaymentDisputes {
    #[doc = "Whether the embedded component is enabled."]
    pub enabled: bool,
    #[doc = "The list of features enabled in the embedded component."]
    pub features:
        Option<stripe_types_01::account_sessions::CreateAccountSessionParamsComponentsPaymentDisputesFeatures>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsPaymentDisputes {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
pub type CreateAccountSessionParamsComponentsPaymentsFeatures =
    stripe_types_02::account_sessions::CreateAccountSessionParamsComponentsDisputesListFeatures;

#[cfg(feature = "account_sessions")]
pub type CreateAccountSessionParamsComponentsPayoutDetailsFeatures =
    stripe_types_03::account_sessions::CreateAccountSessionParamsComponentsBalanceReportFeatures;

#[cfg(feature = "account_sessions")]
pub type CreateAccountSessionParamsComponentsPayoutReconciliationReportFeatures =
    stripe_types_03::account_sessions::CreateAccountSessionParamsComponentsBalanceReportFeatures;

#[cfg(feature = "account_sessions")]
pub type CreateAccountSessionParamsComponentsPayoutsFeatures =
    stripe_types_00::account_sessions::CreateAccountSessionParamsComponentsBalancesFeatures;

#[cfg(feature = "account_sessions")]
pub type CreateAccountSessionParamsComponentsPayoutsListFeatures =
    stripe_types_03::account_sessions::CreateAccountSessionParamsComponentsBalanceReportFeatures;
