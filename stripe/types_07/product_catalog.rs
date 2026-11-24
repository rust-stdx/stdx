#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone)]
pub struct CreateProductCatalogTrialOfferParams {
    #[doc = "Whether the trial offer can be used for new subscriptions."]
    pub active: Option<bool>,
    #[doc = "Duration of one service period of the trial."]
    pub duration: stripe_types_05::product_catalog::CreateProductCatalogTrialOfferParamsDuration,
    #[doc = "Define behavior that occurs at the end of the trial."]
    pub end_behavior: stripe_types_05::product_catalog::CreateProductCatalogTrialOfferParamsEndBehavior,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A brief description of the trial offer, hidden from customers."]
    pub nickname: Option<String>,
    #[doc = "Price configuration during the trial period (amount, billing scheme, etc)."]
    pub price: String,
}

#[cfg(feature = "product_catalog")]
impl crate::params::FormEncode for CreateProductCatalogTrialOfferParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field("duration", &self.duration);
        form.field("end_behavior", &self.end_behavior);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("nickname", self.nickname.as_ref());
        form.field("price", &self.price);
    }
}
