#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BalanceSettingsUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_12::balance_settings::BalanceSettings>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingPortalConfigurationCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::billing_portal::BillingPortalConfiguration>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingPortalConfigurationUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::billing_portal::BillingPortalConfiguration>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingPortalSessionCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::billing_portal::BillingPortalSession>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IdentityVerificationSessionCanceled {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_12::identity::IdentityVerificationSession>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IdentityVerificationSessionCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_12::identity::IdentityVerificationSession>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IdentityVerificationSessionProcessing {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_12::identity::IdentityVerificationSession>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IdentityVerificationSessionRedacted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_12::identity::IdentityVerificationSession>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IdentityVerificationSessionRequiresInput {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_12::identity::IdentityVerificationSession>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IdentityVerificationSessionVerified {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_12::identity::IdentityVerificationSession>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingCardCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::issuing::IssuingCard>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingCardUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::issuing::IssuingCard>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingCardholderCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::issuing::IssuingCardholder>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingCardholderUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::issuing::IssuingCardholder>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PersonCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_12::accounts::Person>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PersonDeleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_12::accounts::Person>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PersonUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_12::accounts::Person>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PlanCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::plans::Plan>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PlanDeleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::plans::Plan>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PlanUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::plans::Plan>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SourceMandateNotification {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_12::sources::SourceMandateNotification>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccountClosed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryFinancialAccount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccountCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryFinancialAccount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccountFeaturesStatusUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryFinancialAccount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingMeterErrorReportTriggered {
    #[doc = "Always `null` because this event type doesn't track related object changes."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<json::OwnedValue>,
    #[doc = "Authentication context needed to fetch the event or related object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: String,
    #[doc = "Additional data about the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<stripe_types_12::events::V1BillingMeterErrorReportTriggeredData>,
    #[doc = "Unique identifier for the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::events::V1AccountApplicationAuthorizedObject,
    #[doc = "Reason for the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<stripe_types_06::events::V1AccountApplicationAuthorizedReason>,
    #[doc = "Object containing the reference to API resource relevant to the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub related_object: Option<stripe_types_06::events::V1BillingMeterCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1BillingMeterErrorReportTriggeredType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingMeterErrorReportTriggered {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingMeterNoMeterFound {
    #[doc = "Always `null` because this event type doesn't track related object changes."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<json::OwnedValue>,
    #[doc = "Authentication context needed to fetch the event or related object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: String,
    #[doc = "Additional data about the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<stripe_types_12::events::V1BillingMeterErrorReportTriggeredData>,
    #[doc = "Unique identifier for the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::events::V1AccountApplicationAuthorizedObject,
    #[doc = "Reason for the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<stripe_types_06::events::V1AccountApplicationAuthorizedReason>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1BillingMeterNoMeterFoundType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingMeterNoMeterFound {
    fn id(&self) -> &str {
        &self.id
    }
}
