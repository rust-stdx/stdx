#[cfg(feature = "country_specs")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CountrySpec {
    #[doc = "The default currency for this country."]
    #[json(default)]
    pub default_currency: String,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::country_specs::CountrySpecObject,
    #[doc = "Currencies that can be accepted in the specific country (for transfers)."]
    #[json(default)]
    pub supported_bank_account_currencies: std::collections::HashMap<String, Vec<String>>,
    #[doc = "Currencies that can be accepted in the specified country (for payments)."]
    #[json(default)]
    pub supported_payment_currencies: Vec<String>,
    #[doc = "Payment methods available in the specified country."]
    #[json(default)]
    pub supported_payment_methods: Vec<String>,
    #[doc = "Countries that can accept transfers from the specified country."]
    #[json(default)]
    pub supported_transfer_countries: Vec<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verification_fields: Option<stripe_types_06::country_specs::CountrySpecVerificationFields>,
}

#[cfg(feature = "country_specs")]
impl crate::PagedItem for CountrySpec {
    fn id(&self) -> &str {
        &self.id
    }
}
