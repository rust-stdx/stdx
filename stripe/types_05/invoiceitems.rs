#[cfg(feature = "invoiceitems")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceitemParamsPriceData {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) w..."]
    pub product: String,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_03::invoiceitems::CreateInvoiceitemParamsPriceDataTaxBehavior>,
    #[doc = "A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "invoiceitems")]
impl crate::params::FormEncode for CreateInvoiceitemParamsPriceData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("currency", &self.currency);
        form.field("product", &self.product);
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "invoiceitems")]
pub type CreateInvoiceitemParamsTaxBehavior =
    stripe_types_03::invoiceitems::CreateInvoiceitemParamsPriceDataTaxBehavior;

#[cfg(feature = "invoiceitems")]
pub type UpdateInvoiceitemParamsPeriod = stripe_types_02::invoiceitems::CreateInvoiceitemParamsPeriod;

#[cfg(feature = "invoiceitems")]
pub type UpdateInvoiceitemParamsPricing = stripe_types_00::invoiceitems::CreateInvoiceitemParamsPricing;

#[cfg(feature = "invoiceitems")]
pub type UpdateInvoiceitemParamsTaxBehavior =
    stripe_types_03::invoiceitems::CreateInvoiceitemParamsPriceDataTaxBehavior;
