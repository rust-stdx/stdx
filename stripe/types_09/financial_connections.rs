#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone)]
pub struct CreateFinancialConnectionSessionParams {
    #[doc = "The account holder to link accounts for."]
    pub account_holder: stripe_types_05::financial_connections::CreateFinancialConnectionSessionParamsAccountHolder,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Filters to restrict the kinds of accounts to collect."]
    pub filters: Option<stripe_types_06::financial_connections::CreateFinancialConnectionSessionParamsFilters>,
    #[doc = "Settings for configuring Session-specific limits."]
    pub limits: Option<stripe_types_02::financial_connections::CreateFinancialConnectionSessionParamsLimits>,
    #[doc = "Customize manual entry behavior"]
    pub manual_entry: Option<stripe_types_06::financial_connections::CreateFinancialConnectionSessionParamsManualEntry>,
    #[doc = "List of data features that you would like to request access to."]
    pub permissions: Vec<stripe_types_01::financial_connections::CreateFinancialConnectionSessionParamsPermissionsItem>,
    #[doc = "List of data features that you would like to retrieve upon account creation."]
    pub prefetch:
        Option<Vec<stripe_types_03::financial_connections::CreateFinancialConnectionSessionParamsPrefetchItem>>,
    #[doc = "For webview integrations only."]
    pub return_url: Option<String>,
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for CreateFinancialConnectionSessionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("account_holder", &self.account_holder);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("filters", self.filters.as_ref());
        form.field_opt("limits", self.limits.as_ref());
        form.field_opt("manual_entry", self.manual_entry.as_ref());
        form.field_array("permissions", self.permissions.as_slice());
        if let Some(values) = self.prefetch.as_ref() {
            form.field_array("prefetch", values.as_slice());
        }
        form.field_opt("return_url", self.return_url.as_ref());
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, Default)]
pub struct UnsubscribeFinancialConnectionAccountParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The list of account features from which you would like to unsubscribe."]
    pub features: Vec<stripe_types_05::financial_connections::UnsubscribeFinancialConnectionAccountParamsFeaturesItem>,
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for UnsubscribeFinancialConnectionAccountParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        for (index, item) in self.features.iter().enumerate() {
            form.field_indexed("features", index, item);
        }
    }
}
