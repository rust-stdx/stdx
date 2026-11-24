#[cfg(feature = "coupons")]
#[derive(Debug, Clone, Default)]
pub struct GetCouponParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "coupons")]
impl crate::params::FormEncode for GetCouponParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
