#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone)]
pub struct CreateThreeDSecureAuthenticationParams {
    #[doc = "Contains additional details about the acquirer for this 3DS Authentication."]
    pub acquirer_details:
        Option<stripe_types_03::three_d_secure::CreateThreeDSecureAuthenticationParamsAcquirerDetails>,
    #[doc = "A non-negative integer representing the amount in the [smallest currency unit](/currencies#zero-decimal)."]
    pub amount: Option<i64>,
    #[doc = "Contains additional details on the channel used for this 3DS Authentication."]
    pub channel: stripe_types_09::three_d_secure::CreateThreeDSecureAuthenticationParamsChannel,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<String>,
    #[doc = "The 3DS directory server with which this 3DS Authentication was processed."]
    pub directory_server:
        Option<stripe_types_00::three_d_secure::CreateThreeDSecureAuthenticationParamsDirectoryServer>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Contains additional details on your flow preference for this 3DS Authentication."]
    pub flow_preference: Option<stripe_types_08::three_d_secure::CreateThreeDSecureAuthenticationParamsFlowPreference>,
    #[doc = "Contains information about future usage of this 3DS Authentication"]
    pub future_usage: Option<stripe_types_12::three_d_secure::CreateThreeDSecureAuthenticationParamsFutureUsage>,
    #[doc = "Indicates whether this 3DS Authentication is being performed for a payment or non-payment use case."]
    pub message_category: stripe_types_02::three_d_secure::CreateThreeDSecureAuthenticationParamsMessageCategory,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "ID of the payment method (a PaymentMethod object) to attach to this 3DS Authentication."]
    pub payment_method: Option<String>,
    #[doc = "Hash used to generate the PaymentMethod to be used for this Authentication."]
    pub payment_method_data:
        Option<stripe_types_09::three_d_secure::CreateThreeDSecureAuthenticationParamsPaymentMethodData>,
    #[doc = "The reason for invoking standalone 3DS."]
    pub reason: Option<stripe_types_00::three_d_secure::CreateThreeDSecureAuthenticationParamsReason>,
    #[doc = "The shipping address requested by the cardholder."]
    pub shipping_address:
        Option<stripe_types_05::three_d_secure::CreateThreeDSecureAuthenticationParamsShippingAddress>,
    #[doc = "Set to `always` to skip the fingerprinting step and submit this Authentication immediately or `if_fingerprinting_not_..."]
    pub submit: Option<stripe_types_03::three_d_secure::CreateThreeDSecureAuthenticationParamsSubmit>,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("acquirer_details", self.acquirer_details.as_ref());
        form.field_opt("amount", self.amount.as_ref());
        form.field("channel", &self.channel);
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("directory_server", self.directory_server.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("flow_preference", self.flow_preference.as_ref());
        form.field_opt("future_usage", self.future_usage.as_ref());
        form.field("message_category", &self.message_category);
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("payment_method", self.payment_method.as_ref());
        form.field_opt("payment_method_data", self.payment_method_data.as_ref());
        form.field_opt("reason", self.reason.as_ref());
        form.field_opt("shipping_address", self.shipping_address.as_ref());
        form.field_opt("submit", self.submit.as_ref());
    }
}
