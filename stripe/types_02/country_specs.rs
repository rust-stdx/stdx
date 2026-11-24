#[cfg(feature = "country_specs")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CountrySpecVerificationFieldDetails {
    #[doc = "Additional fields which are only required for some users."]
    #[json(default)]
    pub additional: Vec<String>,
    #[doc = "Fields which every account must eventually provide."]
    #[json(default)]
    pub minimum: Vec<String>,
}
