#[cfg(feature = "balance_transactions")]
#[derive(Debug, Clone, Default)]
pub struct ListBalanceTransactionsParams {
    pub created: Option<stripe_types_01::balance_transactions::ListBalanceTransactionsParamsCreated>,
    pub currency: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payout: Option<String>,
    pub source: Option<String>,
    pub starting_after: Option<String>,
    pub r#type: Option<String>,
}

#[cfg(feature = "balance_transactions")]
impl crate::params::FormEncode for ListBalanceTransactionsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("payout", self.payout.as_ref());
        form.field_opt("source", self.source.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}
