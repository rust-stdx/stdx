#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsAccountCompanyAddress {
    #[doc = "City, district, suburb, town, or village."]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: Option<String>,
    #[doc = "Address line 1, such as the street, PO Box, or company name."]
    pub line1: Option<String>,
    #[doc = "Address line 2, such as the apartment, suite, unit, or building."]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    pub postal_code: Option<String>,
    #[doc = "State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    pub state: Option<String>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsAccountCompanyAddress {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("line1", self.line1.as_ref());
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
    }
}
