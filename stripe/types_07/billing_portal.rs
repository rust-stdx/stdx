#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalResourceScheduleUpdateAtPeriodEnd {
    #[doc = "List of conditions."]
    #[json(default)]
    pub conditions: Vec<stripe_types_06::billing_portal::PortalResourceScheduleUpdateAtPeriodEndCondition>,
}
