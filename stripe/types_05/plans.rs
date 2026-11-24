#[cfg(feature = "plans")]
#[derive(Debug, Clone, Default)]
pub struct CreatePlanParamsProduct {
    #[doc = "Whether the product is currently available for purchase."]
    pub active: Option<bool>,
    #[doc = "The identifier for the product."]
    pub id: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The product's name, meant to be displayable to the customer."]
    pub name: String,
    #[doc = "An arbitrary string to be displayed on your customer's credit card or bank statement."]
    pub statement_descriptor: Option<String>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<String>,
    #[doc = "Tax details for this product, including the [tax code](/tax/tax-codes) and an optional performance location."]
    pub tax_details: Option<stripe_types_02::plans::CreatePlanParamsProductTaxDetails>,
    #[doc = "A label that represents units of this product."]
    pub unit_label: Option<String>,
}

#[cfg(feature = "plans")]
impl crate::params::FormEncode for CreatePlanParamsProduct {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("id", self.id.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("name", &self.name);
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
        form.field_opt("tax_code", self.tax_code.as_ref());
        form.field_opt("tax_details", self.tax_details.as_ref());
        form.field_opt("unit_label", self.unit_label.as_ref());
    }
}

#[cfg(feature = "plans")]
#[derive(Debug, Clone)]
pub struct CreatePlanParamsTransformUsage {
    #[doc = "Divide usage by this number."]
    pub divide_by: i64,
    #[doc = "After division, either round the result `up` or `down`."]
    pub round: stripe_types_02::plans::CreatePlanParamsTransformUsageRound,
}

#[cfg(feature = "plans")]
impl crate::params::FormEncode for CreatePlanParamsTransformUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("divide_by", &self.divide_by);
        form.field("round", &self.round);
    }
}
