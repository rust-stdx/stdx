#[cfg(feature = "payment_attempt_records")]
#[derive(Debug, Clone, Default)]
pub struct ListPaymentAttemptRecordsParams {
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_record: String,
    pub starting_after: Option<String>,
}

#[cfg(feature = "payment_attempt_records")]
impl crate::params::FormEncode for ListPaymentAttemptRecordsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field("payment_record", &self.payment_record);
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
