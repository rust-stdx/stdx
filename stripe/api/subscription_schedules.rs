#[cfg(feature = "subscription_schedules")]
#[doc = "Cancel a schedule"]
pub async fn cancel_subscription_schedule(
    client: &crate::Client,
    schedule: &str,
    params: &stripe_types_00::subscription_schedules::CancelSubscriptionScheduleParams,
) -> Result<stripe_types_14::subscription_schedules::SubscriptionSchedule, crate::Error> {
    let path = crate::params::format_path("/v1/subscription_schedules/{schedule}/cancel", &[("schedule", schedule)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "subscription_schedules")]
#[doc = "Create a schedule"]
pub async fn create_subscription_schedule(
    client: &crate::Client,
    params: &stripe_types_13::subscription_schedules::CreateSubscriptionScheduleParams,
) -> Result<stripe_types_14::subscription_schedules::SubscriptionSchedule, crate::Error> {
    let path = crate::params::format_path("/v1/subscription_schedules", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "subscription_schedules")]
#[doc = "Retrieve a schedule"]
pub async fn get_subscription_schedule(
    client: &crate::Client,
    schedule: &str,
    params: &stripe_types_00::subscription_schedules::GetSubscriptionScheduleParams,
) -> Result<stripe_types_14::subscription_schedules::SubscriptionSchedule, crate::Error> {
    let path = crate::params::format_path("/v1/subscription_schedules/{schedule}", &[("schedule", schedule)]);
    client.get(&path, params).await
}

#[cfg(feature = "subscription_schedules")]
#[doc = "List all schedules"]
pub async fn list_subscription_schedules(
    client: &crate::Client,
    params: &stripe_types_09::subscription_schedules::ListSubscriptionSchedulesParams,
) -> Result<crate::List<stripe_types_14::subscription_schedules::SubscriptionSchedule>, crate::Error> {
    let path = crate::params::format_path("/v1/subscription_schedules", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "subscription_schedules")]
#[doc = "Release a schedule"]
pub async fn release_subscription_schedule(
    client: &crate::Client,
    schedule: &str,
    params: &stripe_types_00::subscription_schedules::ReleaseSubscriptionScheduleParams,
) -> Result<stripe_types_14::subscription_schedules::SubscriptionSchedule, crate::Error> {
    let path = crate::params::format_path("/v1/subscription_schedules/{schedule}/release", &[("schedule", schedule)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "subscription_schedules")]
#[doc = "Update a schedule"]
pub async fn update_subscription_schedule(
    client: &crate::Client,
    schedule: &str,
    params: &stripe_types_13::subscription_schedules::UpdateSubscriptionScheduleParams,
) -> Result<stripe_types_14::subscription_schedules::SubscriptionSchedule, crate::Error> {
    let path = crate::params::format_path("/v1/subscription_schedules/{schedule}", &[("schedule", schedule)]);
    client.post_form(&path, params).await
}
