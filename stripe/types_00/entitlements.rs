#[cfg(feature = "entitlements")]
#[derive(Debug, Clone, Default)]
pub struct GetEntitlementActiveEntitlementParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "entitlements")]
impl crate::params::FormEncode for GetEntitlementActiveEntitlementParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
