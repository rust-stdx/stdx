#[cfg(feature = "subscriptions")]
#[doc = "Create a subscription"]
pub async fn create_subscription(
    client: &crate::Client,
    params: &stripe_types_13::subscriptions::CreateSubscriptionParams,
) -> Result<stripe_types_14::subscriptions::Subscription, crate::Error> {
    let path = crate::params::format_path("/v1/subscriptions", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "subscriptions")]
#[doc = "Cancel a subscription"]
pub async fn delete_subscription(
    client: &crate::Client,
    subscription_exposed_id: &str,
    params: &stripe_types_10::subscriptions::DeleteSubscriptionParams,
) -> Result<stripe_types_14::subscriptions::Subscription, crate::Error> {
    let path = crate::params::format_path(
        "/v1/subscriptions/{subscription_exposed_id}",
        &[("subscription_exposed_id", subscription_exposed_id)],
    );
    client.delete_form(&path, params).await
}

#[cfg(feature = "subscriptions")]
#[doc = "Delete a subscription discount"]
pub async fn delete_subscription_discount(
    client: &crate::Client,
    subscription_exposed_id: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_15::coupons::DeletedDiscount, crate::Error> {
    let path = crate::params::format_path(
        "/v1/subscriptions/{subscription_exposed_id}/discount",
        &[("subscription_exposed_id", subscription_exposed_id)],
    );
    client.delete_form(&path, params).await
}

#[cfg(feature = "subscriptions")]
#[doc = "Retrieve a subscription"]
pub async fn get_subscription(
    client: &crate::Client,
    subscription_exposed_id: &str,
    params: &stripe_types_02::subscriptions::GetSubscriptionParams,
) -> Result<stripe_types_14::subscriptions::Subscription, crate::Error> {
    let path = crate::params::format_path(
        "/v1/subscriptions/{subscription_exposed_id}",
        &[("subscription_exposed_id", subscription_exposed_id)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "subscriptions")]
#[doc = "List subscriptions"]
pub async fn list_subscriptions(
    client: &crate::Client,
    params: &stripe_types_08::subscriptions::ListSubscriptionsParams,
) -> Result<crate::List<stripe_types_14::subscriptions::Subscription>, crate::Error> {
    let path = crate::params::format_path("/v1/subscriptions", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "subscriptions")]
#[doc = "Migrate a subscription"]
pub async fn migrate_subscription(
    client: &crate::Client,
    subscription: &str,
    params: &stripe_types_12::subscriptions::MigrateSubscriptionParams,
) -> Result<stripe_types_14::subscriptions::Subscription, crate::Error> {
    let path =
        crate::params::format_path("/v1/subscriptions/{subscription}/migrate", &[("subscription", subscription)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "subscriptions")]
#[doc = "Pause a subscription"]
pub async fn pause_subscription(
    client: &crate::Client,
    subscription: &str,
    params: &stripe_types_11::subscriptions::PauseSubscriptionParams,
) -> Result<stripe_types_14::subscriptions::Subscription, crate::Error> {
    let path = crate::params::format_path("/v1/subscriptions/{subscription}/pause", &[("subscription", subscription)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "subscriptions")]
#[doc = "Resume a subscription"]
pub async fn resume_subscription(
    client: &crate::Client,
    subscription: &str,
    params: &stripe_types_11::subscriptions::ResumeSubscriptionParams,
) -> Result<stripe_types_14::subscriptions::Subscription, crate::Error> {
    let path = crate::params::format_path("/v1/subscriptions/{subscription}/resume", &[("subscription", subscription)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "subscriptions")]
#[doc = "Search subscriptions"]
pub async fn search_subscriptions(
    client: &crate::Client,
    params: &stripe_types_02::subscriptions::SearchSubscriptionsParams,
) -> Result<crate::List<stripe_types_14::subscriptions::Subscription>, crate::Error> {
    let path = crate::params::format_path("/v1/subscriptions/search", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "subscriptions")]
#[doc = "Update a subscription"]
pub async fn update_subscription(
    client: &crate::Client,
    subscription_exposed_id: &str,
    params: &stripe_types_13::subscriptions::UpdateSubscriptionParams,
) -> Result<stripe_types_14::subscriptions::Subscription, crate::Error> {
    let path = crate::params::format_path(
        "/v1/subscriptions/{subscription_exposed_id}",
        &[("subscription_exposed_id", subscription_exposed_id)],
    );
    client.post_form(&path, params).await
}
