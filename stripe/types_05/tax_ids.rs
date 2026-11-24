#[cfg(feature = "tax_ids")]
pub type CreateTaxIdParamsOwnerType = stripe_types_00::tax_ids::ListTaxIdsParamsOwnerType;

#[cfg(any(feature = "customers", feature = "tax_ids"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedTaxId {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::tax_ids::TaxIdObject,
}

#[cfg(any(feature = "customers", feature = "tax_ids"))]
impl crate::PagedItem for DeletedTaxId {
    fn id(&self) -> &str {
        &self.id
    }
}
