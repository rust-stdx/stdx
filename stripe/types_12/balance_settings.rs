#[cfg(any(feature = "balance_settings", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BalanceSettings {
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::balance_settings::BalanceSettingsObject,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payments: Option<stripe_types_11::balance_settings::BalanceSettingsResourcePayments>,
}

#[cfg(feature = "balance_settings")]
#[derive(Debug, Clone, Default)]
pub struct CreateBalanceSettingParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Settings that apply to the [Payments Balance](https://docs.stripe.com/api/balance)."]
    pub payments: Option<stripe_types_10::balance_settings::CreateBalanceSettingParamsPayments>,
}

#[cfg(feature = "balance_settings")]
impl crate::params::FormEncode for CreateBalanceSettingParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("payments", self.payments.as_ref());
    }
}
