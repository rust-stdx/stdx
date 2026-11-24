#[cfg(any(feature = "balance", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BalanceAmount {
    #[doc = "Balance amount."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source_types: Option<stripe_types_04::balance::BalanceAmountBySourceType>,
}

#[cfg(any(feature = "balance", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BalanceNetAvailable {
    #[doc = "Net balance amount, subtracting fees from platform-set pricing."]
    #[json(default)]
    pub amount: i64,
    #[doc = "ID of the external account for this net balance (not expandable)."]
    #[json(default)]
    pub destination: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source_types: Option<stripe_types_04::balance::BalanceAmountBySourceType>,
}
