#[cfg(feature = "prices")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePriceParams {
    #[doc = "Whether the price can be used for new purchases."]
    pub active: Option<bool>,
    #[doc = "Prices defined in each available currency option."]
    pub currency_options:
        Option<std::collections::HashMap<String, stripe_types_09::prices::UpdatePriceParamsCurrencyOptionsValue>>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A lookup key used to retrieve prices dynamically from a static string."]
    pub lookup_key: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "A brief description of the price, hidden from customers."]
    pub nickname: Option<String>,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_06::prices::UpdatePriceParamsTaxBehavior>,
    #[doc = "If set to true, will atomically remove the lookup key from the existing price, and assign it to this price."]
    pub transfer_lookup_key: Option<bool>,
}

#[cfg(feature = "prices")]
impl crate::params::FormEncode for UpdatePriceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        if let Some(values) = self.currency_options.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("currency_options", key, value);
            }
        }
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("lookup_key", self.lookup_key.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("nickname", self.nickname.as_ref());
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("transfer_lookup_key", self.transfer_lookup_key.as_ref());
    }
}
