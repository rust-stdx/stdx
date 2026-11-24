#[cfg(feature = "apple_pay")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ApplePayDomain {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[json(default)]
    pub domain_name: String,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::apple_pay::ApplePayDomainObject,
}

#[cfg(feature = "apple_pay")]
impl crate::PagedItem for ApplePayDomain {
    fn id(&self) -> &str {
        &self.id
    }
}
