#[cfg(feature = "credit_notes")]
#[derive(Debug, Clone, Default)]
pub struct CreateCreditNoteParams {
    #[doc = "The integer amount in cents (or local equivalent) representing the total amount of the credit note."]
    pub amount: Option<i64>,
    #[doc = "The integer amount in cents (or local equivalent) representing the amount to credit the customer's balance, which wil..."]
    pub credit_amount: Option<i64>,
    #[doc = "The date when this credit note is in effect."]
    pub effective_at: Option<i64>,
    #[doc = "Type of email to send to the customer, one of `credit_note` or `none` and the default is `credit_note`."]
    pub email_type: Option<stripe_types_02::credit_notes::CreateCreditNoteParamsEmailType>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "ID of the invoice."]
    pub invoice: String,
    #[doc = "Line items that make up the credit note."]
    pub lines: Option<Vec<stripe_types_06::credit_notes::CreateCreditNoteParamsLinesItem>>,
    #[doc = "The credit note's memo appears on the credit note PDF."]
    pub memo: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The integer amount in cents (or local equivalent) representing the amount that is credited outside of Stripe."]
    pub out_of_band_amount: Option<i64>,
    #[doc = "Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`"]
    pub reason: Option<stripe_types_01::credit_notes::CreateCreditNoteParamsReason>,
    #[doc = "The integer amount in cents (or local equivalent) representing the amount to refund."]
    pub refund_amount: Option<i64>,
    #[doc = "Refunds to link to this credit note."]
    pub refunds: Option<Vec<stripe_types_06::credit_notes::CreateCreditNoteParamsRefundsItem>>,
    #[doc = "When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note."]
    pub shipping_cost: Option<stripe_types_03::credit_notes::CreateCreditNoteParamsShippingCost>,
}

#[cfg(feature = "credit_notes")]
impl crate::params::FormEncode for CreateCreditNoteParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("credit_amount", self.credit_amount.as_ref());
        form.field_opt("effective_at", self.effective_at.as_ref());
        form.field_opt("email_type", self.email_type.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("invoice", &self.invoice);
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
    }
}
