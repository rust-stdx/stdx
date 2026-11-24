#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAuthorization {
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The name of the institution that this authorization belongs to."]
    #[json(default)]
    pub institution_name: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::financial_connections::FinancialConnectionsAuthorizationObject,
    #[doc = "The status of the connection to the Authorization."]
    #[json(default)]
    pub status: stripe_types_02::financial_connections::FinancialConnectionsAuthorizationStatus,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<stripe_types_07::payment_methods::BankConnectionsResourceAuthorizationStatusDetails>,
}

#[cfg(feature = "events")]
impl crate::PagedItem for FinancialConnectionsAuthorization {
    fn id(&self) -> &str {
        &self.id
    }
}
