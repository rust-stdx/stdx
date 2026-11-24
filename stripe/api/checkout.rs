#[cfg(feature = "checkout")]
#[doc = "Create a Checkout Session"]
pub async fn create_checkout_session(
    client: &crate::Client,
    params: &stripe_types_13::checkout::CreateCheckoutSessionParams,
) -> Result<stripe_types_15::checkout::CheckoutSession, crate::Error> {
    let path = crate::params::format_path("/v1/checkout/sessions", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "checkout")]
#[doc = "Expire a Checkout Session"]
pub async fn expire_checkout_session(
    client: &crate::Client,
    session: &str,
    params: &stripe_types_05::checkout::ExpireCheckoutSessionParams,
) -> Result<stripe_types_15::checkout::CheckoutSession, crate::Error> {
    let path = crate::params::format_path("/v1/checkout/sessions/{session}/expire", &[("session", session)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "checkout")]
#[doc = "Retrieve a Checkout Session"]
pub async fn get_checkout_session(
    client: &crate::Client,
    session: &str,
    params: &stripe_types_04::checkout::GetCheckoutSessionParams,
) -> Result<stripe_types_15::checkout::CheckoutSession, crate::Error> {
    let path = crate::params::format_path("/v1/checkout/sessions/{session}", &[("session", session)]);
    client.get(&path, params).await
}

#[cfg(feature = "checkout")]
#[doc = "Retrieve a Checkout Session's line items"]
pub async fn list_checkout_session_line_items(
    client: &crate::Client,
    session: &str,
    params: &stripe_types_01::checkout::ListCheckoutSessionLineItemsParams,
) -> Result<crate::List<stripe_types_15::common::Item>, crate::Error> {
    let path = crate::params::format_path("/v1/checkout/sessions/{session}/line_items", &[("session", session)]);
    client.get(&path, params).await
}

#[cfg(feature = "checkout")]
#[doc = "List all Checkout Sessions"]
pub async fn list_checkout_sessions(
    client: &crate::Client,
    params: &stripe_types_06::checkout::ListCheckoutSessionsParams,
) -> Result<crate::List<stripe_types_15::checkout::CheckoutSession>, crate::Error> {
    let path = crate::params::format_path("/v1/checkout/sessions", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "checkout")]
#[doc = "Update a Checkout Session"]
pub async fn update_checkout_session(
    client: &crate::Client,
    session: &str,
    params: &stripe_types_13::checkout::UpdateCheckoutSessionParams,
) -> Result<stripe_types_15::checkout::CheckoutSession, crate::Error> {
    let path = crate::params::format_path("/v1/checkout/sessions/{session}", &[("session", session)]);
    client.post_form(&path, params).await
}
