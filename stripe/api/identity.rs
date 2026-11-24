#[cfg(feature = "identity")]
#[doc = "Cancel a VerificationSession"]
pub async fn cancel_identity_verification_session(
    client: &crate::Client,
    session: &str,
    params: &stripe_types_05::identity::CancelIdentityVerificationSessionParams,
) -> Result<stripe_types_12::identity::IdentityVerificationSession, crate::Error> {
    let path =
        crate::params::format_path("/v1/identity/verification_sessions/{session}/cancel", &[("session", session)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "identity")]
#[doc = "Create a VerificationSession"]
pub async fn create_identity_verification_session(
    client: &crate::Client,
    params: &stripe_types_11::identity::CreateIdentityVerificationSessionParams,
) -> Result<stripe_types_12::identity::IdentityVerificationSession, crate::Error> {
    let path = crate::params::format_path("/v1/identity/verification_sessions", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "identity")]
#[doc = "Retrieve a VerificationReport"]
pub async fn get_identity_verification_report(
    client: &crate::Client,
    report: &str,
    params: &stripe_types_03::identity::GetIdentityVerificationReportParams,
) -> Result<stripe_types_11::identity::IdentityVerificationReport, crate::Error> {
    let path = crate::params::format_path("/v1/identity/verification_reports/{report}", &[("report", report)]);
    client.get(&path, params).await
}

#[cfg(feature = "identity")]
#[doc = "Retrieve a VerificationSession"]
pub async fn get_identity_verification_session(
    client: &crate::Client,
    session: &str,
    params: &stripe_types_05::identity::GetIdentityVerificationSessionParams,
) -> Result<stripe_types_12::identity::IdentityVerificationSession, crate::Error> {
    let path = crate::params::format_path("/v1/identity/verification_sessions/{session}", &[("session", session)]);
    client.get(&path, params).await
}

#[cfg(feature = "identity")]
#[doc = "List VerificationReports"]
pub async fn list_identity_verification_reports(
    client: &crate::Client,
    params: &stripe_types_05::identity::ListIdentityVerificationReportsParams,
) -> Result<crate::List<stripe_types_11::identity::IdentityVerificationReport>, crate::Error> {
    let path = crate::params::format_path("/v1/identity/verification_reports", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "identity")]
#[doc = "List VerificationSessions"]
pub async fn list_identity_verification_sessions(
    client: &crate::Client,
    params: &stripe_types_09::identity::ListIdentityVerificationSessionsParams,
) -> Result<crate::List<stripe_types_12::identity::IdentityVerificationSession>, crate::Error> {
    let path = crate::params::format_path("/v1/identity/verification_sessions", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "identity")]
#[doc = "Redact a VerificationSession"]
pub async fn redact_identity_verification_session(
    client: &crate::Client,
    session: &str,
    params: &stripe_types_06::identity::RedactIdentityVerificationSessionParams,
) -> Result<stripe_types_12::identity::IdentityVerificationSession, crate::Error> {
    let path =
        crate::params::format_path("/v1/identity/verification_sessions/{session}/redact", &[("session", session)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "identity")]
#[doc = "Update a VerificationSession"]
pub async fn update_identity_verification_session(
    client: &crate::Client,
    session: &str,
    params: &stripe_types_12::identity::UpdateIdentityVerificationSessionParams,
) -> Result<stripe_types_12::identity::IdentityVerificationSession, crate::Error> {
    let path = crate::params::format_path("/v1/identity/verification_sessions/{session}", &[("session", session)]);
    client.post_form(&path, params).await
}
