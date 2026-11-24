#[cfg(feature = "shipping_rates")]
#[derive(Debug, Clone, Default)]
pub struct CreateShippingRateParamsDeliveryEstimate {
    #[doc = "The upper bound of the estimated range."]
    pub maximum: Option<stripe_types_05::shipping_rates::CreateShippingRateParamsDeliveryEstimateMaximum>,
    #[doc = "The lower bound of the estimated range."]
    pub minimum: Option<stripe_types_07::shipping_rates::CreateShippingRateParamsDeliveryEstimateMinimum>,
}

#[cfg(feature = "shipping_rates")]
impl crate::params::FormEncode for CreateShippingRateParamsDeliveryEstimate {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("maximum", self.maximum.as_ref());
        form.field_opt("minimum", self.minimum.as_ref());
    }
}
