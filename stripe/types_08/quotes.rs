#[cfg(feature = "quotes")]
#[derive(Debug, Clone, Default)]
pub struct CreateQuoteParamsLineItemsItemPriceData {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) w..."]
    pub product: String,
    #[doc = "The recurring components of a price such as `interval` and `interval_count`."]
    pub recurring: Option<stripe_types_05::quotes::CreateQuoteParamsLineItemsItemPriceDataRecurring>,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_04::quotes::CreateQuoteParamsLineItemsItemPriceDataTaxBehavior>,
    #[doc = "A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for CreateQuoteParamsLineItemsItemPriceData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("currency", &self.currency);
        form.field("product", &self.product);
        form.field_opt("recurring", self.recurring.as_ref());
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone)]
pub enum QuoteApplicationExpanded {
    /// Expanded `application`.
    Application(Box<stripe_types_05::common::Application>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl<'json> json::Decode<'json> for QuoteApplicationExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "application" => {
                QuoteApplicationExpanded::Application(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            _ => QuoteApplicationExpanded::Other(value),
        })
    }
}

#[cfg(feature = "quotes")]
#[derive(Debug, Clone)]
pub struct UpdateQuoteParamsInvoiceSettingsIssuer {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_05::quotes::UpdateQuoteParamsInvoiceSettingsIssuerType,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for UpdateQuoteParamsInvoiceSettingsIssuer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "quotes")]
#[derive(Debug, Clone)]
pub struct UpdateQuoteParamsLineItemsItemPriceDataRecurring {
    #[doc = "Specifies billing frequency."]
    pub interval: stripe_types_06::quotes::UpdateQuoteParamsLineItemsItemPriceDataRecurringInterval,
    #[doc = "The number of intervals between subscription billings."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for UpdateQuoteParamsLineItemsItemPriceDataRecurring {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
    }
}
