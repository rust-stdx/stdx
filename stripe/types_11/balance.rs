#[cfg(any(feature = "balance", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct Balance {
    #[doc = "Available funds that you can transfer or pay out automatically by Stripe or explicitly through the Transfers API..."]
    #[json(default)]
    pub available: Vec<stripe_types_05::balance::BalanceAmount>,
    #[doc = "Funds held due to negative balances on connected accounts where account.controller.requirement_collection..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub connect_reserved: Option<Vec<stripe_types_05::balance::BalanceAmount>>,
    #[doc = "Funds that you can pay out using Instant Payouts."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub instant_available: Option<Vec<stripe_types_07::balance::BalanceAmountNet>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuing: Option<stripe_types_09::balance::BalanceDetail>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::balance::BalanceObject,
    #[doc = "Funds that aren't available in the balance yet."]
    #[json(default)]
    pub pending: Vec<stripe_types_05::balance::BalanceAmount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_and_dispute_prefunding: Option<stripe_types_09::balance::BalanceDetailUngated>,
}
