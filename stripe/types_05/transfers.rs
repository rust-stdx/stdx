#[cfg(feature = "transfers")]
pub type GetTransferReversalParams = stripe_types_02::transfers::GetTransferParams;

#[cfg(feature = "transfers")]
#[derive(Debug, Clone, Default)]
pub struct ListTransfersParams {
    pub created: Option<stripe_types_04::transfers::ListTransfersParamsCreated>,
    pub destination: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub transfer_group: Option<String>,
}

#[cfg(feature = "transfers")]
impl crate::params::FormEncode for ListTransfersParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("destination", self.destination.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("transfer_group", self.transfer_group.as_ref());
    }
}
