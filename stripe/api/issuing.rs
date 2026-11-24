#[cfg(feature = "issuing")]
#[doc = "Approve an authorization"]
pub async fn approve_issuing_authorization(
    client: &crate::Client,
    authorization: &str,
    params: &stripe_types_02::issuing::ApproveIssuingAuthorizationParams,
) -> Result<stripe_types_14::issuing::IssuingAuthorization, crate::Error> {
    let path = crate::params::format_path(
        "/v1/issuing/authorizations/{authorization}/approve",
        &[("authorization", authorization)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Create a card"]
pub async fn create_issuing_card(
    client: &crate::Client,
    params: &stripe_types_12::issuing::CreateIssuingCardParams,
) -> Result<stripe_types_13::issuing::IssuingCard, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/cards", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Create a cardholder"]
pub async fn create_issuing_cardholder(
    client: &crate::Client,
    params: &stripe_types_12::issuing::CreateIssuingCardholderParams,
) -> Result<stripe_types_13::issuing::IssuingCardholder, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/cardholders", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Create a dispute"]
pub async fn create_issuing_dispute(
    client: &crate::Client,
    params: &stripe_types_12::issuing::CreateIssuingDisputeParams,
) -> Result<stripe_types_14::issuing::IssuingDispute, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/disputes", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Create a personalization design"]
pub async fn create_issuing_personalization_design(
    client: &crate::Client,
    params: &stripe_types_06::issuing::CreateIssuingPersonalizationDesignParams,
) -> Result<stripe_types_10::issuing::IssuingPersonalizationDesign, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/personalization_designs", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Decline an authorization"]
pub async fn decline_issuing_authorization(
    client: &crate::Client,
    authorization: &str,
    params: &stripe_types_06::issuing::DeclineIssuingAuthorizationParams,
) -> Result<stripe_types_14::issuing::IssuingAuthorization, crate::Error> {
    let path = crate::params::format_path(
        "/v1/issuing/authorizations/{authorization}/decline",
        &[("authorization", authorization)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Retrieve an authorization"]
pub async fn get_issuing_authorization(
    client: &crate::Client,
    authorization: &str,
    params: &stripe_types_04::issuing::GetIssuingAuthorizationParams,
) -> Result<stripe_types_14::issuing::IssuingAuthorization, crate::Error> {
    let path = crate::params::format_path(
        "/v1/issuing/authorizations/{authorization}",
        &[("authorization", authorization)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Retrieve a card"]
pub async fn get_issuing_card(
    client: &crate::Client,
    card: &str,
    params: &stripe_types_06::issuing::GetIssuingCardParams,
) -> Result<stripe_types_13::issuing::IssuingCard, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/cards/{card}", &[("card", card)]);
    client.get(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Retrieve a cardholder"]
pub async fn get_issuing_cardholder(
    client: &crate::Client,
    cardholder: &str,
    params: &stripe_types_05::issuing::GetIssuingCardholderParams,
) -> Result<stripe_types_13::issuing::IssuingCardholder, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/cardholders/{cardholder}", &[("cardholder", cardholder)]);
    client.get(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Retrieve a dispute"]
pub async fn get_issuing_dispute(
    client: &crate::Client,
    dispute: &str,
    params: &stripe_types_05::issuing::GetIssuingDisputeParams,
) -> Result<stripe_types_14::issuing::IssuingDispute, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/disputes/{dispute}", &[("dispute", dispute)]);
    client.get(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Retrieve a personalization design"]
pub async fn get_issuing_personalization_design(
    client: &crate::Client,
    personalization_design: &str,
    params: &stripe_types_05::issuing::GetIssuingPersonalizationDesignParams,
) -> Result<stripe_types_10::issuing::IssuingPersonalizationDesign, crate::Error> {
    let path = crate::params::format_path(
        "/v1/issuing/personalization_designs/{personalization_design}",
        &[("personalization_design", personalization_design)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Retrieve a physical bundle"]
pub async fn get_issuing_physical_bundle(
    client: &crate::Client,
    physical_bundle: &str,
    params: &stripe_types_05::issuing::GetIssuingPhysicalBundleParams,
) -> Result<stripe_types_07::issuing::IssuingPhysicalBundle, crate::Error> {
    let path = crate::params::format_path(
        "/v1/issuing/physical_bundles/{physical_bundle}",
        &[("physical_bundle", physical_bundle)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Retrieve an issuing token"]
pub async fn get_issuing_token(
    client: &crate::Client,
    token: &str,
    params: &stripe_types_06::issuing::GetIssuingTokenParams,
) -> Result<stripe_types_13::issuing::IssuingToken, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/tokens/{token}", &[("token", token)]);
    client.get(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Retrieve a transaction"]
pub async fn get_issuing_transaction(
    client: &crate::Client,
    transaction: &str,
    params: &stripe_types_06::issuing::GetIssuingTransactionParams,
) -> Result<stripe_types_14::issuing::IssuingTransaction, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/transactions/{transaction}", &[("transaction", transaction)]);
    client.get(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "List all authorizations"]
pub async fn list_issuing_authorizations(
    client: &crate::Client,
    params: &stripe_types_06::issuing::ListIssuingAuthorizationsParams,
) -> Result<crate::List<stripe_types_14::issuing::IssuingAuthorization>, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/authorizations", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "List all cardholders"]
pub async fn list_issuing_cardholders(
    client: &crate::Client,
    params: &stripe_types_09::issuing::ListIssuingCardholdersParams,
) -> Result<crate::List<stripe_types_13::issuing::IssuingCardholder>, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/cardholders", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "List all cards"]
pub async fn list_issuing_cards(
    client: &crate::Client,
    params: &stripe_types_07::issuing::ListIssuingCardsParams,
) -> Result<crate::List<stripe_types_13::issuing::IssuingCard>, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/cards", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "List all disputes"]
pub async fn list_issuing_disputes(
    client: &crate::Client,
    params: &stripe_types_07::issuing::ListIssuingDisputesParams,
) -> Result<crate::List<stripe_types_14::issuing::IssuingDispute>, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/disputes", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "List all personalization designs"]
pub async fn list_issuing_personalization_designs(
    client: &crate::Client,
    params: &stripe_types_05::issuing::ListIssuingPersonalizationDesignsParams,
) -> Result<crate::List<stripe_types_10::issuing::IssuingPersonalizationDesign>, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/personalization_designs", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "List all physical bundles"]
pub async fn list_issuing_physical_bundles(
    client: &crate::Client,
    params: &stripe_types_05::issuing::ListIssuingPhysicalBundlesParams,
) -> Result<crate::List<stripe_types_07::issuing::IssuingPhysicalBundle>, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/physical_bundles", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "List all issuing tokens for card"]
pub async fn list_issuing_tokens(
    client: &crate::Client,
    params: &stripe_types_08::issuing::ListIssuingTokensParams,
) -> Result<crate::List<stripe_types_13::issuing::IssuingToken>, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/tokens", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "List all transactions"]
pub async fn list_issuing_transactions(
    client: &crate::Client,
    params: &stripe_types_08::issuing::ListIssuingTransactionsParams,
) -> Result<crate::List<stripe_types_14::issuing::IssuingTransaction>, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/transactions", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Submit a dispute"]
pub async fn submit_issuing_dispute(
    client: &crate::Client,
    dispute: &str,
    params: &stripe_types_05::issuing::SubmitIssuingDisputeParams,
) -> Result<stripe_types_14::issuing::IssuingDispute, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/disputes/{dispute}/submit", &[("dispute", dispute)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Update an authorization"]
pub async fn update_issuing_authorization(
    client: &crate::Client,
    authorization: &str,
    params: &stripe_types_01::issuing::UpdateIssuingAuthorizationParams,
) -> Result<stripe_types_14::issuing::IssuingAuthorization, crate::Error> {
    let path = crate::params::format_path(
        "/v1/issuing/authorizations/{authorization}",
        &[("authorization", authorization)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Update a card"]
pub async fn update_issuing_card(
    client: &crate::Client,
    card: &str,
    params: &stripe_types_12::issuing::UpdateIssuingCardParams,
) -> Result<stripe_types_13::issuing::IssuingCard, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/cards/{card}", &[("card", card)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Update a cardholder"]
pub async fn update_issuing_cardholder(
    client: &crate::Client,
    cardholder: &str,
    params: &stripe_types_12::issuing::UpdateIssuingCardholderParams,
) -> Result<stripe_types_13::issuing::IssuingCardholder, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/cardholders/{cardholder}", &[("cardholder", cardholder)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Update a dispute"]
pub async fn update_issuing_dispute(
    client: &crate::Client,
    dispute: &str,
    params: &stripe_types_12::issuing::UpdateIssuingDisputeParams,
) -> Result<stripe_types_14::issuing::IssuingDispute, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/disputes/{dispute}", &[("dispute", dispute)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Update a personalization design"]
pub async fn update_issuing_personalization_design(
    client: &crate::Client,
    personalization_design: &str,
    params: &stripe_types_07::issuing::UpdateIssuingPersonalizationDesignParams,
) -> Result<stripe_types_10::issuing::IssuingPersonalizationDesign, crate::Error> {
    let path = crate::params::format_path(
        "/v1/issuing/personalization_designs/{personalization_design}",
        &[("personalization_design", personalization_design)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Update a token status"]
pub async fn update_issuing_token(
    client: &crate::Client,
    token: &str,
    params: &stripe_types_06::issuing::UpdateIssuingTokenParams,
) -> Result<stripe_types_13::issuing::IssuingToken, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/tokens/{token}", &[("token", token)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "issuing")]
#[doc = "Update a transaction"]
pub async fn update_issuing_transaction(
    client: &crate::Client,
    transaction: &str,
    params: &stripe_types_05::issuing::UpdateIssuingTransactionParams,
) -> Result<stripe_types_14::issuing::IssuingTransaction, crate::Error> {
    let path = crate::params::format_path("/v1/issuing/transactions/{transaction}", &[("transaction", transaction)]);
    client.post_form(&path, params).await
}
