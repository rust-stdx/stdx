#[cfg(feature = "payment_attempt_records")]
#[derive(Debug, Clone, Default)]
pub struct GetPaymentAttemptRecordParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "payment_attempt_records")]
impl crate::params::FormEncode for GetPaymentAttemptRecordParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
