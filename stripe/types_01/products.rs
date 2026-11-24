#[cfg(feature = "products")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateProductParamsDefaultPriceDataCurrencyOptionsValueTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "products")]
impl CreateProductParamsDefaultPriceDataCurrencyOptionsValueTaxBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateProductParamsDefaultPriceDataCurrencyOptionsValueTaxBehavior::Exclusive => "exclusive",
            CreateProductParamsDefaultPriceDataCurrencyOptionsValueTaxBehavior::Inclusive => "inclusive",
            CreateProductParamsDefaultPriceDataCurrencyOptionsValueTaxBehavior::Unspecified => "unspecified",
            CreateProductParamsDefaultPriceDataCurrencyOptionsValueTaxBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "products")]
impl<'json> json::Decode<'json> for CreateProductParamsDefaultPriceDataCurrencyOptionsValueTaxBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateProductParamsDefaultPriceDataCurrencyOptionsValueTaxBehavior::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclusive" => CreateProductParamsDefaultPriceDataCurrencyOptionsValueTaxBehavior::Exclusive,
            "inclusive" => CreateProductParamsDefaultPriceDataCurrencyOptionsValueTaxBehavior::Inclusive,
            "unspecified" => CreateProductParamsDefaultPriceDataCurrencyOptionsValueTaxBehavior::Unspecified,
            other => CreateProductParamsDefaultPriceDataCurrencyOptionsValueTaxBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "products")]
impl crate::params::FormEncode for CreateProductParamsDefaultPriceDataCurrencyOptionsValueTaxBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "products")]
#[derive(Debug, Clone, Default)]
pub struct CreateProductParamsTaxDetails {
    #[doc = "A tax location ID."]
    pub performance_location: Option<String>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<json::OwnedValue>,
}

#[cfg(feature = "products")]
impl crate::params::FormEncode for CreateProductParamsTaxDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("performance_location", self.performance_location.as_ref());
    }
}

#[cfg(feature = "products")]
#[derive(Debug, Clone, Default)]
pub struct GetProductParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "products")]
impl crate::params::FormEncode for GetProductParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "products")]
#[derive(Debug, Clone, Default)]
pub struct ListProductsParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "products")]
impl crate::params::FormEncode for ListProductsParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}

#[cfg(feature = "products")]
#[derive(Debug, Clone, Default)]
pub struct SearchProductsParams {
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub page: Option<String>,
    pub query: String,
}

#[cfg(feature = "products")]
impl crate::params::FormEncode for SearchProductsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("page", self.page.as_ref());
        form.field("query", &self.query);
    }
}
