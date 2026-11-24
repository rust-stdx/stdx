#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParams {
    #[doc = "Time at which the Tax Registration becomes active."]
    pub active_from: json::OwnedValue,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: String,
    #[doc = "Specific options for a registration in the specified `country`."]
    pub country_options: stripe_types_12::tax::CreateTaxRegistrationParamsCountryOptions,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "If set, the Tax Registration stops being active at this time."]
    pub expires_at: Option<i64>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("country", &self.country);
        form.field("country_options", &self.country_options);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("expires_at", self.expires_at.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxRegistration {
    #[doc = "Time at which the registration becomes active."]
    #[json(default)]
    pub active_from: i64,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    #[json(default)]
    pub country: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country_options: Option<stripe_types_12::tax::TaxProductRegistrationsResourceCountryOptions>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "If set, the registration stops being active at this time."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::tax::TaxRegistrationObject,
    #[doc = "The status of the registration."]
    #[json(default)]
    pub status: stripe_types_01::tax::TaxRegistrationStatus,
}

#[cfg(feature = "tax")]
impl crate::PagedItem for TaxRegistration {
    fn id(&self) -> &str {
        &self.id
    }
}
