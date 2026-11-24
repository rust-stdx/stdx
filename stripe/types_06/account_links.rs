#[cfg(feature = "account_links")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct AccountLink {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The timestamp at which this account link will expire."]
    #[json(default)]
    pub expires_at: i64,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::account_links::AccountLinkObject,
    #[doc = "The URL for the account link."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "account_links")]
pub type CreateAccountLinkParamsCollectionOptionsFields =
    stripe_types_00::account_links::CreateAccountLinkParamsCollect;
