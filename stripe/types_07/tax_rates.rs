#[cfg(feature = "tax_rates")]
#[derive(Debug, Clone, Default)]
pub struct UpdateTaxRateParams {
    #[doc = "Flag determining whether the tax rate is active or inactive (archived)."]
    pub active: Option<bool>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: Option<String>,
    #[doc = "An arbitrary string attached to the tax rate for your internal use only."]
    pub description: Option<String>,
    #[doc = "The display name of the tax rate, which will be shown to users."]
    pub display_name: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The jurisdiction for the tax rate."]
    pub jurisdiction: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "[ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2), without country prefix."]
    pub state: Option<String>,
    #[doc = "The high-level tax type, such as `vat` or `sales_tax`."]
    pub tax_type: Option<stripe_types_06::tax_rates::UpdateTaxRateParamsTaxType>,
}

#[cfg(feature = "tax_rates")]
impl crate::params::FormEncode for UpdateTaxRateParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("display_name", self.display_name.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("jurisdiction", self.jurisdiction.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("state", self.state.as_ref());
        form.field_opt("tax_type", self.tax_type.as_ref());
    }
}
