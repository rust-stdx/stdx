#[cfg(feature = "invoiceitems")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceitemParams {
    #[doc = "The integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice."]
    pub amount: Option<i64>,
    #[doc = "An arbitrary string which you can attach to the invoice item."]
    pub description: Option<String>,
    #[doc = "Controls whether discounts apply to this invoice item."]
    pub discountable: Option<bool>,
    #[doc = "The coupons, promotion codes & existing discounts which apply to the invoice item or invoice line item."]
    pub discounts: Option<json::OwnedValue>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Pass an empty string to remove previously-defined invoicing rules."]
    pub invoicing_rules: Option<stripe_types_00::invoiceitems::UpdateInvoiceitemParamsInvoicingRules>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The period associated with this invoice item."]
    pub period: Option<stripe_types_05::invoiceitems::UpdateInvoiceitemParamsPeriod>,
    #[doc = "Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline."]
    pub price_data: Option<stripe_types_08::invoiceitems::UpdateInvoiceitemParamsPriceData>,
    #[doc = "The pricing information for the invoice item."]
    pub pricing: Option<stripe_types_05::invoiceitems::UpdateInvoiceitemParamsPricing>,
    #[doc = "Non-negative integer."]
    pub quantity: Option<i64>,
    #[doc = "Non-negative decimal with at most 12 decimal places."]
    pub quantity_decimal: Option<String>,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_05::invoiceitems::UpdateInvoiceitemParamsTaxBehavior>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<json::OwnedValue>,
    #[doc = "The tax rates which apply to the invoice item."]
    pub tax_rates: Option<json::OwnedValue>,
    #[doc = "The decimal unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "invoiceitems")]
impl crate::params::FormEncode for UpdateInvoiceitemParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("discountable", self.discountable.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("invoicing_rules", self.invoicing_rules.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("period", self.period.as_ref());
        form.field_opt("price_data", self.price_data.as_ref());
        form.field_opt("pricing", self.pricing.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
        form.field_opt("quantity_decimal", self.quantity_decimal.as_ref());
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}
