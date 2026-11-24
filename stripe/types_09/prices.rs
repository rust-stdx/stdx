#[cfg(feature = "prices")]
#[derive(Debug, Clone)]
pub struct CreatePriceParamsRecurring {
    #[doc = "Specifies billing frequency."]
    pub interval: stripe_types_05::prices::CreatePriceParamsRecurringInterval,
    #[doc = "The number of intervals between subscription billings."]
    pub interval_count: Option<i64>,
    #[doc = "The meter tracking the usage of a metered price"]
    pub meter: Option<String>,
    #[doc = "Default number of trial days when subscribing a customer to this price using `trial_from_plan=true`..."]
    pub trial_period_days: Option<i64>,
    #[doc = "Configures how the quantity per period should be determined."]
    pub usage_type: Option<stripe_types_05::prices::CreatePriceParamsRecurringUsageType>,
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for CreatePriceParamsRecurring {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
        form.field_opt("meter", self.meter.as_ref());
        form.field_opt("trial_period_days", self.trial_period_days.as_ref());
        form.field_opt("usage_type", self.usage_type.as_ref());
    }
}

#[cfg(feature = "prices")]
#[derive(Debug, Clone, Default)]
pub struct ListPricesParams {
    pub active: Option<bool>,
    pub created: Option<stripe_types_00::prices::ListPricesParamsCreated>,
    pub currency: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub lookup_keys: Option<Vec<String>>,
    pub product: Option<String>,
    pub recurring: Option<stripe_types_06::prices::ListPricesParamsRecurring>,
    pub starting_after: Option<String>,
    pub r#type: Option<stripe_types_03::prices::ListPricesParamsType>,
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for ListPricesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        if let Some(values) = self.lookup_keys.as_ref() {
            form.field_array("lookup_keys", values.as_slice());
        }
        form.field_opt("product", self.product.as_ref());
        form.field_opt("recurring", self.recurring.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "prices")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePriceParamsCurrencyOptionsValue {
    #[doc = "When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment L..."]
    pub custom_unit_amount: Option<stripe_types_05::prices::UpdatePriceParamsCurrencyOptionsValueCustomUnitAmount>,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_06::prices::UpdatePriceParamsCurrencyOptionsValueTaxBehavior>,
    #[doc = "Each element represents a pricing tier."]
    pub tiers: Option<Vec<stripe_types_05::prices::UpdatePriceParamsCurrencyOptionsValueTiersItem>>,
    #[doc = "A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for UpdatePriceParamsCurrencyOptionsValue {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("custom_unit_amount", self.custom_unit_amount.as_ref());
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        if let Some(values) = self.tiers.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("tiers", index, item);
            }
        }
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}
