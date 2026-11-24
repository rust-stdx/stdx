#[cfg(feature = "products")]
pub type CreateProductParamsDefaultPriceDataCustomUnitAmount =
    stripe_types_02::products::CreateProductParamsDefaultPriceDataCurrencyOptionsValueCustomUnitAmount;

#[cfg(feature = "products")]
#[derive(Debug, Clone)]
pub struct CreateProductParamsDefaultPriceDataRecurring {
    #[doc = "Specifies billing frequency."]
    pub interval: stripe_types_00::products::CreateProductParamsDefaultPriceDataRecurringInterval,
    #[doc = "The number of intervals between subscription billings."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "products")]
impl crate::params::FormEncode for CreateProductParamsDefaultPriceDataRecurring {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
    }
}

#[cfg(feature = "products")]
pub type CreateProductParamsType = stripe_types_02::products::ListProductsParamsType;

#[cfg(feature = "products")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedProduct {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::products::ProductObject,
}

#[cfg(feature = "products")]
impl crate::PagedItem for DeletedProduct {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "products")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedProductFeature {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::products::ProductFeatureObject,
}

#[cfg(feature = "products")]
impl crate::PagedItem for DeletedProductFeature {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "products")]
#[derive(Debug, Clone, Default)]
pub struct ListProductsParams {
    pub active: Option<bool>,
    pub created: Option<stripe_types_01::products::ListProductsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub ids: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub shippable: Option<bool>,
    pub starting_after: Option<String>,
    pub r#type: Option<stripe_types_02::products::ListProductsParamsType>,
    pub url: Option<String>,
}

#[cfg(feature = "products")]
impl crate::params::FormEncode for ListProductsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.ids.as_ref() {
            form.field_array("ids", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("shippable", self.shippable.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("type", self.r#type.as_ref());
        form.field_opt("url", self.url.as_ref());
    }
}
