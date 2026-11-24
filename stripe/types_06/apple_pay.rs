#[cfg(feature = "apple_pay")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedApplePayDomain {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::apple_pay::ApplePayDomainObject,
}

#[cfg(feature = "apple_pay")]
impl crate::PagedItem for DeletedApplePayDomain {
    fn id(&self) -> &str {
        &self.id
    }
}
