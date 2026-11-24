#[cfg(feature = "products")]
#[derive(Debug, Clone, Default)]
pub struct CreateProductParamsDefaultPriceData {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "Prices defined in each available currency option."]
    pub currency_options: Option<
        std::collections::HashMap<
            String,
            stripe_types_05::products::CreateProductParamsDefaultPriceDataCurrencyOptionsValue,
        >,
    >,
    #[doc = "When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment L..."]
    pub custom_unit_amount: Option<stripe_types_06::products::CreateProductParamsDefaultPriceDataCustomUnitAmount>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The recurring components of a price such as `interval` and `interval_count`."]
    pub recurring: Option<stripe_types_06::products::CreateProductParamsDefaultPriceDataRecurring>,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_05::products::CreateProductParamsDefaultPriceDataTaxBehavior>,
    #[doc = "A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "products")]
impl crate::params::FormEncode for CreateProductParamsDefaultPriceData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("currency", &self.currency);
        if let Some(values) = self.currency_options.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("currency_options", key, value);
            }
        }
        form.field_opt("custom_unit_amount", self.custom_unit_amount.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("recurring", self.recurring.as_ref());
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "products")]
#[derive(Debug, Clone, Default)]
pub struct UpdateProductParams {
    #[doc = "Whether the product is available for purchase."]
    pub active: Option<bool>,
    #[doc = "The ID of the [Price](https://docs.stripe.com/api/prices) object that is the default price for this product."]
    pub default_price: Option<String>,
    #[doc = "The product's description, meant to be displayable to the customer."]
    pub description: Option<json::OwnedValue>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A list of up to 8 URLs of images for this product, meant to be displayable to the customer."]
    pub images: Option<json::OwnedValue>,
    #[doc = "A list of up to 15 marketing features for this product."]
    pub marketing_features: Option<json::OwnedValue>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The product's name, meant to be displayable to the customer."]
    pub name: Option<String>,
    #[doc = "The dimensions of this product for shipping purposes."]
    pub package_dimensions: Option<stripe_types_05::products::UpdateProductParamsPackageDimensions>,
    #[doc = "Whether this product is shipped (i.e., physical goods)."]
    pub shippable: Option<bool>,
    #[doc = "An arbitrary string to be displayed on your customer's credit card or bank statement."]
    pub statement_descriptor: Option<String>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<json::OwnedValue>,
    #[doc = "Tax details for this product, including the [tax code](/tax/tax-codes) and an optional performance location."]
    pub tax_details: Option<stripe_types_05::products::UpdateProductParamsTaxDetails>,
    #[doc = "A label that represents units of this product."]
    pub unit_label: Option<json::OwnedValue>,
    #[doc = "A URL of a publicly-accessible webpage for this product."]
    pub url: Option<json::OwnedValue>,
}

#[cfg(feature = "products")]
impl crate::params::FormEncode for UpdateProductParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("default_price", self.default_price.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("name", self.name.as_ref());
        form.field_opt("package_dimensions", self.package_dimensions.as_ref());
        form.field_opt("shippable", self.shippable.as_ref());
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
        form.field_opt("tax_details", self.tax_details.as_ref());
    }
}
