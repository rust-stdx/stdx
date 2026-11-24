#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, Default)]
pub struct CreateFinancialConnectionSessionParamsFilters {
    #[doc = "Restricts the Session to subcategories of accounts that can be linked."]
    pub account_subcategories: Option<Vec<stripe_types_01::financial_connections::CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem>>,
    #[doc = "Country from which to collect accounts."]
    pub country: Option<String>,
    #[doc = "Whether the session should require payment method support and successful account number retrieval before completion."]
    pub require_payment_method_support: Option<stripe_types_00::financial_connections::CreateFinancialConnectionSessionParamsFiltersRequirePaymentMethodSupport>,
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for CreateFinancialConnectionSessionParamsFilters {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.account_subcategories.as_ref() {
            form.field_array("account_subcategories", values.as_slice());
        }
        form.field_opt("country", self.country.as_ref());
        form.field_opt("require_payment_method_support", self.require_payment_method_support.as_ref());
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, Default)]
pub struct CreateFinancialConnectionSessionParamsManualEntry {
    #[doc = "How manual entry should be handled."]
    pub mode: Option<stripe_types_04::financial_connections::CreateFinancialConnectionSessionParamsManualEntryMode>,
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for CreateFinancialConnectionSessionParamsManualEntry {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mode", self.mode.as_ref());
    }
}

#[cfg(feature = "financial_connections")]
pub type DisconnectFinancialConnectionAccountParams =
    stripe_types_03::financial_connections::GetFinancialConnectionAccountParams;

#[cfg(any(feature = "events", feature = "financial_connections"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAccountOwner {
    #[doc = "The email address of the owner."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The full name of the owner."]
    #[json(default)]
    pub name: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::financial_connections::FinancialConnectionsAccountOwnerObject,
    #[doc = "The ownership object that this owner belongs to."]
    #[json(default)]
    pub ownership: String,
    #[doc = "The raw phone number of the owner."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The raw physical address of the owner."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub raw_address: Option<String>,
    #[doc = "The timestamp of the refresh that updated this owner."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refreshed_at: Option<i64>,
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl crate::PagedItem for FinancialConnectionsAccountOwner {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "financial_connections")]
pub type GetFinancialConnectionSessionParams =
    stripe_types_03::financial_connections::GetFinancialConnectionAccountParams;

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, Default)]
pub struct ListFinancialConnectionAccountsParams {
    pub account_holder:
        Option<stripe_types_00::financial_connections::ListFinancialConnectionAccountsParamsAccountHolder>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub session: Option<String>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for ListFinancialConnectionAccountsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder", self.account_holder.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("session", self.session.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, Default)]
pub struct RefreshFinancialConnectionAccountParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The list of account features that you would like to refresh."]
    pub features: Vec<stripe_types_04::financial_connections::RefreshFinancialConnectionAccountParamsFeaturesItem>,
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for RefreshFinancialConnectionAccountParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_array("features", self.features.as_slice());
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, Default)]
pub struct SubscribeFinancialConnectionAccountParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The list of account features to which you would like to subscribe."]
    pub features: Vec<stripe_types_02::financial_connections::SubscribeFinancialConnectionAccountParamsFeaturesItem>,
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for SubscribeFinancialConnectionAccountParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_array("features", self.features.as_slice());
    }
}
