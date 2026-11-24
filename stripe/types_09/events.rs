#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct AccountApplicationAuthorized {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_05::common::Application>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct AppsInstallDeleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_05::apps::AppsInstall>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CouponCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_06::coupons::Coupon>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FileCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_06::files::File>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReservePlanExpired {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_06::treasury::ReservePlan>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxRateCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_06::tax_rates::TaxRate>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1AccountExternalAccountCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub r#type: stripe_types_04::events::V1AccountExternalAccountCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1AccountExternalAccountCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1AccountExternalAccountUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub r#type: stripe_types_00::events::V1AccountExternalAccountUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1AccountExternalAccountUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ApplicationFeeRefunded {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1ApplicationFeeCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1ApplicationFeeRefundedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ApplicationFeeRefunded {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BalanceAvailable {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1BalanceAvailableRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1BalanceAvailableType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BalanceAvailable {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BalanceSettingsUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1BalanceSettingsUpdatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1BalanceSettingsUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BalanceSettingsUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingAlertTriggered {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1BillingAlertTriggeredRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1BillingAlertTriggeredType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingAlertTriggered {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingCreditGrantCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1BillingCreditGrantCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1BillingCreditGrantCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingCreditGrantCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingMeterCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub r#type: stripe_types_01::events::V1BillingMeterCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingMeterCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingPortalConfigurationCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1BillingPortalConfigurationCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1BillingPortalConfigurationCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingPortalConfigurationCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CapabilityUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1CapabilityUpdatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1CapabilityUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CapabilityUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CashBalanceFundsAvailable {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1CashBalanceFundsAvailableRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1CashBalanceFundsAvailableType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CashBalanceFundsAvailable {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ChargeDisputeCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1ChargeDisputeClosedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1ChargeDisputeCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ChargeDisputeCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ChargeDisputeUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1ChargeDisputeClosedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1ChargeDisputeUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ChargeDisputeUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ChargeRefundUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1ChargeRefundUpdatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1ChargeRefundUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ChargeRefundUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ChargeRefunded {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1ChargeCapturedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1ChargeRefundedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ChargeRefunded {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ChargeSucceeded {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1ChargeCapturedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1ChargeSucceededType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ChargeSucceeded {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ChargeUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1ChargeCapturedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1ChargeUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ChargeUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CheckoutSessionCompleted {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1CheckoutSessionAsyncPaymentFailedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1CheckoutSessionCompletedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CheckoutSessionCompleted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ClimateOrderProductSubstituted {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1ClimateOrderCanceledRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1ClimateOrderProductSubstitutedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ClimateOrderProductSubstituted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ClimateProductPricingUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1ClimateProductCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1ClimateProductPricingUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ClimateProductPricingUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CouponUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1CouponCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1CouponUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CouponUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CreditNoteCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1CreditNoteCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1CreditNoteCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CreditNoteCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerCashBalanceTransactionCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1CustomerCashBalanceTransactionCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1CustomerCashBalanceTransactionCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerCashBalanceTransactionCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1CustomerCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1CustomerCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerDiscountDeleted {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1CustomerDiscountCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1CustomerDiscountDeletedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerDiscountDeleted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerSubscriptionDeleted {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1CustomerSubscriptionCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1CustomerSubscriptionDeletedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerSubscriptionDeleted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerTaxIdCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1CustomerTaxIdCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1CustomerTaxIdCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerTaxIdCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1FinancialConnectionsAccountAccountNumbersUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object:
        Option<stripe_types_05::events::V1FinancialConnectionsAccountAccountNumbersUpdatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1FinancialConnectionsAccountAccountNumbersUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1FinancialConnectionsAccountAccountNumbersUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1FinancialConnectionsAccountDeactivated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object:
        Option<stripe_types_05::events::V1FinancialConnectionsAccountAccountNumbersUpdatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1FinancialConnectionsAccountDeactivatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1FinancialConnectionsAccountDeactivated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1FinancialConnectionsAccountRefreshedOwnership {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object:
        Option<stripe_types_05::events::V1FinancialConnectionsAccountAccountNumbersUpdatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1FinancialConnectionsAccountRefreshedOwnershipType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1FinancialConnectionsAccountRefreshedOwnership {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IdentityVerificationSessionRequiresInput {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1IdentityVerificationSessionCanceledRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1IdentityVerificationSessionRequiresInputType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IdentityVerificationSessionRequiresInput {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IdentityVerificationSessionVerified {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1IdentityVerificationSessionCanceledRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1IdentityVerificationSessionVerifiedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IdentityVerificationSessionVerified {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoiceFinalized {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1InvoiceCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1InvoiceFinalizedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoiceFinalized {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoiceOverdue {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1InvoiceCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1InvoiceOverdueType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoiceOverdue {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoicePaymentFailed {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1InvoiceCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1InvoicePaymentFailedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoicePaymentFailed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoiceSent {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1InvoiceCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1InvoiceSentType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoiceSent {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoiceitemCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1InvoiceitemCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1InvoiceitemCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoiceitemCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingAuthorizationCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1IssuingAuthorizationCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1IssuingAuthorizationCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingAuthorizationCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingCardUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1IssuingCardCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1IssuingCardUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingCardUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingCardholderCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1IssuingCardholderCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1IssuingCardholderCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingCardholderCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingCardholderUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1IssuingCardholderCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1IssuingCardholderUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingCardholderUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingDisputeClosed {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1IssuingDisputeClosedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1IssuingDisputeClosedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingDisputeClosed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingDisputeFundsRescinded {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1IssuingDisputeClosedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1IssuingDisputeFundsRescindedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingDisputeFundsRescinded {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingDisputeUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1IssuingDisputeClosedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1IssuingDisputeUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingDisputeUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingPersonalizationDesignActivated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1IssuingPersonalizationDesignActivatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1IssuingPersonalizationDesignActivatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingPersonalizationDesignActivated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingTransactionUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1IssuingTransactionCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1IssuingTransactionUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingTransactionUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PaymentIntentAmountCapturableUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1PaymentIntentAmountCapturableUpdatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1PaymentIntentAmountCapturableUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PaymentIntentAmountCapturableUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PaymentIntentSucceeded {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1PaymentIntentAmountCapturableUpdatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1PaymentIntentSucceededType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PaymentIntentSucceeded {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PaymentMethodAutomaticallyUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1PaymentMethodAttachedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1PaymentMethodAutomaticallyUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PaymentMethodAutomaticallyUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PaymentMethodDetached {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1PaymentMethodAttachedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1PaymentMethodDetachedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PaymentMethodDetached {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PayoutCanceled {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1PayoutCanceledRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1PayoutCanceledType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PayoutCanceled {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PayoutFailed {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1PayoutCanceledRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1PayoutFailedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PayoutFailed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PayoutUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1PayoutCanceledRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1PayoutUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PayoutUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PersonUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1PersonCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1PersonUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PersonUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PlanCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1PlanCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1PlanCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PlanCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ProductUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1ProductCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1ProductUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ProductUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1RadarEarlyFraudWarningCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1RadarEarlyFraudWarningCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1RadarEarlyFraudWarningCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1RadarEarlyFraudWarningCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1RefundCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1ChargeRefundUpdatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1RefundCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1RefundCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ReviewClosed {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1ReviewClosedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1ReviewClosedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ReviewClosed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SetupIntentCanceled {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1SetupIntentCanceledRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1SetupIntentCanceledType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SetupIntentCanceled {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SetupIntentRequiresAction {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1SetupIntentCanceledRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1SetupIntentRequiresActionType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SetupIntentRequiresAction {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SigmaScheduledQueryRunCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1SigmaScheduledQueryRunCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1SigmaScheduledQueryRunCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SigmaScheduledQueryRunCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SubscriptionScheduleAborted {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1SubscriptionScheduleAbortedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1SubscriptionScheduleAbortedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SubscriptionScheduleAborted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SubscriptionScheduleExpiring {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1SubscriptionScheduleAbortedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1SubscriptionScheduleExpiringType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SubscriptionScheduleExpiring {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TaxRateUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1TaxRateCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1TaxRateUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TaxRateUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TaxSettingsUpdated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1TaxSettingsUpdatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1TaxSettingsUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TaxSettingsUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TestHelpersTestClockCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1TestHelpersTestClockAdvancingRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1TestHelpersTestClockCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TestHelpersTestClockCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TestHelpersTestClockInternalFailure {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1TestHelpersTestClockAdvancingRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1TestHelpersTestClockInternalFailureType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TestHelpersTestClockInternalFailure {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TestHelpersTestClockReady {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1TestHelpersTestClockAdvancingRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1TestHelpersTestClockReadyType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TestHelpersTestClockReady {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TopupCanceled {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1TopupCanceledRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1TopupCanceledType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TopupCanceled {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TopupCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V1TopupCanceledRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1TopupCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TopupCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TransferCreated {
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
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_05::events::V1TransferCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1TransferCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TransferCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationCustomerUpdated {
    #[doc = "Changes that the event makes to properties in the related object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<stripe_types_02::events::V2CommerceProductCatalogImportsFailedChanges>,
    #[doc = "Authentication context needed to fetch the event or related object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: String,
    #[doc = "Additional data about the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V2CoreAccountClosedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V2CoreAccountConfigurationCustomerUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountConfigurationCustomerUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationMerchantUpdated {
    #[doc = "Changes that the event makes to properties in the related object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<stripe_types_02::events::V2CommerceProductCatalogImportsFailedChanges>,
    #[doc = "Authentication context needed to fetch the event or related object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: String,
    #[doc = "Additional data about the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<stripe_types_00::events::V1AccountApplicationAuthorizedData>,
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
    pub related_object: Option<stripe_types_06::events::V2CoreAccountClosedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V2CoreAccountConfigurationMerchantUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountConfigurationMerchantUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountLinkReturned {
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
    pub data: Option<stripe_types_05::events::V2CoreAccountLinkReturnedData>,
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
    pub r#type: stripe_types_04::events::V2CoreAccountLinkReturnedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountLinkReturned {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountPersonCreated {
    #[doc = "Changes that the event makes to properties in the related object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<stripe_types_03::events::V2CoreAccountCreatedChanges>,
    #[doc = "Authentication context needed to fetch the event or related object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: String,
    #[doc = "Additional data about the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<stripe_types_01::events::V2CoreAccountPersonCreatedData>,
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
    pub related_object: Option<stripe_types_06::events::V2CoreAccountPersonCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V2CoreAccountPersonCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountPersonCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountPersonDeleted {
    #[doc = "Changes that the event makes to properties in the related object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<stripe_types_04::events::V2CoreAccountPersonDeletedChanges>,
    #[doc = "Authentication context needed to fetch the event or related object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: String,
    #[doc = "Additional data about the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<stripe_types_01::events::V2CoreAccountPersonCreatedData>,
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
    pub related_object: Option<stripe_types_06::events::V2CoreAccountPersonCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V2CoreAccountPersonDeletedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountPersonDeleted {
    fn id(&self) -> &str {
        &self.id
    }
}
