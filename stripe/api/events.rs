#[cfg(feature = "events")]
#[doc = "Retrieve an event"]
pub async fn get_event(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_02::events::GetEventParams,
) -> Result<stripe_types_05::events::Event, crate::Error> {
    let path = crate::params::format_path("/v1/events/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "events")]
#[doc = "List all events"]
pub async fn list_events(
    client: &crate::Client,
    params: &stripe_types_06::events::ListEventsParams,
) -> Result<crate::List<stripe_types_05::events::Event>, crate::Error> {
    let path = crate::params::format_path("/v1/events", &[]);
    client.get(&path, params).await
}
