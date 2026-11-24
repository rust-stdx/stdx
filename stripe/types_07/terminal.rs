#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CollectInputsParamsInputsItemSelection {
    #[doc = "List of choices for the `selection` input"]
    pub choices: Vec<stripe_types_06::terminal::CollectInputsParamsInputsItemSelectionChoicesItem>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CollectInputsParamsInputsItemSelection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        for (index, item) in self.choices.iter().enumerate() {
            form.field_indexed("choices", index, item);
        }
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CollectPaymentMethodParams {
    #[doc = "Configuration overrides for this collection, such as tipping, surcharging, and customer cancellation settings."]
    pub collect_config: Option<stripe_types_05::terminal::CollectPaymentMethodParamsCollectConfig>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The ID of the PaymentIntent to collect a payment method for."]
    pub payment_intent: String,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CollectPaymentMethodParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("collect_config", self.collect_config.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("payment_intent", &self.payment_intent);
    }
}
