#[cfg(feature = "reviews")]
pub type ApproveReviewParams = stripe_types_04::reviews::GetReviewParams;

#[cfg(feature = "reviews")]
#[derive(Debug, Clone, Default)]
pub struct ListReviewsParams {
    pub created: Option<stripe_types_01::reviews::ListReviewsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "reviews")]
impl crate::params::FormEncode for ListReviewsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
