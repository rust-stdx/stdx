#[cfg(feature = "v2_billing")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateBillingMeterEventAdjustmentParams {
    #[doc = "Specifies which event to cancel."]
    pub cancel: stripe_types_02::v2_billing::CreateBillingMeterEventAdjustmentParamsCancel,
    #[doc = "The name of the meter event."]
    pub event_name: String,
    #[doc = "Specifies the type of cancellation."]
    #[json(rename = "type")]
    pub r#type: stripe_types_01::v2_billing::CreateBillingMeterEventAdjustmentParamsType,
}

#[cfg(feature = "v2_billing")]
pub type CreateBillingMeterEventParams = stripe_types_00::v2_billing::MeterEventStreamParamsEventsItem;

#[cfg(feature = "v2_billing")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct MeterEventStreamParams {
    #[doc = "List of meter events to include in the request."]
    pub events: Vec<stripe_types_00::v2_billing::MeterEventStreamParamsEventsItem>,
}

#[cfg(feature = "v2_billing")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2BillingMeterEvent {
    #[doc = "The creation time of this meter event."]
    #[json(default)]
    pub created: String,
    #[doc = "The name of the meter event."]
    #[json(default)]
    pub event_name: String,
    #[doc = "A unique identifier for the event."]
    #[json(default)]
    pub identifier: String,
    #[doc = "Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::v2_billing::V2BillingMeterEventObject,
    #[doc = "The payload of the event."]
    #[json(default)]
    pub payload: std::collections::HashMap<String, String>,
    #[doc = "The time of the event."]
    #[json(default)]
    pub timestamp: String,
}
