#[cfg(feature = "payment_intents")]
#[doc = "Reconcile a customer_balance PaymentIntent"]
pub async fn apply_customer_balance(
    client: &crate::Client,
    intent: &str,
    params: &stripe_types_03::payment_intents::ApplyCustomerBalanceParams,
) -> Result<stripe_types_14::payment_intents::PaymentIntent, crate::Error> {
    let path = crate::params::format_path("/v1/payment_intents/{intent}/apply_customer_balance", &[("intent", intent)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_intents")]
#[doc = "Cancel a PaymentIntent"]
pub async fn cancel_payment_intent(
    client: &crate::Client,
    intent: &str,
    params: &stripe_types_05::payment_intents::CancelPaymentIntentParams,
) -> Result<stripe_types_14::payment_intents::PaymentIntent, crate::Error> {
    let path = crate::params::format_path("/v1/payment_intents/{intent}/cancel", &[("intent", intent)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_intents")]
#[doc = "Capture a PaymentIntent"]
pub async fn capture_payment_intent(
    client: &crate::Client,
    intent: &str,
    params: &stripe_types_12::payment_intents::CapturePaymentIntentParams,
) -> Result<stripe_types_14::payment_intents::PaymentIntent, crate::Error> {
    let path = crate::params::format_path("/v1/payment_intents/{intent}/capture", &[("intent", intent)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_intents")]
#[doc = "Confirm a PaymentIntent"]
pub async fn confirm_payment_intent(
    client: &crate::Client,
    intent: &str,
    params: &stripe_types_13::payment_intents::ConfirmPaymentIntentParams,
) -> Result<stripe_types_14::payment_intents::PaymentIntent, crate::Error> {
    let path = crate::params::format_path("/v1/payment_intents/{intent}/confirm", &[("intent", intent)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_intents")]
#[doc = "Create a PaymentIntent"]
pub async fn create_payment_intent(
    client: &crate::Client,
    params: &stripe_types_13::payment_intents::CreatePaymentIntentParams,
) -> Result<stripe_types_14::payment_intents::PaymentIntent, crate::Error> {
    let path = crate::params::format_path("/v1/payment_intents", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_intents")]
#[doc = "Retrieve a PaymentIntent"]
pub async fn get_payment_intent(
    client: &crate::Client,
    intent: &str,
    params: &stripe_types_03::payment_intents::GetPaymentIntentParams,
) -> Result<stripe_types_14::payment_intents::PaymentIntent, crate::Error> {
    let path = crate::params::format_path("/v1/payment_intents/{intent}", &[("intent", intent)]);
    client.get(&path, params).await
}

#[cfg(feature = "payment_intents")]
#[doc = "Increment an authorization"]
pub async fn increment_authorization(
    client: &crate::Client,
    intent: &str,
    params: &stripe_types_12::payment_intents::IncrementAuthorizationParams,
) -> Result<stripe_types_14::payment_intents::PaymentIntent, crate::Error> {
    let path =
        crate::params::format_path("/v1/payment_intents/{intent}/increment_authorization", &[("intent", intent)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_intents")]
#[doc = "List all PaymentIntent LineItems"]
pub async fn list_payment_intent_amount_details_line_items(
    client: &crate::Client,
    intent: &str,
    params: &stripe_types_01::payment_intents::ListPaymentIntentAmountDetailsLineItemsParams,
) -> Result<crate::List<stripe_types_10::payment_intents::PaymentIntentAmountDetailsLineItem>, crate::Error> {
    let path =
        crate::params::format_path("/v1/payment_intents/{intent}/amount_details_line_items", &[("intent", intent)]);
    client.get(&path, params).await
}

#[cfg(feature = "payment_intents")]
#[doc = "List all PaymentIntents"]
pub async fn list_payment_intents(
    client: &crate::Client,
    params: &stripe_types_05::payment_intents::ListPaymentIntentsParams,
) -> Result<crate::List<stripe_types_14::payment_intents::PaymentIntent>, crate::Error> {
    let path = crate::params::format_path("/v1/payment_intents", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "payment_intents")]
#[doc = "Search PaymentIntents"]
pub async fn search_payment_intents(
    client: &crate::Client,
    params: &stripe_types_04::payment_intents::SearchPaymentIntentsParams,
) -> Result<crate::List<stripe_types_14::payment_intents::PaymentIntent>, crate::Error> {
    let path = crate::params::format_path("/v1/payment_intents/search", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "payment_intents")]
#[doc = "Update a PaymentIntent"]
pub async fn update_payment_intent(
    client: &crate::Client,
    intent: &str,
    params: &stripe_types_13::payment_intents::UpdatePaymentIntentParams,
) -> Result<stripe_types_14::payment_intents::PaymentIntent, crate::Error> {
    let path = crate::params::format_path("/v1/payment_intents/{intent}", &[("intent", intent)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_intents")]
#[doc = "Verify microdeposits on a PaymentIntent"]
pub async fn verify_microdeposits(
    client: &crate::Client,
    intent: &str,
    params: &stripe_types_01::payment_intents::VerifyMicrodepositsParams,
) -> Result<stripe_types_14::payment_intents::PaymentIntent, crate::Error> {
    let path = crate::params::format_path("/v1/payment_intents/{intent}/verify_microdeposits", &[("intent", intent)]);
    client.post_form(&path, params).await
}
