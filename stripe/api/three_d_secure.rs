#[cfg(feature = "three_d_secure")]
#[doc = "Cancel a 3DS Authentication"]
pub async fn cancel_three_d_secure_authentication(
    client: &crate::Client,
    authentication: &str,
    params: &stripe_types_01::three_d_secure::CancelThreeDSecureAuthenticationParams,
) -> Result<stripe_types_15::three_d_secure::ThreeDSecureAuthentication, crate::Error> {
    let path = crate::params::format_path(
        "/v1/three_d_secure/authentications/{authentication}/cancel",
        &[("authentication", authentication)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "three_d_secure")]
#[doc = "Create a 3DS Authentication"]
pub async fn create_three_d_secure_authentication(
    client: &crate::Client,
    params: &stripe_types_13::three_d_secure::CreateThreeDSecureAuthenticationParams,
) -> Result<stripe_types_15::three_d_secure::ThreeDSecureAuthentication, crate::Error> {
    let path = crate::params::format_path("/v1/three_d_secure/authentications", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "three_d_secure")]
#[doc = "Retrieve a 3DS Authentication"]
pub async fn get_three_d_secure_authentication(
    client: &crate::Client,
    authentication: &str,
    params: &stripe_types_04::three_d_secure::GetThreeDSecureAuthenticationParams,
) -> Result<stripe_types_15::three_d_secure::ThreeDSecureAuthentication, crate::Error> {
    let path = crate::params::format_path(
        "/v1/three_d_secure/authentications/{authentication}",
        &[("authentication", authentication)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "three_d_secure")]
#[doc = "List all 3D Secure Authentications"]
pub async fn list_three_d_secure_authentications(
    client: &crate::Client,
    params: &stripe_types_05::three_d_secure::ListThreeDSecureAuthenticationsParams,
) -> Result<crate::List<stripe_types_15::three_d_secure::ThreeDSecureAuthentication>, crate::Error> {
    let path = crate::params::format_path("/v1/three_d_secure/authentications", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "three_d_secure")]
#[doc = "Submit a 3DS Authentication"]
pub async fn submit_three_d_secure_authentication(
    client: &crate::Client,
    authentication: &str,
    params: &stripe_types_03::three_d_secure::SubmitThreeDSecureAuthenticationParams,
) -> Result<stripe_types_15::three_d_secure::ThreeDSecureAuthentication, crate::Error> {
    let path = crate::params::format_path(
        "/v1/three_d_secure/authentications/{authentication}/submit",
        &[("authentication", authentication)],
    );
    client.post_form(&path, params).await
}
