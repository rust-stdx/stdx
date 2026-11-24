#[cfg(feature = "application_fees")]
#[derive(Debug, Clone, Default)]
pub struct ListApplicationFeesParams {
    pub charge: Option<String>,
    pub created: Option<stripe_types_02::application_fees::ListApplicationFeesParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "application_fees")]
impl crate::params::FormEncode for ListApplicationFeesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("charge", self.charge.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
