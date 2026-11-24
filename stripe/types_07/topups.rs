#[cfg(feature = "topups")]
#[derive(Debug, Clone, Default)]
pub struct CreateTopupParamsPaymentMethodOptions {
    pub us_bank_account: Option<stripe_types_05::topups::CreateTopupParamsPaymentMethodOptionsUsBankAccount>,
}

#[cfg(feature = "topups")]
impl crate::params::FormEncode for CreateTopupParamsPaymentMethodOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
    }
}

#[cfg(feature = "topups")]
#[derive(Debug, Clone, Default)]
pub struct ListTopupsParams {
    pub amount: Option<stripe_types_04::topups::ListTopupsParamsAmount>,
    pub created: Option<stripe_types_05::topups::ListTopupsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_04::topups::ListTopupsParamsStatus>,
}

#[cfg(feature = "topups")]
impl crate::params::FormEncode for ListTopupsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}
