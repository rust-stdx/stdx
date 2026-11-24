#[cfg(feature = "v2_billing")]
#[doc = "Create a Meter Event with synchronous validation"]
pub async fn create_billing_meter_event(
    client: &crate::Client,
    params: &stripe_types_06::v2_billing::CreateBillingMeterEventParams,
) -> Result<stripe_types_06::v2_billing::V2BillingMeterEvent, crate::Error> {
    let path = crate::params::format_path("/v2/billing/meter_events", &[]);
    client.post_json(&path, params).await
}

#[cfg(feature = "v2_billing")]
#[doc = "Create a Meter Event Adjustment"]
pub async fn create_billing_meter_event_adjustment(
    client: &crate::Client,
    params: &stripe_types_06::v2_billing::CreateBillingMeterEventAdjustmentParams,
) -> Result<stripe_types_05::v2_billing::V2BillingMeterEventAdjustment, crate::Error> {
    let path = crate::params::format_path("/v2/billing/meter_event_adjustments", &[]);
    client.post_json(&path, params).await
}

#[cfg(feature = "v2_billing")]
#[doc = "Create a Meter Event Stream Authentication Session"]
pub async fn meter_event_session(
    client: &crate::Client,
    params: &crate::params::Empty,
) -> Result<stripe_types_05::v2_billing::V2BillingMeterEventSession, crate::Error> {
    let path = crate::params::format_path("/v2/billing/meter_event_session", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "v2_billing")]
#[doc = "Create a Meter Event with asynchronous validation"]
pub async fn meter_event_stream(
    client: &crate::Client,
    params: &stripe_types_06::v2_billing::MeterEventStreamParams,
) -> Result<stripe_types_01::v2_billing::MeterEventStreamResponse, crate::Error> {
    let path = crate::params::format_path("/v2/billing/meter_event_stream", &[]);
    client.post_json(&path, params).await
}
