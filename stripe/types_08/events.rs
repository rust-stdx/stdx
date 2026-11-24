#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct AccountApplicationDeauthorized {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_05::common::Application>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct AppsInstallCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_05::apps::AppsInstall>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct AppsInstallUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_05::apps::AppsInstall>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CouponDeleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_06::coupons::Coupon>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReportingReportTypeUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_06::reporting::ReportingReportType>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReservePlanCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_06::treasury::ReservePlan>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1AccountExternalAccountDeleted {
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
    pub r#type: stripe_types_03::events::V1AccountExternalAccountDeletedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1AccountExternalAccountDeleted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1AccountUpdated {
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
    pub related_object: Option<stripe_types_05::events::V1AccountUpdatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1AccountUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1AccountUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ApplicationFeeCreated {
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
    pub r#type: stripe_types_03::events::V1ApplicationFeeCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ApplicationFeeCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ApplicationFeeRefundUpdated {
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
    pub related_object: Option<stripe_types_05::events::V1ApplicationFeeRefundUpdatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1ApplicationFeeRefundUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ApplicationFeeRefundUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingCreditGrantUpdated {
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
    pub r#type: stripe_types_02::events::V1BillingCreditGrantUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingCreditGrantUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItem {
    #[doc = "Open Enum."]
    #[json(default)]
    pub code: stripe_types_01::events::V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode,
    #[doc = "The number of errors of this type."]
    #[json(default)]
    pub error_count: i64,
    #[doc = "A list of sample errors of this type."]
    #[json(default)]
    pub sample_errors:
        Vec<stripe_types_05::events::V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemSampleErrorsItem>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingMeterReactivated {
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
    pub r#type: stripe_types_02::events::V1BillingMeterReactivatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingMeterReactivated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingPortalConfigurationUpdated {
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
    pub r#type: stripe_types_03::events::V1BillingPortalConfigurationUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingPortalConfigurationUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingPortalSessionCreated {
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
    pub r#type: stripe_types_03::events::V1BillingPortalSessionCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingPortalSessionCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ChargeCaptured {
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
    pub r#type: stripe_types_02::events::V1ChargeCapturedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ChargeCaptured {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ChargeDisputeClosed {
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
    pub r#type: stripe_types_02::events::V1ChargeDisputeClosedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ChargeDisputeClosed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ChargeExpired {
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
    pub r#type: stripe_types_02::events::V1ChargeExpiredType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ChargeExpired {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ClimateOrderCreated {
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
    pub r#type: stripe_types_00::events::V1ClimateOrderCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ClimateOrderCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ClimateOrderDelayed {
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
    pub r#type: stripe_types_01::events::V1ClimateOrderDelayedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ClimateOrderDelayed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CouponCreated {
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
    pub r#type: stripe_types_01::events::V1CouponCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CouponCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CreditNoteVoided {
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
    pub r#type: stripe_types_00::events::V1CreditNoteVoidedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CreditNoteVoided {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerSubscriptionCreated {
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
    pub r#type: stripe_types_02::events::V1CustomerSubscriptionCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerSubscriptionCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerSubscriptionPaused {
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
    pub r#type: stripe_types_04::events::V1CustomerSubscriptionPausedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerSubscriptionPaused {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerSubscriptionPendingUpdateApplied {
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
    pub r#type: stripe_types_04::events::V1CustomerSubscriptionPendingUpdateAppliedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerSubscriptionPendingUpdateApplied {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerSubscriptionUpdated {
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
    pub r#type: stripe_types_00::events::V1CustomerSubscriptionUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerSubscriptionUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerTaxIdUpdated {
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
    pub r#type: stripe_types_03::events::V1CustomerTaxIdUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerTaxIdUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerUpdated {
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
    pub r#type: stripe_types_01::events::V1CustomerUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1EntitlementsActiveEntitlementSummaryUpdated {
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
    pub r#type: stripe_types_02::events::V1EntitlementsActiveEntitlementSummaryUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1EntitlementsActiveEntitlementSummaryUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1FinancialConnectionsAccountCreated {
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
    pub r#type: stripe_types_03::events::V1FinancialConnectionsAccountCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1FinancialConnectionsAccountCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1FinancialConnectionsAccountExpectedDeactivationDateUpdated {
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
    pub r#type: stripe_types_03::events::V1FinancialConnectionsAccountExpectedDeactivationDateUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1FinancialConnectionsAccountExpectedDeactivationDateUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1FinancialConnectionsAccountReactivated {
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
    pub r#type: stripe_types_01::events::V1FinancialConnectionsAccountReactivatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1FinancialConnectionsAccountReactivated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1FinancialConnectionsAccountRefreshedTransactions {
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
    pub r#type: stripe_types_01::events::V1FinancialConnectionsAccountRefreshedTransactionsType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1FinancialConnectionsAccountRefreshedTransactions {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1FinancialConnectionsAccountUpcomingDeactivation {
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
    pub r#type: stripe_types_04::events::V1FinancialConnectionsAccountUpcomingDeactivationType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1FinancialConnectionsAccountUpcomingDeactivation {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IdentityVerificationSessionCreated {
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
    pub r#type: stripe_types_04::events::V1IdentityVerificationSessionCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IdentityVerificationSessionCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IdentityVerificationSessionProcessing {
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
    pub r#type: stripe_types_04::events::V1IdentityVerificationSessionProcessingType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IdentityVerificationSessionProcessing {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IdentityVerificationSessionRedacted {
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
    pub r#type: stripe_types_02::events::V1IdentityVerificationSessionRedactedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IdentityVerificationSessionRedacted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoiceDeleted {
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
    pub r#type: stripe_types_04::events::V1InvoiceDeletedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoiceDeleted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoiceMarkedUncollectible {
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
    pub r#type: stripe_types_00::events::V1InvoiceMarkedUncollectibleType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoiceMarkedUncollectible {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoicePaid {
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
    pub r#type: stripe_types_03::events::V1InvoicePaidType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoicePaid {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoicePaymentActionRequired {
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
    pub r#type: stripe_types_04::events::V1InvoicePaymentActionRequiredType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoicePaymentActionRequired {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoicePaymentSucceeded {
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
    pub r#type: stripe_types_00::events::V1InvoicePaymentSucceededType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoicePaymentSucceeded {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoiceUpcoming {
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
    pub r#type: stripe_types_03::events::V1InvoiceUpcomingType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoiceUpcoming {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoiceVoided {
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
    pub r#type: stripe_types_03::events::V1InvoiceVoidedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoiceVoided {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoiceitemDeleted {
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
    pub r#type: stripe_types_04::events::V1InvoiceitemDeletedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoiceitemDeleted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingAuthorizationUpdated {
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
    pub r#type: stripe_types_04::events::V1IssuingAuthorizationUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingAuthorizationUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingDisputeFundsReinstated {
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
    pub r#type: stripe_types_04::events::V1IssuingDisputeFundsReinstatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingDisputeFundsReinstated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingDisputeSubmitted {
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
    pub r#type: stripe_types_02::events::V1IssuingDisputeSubmittedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingDisputeSubmitted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingPersonalizationDesignDeactivated {
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
    pub r#type: stripe_types_01::events::V1IssuingPersonalizationDesignDeactivatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingPersonalizationDesignDeactivated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingPersonalizationDesignUpdated {
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
    pub r#type: stripe_types_02::events::V1IssuingPersonalizationDesignUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingPersonalizationDesignUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingTokenUpdated {
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
    pub related_object: Option<stripe_types_05::events::V1IssuingTokenCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1IssuingTokenUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingTokenUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingTransactionCreated {
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
    pub r#type: stripe_types_04::events::V1IssuingTransactionCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingTransactionCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingTransactionPurchaseDetailsReceiptUpdated {
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
    pub r#type: stripe_types_00::events::V1IssuingTransactionPurchaseDetailsReceiptUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingTransactionPurchaseDetailsReceiptUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1MandateUpdated {
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
    pub related_object: Option<stripe_types_06::events::V1MandateUpdatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1MandateUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1MandateUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PaymentIntentCreated {
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
    pub r#type: stripe_types_00::events::V1PaymentIntentCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PaymentIntentCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PaymentIntentPartiallyFunded {
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
    pub r#type: stripe_types_01::events::V1PaymentIntentPartiallyFundedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PaymentIntentPartiallyFunded {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PaymentIntentPaymentFailed {
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
    pub r#type: stripe_types_01::events::V1PaymentIntentPaymentFailedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PaymentIntentPaymentFailed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PaymentIntentRequiresAction {
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
    pub r#type: stripe_types_00::events::V1PaymentIntentRequiresActionType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PaymentIntentRequiresAction {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PaymentLinkCreated {
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
    pub related_object: Option<stripe_types_05::events::V1PaymentLinkCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1PaymentLinkCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PaymentLinkCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PaymentMethodAttached {
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
    pub r#type: stripe_types_01::events::V1PaymentMethodAttachedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PaymentMethodAttached {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PaymentMethodUpdated {
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
    pub r#type: stripe_types_04::events::V1PaymentMethodUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PaymentMethodUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PayoutReconciliationCompleted {
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
    pub r#type: stripe_types_00::events::V1PayoutReconciliationCompletedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PayoutReconciliationCompleted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PersonCreated {
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
    pub r#type: stripe_types_04::events::V1PersonCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PersonCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PlanDeleted {
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
    pub r#type: stripe_types_02::events::V1PlanDeletedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PlanDeleted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PriceCreated {
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
    pub related_object: Option<stripe_types_05::events::V1PriceCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1PriceCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PriceCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PriceUpdated {
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
    pub related_object: Option<stripe_types_05::events::V1PriceCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1PriceUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PriceUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ProductDeleted {
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
    pub r#type: stripe_types_03::events::V1ProductDeletedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ProductDeleted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PromotionCodeUpdated {
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
    pub related_object: Option<stripe_types_05::events::V1PromotionCodeCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1PromotionCodeUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PromotionCodeUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1QuoteAccepted {
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
    pub related_object: Option<stripe_types_05::events::V1QuoteAcceptedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1QuoteAcceptedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1QuoteAccepted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1RefundFailed {
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
    pub r#type: stripe_types_00::events::V1RefundFailedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1RefundFailed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SetupIntentSetupFailed {
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
    pub r#type: stripe_types_02::events::V1SetupIntentSetupFailedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SetupIntentSetupFailed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SourceChargeable {
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
    pub related_object: Option<stripe_types_05::events::V1SourceCanceledRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1SourceChargeableType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SourceChargeable {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SourceRefundAttributesRequired {
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
    pub related_object: Option<stripe_types_05::events::V1SourceCanceledRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1SourceRefundAttributesRequiredType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SourceRefundAttributesRequired {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SubscriptionScheduleCompleted {
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
    pub r#type: stripe_types_01::events::V1SubscriptionScheduleCompletedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SubscriptionScheduleCompleted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SubscriptionScheduleUpdated {
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
    pub r#type: stripe_types_02::events::V1SubscriptionScheduleUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SubscriptionScheduleUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TaxRateCreated {
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
    pub r#type: stripe_types_00::events::V1TaxRateCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TaxRateCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TopupFailed {
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
    pub r#type: stripe_types_03::events::V1TopupFailedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TopupFailed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TopupReversed {
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
    pub r#type: stripe_types_00::events::V1TopupReversedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TopupReversed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CommerceProductCatalogImportsSucceeded {
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
    pub related_object: Option<stripe_types_06::events::V2CommerceProductCatalogImportsFailedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V2CommerceProductCatalogImportsSucceededType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CommerceProductCatalogImportsSucceeded {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CommerceProductCatalogImportsSucceededWithErrors {
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
    pub related_object: Option<stripe_types_06::events::V2CommerceProductCatalogImportsFailedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V2CommerceProductCatalogImportsSucceededWithErrorsType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CommerceProductCatalogImportsSucceededWithErrors {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationCustomerCapabilityStatusUpdated {
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
    pub data: Option<stripe_types_05::events::V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedData>,
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
    pub r#type: stripe_types_03::events::V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountConfigurationCustomerCapabilityStatusUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationRecipientUpdated {
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
    pub r#type: stripe_types_03::events::V2CoreAccountConfigurationRecipientUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountConfigurationRecipientUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountDefaultsUpdated {
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
    pub r#type: stripe_types_04::events::V2CoreAccountDefaultsUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountDefaultsUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountPersonUpdated {
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
    pub r#type: stripe_types_01::events::V2CoreAccountPersonUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountPersonUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountUpdated {
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
    pub r#type: stripe_types_03::events::V2CoreAccountUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}
