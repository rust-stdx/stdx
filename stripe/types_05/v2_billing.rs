#[cfg(feature = "v2_billing")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2BillingMeterEventAdjustment {
    #[doc = "Specifies which event to cancel."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cancel: Option<stripe_types_04::v2_billing::V2BillingMeterEventAdjustmentCancel>,
    #[doc = "The time the adjustment was created."]
    #[json(default)]
    pub created: String,
    #[doc = "The name of the meter event."]
    #[json(default)]
    pub event_name: String,
    #[doc = "The unique ID of this meter event adjustment."]
    #[json(default)]
    pub id: String,
    #[doc = "Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::v2_billing::V2BillingMeterEventAdjustmentObject,
    #[doc = "Open Enum."]
    #[json(default)]
    pub status: stripe_types_04::v2_billing::V2BillingMeterEventAdjustmentStatus,
    #[doc = "Open Enum."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::v2_billing::V2BillingMeterEventAdjustmentType,
}

#[cfg(feature = "v2_billing")]
impl crate::PagedItem for V2BillingMeterEventAdjustment {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "v2_billing")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2BillingMeterEventSession {
    #[doc = "The authentication token for this session."]
    #[json(default)]
    pub authentication_token: String,
    #[doc = "The creation time of this session."]
    #[json(default)]
    pub created: String,
    #[doc = "The time at which this session expires."]
    #[json(default)]
    pub expires_at: String,
    #[doc = "The unique ID of this auth session."]
    #[json(default)]
    pub id: String,
    #[doc = "Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::v2_billing::V2BillingMeterEventSessionObject,
}

#[cfg(feature = "v2_billing")]
impl crate::PagedItem for V2BillingMeterEventSession {
    fn id(&self) -> &str {
        &self.id
    }
}
