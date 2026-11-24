#[cfg(feature = "setup_intents")]
#[doc = "Cancel a SetupIntent"]
pub async fn cancel_setup_intent(
    client: &crate::Client,
    intent: &str,
    params: &stripe_types_05::setup_intents::CancelSetupIntentParams,
) -> Result<stripe_types_14::setup_intents::SetupIntent, crate::Error> {
    let path = crate::params::format_path("/v1/setup_intents/{intent}/cancel", &[("intent", intent)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "setup_intents")]
#[doc = "Confirm a SetupIntent"]
pub async fn confirm_setup_intent(
    client: &crate::Client,
    intent: &str,
    params: &stripe_types_13::setup_intents::ConfirmSetupIntentParams,
) -> Result<stripe_types_14::setup_intents::SetupIntent, crate::Error> {
    let path = crate::params::format_path("/v1/setup_intents/{intent}/confirm", &[("intent", intent)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "setup_intents")]
#[doc = "Create a SetupIntent"]
pub async fn create_setup_intent(
    client: &crate::Client,
    params: &stripe_types_13::setup_intents::CreateSetupIntentParams,
) -> Result<stripe_types_14::setup_intents::SetupIntent, crate::Error> {
    let path = crate::params::format_path("/v1/setup_intents", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "setup_intents")]
#[doc = "Retrieve a SetupIntent"]
pub async fn get_setup_intent(
    client: &crate::Client,
    intent: &str,
    params: &stripe_types_02::setup_intents::GetSetupIntentParams,
) -> Result<stripe_types_14::setup_intents::SetupIntent, crate::Error> {
    let path = crate::params::format_path("/v1/setup_intents/{intent}", &[("intent", intent)]);
    client.get(&path, params).await
}

#[cfg(feature = "setup_intents")]
#[doc = "List all SetupIntents"]
pub async fn list_setup_intents(
    client: &crate::Client,
    params: &stripe_types_06::setup_intents::ListSetupIntentsParams,
) -> Result<crate::List<stripe_types_14::setup_intents::SetupIntent>, crate::Error> {
    let path = crate::params::format_path("/v1/setup_intents", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "setup_intents")]
#[doc = "Update a SetupIntent"]
pub async fn update_setup_intent(
    client: &crate::Client,
    intent: &str,
    params: &stripe_types_13::setup_intents::UpdateSetupIntentParams,
) -> Result<stripe_types_14::setup_intents::SetupIntent, crate::Error> {
    let path = crate::params::format_path("/v1/setup_intents/{intent}", &[("intent", intent)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "setup_intents")]
#[doc = "Verify microdeposits on a SetupIntent"]
pub async fn verify_microdeposits(
    client: &crate::Client,
    intent: &str,
    params: &stripe_types_02::setup_intents::VerifyMicrodepositsParams,
) -> Result<stripe_types_14::setup_intents::SetupIntent, crate::Error> {
    let path = crate::params::format_path("/v1/setup_intents/{intent}/verify_microdeposits", &[("intent", intent)]);
    client.post_form(&path, params).await
}
