#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CouponUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_06::coupons::Coupon>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReservePlanDisabled {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_06::treasury::ReservePlan>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReservePlanUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_06::treasury::ReservePlan>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SourceTransactionCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_06::sources::SourceTransaction>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SourceTransactionUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_06::sources::SourceTransaction>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxRateUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_06::tax_rates::TaxRate>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1AccountApplicationAuthorized {
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
    pub r#type: stripe_types_00::events::V1AccountApplicationAuthorizedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1AccountApplicationAuthorized {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1AccountApplicationDeauthorized {
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
    pub r#type: stripe_types_03::events::V1AccountApplicationDeauthorizedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1AccountApplicationDeauthorized {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingCreditBalanceTransactionCreated {
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
    pub related_object: Option<stripe_types_05::events::V1BillingCreditBalanceTransactionCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1BillingCreditBalanceTransactionCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingCreditBalanceTransactionCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingMeterDeactivated {
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
    pub r#type: stripe_types_02::events::V1BillingMeterDeactivatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingMeterDeactivated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingMeterUpdated {
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
    pub r#type: stripe_types_03::events::V1BillingMeterUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingMeterUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ChargeDisputeFundsReinstated {
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
    pub r#type: stripe_types_04::events::V1ChargeDisputeFundsReinstatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ChargeDisputeFundsReinstated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ChargeDisputeFundsWithdrawn {
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
    pub r#type: stripe_types_04::events::V1ChargeDisputeFundsWithdrawnType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ChargeDisputeFundsWithdrawn {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ChargeFailed {
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
    pub r#type: stripe_types_03::events::V1ChargeFailedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ChargeFailed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ChargePending {
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
    pub r#type: stripe_types_03::events::V1ChargePendingType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ChargePending {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CheckoutSessionAsyncPaymentFailed {
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
    pub r#type: stripe_types_02::events::V1CheckoutSessionAsyncPaymentFailedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CheckoutSessionAsyncPaymentFailed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CheckoutSessionAsyncPaymentSucceeded {
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
    pub r#type: stripe_types_04::events::V1CheckoutSessionAsyncPaymentSucceededType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CheckoutSessionAsyncPaymentSucceeded {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CheckoutSessionExpired {
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
    pub r#type: stripe_types_00::events::V1CheckoutSessionExpiredType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CheckoutSessionExpired {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ClimateOrderCanceled {
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
    pub r#type: stripe_types_01::events::V1ClimateOrderCanceledType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ClimateOrderCanceled {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ClimateOrderDelivered {
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
    pub r#type: stripe_types_01::events::V1ClimateOrderDeliveredType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ClimateOrderDelivered {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ClimateProductCreated {
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
    pub r#type: stripe_types_00::events::V1ClimateProductCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ClimateProductCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CouponDeleted {
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
    pub r#type: stripe_types_04::events::V1CouponDeletedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CouponDeleted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CreditNoteUpdated {
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
    pub r#type: stripe_types_02::events::V1CreditNoteUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CreditNoteUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerDeleted {
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
    pub r#type: stripe_types_00::events::V1CustomerDeletedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerDeleted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerDiscountCreated {
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
    pub r#type: stripe_types_04::events::V1CustomerDiscountCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerDiscountCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerDiscountUpdated {
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
    pub r#type: stripe_types_03::events::V1CustomerDiscountUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerDiscountUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerSubscriptionPendingUpdateExpired {
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
    pub r#type: stripe_types_01::events::V1CustomerSubscriptionPendingUpdateExpiredType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerSubscriptionPendingUpdateExpired {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerSubscriptionResumed {
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
    pub r#type: stripe_types_04::events::V1CustomerSubscriptionResumedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerSubscriptionResumed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerSubscriptionTrialWillEnd {
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
    pub r#type: stripe_types_00::events::V1CustomerSubscriptionTrialWillEndType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerSubscriptionTrialWillEnd {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerTaxIdDeleted {
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
    pub r#type: stripe_types_02::events::V1CustomerTaxIdDeletedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerTaxIdDeleted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1FileCreated {
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
    pub related_object: Option<stripe_types_05::events::V1FileCreatedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1FileCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1FileCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1FinancialConnectionsAccountDisconnected {
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
    pub r#type: stripe_types_04::events::V1FinancialConnectionsAccountDisconnectedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1FinancialConnectionsAccountDisconnected {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1FinancialConnectionsAccountRefreshedBalance {
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
    pub r#type: stripe_types_01::events::V1FinancialConnectionsAccountRefreshedBalanceType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1FinancialConnectionsAccountRefreshedBalance {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1FinancialConnectionsAccountSupportedPaymentMethodTypesUpdated {
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
    pub r#type: stripe_types_04::events::V1FinancialConnectionsAccountSupportedPaymentMethodTypesUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1FinancialConnectionsAccountSupportedPaymentMethodTypesUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1FinancialConnectionsAccountUpcomingAccountNumberExpiry {
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
    pub r#type: stripe_types_01::events::V1FinancialConnectionsAccountUpcomingAccountNumberExpiryType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1FinancialConnectionsAccountUpcomingAccountNumberExpiry {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IdentityVerificationSessionCanceled {
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
    pub r#type: stripe_types_01::events::V1IdentityVerificationSessionCanceledType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IdentityVerificationSessionCanceled {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoiceCreated {
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
    pub r#type: stripe_types_00::events::V1InvoiceCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoiceCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoiceFinalizationFailed {
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
    pub r#type: stripe_types_01::events::V1InvoiceFinalizationFailedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoiceFinalizationFailed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoiceOverpaid {
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
    pub r#type: stripe_types_02::events::V1InvoiceOverpaidType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoiceOverpaid {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoicePaymentAttemptRequired {
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
    pub r#type: stripe_types_01::events::V1InvoicePaymentAttemptRequiredType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoicePaymentAttemptRequired {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoicePaymentPaid {
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
    pub related_object: Option<stripe_types_06::events::V1InvoicePaymentPaidRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1InvoicePaymentPaidType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoicePaymentPaid {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoiceUpdated {
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
    pub r#type: stripe_types_02::events::V1InvoiceUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoiceUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoiceWillBeDue {
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
    pub r#type: stripe_types_01::events::V1InvoiceWillBeDueType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoiceWillBeDue {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingAuthorizationRequest {
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
    pub r#type: stripe_types_03::events::V1IssuingAuthorizationRequestType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingAuthorizationRequest {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingCardCreated {
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
    pub r#type: stripe_types_01::events::V1IssuingCardCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingCardCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingDisputeCreated {
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
    pub r#type: stripe_types_02::events::V1IssuingDisputeCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingDisputeCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingPersonalizationDesignRejected {
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
    pub r#type: stripe_types_00::events::V1IssuingPersonalizationDesignRejectedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingPersonalizationDesignRejected {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingTokenCreated {
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
    pub r#type: stripe_types_03::events::V1IssuingTokenCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingTokenCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PaymentIntentCanceled {
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
    pub r#type: stripe_types_04::events::V1PaymentIntentCanceledType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PaymentIntentCanceled {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PaymentIntentProcessing {
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
    pub r#type: stripe_types_04::events::V1PaymentIntentProcessingType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PaymentIntentProcessing {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PaymentLinkUpdated {
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
    pub r#type: stripe_types_04::events::V1PaymentLinkUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PaymentLinkUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PayoutCreated {
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
    pub r#type: stripe_types_00::events::V1PayoutCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PayoutCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PayoutPaid {
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
    pub r#type: stripe_types_01::events::V1PayoutPaidType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PayoutPaid {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PersonDeleted {
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
    pub r#type: stripe_types_02::events::V1PersonDeletedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PersonDeleted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PlanUpdated {
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
    pub r#type: stripe_types_00::events::V1PlanUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PlanUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PriceDeleted {
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
    pub r#type: stripe_types_01::events::V1PriceDeletedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PriceDeleted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ProductCreated {
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
    pub r#type: stripe_types_04::events::V1ProductCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ProductCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PromotionCodeCreated {
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
    pub r#type: stripe_types_01::events::V1PromotionCodeCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PromotionCodeCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1QuoteCanceled {
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
    pub r#type: stripe_types_01::events::V1QuoteCanceledType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1QuoteCanceled {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1QuoteCreated {
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
    pub r#type: stripe_types_04::events::V1QuoteCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1QuoteCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1QuoteFinalized {
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
    pub r#type: stripe_types_01::events::V1QuoteFinalizedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1QuoteFinalized {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1RadarEarlyFraudWarningUpdated {
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
    pub r#type: stripe_types_02::events::V1RadarEarlyFraudWarningUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1RadarEarlyFraudWarningUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1RefundUpdated {
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
    pub r#type: stripe_types_02::events::V1RefundUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1RefundUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ReviewOpened {
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
    pub r#type: stripe_types_02::events::V1ReviewOpenedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ReviewOpened {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SetupIntentCreated {
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
    pub r#type: stripe_types_02::events::V1SetupIntentCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SetupIntentCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SetupIntentSucceeded {
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
    pub r#type: stripe_types_03::events::V1SetupIntentSucceededType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SetupIntentSucceeded {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SourceCanceled {
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
    pub r#type: stripe_types_00::events::V1SourceCanceledType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SourceCanceled {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SourceFailed {
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
    pub r#type: stripe_types_03::events::V1SourceFailedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SourceFailed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SubscriptionScheduleCanceled {
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
    pub r#type: stripe_types_01::events::V1SubscriptionScheduleCanceledType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SubscriptionScheduleCanceled {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SubscriptionScheduleCreated {
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
    pub r#type: stripe_types_01::events::V1SubscriptionScheduleCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SubscriptionScheduleCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SubscriptionScheduleReleased {
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
    pub r#type: stripe_types_04::events::V1SubscriptionScheduleReleasedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SubscriptionScheduleReleased {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TerminalReaderActionFailed {
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
    pub related_object: Option<stripe_types_06::events::V1TerminalReaderActionFailedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1TerminalReaderActionFailedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TerminalReaderActionFailed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TerminalReaderActionSucceeded {
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
    pub related_object: Option<stripe_types_06::events::V1TerminalReaderActionFailedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1TerminalReaderActionSucceededType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TerminalReaderActionSucceeded {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TerminalReaderActionUpdated {
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
    pub related_object: Option<stripe_types_06::events::V1TerminalReaderActionFailedRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1TerminalReaderActionUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TerminalReaderActionUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TestHelpersTestClockAdvancing {
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
    pub r#type: stripe_types_03::events::V1TestHelpersTestClockAdvancingType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TestHelpersTestClockAdvancing {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TestHelpersTestClockDeleted {
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
    pub r#type: stripe_types_04::events::V1TestHelpersTestClockDeletedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TestHelpersTestClockDeleted {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TopupSucceeded {
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
    pub r#type: stripe_types_04::events::V1TopupSucceededType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TopupSucceeded {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TransferReversed {
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
    pub r#type: stripe_types_04::events::V1TransferReversedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TransferReversed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TransferUpdated {
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
    pub r#type: stripe_types_04::events::V1TransferUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TransferUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CommerceProductCatalogImportsFailed {
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
    pub r#type: stripe_types_03::events::V2CommerceProductCatalogImportsFailedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CommerceProductCatalogImportsFailed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CommerceProductCatalogImportsProcessing {
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
    pub r#type: stripe_types_04::events::V2CommerceProductCatalogImportsProcessingType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CommerceProductCatalogImportsProcessing {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountClosed {
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
    pub r#type: stripe_types_00::events::V2CoreAccountClosedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountClosed {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationMerchantCapabilityStatusUpdated {
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
    pub data: Option<stripe_types_06::events::V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedData>,
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
    pub r#type: stripe_types_00::events::V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountConfigurationMerchantCapabilityStatusUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationRecipientCapabilityStatusUpdated {
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
    pub data: Option<stripe_types_06::events::V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedData>,
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
    pub r#type: stripe_types_02::events::V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountConfigurationRecipientCapabilityStatusUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountCreated {
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
    pub r#type: stripe_types_00::events::V2CoreAccountCreatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountCreated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountFutureRequirementsUpdated {
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
    pub r#type: stripe_types_03::events::V2CoreAccountFutureRequirementsUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountFutureRequirementsUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityUpdated {
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
    pub r#type: stripe_types_04::events::V2CoreAccountIdentityUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountIdentityUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountRequirementsUpdated {
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
    pub r#type: stripe_types_04::events::V2CoreAccountRequirementsUpdatedType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountRequirementsUpdated {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreEventDestinationPing {
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
    pub related_object: Option<stripe_types_05::events::V2CoreEventDestinationPingRelatedObject>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V2CoreEventDestinationPingType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreEventDestinationPing {
    fn id(&self) -> &str {
        &self.id
    }
}
