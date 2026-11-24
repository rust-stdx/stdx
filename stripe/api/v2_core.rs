#[cfg(feature = "v2_core")]
#[doc = "Close an account"]
pub async fn close_core_account(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_05::v2_core::CloseCoreAccountParams,
) -> Result<stripe_types_13::v2_core::V2CoreAccount, crate::Error> {
    let path = crate::params::format_path("/v2/core/accounts/{id}/close", &[("id", id)]);
    client.post_json(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Create an account"]
pub async fn create_core_account(
    client: &crate::Client,
    params: &stripe_types_13::v2_core::CreateCoreAccountParams,
) -> Result<stripe_types_13::v2_core::V2CoreAccount, crate::Error> {
    let path = crate::params::format_path("/v2/core/accounts", &[]);
    client.post_json(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Create an account link"]
pub async fn create_core_account_link(
    client: &crate::Client,
    params: &stripe_types_13::v2_core::CreateCoreAccountLinkParams,
) -> Result<stripe_types_12::v2_core::V2CoreAccountLink, crate::Error> {
    let path = crate::params::format_path("/v2/core/account_links", &[]);
    client.post_json(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Create a person"]
pub async fn create_core_account_person(
    client: &crate::Client,
    account_id: &str,
    params: &stripe_types_12::v2_core::CreateCoreAccountPersonParams,
) -> Result<stripe_types_11::v2_core::V2CoreAccountPerson, crate::Error> {
    let path = crate::params::format_path("/v2/core/accounts/{account_id}/persons", &[("account_id", account_id)]);
    client.post_json(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Create a person token"]
pub async fn create_core_account_person_token(
    client: &crate::Client,
    account_id: &str,
    params: &stripe_types_12::v2_core::CreateCoreAccountPersonTokenParams,
) -> Result<stripe_types_05::v2_core::V2CoreAccountPersonToken, crate::Error> {
    let path =
        crate::params::format_path("/v2/core/accounts/{account_id}/person_tokens", &[("account_id", account_id)]);
    client.post_json(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Create an account token"]
pub async fn create_core_account_token(
    client: &crate::Client,
    params: &stripe_types_13::v2_core::CreateCoreAccountTokenParams,
) -> Result<stripe_types_05::v2_core::V2CoreAccountToken, crate::Error> {
    let path = crate::params::format_path("/v2/core/account_tokens", &[]);
    client.post_json(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Create an Event Destination"]
pub async fn create_core_event_destination(
    client: &crate::Client,
    params: &stripe_types_06::v2_core::CreateCoreEventDestinationParams,
) -> Result<stripe_types_11::v2_core::V2CoreEventDestination, crate::Error> {
    let path = crate::params::format_path("/v2/core/event_destinations", &[]);
    client.post_json(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Delete a person"]
pub async fn delete_core_account_person(
    client: &crate::Client,
    account_id: &str,
    id: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_03::common::V2DeletedObject, crate::Error> {
    let path = crate::params::format_path(
        "/v2/core/accounts/{account_id}/persons/{id}",
        &[("account_id", account_id), ("id", id)],
    );
    client.delete_form(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Delete an Event Destination"]
pub async fn delete_core_event_destination(
    client: &crate::Client,
    id: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_03::common::V2DeletedObject, crate::Error> {
    let path = crate::params::format_path("/v2/core/event_destinations/{id}", &[("id", id)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Disable an Event Destination"]
pub async fn disable_core_event_destination(
    client: &crate::Client,
    id: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_11::v2_core::V2CoreEventDestination, crate::Error> {
    let path = crate::params::format_path("/v2/core/event_destinations/{id}/disable", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Enable an Event Destination"]
pub async fn enable_core_event_destination(
    client: &crate::Client,
    id: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_11::v2_core::V2CoreEventDestination, crate::Error> {
    let path = crate::params::format_path("/v2/core/event_destinations/{id}/enable", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Retrieve an account"]
pub async fn get_core_account(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_05::v2_core::GetCoreAccountParams,
) -> Result<stripe_types_13::v2_core::V2CoreAccount, crate::Error> {
    let path = crate::params::format_path("/v2/core/accounts/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Retrieve a person"]
pub async fn get_core_account_person(
    client: &crate::Client,
    account_id: &str,
    id: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_11::v2_core::V2CoreAccountPerson, crate::Error> {
    let path = crate::params::format_path(
        "/v2/core/accounts/{account_id}/persons/{id}",
        &[("account_id", account_id), ("id", id)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Retrieve a person token"]
pub async fn get_core_account_person_token(
    client: &crate::Client,
    account_id: &str,
    id: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_05::v2_core::V2CoreAccountPersonToken, crate::Error> {
    let path = crate::params::format_path(
        "/v2/core/accounts/{account_id}/person_tokens/{id}",
        &[("account_id", account_id), ("id", id)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Retrieve an account token"]
pub async fn get_core_account_token(
    client: &crate::Client,
    id: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_05::v2_core::V2CoreAccountToken, crate::Error> {
    let path = crate::params::format_path("/v2/core/account_tokens/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Retrieve an Event"]
pub async fn get_core_event(
    client: &crate::Client,
    id: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_09::v2_core::V2CoreEvent, crate::Error> {
    let path = crate::params::format_path("/v2/core/events/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Retrieve an Event Destination"]
pub async fn get_core_event_destination(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_07::v2_core::GetCoreEventDestinationParams,
) -> Result<stripe_types_11::v2_core::V2CoreEventDestination, crate::Error> {
    let path = crate::params::format_path("/v2/core/event_destinations/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "List persons"]
pub async fn list_core_account_persons(
    client: &crate::Client,
    account_id: &str,
    params: &stripe_types_03::v2_core::ListCoreAccountPersonsParams,
) -> Result<crate::List<stripe_types_11::v2_core::V2CoreAccountPerson>, crate::Error> {
    let path = crate::params::format_path("/v2/core/accounts/{account_id}/persons", &[("account_id", account_id)]);
    client.get(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "List accounts"]
pub async fn list_core_accounts(
    client: &crate::Client,
    params: &stripe_types_05::v2_core::ListCoreAccountsParams,
) -> Result<crate::List<stripe_types_13::v2_core::V2CoreAccount>, crate::Error> {
    let path = crate::params::format_path("/v2/core/accounts", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "List Event Destinations"]
pub async fn list_core_event_destinations(
    client: &crate::Client,
    params: &stripe_types_05::v2_core::ListCoreEventDestinationsParams,
) -> Result<crate::List<stripe_types_11::v2_core::V2CoreEventDestination>, crate::Error> {
    let path = crate::params::format_path("/v2/core/event_destinations", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "List Events"]
pub async fn list_core_events(
    client: &crate::Client,
    params: &stripe_types_06::v2_core::ListCoreEventsParams,
) -> Result<crate::List<stripe_types_09::v2_core::V2CoreEvent>, crate::Error> {
    let path = crate::params::format_path("/v2/core/events", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Ping an Event Destination"]
pub async fn ping_core_event_destination(
    client: &crate::Client,
    id: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_09::v2_core::V2CoreEvent, crate::Error> {
    let path = crate::params::format_path("/v2/core/event_destinations/{id}/ping", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Update an account"]
pub async fn update_core_account(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_13::v2_core::UpdateCoreAccountParams,
) -> Result<stripe_types_13::v2_core::V2CoreAccount, crate::Error> {
    let path = crate::params::format_path("/v2/core/accounts/{id}", &[("id", id)]);
    client.post_json(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Update a person"]
pub async fn update_core_account_person(
    client: &crate::Client,
    account_id: &str,
    id: &str,
    params: &stripe_types_12::v2_core::UpdateCoreAccountPersonParams,
) -> Result<stripe_types_11::v2_core::V2CoreAccountPerson, crate::Error> {
    let path = crate::params::format_path(
        "/v2/core/accounts/{account_id}/persons/{id}",
        &[("account_id", account_id), ("id", id)],
    );
    client.post_json(&path, params).await
}

#[cfg(feature = "v2_core")]
#[doc = "Update an Event Destination"]
pub async fn update_core_event_destination(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_07::v2_core::UpdateCoreEventDestinationParams,
) -> Result<stripe_types_11::v2_core::V2CoreEventDestination, crate::Error> {
    let path = crate::params::format_path("/v2/core/event_destinations/{id}", &[("id", id)]);
    client.post_json(&path, params).await
}
