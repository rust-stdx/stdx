#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSession {
    #[doc = "The client secret of this Customer Session."]
    #[json(default)]
    pub client_secret: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<stripe_types_11::customer_sessions::CustomerSessionResourceComponents>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The Customer the Customer Session was created for."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<stripe_types_14::customers::Customer>>,
    #[doc = "The Account that the Customer Session was created for."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "The timestamp at which this Customer Session will expire."]
    #[json(default)]
    pub expires_at: i64,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::customer_sessions::CustomerSessionObject,
}
