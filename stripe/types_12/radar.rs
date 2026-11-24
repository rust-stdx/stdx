#[cfg(feature = "radar")]
#[derive(Debug, Clone)]
pub struct CreateRadarPaymentEvaluationParams {
    #[doc = "Details about the Client Device Metadata to associate with the payment evaluation."]
    pub client_device_metadata_details:
        Option<stripe_types_01::radar::CreateRadarPaymentEvaluationParamsClientDeviceMetadataDetails>,
    #[doc = "Details about the customer associated with the payment evaluation."]
    pub customer_details: stripe_types_02::radar::CreateRadarPaymentEvaluationParamsCustomerDetails,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Details about the payment."]
    pub payment_details: stripe_types_11::radar::CreateRadarPaymentEvaluationParamsPaymentDetails,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for CreateRadarPaymentEvaluationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("client_device_metadata_details", self.client_device_metadata_details.as_ref());
        form.field("customer_details", &self.customer_details);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("payment_details", &self.payment_details);
    }
}
