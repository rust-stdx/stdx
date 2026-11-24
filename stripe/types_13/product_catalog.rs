#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ProductCatalogImplEndpointsTrialOfferResourceTransition {
    #[doc = "The new price to use at the end of the trial offer period."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<crate::Expandable<stripe_types_12::product_catalog::ProductCatalogTrialOfferPriceExpanded>>,
}

#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehavior {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transition: Option<crate::product_catalog::ProductCatalogImplEndpointsTrialOfferResourceTransition>,
    #[doc = "The type of behavior when the trial offer ends."]
    #[json(rename = "type", default)]
    pub r#type:
        stripe_types_03::product_catalog::ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType,
}

#[cfg(feature = "product_catalog")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ProductCatalogTrialOffer {
    #[doc = "Whether the trial offer is active."]
    #[json(default)]
    pub active: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub duration:
        Option<stripe_types_05::product_catalog::ProductCatalogImplEndpointsTrialOfferResourceTrialOfferDuration>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub end_behavior:
        Option<crate::product_catalog::ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehavior>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "A brief description of the trial offer, hidden from customers."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::product_catalog::ProductCatalogTrialOfferObject,
    #[doc = "The price during the trial offer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<crate::Expandable<stripe_types_12::product_catalog::ProductCatalogTrialOfferPriceExpanded>>,
}

#[cfg(feature = "product_catalog")]
impl crate::PagedItem for ProductCatalogTrialOffer {
    fn id(&self) -> &str {
        &self.id
    }
}
