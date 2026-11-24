#[cfg(feature = "products")]
#[derive(Debug, Clone, Default)]
pub struct CreateProductParamsDefaultPriceDataCurrencyOptionsValue {
    #[doc = "When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment L..."]
    pub custom_unit_amount:
        Option<stripe_types_02::products::CreateProductParamsDefaultPriceDataCurrencyOptionsValueCustomUnitAmount>,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior:
        Option<stripe_types_01::products::CreateProductParamsDefaultPriceDataCurrencyOptionsValueTaxBehavior>,
    #[doc = "Each element represents a pricing tier."]
    pub tiers: Option<Vec<stripe_types_03::products::CreateProductParamsDefaultPriceDataCurrencyOptionsValueTiersItem>>,
    #[doc = "A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "products")]
impl crate::params::FormEncode for CreateProductParamsDefaultPriceDataCurrencyOptionsValue {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("custom_unit_amount", self.custom_unit_amount.as_ref());
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        if let Some(values) = self.tiers.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("tiers", index, item);
            }
        }
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "products")]
pub type CreateProductParamsDefaultPriceDataTaxBehavior =
    stripe_types_01::products::CreateProductParamsDefaultPriceDataCurrencyOptionsValueTaxBehavior;

#[cfg(feature = "products")]
pub type GetProductFeatureParams = stripe_types_01::products::GetProductParams;

#[cfg(feature = "products")]
pub type UpdateProductParamsPackageDimensions = stripe_types_00::products::CreateProductParamsPackageDimensions;

#[cfg(feature = "products")]
pub type UpdateProductParamsTaxDetails = stripe_types_01::products::CreateProductParamsTaxDetails;
