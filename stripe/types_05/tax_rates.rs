#[cfg(feature = "tax_rates")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRateParams {
    #[doc = "Flag determining whether the tax rate is active or inactive (archived)."]
    pub active: Option<bool>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: Option<String>,
    #[doc = "An arbitrary string attached to the tax rate for your internal use only."]
    pub description: Option<String>,
    #[doc = "The display name of the tax rate, which will be shown to users."]
    pub display_name: String,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "This specifies if the tax rate is inclusive or exclusive."]
    pub inclusive: bool,
    #[doc = "The jurisdiction for the tax rate."]
    pub jurisdiction: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "This represents the tax rate percent out of 100."]
    pub percentage: f64,
    #[doc = "[ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2), without country prefix."]
    pub state: Option<String>,
    #[doc = "The high-level tax type, such as `vat` or `sales_tax`."]
    pub tax_type: Option<stripe_types_01::tax_rates::CreateTaxRateParamsTaxType>,
}

#[cfg(feature = "tax_rates")]
impl crate::params::FormEncode for CreateTaxRateParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field("display_name", &self.display_name);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("inclusive", &self.inclusive);
        form.field_opt("jurisdiction", self.jurisdiction.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("percentage", &self.percentage);
        form.field_opt("state", self.state.as_ref());
        form.field_opt("tax_type", self.tax_type.as_ref());
    }
}

#[cfg(feature = "tax_rates")]
#[derive(Debug, Clone, Default)]
pub struct ListTaxRatesParams {
    pub active: Option<bool>,
    pub created: Option<stripe_types_01::tax_rates::ListTaxRatesParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub inclusive: Option<bool>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "tax_rates")]
impl crate::params::FormEncode for ListTaxRatesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("inclusive", self.inclusive.as_ref());
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
