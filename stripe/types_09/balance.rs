#[cfg(any(feature = "balance", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BalanceDetail {
    #[doc = "Funds that are available for use."]
    #[json(default)]
    pub available: Vec<stripe_types_05::balance::BalanceAmount>,
}

#[cfg(any(feature = "balance", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BalanceDetailUngated {
    #[doc = "Funds that are available for use."]
    #[json(default)]
    pub available: Vec<stripe_types_05::balance::BalanceAmount>,
    #[doc = "Funds that are pending"]
    #[json(default)]
    pub pending: Vec<stripe_types_05::balance::BalanceAmount>,
}
