#[cfg(feature = "shipping_rates")]
#[derive(Debug, Clone)]
pub struct CreateShippingRateParamsDeliveryEstimateMaximum {
    #[doc = "A unit of time."]
    pub unit: stripe_types_00::shipping_rates::CreateShippingRateParamsDeliveryEstimateMaximumUnit,
    #[doc = "Must be greater than 0."]
    pub value: i64,
}

#[cfg(feature = "shipping_rates")]
impl crate::params::FormEncode for CreateShippingRateParamsDeliveryEstimateMaximum {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("unit", &self.unit);
        form.field("value", &self.value);
    }
}

#[cfg(feature = "shipping_rates")]
#[derive(Debug, Clone, Default)]
pub struct CreateShippingRateParamsFixedAmountCurrencyOptionsValue {
    #[doc = "A non-negative integer in cents representing how much to charge."]
    pub amount: i64,
    #[doc = "Specifies whether the rate is considered inclusive of taxes or exclusive of taxes."]
    pub tax_behavior:
        Option<stripe_types_00::shipping_rates::CreateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior>,
}

#[cfg(feature = "shipping_rates")]
impl crate::params::FormEncode for CreateShippingRateParamsFixedAmountCurrencyOptionsValue {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
    }
}

#[cfg(feature = "shipping_rates")]
#[derive(Debug, Clone, Default)]
pub struct ListShippingRatesParams {
    pub active: Option<bool>,
    pub created: Option<stripe_types_02::shipping_rates::ListShippingRatesParamsCreated>,
    pub currency: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "shipping_rates")]
impl crate::params::FormEncode for ListShippingRatesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "shipping_rates")]
pub type UpdateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior =
    stripe_types_00::shipping_rates::CreateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior;

#[cfg(feature = "shipping_rates")]
pub type UpdateShippingRateParamsTaxBehavior =
    stripe_types_00::shipping_rates::CreateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior;
