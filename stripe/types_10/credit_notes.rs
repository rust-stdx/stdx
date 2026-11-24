#[cfg(feature = "credit_notes")]
#[derive(Debug, Clone, Default)]
pub struct ListCreditNotePreviewLinesParams {
    pub amount: Option<i64>,
    pub credit_amount: Option<i64>,
    pub effective_at: Option<i64>,
    pub email_type: Option<stripe_types_06::credit_notes::ListCreditNotePreviewLinesParamsEmailType>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub invoice: String,
    pub limit: Option<i64>,
    pub lines: Option<Vec<stripe_types_09::credit_notes::ListCreditNotePreviewLinesParamsLinesItem>>,
    pub memo: Option<String>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub out_of_band_amount: Option<i64>,
    pub reason: Option<stripe_types_06::credit_notes::ListCreditNotePreviewLinesParamsReason>,
    pub refund_amount: Option<i64>,
    pub refunds: Option<Vec<stripe_types_07::credit_notes::ListCreditNotePreviewLinesParamsRefundsItem>>,
    pub shipping_cost: Option<stripe_types_06::credit_notes::ListCreditNotePreviewLinesParamsShippingCost>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "credit_notes")]
impl crate::params::FormEncode for ListCreditNotePreviewLinesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("credit_amount", self.credit_amount.as_ref());
        form.field_opt("effective_at", self.effective_at.as_ref());
        form.field_opt("email_type", self.email_type.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("invoice", &self.invoice);
        form.field_opt("limit", self.limit.as_ref());
        if let Some(values) = self.lines.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("lines", index, item);
            }
        }
        form.field_opt("memo", self.memo.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("out_of_band_amount", self.out_of_band_amount.as_ref());
        form.field_opt("reason", self.reason.as_ref());
        form.field_opt("refund_amount", self.refund_amount.as_ref());
        if let Some(values) = self.refunds.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("refunds", index, item);
            }
        }
        form.field_opt("shipping_cost", self.shipping_cost.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
