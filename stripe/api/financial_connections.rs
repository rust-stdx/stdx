#[cfg(feature = "financial_connections")]
#[doc = "Create a Session"]
pub async fn create_financial_connection_session(
    client: &crate::Client,
    params: &stripe_types_09::financial_connections::CreateFinancialConnectionSessionParams,
) -> Result<stripe_types_15::financial_connections::FinancialConnectionsSession, crate::Error> {
    let path = crate::params::format_path("/v1/financial_connections/sessions", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "financial_connections")]
#[doc = "Disconnect an Account"]
pub async fn disconnect_financial_connection_account(
    client: &crate::Client,
    account: &str,
    params: &stripe_types_06::financial_connections::DisconnectFinancialConnectionAccountParams,
) -> Result<stripe_types_15::financial_connections::FinancialConnectionsAccount, crate::Error> {
    let path = crate::params::format_path(
        "/v1/financial_connections/accounts/{account}/disconnect",
        &[("account", account)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "financial_connections")]
#[doc = "Retrieve an Account"]
pub async fn get_financial_connection_account(
    client: &crate::Client,
    account: &str,
    params: &stripe_types_03::financial_connections::GetFinancialConnectionAccountParams,
) -> Result<stripe_types_15::financial_connections::FinancialConnectionsAccount, crate::Error> {
    let path = crate::params::format_path("/v1/financial_connections/accounts/{account}", &[("account", account)]);
    client.get(&path, params).await
}

#[cfg(feature = "financial_connections")]
#[doc = "Retrieve a Session"]
pub async fn get_financial_connection_session(
    client: &crate::Client,
    session: &str,
    params: &stripe_types_06::financial_connections::GetFinancialConnectionSessionParams,
) -> Result<stripe_types_15::financial_connections::FinancialConnectionsSession, crate::Error> {
    let path = crate::params::format_path("/v1/financial_connections/sessions/{session}", &[("session", session)]);
    client.get(&path, params).await
}

#[cfg(feature = "financial_connections")]
#[doc = "Retrieve a Transaction"]
pub async fn get_financial_connection_transaction(
    client: &crate::Client,
    transaction: &str,
    params: &stripe_types_05::financial_connections::GetFinancialConnectionTransactionParams,
) -> Result<stripe_types_05::financial_connections::FinancialConnectionsTransaction, crate::Error> {
    let path = crate::params::format_path(
        "/v1/financial_connections/transactions/{transaction}",
        &[("transaction", transaction)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "financial_connections")]
#[doc = "List Account Owners"]
pub async fn list_financial_connection_account_owners(
    client: &crate::Client,
    account: &str,
    params: &stripe_types_03::financial_connections::ListFinancialConnectionAccountOwnersParams,
) -> Result<crate::List<stripe_types_06::financial_connections::FinancialConnectionsAccountOwner>, crate::Error> {
    let path =
        crate::params::format_path("/v1/financial_connections/accounts/{account}/owners", &[("account", account)]);
    client.get(&path, params).await
}

#[cfg(feature = "financial_connections")]
#[doc = "List Accounts"]
pub async fn list_financial_connection_accounts(
    client: &crate::Client,
    params: &stripe_types_06::financial_connections::ListFinancialConnectionAccountsParams,
) -> Result<crate::List<stripe_types_15::financial_connections::FinancialConnectionsAccount>, crate::Error> {
    let path = crate::params::format_path("/v1/financial_connections/accounts", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "financial_connections")]
#[doc = "List Transactions"]
pub async fn list_financial_connection_transactions(
    client: &crate::Client,
    params: &stripe_types_05::financial_connections::ListFinancialConnectionTransactionsParams,
) -> Result<crate::List<stripe_types_05::financial_connections::FinancialConnectionsTransaction>, crate::Error> {
    let path = crate::params::format_path("/v1/financial_connections/transactions", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "financial_connections")]
#[doc = "Refresh Account data"]
pub async fn refresh_financial_connection_account(
    client: &crate::Client,
    account: &str,
    params: &stripe_types_06::financial_connections::RefreshFinancialConnectionAccountParams,
) -> Result<stripe_types_15::financial_connections::FinancialConnectionsAccount, crate::Error> {
    let path =
        crate::params::format_path("/v1/financial_connections/accounts/{account}/refresh", &[("account", account)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "financial_connections")]
#[doc = "Subscribe to data refreshes for an Account"]
pub async fn subscribe_financial_connection_account(
    client: &crate::Client,
    account: &str,
    params: &stripe_types_06::financial_connections::SubscribeFinancialConnectionAccountParams,
) -> Result<stripe_types_15::financial_connections::FinancialConnectionsAccount, crate::Error> {
    let path = crate::params::format_path(
        "/v1/financial_connections/accounts/{account}/subscribe",
        &[("account", account)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "financial_connections")]
#[doc = "Unsubscribe from data refreshes for an Account"]
pub async fn unsubscribe_financial_connection_account(
    client: &crate::Client,
    account: &str,
    params: &stripe_types_09::financial_connections::UnsubscribeFinancialConnectionAccountParams,
) -> Result<stripe_types_15::financial_connections::FinancialConnectionsAccount, crate::Error> {
    let path = crate::params::format_path(
        "/v1/financial_connections/accounts/{account}/unsubscribe",
        &[("account", account)],
    );
    client.post_form(&path, params).await
}
