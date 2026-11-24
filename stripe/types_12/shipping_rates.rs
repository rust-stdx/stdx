#[cfg(feature = "shipping_rates")]
#[derive(Debug, Clone, Default)]
pub struct CreateShippingRateParams {
    #[doc = "The estimated range for how long shipping will take, meant to be displayable to the customer."]
    pub delivery_estimate: Option<stripe_types_10::shipping_rates::CreateShippingRateParamsDeliveryEstimate>,
    #[doc = "The name of the shipping rate, meant to be displayable to the customer."]
    pub display_name: String,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Describes a fixed amount to charge for shipping."]
    pub fixed_amount: Option<stripe_types_08::shipping_rates::CreateShippingRateParamsFixedAmount>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Specifies whether the rate is considered inclusive of taxes or exclusive of taxes."]
    pub tax_behavior: Option<stripe_types_06::shipping_rates::CreateShippingRateParamsTaxBehavior>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<String>,
    #[doc = "The type of calculation to use on the shipping rate."]
    pub r#type: Option<stripe_types_04::shipping_rates::CreateShippingRateParamsType>,
}

#[cfg(feature = "shipping_rates")]
impl crate::params::FormEncode for CreateShippingRateParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("delivery_estimate", self.delivery_estimate.as_ref());
        form.field("display_name", &self.display_name);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("fixed_amount", self.fixed_amount.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("tax_code", self.tax_code.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "shipping_rates")]
#[derive(Debug, Clone, Default)]
pub struct UpdateShippingRateParams {
    #[doc = "Whether the shipping rate can be used for new purchases."]
    pub active: Option<bool>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Describes a fixed amount to charge for shipping."]
    pub fixed_amount: Option<stripe_types_11::shipping_rates::UpdateShippingRateParamsFixedAmount>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Specifies whether the rate is considered inclusive of taxes or exclusive of taxes."]
    pub tax_behavior: Option<stripe_types_05::shipping_rates::UpdateShippingRateParamsTaxBehavior>,
}

#[cfg(feature = "shipping_rates")]
impl crate::params::FormEncode for UpdateShippingRateParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("fixed_amount", self.fixed_amount.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
    }
}
