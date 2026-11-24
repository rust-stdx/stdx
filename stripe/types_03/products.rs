#[cfg(feature = "products")]
#[derive(Debug, Clone, Default)]
pub struct CreateProductFeatureParams {
    #[doc = "The ID of the [Feature](https://docs.stripe.com/api/entitlements/feature) object attached to this product."]
    pub entitlement_feature: String,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "products")]
impl crate::params::FormEncode for CreateProductFeatureParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("entitlement_feature", &self.entitlement_feature);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "products")]
#[derive(Debug, Clone, Default)]
pub struct CreateProductParamsDefaultPriceDataCurrencyOptionsValueTiersItem {
    #[doc = "The flat billing amount for an entire tier, regardless of the number of units in the tier."]
    pub flat_amount: Option<i64>,
    #[doc = "Same as `flat_amount`, but accepts a decimal value representing an integer in the minor units of the currency."]
    pub flat_amount_decimal: Option<String>,
    #[doc = "The per unit billing amount for each individual unit for which this tier applies."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
    #[doc = "Specifies the upper bound of this tier."]
    pub up_to: json::OwnedValue,
}

#[cfg(feature = "products")]
impl crate::params::FormEncode for CreateProductParamsDefaultPriceDataCurrencyOptionsValueTiersItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("flat_amount", self.flat_amount.as_ref());
        form.field_opt("flat_amount_decimal", self.flat_amount_decimal.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "products")]
#[derive(Debug, Clone, Default)]
pub struct CreateProductParamsMarketingFeaturesItem {
    #[doc = "The marketing feature name."]
    pub name: String,
}

#[cfg(feature = "products")]
impl crate::params::FormEncode for CreateProductParamsMarketingFeaturesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("name", &self.name);
    }
}

#[cfg(feature = "products")]
#[derive(Debug, Clone, PartialEq)]
pub enum ProductFeatureObject {
    ProductFeature,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "products")]
impl ProductFeatureObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ProductFeatureObject::ProductFeature => "product_feature",
            ProductFeatureObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "products")]
impl Default for ProductFeatureObject {
    fn default() -> Self {
        ProductFeatureObject::Unknown(String::new())
    }
}

#[cfg(feature = "products")]
impl<'json> json::Decode<'json> for ProductFeatureObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ProductFeatureObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "product_feature" => ProductFeatureObject::ProductFeature,
            other => ProductFeatureObject::Unknown(other.to_string()),
        })
    }
}
