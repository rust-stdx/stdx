#[cfg(any(feature = "entitlements", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum EntitlementsActiveEntitlementObject {
    EntitlementsActiveEntitlement,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "entitlements", feature = "events"))]
impl EntitlementsActiveEntitlementObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            EntitlementsActiveEntitlementObject::EntitlementsActiveEntitlement => "entitlements.active_entitlement",
            EntitlementsActiveEntitlementObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "entitlements", feature = "events"))]
impl Default for EntitlementsActiveEntitlementObject {
    fn default() -> Self {
        EntitlementsActiveEntitlementObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "entitlements", feature = "events"))]
impl<'json> json::Decode<'json> for EntitlementsActiveEntitlementObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(EntitlementsActiveEntitlementObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "entitlements.active_entitlement" => EntitlementsActiveEntitlementObject::EntitlementsActiveEntitlement,
            other => EntitlementsActiveEntitlementObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "entitlements")]
#[derive(Debug, Clone, Default)]
pub struct ListEntitlementActiveEntitlementsParams {
    pub customer: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "entitlements")]
impl crate::params::FormEncode for ListEntitlementActiveEntitlementsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("customer", &self.customer);
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "entitlements")]
#[derive(Debug, Clone, Default)]
pub struct UpdateEntitlementFeatureParams {
    #[doc = "Inactive features cannot be attached to new products and will not be returned from the features list endpoint."]
    pub active: Option<bool>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of key-value pairs that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The feature's name, for your own purpose, not meant to be displayable to the customer."]
    pub name: Option<String>,
}

#[cfg(feature = "entitlements")]
impl crate::params::FormEncode for UpdateEntitlementFeatureParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("name", self.name.as_ref());
    }
}
