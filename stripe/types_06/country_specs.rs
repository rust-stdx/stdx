#[cfg(feature = "country_specs")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CountrySpecVerificationFields {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<stripe_types_02::country_specs::CountrySpecVerificationFieldDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub individual: Option<stripe_types_02::country_specs::CountrySpecVerificationFieldDetails>,
}
