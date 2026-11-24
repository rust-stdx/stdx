#[cfg(feature = "climate")]
pub type CancelClimateOrderParams = stripe_types_04::climate::GetClimateOrderParams;

#[cfg(any(feature = "climate", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ClimateSupplier {
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "Link to a webpage to learn more about the supplier."]
    #[json(default)]
    pub info_url: String,
    #[doc = "Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "The locations in which this supplier operates."]
    #[json(default)]
    pub locations: Vec<stripe_types_04::climate::ClimateRemovalsLocation>,
    #[doc = "Name of this carbon removal supplier."]
    #[json(default)]
    pub name: String,
    #[doc = "String representing the object’s type."]
    #[json(default)]
    pub object: stripe_types_02::climate::ClimateSupplierObject,
    #[doc = "The scientific pathway used for carbon removal."]
    #[json(default)]
    pub removal_pathway: stripe_types_02::climate::ClimateSupplierRemovalPathway,
}

#[cfg(any(feature = "climate", feature = "events"))]
impl crate::PagedItem for ClimateSupplier {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "climate")]
#[derive(Debug, Clone, Default)]
pub struct CreateClimateOrderParams {
    #[doc = "Requested amount of carbon removal units."]
    pub amount: Option<i64>,
    #[doc = "Publicly sharable reference for the end beneficiary of carbon removal."]
    pub beneficiary: Option<stripe_types_01::climate::CreateClimateOrderParamsBeneficiary>,
    #[doc = "Request currency for the order as a three-letter ISO currency code..."]
    pub currency: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Requested number of tons for the order."]
    pub metric_tons: Option<String>,
    #[doc = "Unique identifier of the Climate product."]
    pub product: String,
}

#[cfg(feature = "climate")]
impl crate::params::FormEncode for CreateClimateOrderParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("beneficiary", self.beneficiary.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("metric_tons", self.metric_tons.as_ref());
        form.field("product", &self.product);
    }
}

#[cfg(feature = "climate")]
pub type GetClimateSupplierParams = stripe_types_04::climate::GetClimateOrderParams;

#[cfg(feature = "climate")]
pub type ListClimateProductsParams = stripe_types_02::climate::ListClimateOrdersParams;

#[cfg(feature = "climate")]
#[derive(Debug, Clone, Default)]
pub struct UpdateClimateOrderParams {
    #[doc = "Publicly sharable reference for the end beneficiary of carbon removal."]
    pub beneficiary: Option<stripe_types_00::climate::UpdateClimateOrderParamsBeneficiary>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "climate")]
impl crate::params::FormEncode for UpdateClimateOrderParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("beneficiary", self.beneficiary.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
    }
}
