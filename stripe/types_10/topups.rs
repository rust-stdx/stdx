#[cfg(feature = "topups")]
#[derive(Debug, Clone, Default)]
pub struct CreateTopupParams {
    #[doc = "A positive integer representing how much to transfer."]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The ID of a PaymentMethod representing the payment method to be used for the top-up."]
    pub payment_method: Option<String>,
    #[doc = "Payment method-specific configuration for this top-up."]
    pub payment_method_options: Option<stripe_types_07::topups::CreateTopupParamsPaymentMethodOptions>,
    #[doc = "The ID of a source to transfer funds from."]
    pub source: Option<String>,
    #[doc = "Extra information about a top-up for the source's bank statement."]
    pub statement_descriptor: Option<String>,
    #[doc = "A string that identifies this top-up as part of a group."]
    pub transfer_group: Option<String>,
}

#[cfg(feature = "topups")]
impl crate::params::FormEncode for CreateTopupParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("currency", &self.currency);
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("payment_method", self.payment_method.as_ref());
        form.field_opt("payment_method_options", self.payment_method_options.as_ref());
        form.field_opt("source", self.source.as_ref());
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
        form.field_opt("transfer_group", self.transfer_group.as_ref());
    }
}
