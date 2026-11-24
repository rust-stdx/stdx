#[cfg(feature = "sigma")]
#[derive(Debug, Clone, Default)]
pub struct GetSigmaScheduledQueryRunParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "sigma")]
impl crate::params::FormEncode for GetSigmaScheduledQueryRunParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "sigma")]
#[derive(Debug, Clone, Default)]
pub struct ListSigmaScheduledQueryRunsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "sigma")]
impl crate::params::FormEncode for ListSigmaScheduledQueryRunsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
