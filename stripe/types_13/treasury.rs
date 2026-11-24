#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccount {
    #[doc = "The array of paths to active Features in the Features hash."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub active_features: Option<Vec<stripe_types_03::treasury::TreasuryFinancialAccountActiveFeaturesItem>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<stripe_types_03::treasury::TreasuryFinancialAccountsResourceBalance>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    #[json(default)]
    pub country: String,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<stripe_types_12::treasury::TreasuryFinancialAccountFeatures>,
    #[doc = "The set of credentials that resolve to a FinancialAccount."]
    #[json(default)]
    pub financial_addresses: Vec<stripe_types_06::treasury::TreasuryFinancialAccountsResourceFinancialAddress>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The nickname for the FinancialAccount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::treasury::TreasuryFinancialAccountObject,
    #[doc = "The array of paths to pending Features in the Features hash."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pending_features: Option<Vec<stripe_types_03::treasury::TreasuryFinancialAccountActiveFeaturesItem>>,
    #[doc = "The set of functionalities that the platform can restrict on the FinancialAccount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub platform_restrictions: Option<stripe_types_05::treasury::TreasuryFinancialAccountsResourcePlatformRestrictions>,
    #[doc = "The array of paths to restricted Features in the Features hash."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub restricted_features: Option<Vec<stripe_types_03::treasury::TreasuryFinancialAccountActiveFeaturesItem>>,
    #[doc = "Status of this FinancialAccount."]
    #[json(default)]
    pub status: stripe_types_02::treasury::TreasuryFinancialAccountStatus,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<stripe_types_07::treasury::TreasuryFinancialAccountsResourceStatusDetails>,
    #[doc = "The currencies the FinancialAccount can hold a balance in."]
    #[json(default)]
    pub supported_currencies: Vec<String>,
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl crate::PagedItem for TreasuryFinancialAccount {
    fn id(&self) -> &str {
        &self.id
    }
}
