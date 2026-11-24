#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct AccountSession {
    #[doc = "The ID of the account the AccountSession was created for"]
    #[json(default)]
    pub account: String,
    #[doc = "The client secret of this AccountSession."]
    #[json(default)]
    pub client_secret: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<stripe_types_07::accounts::ConnectEmbeddedAccountSessionCreateComponents>,
    #[doc = "The timestamp at which this AccountSession will expire."]
    #[json(default)]
    pub expires_at: i64,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::account_sessions::AccountSessionObject,
}
