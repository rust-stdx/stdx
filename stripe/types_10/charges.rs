#[cfg(feature = "charges")]
#[derive(Debug, Clone, Default)]
pub struct UpdateChargeParams {
    #[doc = "The ID of an existing customer that will be associated with this request."]
    pub customer: Option<String>,
    #[doc = "An arbitrary string which you can attach to a charge object."]
    pub description: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A set of key-value pairs you can attach to a charge giving information about its riskiness."]
    pub fraud_details: Option<stripe_types_05::charges::UpdateChargeParamsFraudDetails>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "This is the email address that the receipt for this charge will be sent to."]
    pub receipt_email: Option<String>,
    #[doc = "Shipping information for the charge."]
    pub shipping: Option<stripe_types_08::charges::UpdateChargeParamsShipping>,
    #[doc = "A string that identifies this transaction as part of a group."]
    pub transfer_group: Option<String>,
}

#[cfg(feature = "charges")]
impl crate::params::FormEncode for UpdateChargeParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("fraud_details", self.fraud_details.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("receipt_email", self.receipt_email.as_ref());
        form.field_opt("shipping", self.shipping.as_ref());
        form.field_opt("transfer_group", self.transfer_group.as_ref());
    }
}
