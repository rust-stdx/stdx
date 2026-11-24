#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone)]
pub struct CreateProductCatalogTrialOfferParamsDuration {
    #[doc = "The relative duration of the trial period computed as the number of recurring price intervals."]
    pub relative: Option<stripe_types_02::product_catalog::CreateProductCatalogTrialOfferParamsDurationRelative>,
    #[doc = "Specifies how the trial offer duration is determined."]
    pub r#type: stripe_types_01::product_catalog::CreateProductCatalogTrialOfferParamsDurationType,
}

#[cfg(feature = "product_catalog")]
impl crate::params::FormEncode for CreateProductCatalogTrialOfferParamsDuration {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("relative", self.relative.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone)]
pub struct CreateProductCatalogTrialOfferParamsEndBehavior {
    #[doc = "The transition to apply when the trial offer ends."]
    pub transition: stripe_types_00::product_catalog::CreateProductCatalogTrialOfferParamsEndBehaviorTransition,
}

#[cfg(feature = "product_catalog")]
impl crate::params::FormEncode for CreateProductCatalogTrialOfferParamsEndBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("transition", &self.transition);
    }
}

#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone, Default)]
pub struct ListProductCatalogTrialOffersParams {
    pub active: Option<bool>,
    pub created: Option<stripe_types_00::product_catalog::ListProductCatalogTrialOffersParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub prices: Option<Vec<String>>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "product_catalog")]
impl crate::params::FormEncode for ListProductCatalogTrialOffersParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        if let Some(values) = self.prices.as_ref() {
            form.field_array("prices", values.as_slice());
        }
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ProductCatalogImplEndpointsTrialOfferResourceTrialOfferDuration {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub relative:
        Option<stripe_types_00::product_catalog::ProductCatalogImplEndpointsTrialOfferResourceRelativeDuration>,
    #[doc = "The type of trial offer duration."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::product_catalog::ProductCatalogImplEndpointsTrialOfferResourceTrialOfferDurationType,
}
