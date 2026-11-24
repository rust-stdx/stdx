#[cfg(feature = "ephemeral_keys")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct EphemeralKey {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Time at which the key will expire."]
    #[json(default)]
    pub expires: i64,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::ephemeral_keys::EphemeralKeyObject,
    #[doc = "The key's secret."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[cfg(feature = "ephemeral_keys")]
impl crate::PagedItem for EphemeralKey {
    fn id(&self) -> &str {
        &self.id
    }
}
