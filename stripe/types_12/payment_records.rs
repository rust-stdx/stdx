#[cfg(feature = "payment_records")]
#[derive(Debug, Clone, Default)]
pub struct ReportPaymentAttemptParams {
    #[doc = "Information about the payment attempt cancelation."]
    pub canceled: Option<stripe_types_05::payment_records::ReportPaymentAttemptParamsCanceled>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Information about the payment attempt failure."]
    pub failed: Option<stripe_types_05::payment_records::ReportPaymentAttemptParamsFailed>,
    #[doc = "Information about the payment attempt guarantee."]
    pub guaranteed: Option<stripe_types_06::payment_records::ReportPaymentAttemptParamsGuaranteed>,
    #[doc = "When the reported payment was initiated."]
    pub initiated_at: i64,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The outcome of the reported payment."]
    pub outcome: Option<stripe_types_06::payment_records::ReportPaymentAttemptParamsOutcome>,
    #[doc = "Information about the Payment Method debited for this payment."]
    pub payment_method_details:
        Option<stripe_types_11::payment_records::ReportPaymentAttemptParamsPaymentMethodDetails>,
    #[doc = "Shipping information for this payment."]
    pub shipping_details: Option<stripe_types_09::payment_records::ReportPaymentAttemptParamsShippingDetails>,
}

#[cfg(feature = "payment_records")]
impl crate::params::FormEncode for ReportPaymentAttemptParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("canceled", self.canceled.as_ref());
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("failed", self.failed.as_ref());
        form.field_opt("guaranteed", self.guaranteed.as_ref());
        form.field("initiated_at", &self.initiated_at);
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("outcome", self.outcome.as_ref());
        form.field_opt("payment_method_details", self.payment_method_details.as_ref());
        form.field_opt("shipping_details", self.shipping_details.as_ref());
    }
}
