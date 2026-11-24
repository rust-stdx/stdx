#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone, Default)]
pub struct CreateProductCatalogTrialOfferParamsDurationRelative {
    #[doc = "The number of recurring price's interval to apply for the trial period."]
    pub iterations: i64,
}

#[cfg(feature = "product_catalog")]
impl crate::params::FormEncode for CreateProductCatalogTrialOfferParamsDurationRelative {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("iterations", &self.iterations);
    }
}
