#[cfg(feature = "subscription_items")]
#[doc = "Create a subscription item"]
pub async fn create_subscription_item(
    client: &crate::Client,
    params: &stripe_types_10::subscription_items::CreateSubscriptionItemParams,
) -> Result<stripe_types_14::subscription_items::SubscriptionItem, crate::Error> {
    let path = crate::params::format_path("/v1/subscription_items", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "subscription_items")]
#[doc = "Delete a subscription item"]
pub async fn delete_subscription_item(
    client: &crate::Client,
    item: &str,
    params: &stripe_types_08::subscription_items::DeleteSubscriptionItemParams,
) -> Result<stripe_types_05::subscription_items::DeletedSubscriptionItem, crate::Error> {
    let path = crate::params::format_path("/v1/subscription_items/{item}", &[("item", item)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "subscription_items")]
#[doc = "Retrieve a subscription item"]
pub async fn get_subscription_item(
    client: &crate::Client,
    item: &str,
    params: &stripe_types_03::subscription_items::GetSubscriptionItemParams,
) -> Result<stripe_types_14::subscription_items::SubscriptionItem, crate::Error> {
    let path = crate::params::format_path("/v1/subscription_items/{item}", &[("item", item)]);
    client.get(&path, params).await
}

#[cfg(feature = "subscription_items")]
#[doc = "List all subscription items"]
pub async fn list_subscription_items(
    client: &crate::Client,
    params: &stripe_types_03::subscription_items::ListSubscriptionItemsParams,
) -> Result<crate::List<stripe_types_14::subscription_items::SubscriptionItem>, crate::Error> {
    let path = crate::params::format_path("/v1/subscription_items", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "subscription_items")]
#[doc = "Update a subscription item"]
pub async fn update_subscription_item(
    client: &crate::Client,
    item: &str,
    params: &stripe_types_12::subscription_items::UpdateSubscriptionItemParams,
) -> Result<stripe_types_14::subscription_items::SubscriptionItem, crate::Error> {
    let path = crate::params::format_path("/v1/subscription_items/{item}", &[("item", item)]);
    client.post_form(&path, params).await
}
