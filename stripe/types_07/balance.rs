#[cfg(any(feature = "balance", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BalanceAmountNet {
    #[doc = "Balance amount."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "Breakdown of balance by destination."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub net_available: Option<Vec<stripe_types_05::balance::BalanceNetAvailable>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source_types: Option<stripe_types_04::balance::BalanceAmountBySourceType>,
}
