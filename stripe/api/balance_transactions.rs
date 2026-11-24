#[cfg(feature = "balance_transactions")]
#[doc = "Retrieve a balance transaction"]
pub async fn get_balance_transaction(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_03::balance_transactions::GetBalanceTransactionParams,
) -> Result<stripe_types_14::balance_transactions::BalanceTransaction, crate::Error> {
    let path = crate::params::format_path("/v1/balance_transactions/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "balance_transactions")]
#[doc = "List all balance transactions"]
pub async fn list_balance_transactions(
    client: &crate::Client,
    params: &stripe_types_06::balance_transactions::ListBalanceTransactionsParams,
) -> Result<crate::List<stripe_types_14::balance_transactions::BalanceTransaction>, crate::Error> {
    let path = crate::params::format_path("/v1/balance_transactions", &[]);
    client.get(&path, params).await
}
