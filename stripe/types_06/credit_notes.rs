#[cfg(feature = "credit_notes")]
#[derive(Debug, Clone)]
pub struct CreateCreditNoteParamsLinesItem {
    #[doc = "The line item amount to credit."]
    pub amount: Option<i64>,
    #[doc = "The description of the credit note line item."]
    pub description: Option<String>,
    #[doc = "The invoice line item to credit."]
    pub invoice_line_item: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The line item quantity to credit."]
    pub quantity: Option<i64>,
    #[doc = "A list of up to 20 tax amounts for the credit note line item."]
    pub tax_amounts: Option<json::OwnedValue>,
    #[doc = "The tax rates which apply to the credit note line item."]
    pub tax_rates: Option<json::OwnedValue>,
    #[doc = "Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`."]
    pub r#type: stripe_types_03::credit_notes::CreateCreditNoteParamsLinesItemType,
    #[doc = "The integer unit amount in cents (or local equivalent) of the credit note line item."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "credit_notes")]
impl crate::params::FormEncode for CreateCreditNoteParamsLinesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("invoice_line_item", self.invoice_line_item.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("quantity", self.quantity.as_ref());
        form.field("type", &self.r#type);
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "credit_notes")]
#[derive(Debug, Clone, Default)]
pub struct CreateCreditNoteParamsRefundsItem {
    #[doc = "Amount of the refund that applies to this credit note, in cents (or local equivalent)."]
    pub amount_refunded: Option<i64>,
    #[doc = "The PaymentRecord refund details to link to this credit note."]
    pub payment_record_refund:
        Option<stripe_types_03::credit_notes::CreateCreditNoteParamsRefundsItemPaymentRecordRefund>,
    #[doc = "ID of an existing refund to link this credit note to."]
    pub refund: Option<String>,
    #[doc = "Type of the refund, one of `refund` or `payment_record_refund`."]
    pub r#type: Option<stripe_types_03::credit_notes::CreateCreditNoteParamsRefundsItemType>,
}

#[cfg(feature = "credit_notes")]
impl crate::params::FormEncode for CreateCreditNoteParamsRefundsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_refunded", self.amount_refunded.as_ref());
        form.field_opt("payment_record_refund", self.payment_record_refund.as_ref());
        form.field_opt("refund", self.refund.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "credit_notes")]
pub type GetCreditNotePreviewParamsLinesItemType = stripe_types_03::credit_notes::CreateCreditNoteParamsLinesItemType;

#[cfg(feature = "credit_notes")]
pub type GetCreditNotePreviewParamsRefundsItemPaymentRecordRefund =
    stripe_types_03::credit_notes::CreateCreditNoteParamsRefundsItemPaymentRecordRefund;

#[cfg(feature = "credit_notes")]
pub type ListCreditNotePreviewLinesParamsEmailType = stripe_types_02::credit_notes::CreateCreditNoteParamsEmailType;

#[cfg(feature = "credit_notes")]
pub type ListCreditNotePreviewLinesParamsLinesItemType =
    stripe_types_03::credit_notes::CreateCreditNoteParamsLinesItemType;

#[cfg(feature = "credit_notes")]
pub type ListCreditNotePreviewLinesParamsReason = stripe_types_01::credit_notes::CreateCreditNoteParamsReason;

#[cfg(feature = "credit_notes")]
pub type ListCreditNotePreviewLinesParamsRefundsItemType =
    stripe_types_03::credit_notes::CreateCreditNoteParamsRefundsItemType;

#[cfg(feature = "credit_notes")]
pub type ListCreditNotePreviewLinesParamsShippingCost =
    stripe_types_03::credit_notes::CreateCreditNoteParamsShippingCost;

#[cfg(feature = "credit_notes")]
#[derive(Debug, Clone, Default)]
pub struct ListCreditNotesParams {
    pub created: Option<stripe_types_02::credit_notes::ListCreditNotesParamsCreated>,
    pub customer: Option<String>,
    pub customer_account: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub invoice: Option<String>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "credit_notes")]
impl crate::params::FormEncode for ListCreditNotesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("invoice", self.invoice.as_ref());
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "credit_notes")]
pub type VoidCreditNoteParams = stripe_types_00::credit_notes::GetCreditNoteParams;
