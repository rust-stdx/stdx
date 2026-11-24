#[cfg(feature = "tax_ids")]
#[derive(Debug, Clone, Default)]
pub struct ListTaxIdsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub owner: Option<stripe_types_06::tax_ids::ListTaxIdsParamsOwner>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "tax_ids")]
impl crate::params::FormEncode for ListTaxIdsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("owner", self.owner.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
