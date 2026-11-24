#[cfg(feature = "payment_records")]
#[derive(Debug, Clone, Default)]
pub struct ReportPaymentAttemptParamsPaymentMethodDetails {
    #[doc = "The billing details associated with the method of payment."]
    pub billing_details:
        Option<stripe_types_09::payment_records::ReportPaymentAttemptParamsPaymentMethodDetailsBillingDetails>,
    #[doc = "Information about the custom (user-defined) payment method used to make this payment."]
    pub custom: Option<stripe_types_05::payment_records::ReportPaymentAttemptParamsPaymentMethodDetailsCustom>,
    #[doc = "ID of the Stripe Payment Method used to make this payment."]
    pub payment_method: Option<String>,
    #[doc = "The type of the payment method details."]
    pub r#type: Option<stripe_types_05::payment_records::ReportPaymentAttemptParamsPaymentMethodDetailsType>,
}

#[cfg(feature = "payment_records")]
impl crate::params::FormEncode for ReportPaymentAttemptParamsPaymentMethodDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_details", self.billing_details.as_ref());
        form.field_opt("custom", self.custom.as_ref());
        form.field_opt("payment_method", self.payment_method.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "payment_records")]
#[derive(Debug, Clone)]
pub struct ReportPaymentParams {
    #[doc = "The amount you initially requested for this payment."]
    pub amount_requested: stripe_types_03::payment_records::ReportPaymentParamsAmountRequested,
    #[doc = "Information about the payment attempt cancelation."]
    pub canceled: Option<stripe_types_01::payment_records::ReportPaymentParamsCanceled>,
    #[doc = "Customer information for this payment."]
    pub customer_details: Option<stripe_types_01::payment_records::ReportPaymentParamsCustomerDetails>,
    #[doc = "Indicates whether the customer was present in your checkout flow during this payment."]
    pub customer_presence: Option<stripe_types_04::payment_records::ReportPaymentParamsCustomerPresence>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Information about the payment attempt failure."]
    pub failed: Option<stripe_types_01::payment_records::ReportPaymentParamsFailed>,
    #[doc = "Information about the payment attempt guarantee."]
    pub guaranteed: Option<stripe_types_00::payment_records::ReportPaymentParamsGuaranteed>,
    #[doc = "When the reported payment was initiated."]
    pub initiated_at: i64,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The outcome of the reported payment."]
    pub outcome: Option<stripe_types_00::payment_records::ReportPaymentParamsOutcome>,
    #[doc = "Information about the Payment Method debited for this payment."]
    pub payment_method_details: stripe_types_09::payment_records::ReportPaymentParamsPaymentMethodDetails,
    #[doc = "Processor information for this payment."]
    pub processor_details: Option<stripe_types_08::payment_records::ReportPaymentParamsProcessorDetails>,
    #[doc = "Shipping information for this payment."]
    pub shipping_details: Option<stripe_types_07::payment_records::ReportPaymentParamsShippingDetails>,
}

#[cfg(feature = "payment_records")]
impl crate::params::FormEncode for ReportPaymentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount_requested", &self.amount_requested);
        form.field_opt("canceled", self.canceled.as_ref());
        form.field_opt("customer_details", self.customer_details.as_ref());
        form.field_opt("customer_presence", self.customer_presence.as_ref());
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
        form.field("payment_method_details", &self.payment_method_details);
        form.field_opt("processor_details", self.processor_details.as_ref());
        form.field_opt("shipping_details", self.shipping_details.as_ref());
    }
}

#[cfg(feature = "payment_records")]
#[derive(Debug, Clone)]
pub struct ReportRefundParams {
    #[doc = "A positive integer in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal) representing how ..."]
    pub amount: Option<stripe_types_06::payment_records::ReportRefundParamsAmount>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "When the reported refund was initiated."]
    pub initiated_at: Option<i64>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The outcome of the reported refund."]
    pub outcome: stripe_types_01::payment_records::ReportRefundParamsOutcome,
    #[doc = "Processor information for this refund."]
    pub processor_details: stripe_types_07::payment_records::ReportRefundParamsProcessorDetails,
    #[doc = "Information about the payment attempt refund."]
    pub refunded: Option<stripe_types_03::payment_records::ReportRefundParamsRefunded>,
}

#[cfg(feature = "payment_records")]
impl crate::params::FormEncode for ReportRefundParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("initiated_at", self.initiated_at.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("outcome", &self.outcome);
        form.field("processor_details", &self.processor_details);
        form.field_opt("refunded", self.refunded.as_ref());
    }
}
