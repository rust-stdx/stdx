#[cfg(any(feature = "accounts", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct Capability {
    #[doc = "The account for which the capability enables functionality."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<crate::Expandable<stripe_types_14::account::Account>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<stripe_types_08::account::AccountCapabilityFutureRequirements>,
    #[doc = "The identifier for the capability."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::accounts::CapabilityObject,
    #[doc = "Whether the capability has been requested."]
    #[json(default)]
    pub requested: bool,
    #[doc = "Time at which the capability was requested."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub requirements: Option<stripe_types_09::account::AccountCapabilityRequirements>,
    #[doc = "The status of the capability."]
    #[json(default)]
    pub status: stripe_types_02::accounts::CapabilityStatus,
}

#[cfg(any(feature = "accounts", feature = "events"))]
impl crate::PagedItem for Capability {
    fn id(&self) -> &str {
        &self.id
    }
}
