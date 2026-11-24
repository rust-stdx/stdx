#[cfg(feature = "payouts")]
#[derive(Debug, Clone, Default)]
pub struct ListPayoutsParams {
    pub arrival_date: Option<stripe_types_02::payouts::ListPayoutsParamsArrivalDate>,
    pub created: Option<stripe_types_05::payouts::ListPayoutsParamsCreated>,
    pub destination: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
}

#[cfg(feature = "payouts")]
impl crate::params::FormEncode for ListPayoutsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("arrival_date", self.arrival_date.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("destination", self.destination.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}
