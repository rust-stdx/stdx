#[cfg(any(feature = "events", feature = "financial_connections"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAccountOwnership {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::financial_connections::FinancialConnectionsAccountOwnershipObject,
    #[doc = "A paginated list of owners for this account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub owners: Option<crate::List<stripe_types_06::financial_connections::FinancialConnectionsAccountOwner>>,
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl crate::PagedItem for FinancialConnectionsAccountOwnership {
    fn id(&self) -> &str {
        &self.id
    }
}
