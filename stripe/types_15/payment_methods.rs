#[cfg(any(feature = "events", feature = "financial_connections"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BankConnectionsResourceAccountholder {
    #[doc = "The ID of the Stripe account that this account belongs to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<crate::Expandable<stripe_types_14::account::Account>>,
    #[doc = "The ID for an Account representing a customer that this account belongs to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<stripe_types_14::customers::Customer>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "Type of account holder that this account belongs to."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::payment_methods::BankConnectionsResourceAccountholderType,
}
