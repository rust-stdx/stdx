#[cfg(feature = "entitlements")]
#[derive(Debug, Clone, Default)]
pub struct CreateEntitlementFeatureParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A unique key you provide as your own system identifier."]
    pub lookup_key: String,
    #[doc = "Set of key-value pairs that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The feature's name, for your own purpose, not meant to be displayable to the customer."]
    pub name: String,
}

#[cfg(feature = "entitlements")]
impl crate::params::FormEncode for CreateEntitlementFeatureParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("lookup_key", &self.lookup_key);
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("name", &self.name);
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum EntitlementsActiveEntitlementSummaryObject {
    EntitlementsActiveEntitlementSummary,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl EntitlementsActiveEntitlementSummaryObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            EntitlementsActiveEntitlementSummaryObject::EntitlementsActiveEntitlementSummary => {
                "entitlements.active_entitlement_summary"
            }
            EntitlementsActiveEntitlementSummaryObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for EntitlementsActiveEntitlementSummaryObject {
    fn default() -> Self {
        EntitlementsActiveEntitlementSummaryObject::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for EntitlementsActiveEntitlementSummaryObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(EntitlementsActiveEntitlementSummaryObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "entitlements.active_entitlement_summary" => {
                EntitlementsActiveEntitlementSummaryObject::EntitlementsActiveEntitlementSummary
            }
            other => EntitlementsActiveEntitlementSummaryObject::Unknown(other.to_string()),
        })
    }
}
