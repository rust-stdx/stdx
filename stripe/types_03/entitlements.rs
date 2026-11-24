#[cfg(any(feature = "entitlements", feature = "events", feature = "products"))]
#[derive(Debug, Clone, PartialEq)]
pub enum EntitlementsFeatureObject {
    EntitlementsFeature,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "entitlements", feature = "events", feature = "products"))]
impl EntitlementsFeatureObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            EntitlementsFeatureObject::EntitlementsFeature => "entitlements.feature",
            EntitlementsFeatureObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "entitlements", feature = "events", feature = "products"))]
impl Default for EntitlementsFeatureObject {
    fn default() -> Self {
        EntitlementsFeatureObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "entitlements", feature = "events", feature = "products"))]
impl<'json> json::Decode<'json> for EntitlementsFeatureObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(EntitlementsFeatureObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "entitlements.feature" => EntitlementsFeatureObject::EntitlementsFeature,
            other => EntitlementsFeatureObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "entitlements")]
#[derive(Debug, Clone, Default)]
pub struct ListEntitlementFeaturesParams {
    pub archived: Option<bool>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub lookup_key: Option<String>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "entitlements")]
impl crate::params::FormEncode for ListEntitlementFeaturesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("archived", self.archived.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("lookup_key", self.lookup_key.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
