#[cfg(any(feature = "financial_connections", feature = "tokens"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct Token {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<stripe_types_14::payment_methods::BankAccount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_14::payment_methods::Card>,
    #[doc = "IP address of the client that generates the token."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::tokens::TokenObject,
    #[doc = "Type of the token: `account`, `bank_account`, `card`, or `pii`."]
    #[json(rename = "type", default)]
    pub r#type: String,
    #[doc = "Determines if you have already used this token (you can only use tokens once)."]
    #[json(default)]
    pub used: bool,
}

#[cfg(any(feature = "financial_connections", feature = "tokens"))]
impl crate::PagedItem for Token {
    fn id(&self) -> &str {
        &self.id
    }
}
