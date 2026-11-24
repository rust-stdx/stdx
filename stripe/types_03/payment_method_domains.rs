#[cfg(feature = "payment_method_domains")]
#[derive(Debug, Clone, Default)]
pub struct ListPaymentMethodDomainsParams {
    pub domain_name: Option<String>,
    pub enabled: Option<bool>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "payment_method_domains")]
impl crate::params::FormEncode for ListPaymentMethodDomainsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("domain_name", self.domain_name.as_ref());
        form.field_opt("enabled", self.enabled.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
