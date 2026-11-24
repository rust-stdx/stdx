#[cfg(feature = "treasury")]
#[doc = "Cancel an InboundTransfer"]
pub async fn cancel_treasury_inbound_transfer(
    client: &crate::Client,
    inbound_transfer: &str,
    params: &stripe_types_06::treasury::CancelTreasuryInboundTransferParams,
) -> Result<stripe_types_15::treasury::TreasuryInboundTransfer, crate::Error> {
    let path = crate::params::format_path(
        "/v1/treasury/inbound_transfers/{inbound_transfer}/cancel",
        &[("inbound_transfer", inbound_transfer)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Cancel an OutboundPayment"]
pub async fn cancel_treasury_outbound_payment(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_05::treasury::CancelTreasuryOutboundPaymentParams,
) -> Result<stripe_types_15::treasury::TreasuryOutboundPayment, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/outbound_payments/{id}/cancel", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Cancel an OutboundTransfer"]
pub async fn cancel_treasury_outbound_transfer(
    client: &crate::Client,
    outbound_transfer: &str,
    params: &stripe_types_06::treasury::CancelTreasuryOutboundTransferParams,
) -> Result<stripe_types_15::treasury::TreasuryOutboundTransfer, crate::Error> {
    let path = crate::params::format_path(
        "/v1/treasury/outbound_transfers/{outbound_transfer}/cancel",
        &[("outbound_transfer", outbound_transfer)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Close a FinancialAccount"]
pub async fn close_treasury_financial_account(
    client: &crate::Client,
    financial_account: &str,
    params: &stripe_types_11::treasury::CloseTreasuryFinancialAccountParams,
) -> Result<stripe_types_13::treasury::TreasuryFinancialAccount, crate::Error> {
    let path = crate::params::format_path(
        "/v1/treasury/financial_accounts/{financial_account}/close",
        &[("financial_account", financial_account)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Create a CreditReversal"]
pub async fn create_treasury_credit_reversal(
    client: &crate::Client,
    params: &stripe_types_04::treasury::CreateTreasuryCreditReversalParams,
) -> Result<stripe_types_15::treasury::TreasuryCreditReversal, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/credit_reversals", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Create a DebitReversal"]
pub async fn create_treasury_debit_reversal(
    client: &crate::Client,
    params: &stripe_types_01::treasury::CreateTreasuryDebitReversalParams,
) -> Result<stripe_types_15::treasury::TreasuryDebitReversal, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/debit_reversals", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Create a FinancialAccount"]
pub async fn create_treasury_financial_account(
    client: &crate::Client,
    params: &stripe_types_12::treasury::CreateTreasuryFinancialAccountParams,
) -> Result<stripe_types_13::treasury::TreasuryFinancialAccount, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/financial_accounts", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Update FinancialAccount Features"]
pub async fn create_treasury_financial_account_feature(
    client: &crate::Client,
    financial_account: &str,
    params: &stripe_types_11::treasury::CreateTreasuryFinancialAccountFeatureParams,
) -> Result<stripe_types_12::treasury::TreasuryFinancialAccountFeatures, crate::Error> {
    let path = crate::params::format_path(
        "/v1/treasury/financial_accounts/{financial_account}/features",
        &[("financial_account", financial_account)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Create an InboundTransfer"]
pub async fn create_treasury_inbound_transfer(
    client: &crate::Client,
    params: &stripe_types_01::treasury::CreateTreasuryInboundTransferParams,
) -> Result<stripe_types_15::treasury::TreasuryInboundTransfer, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/inbound_transfers", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Create an OutboundPayment"]
pub async fn create_treasury_outbound_payment(
    client: &crate::Client,
    params: &stripe_types_11::treasury::CreateTreasuryOutboundPaymentParams,
) -> Result<stripe_types_15::treasury::TreasuryOutboundPayment, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/outbound_payments", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Create an OutboundTransfer"]
pub async fn create_treasury_outbound_transfer(
    client: &crate::Client,
    params: &stripe_types_12::treasury::CreateTreasuryOutboundTransferParams,
) -> Result<stripe_types_15::treasury::TreasuryOutboundTransfer, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/outbound_transfers", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Retrieve a CreditReversal"]
pub async fn get_treasury_credit_reversal(
    client: &crate::Client,
    credit_reversal: &str,
    params: &stripe_types_01::treasury::GetTreasuryCreditReversalParams,
) -> Result<stripe_types_15::treasury::TreasuryCreditReversal, crate::Error> {
    let path = crate::params::format_path(
        "/v1/treasury/credit_reversals/{credit_reversal}",
        &[("credit_reversal", credit_reversal)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Retrieve a DebitReversal"]
pub async fn get_treasury_debit_reversal(
    client: &crate::Client,
    debit_reversal: &str,
    params: &stripe_types_05::treasury::GetTreasuryDebitReversalParams,
) -> Result<stripe_types_15::treasury::TreasuryDebitReversal, crate::Error> {
    let path = crate::params::format_path(
        "/v1/treasury/debit_reversals/{debit_reversal}",
        &[("debit_reversal", debit_reversal)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Retrieve a FinancialAccount"]
pub async fn get_treasury_financial_account(
    client: &crate::Client,
    financial_account: &str,
    params: &stripe_types_06::treasury::GetTreasuryFinancialAccountParams,
) -> Result<stripe_types_13::treasury::TreasuryFinancialAccount, crate::Error> {
    let path = crate::params::format_path(
        "/v1/treasury/financial_accounts/{financial_account}",
        &[("financial_account", financial_account)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Retrieve FinancialAccount Features"]
pub async fn get_treasury_financial_account_feature(
    client: &crate::Client,
    financial_account: &str,
    params: &stripe_types_06::treasury::GetTreasuryFinancialAccountFeatureParams,
) -> Result<stripe_types_12::treasury::TreasuryFinancialAccountFeatures, crate::Error> {
    let path = crate::params::format_path(
        "/v1/treasury/financial_accounts/{financial_account}/features",
        &[("financial_account", financial_account)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Retrieve an InboundTransfer"]
pub async fn get_treasury_inbound_transfer(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::treasury::GetTreasuryInboundTransferParams,
) -> Result<stripe_types_15::treasury::TreasuryInboundTransfer, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/inbound_transfers/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Retrieve an OutboundPayment"]
pub async fn get_treasury_outbound_payment(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_05::treasury::GetTreasuryOutboundPaymentParams,
) -> Result<stripe_types_15::treasury::TreasuryOutboundPayment, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/outbound_payments/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Retrieve an OutboundTransfer"]
pub async fn get_treasury_outbound_transfer(
    client: &crate::Client,
    outbound_transfer: &str,
    params: &stripe_types_05::treasury::GetTreasuryOutboundTransferParams,
) -> Result<stripe_types_15::treasury::TreasuryOutboundTransfer, crate::Error> {
    let path = crate::params::format_path(
        "/v1/treasury/outbound_transfers/{outbound_transfer}",
        &[("outbound_transfer", outbound_transfer)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Retrieve a ReceivedCredit"]
pub async fn get_treasury_received_credit(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_05::treasury::GetTreasuryReceivedCreditParams,
) -> Result<stripe_types_15::treasury::TreasuryReceivedCredit, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/received_credits/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Retrieve a ReceivedDebit"]
pub async fn get_treasury_received_debit(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::treasury::GetTreasuryReceivedDebitParams,
) -> Result<stripe_types_15::treasury::TreasuryReceivedDebit, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/received_debits/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Retrieve a Transaction"]
pub async fn get_treasury_transaction(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::treasury::GetTreasuryTransactionParams,
) -> Result<stripe_types_15::treasury::TreasuryTransaction, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/transactions/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Retrieve a TransactionEntry"]
pub async fn get_treasury_transaction_entry(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::treasury::GetTreasuryTransactionEntryParams,
) -> Result<stripe_types_15::treasury::TreasuryTransactionEntry, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/transaction_entries/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "List all CreditReversals"]
pub async fn list_treasury_credit_reversals(
    client: &crate::Client,
    params: &stripe_types_06::treasury::ListTreasuryCreditReversalsParams,
) -> Result<crate::List<stripe_types_15::treasury::TreasuryCreditReversal>, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/credit_reversals", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "List all DebitReversals"]
pub async fn list_treasury_debit_reversals(
    client: &crate::Client,
    params: &stripe_types_06::treasury::ListTreasuryDebitReversalsParams,
) -> Result<crate::List<stripe_types_15::treasury::TreasuryDebitReversal>, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/debit_reversals", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "List all FinancialAccounts"]
pub async fn list_treasury_financial_accounts(
    client: &crate::Client,
    params: &stripe_types_06::treasury::ListTreasuryFinancialAccountsParams,
) -> Result<crate::List<stripe_types_13::treasury::TreasuryFinancialAccount>, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/financial_accounts", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "List all InboundTransfers"]
pub async fn list_treasury_inbound_transfers(
    client: &crate::Client,
    params: &stripe_types_05::treasury::ListTreasuryInboundTransfersParams,
) -> Result<crate::List<stripe_types_15::treasury::TreasuryInboundTransfer>, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/inbound_transfers", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "List all OutboundPayments"]
pub async fn list_treasury_outbound_payments(
    client: &crate::Client,
    params: &stripe_types_09::treasury::ListTreasuryOutboundPaymentsParams,
) -> Result<crate::List<stripe_types_15::treasury::TreasuryOutboundPayment>, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/outbound_payments", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "List all OutboundTransfers"]
pub async fn list_treasury_outbound_transfers(
    client: &crate::Client,
    params: &stripe_types_09::treasury::ListTreasuryOutboundTransfersParams,
) -> Result<crate::List<stripe_types_15::treasury::TreasuryOutboundTransfer>, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/outbound_transfers", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "List all ReceivedCredits"]
pub async fn list_treasury_received_credits(
    client: &crate::Client,
    params: &stripe_types_07::treasury::ListTreasuryReceivedCreditsParams,
) -> Result<crate::List<stripe_types_15::treasury::TreasuryReceivedCredit>, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/received_credits", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "List all ReceivedDebits"]
pub async fn list_treasury_received_debits(
    client: &crate::Client,
    params: &stripe_types_09::treasury::ListTreasuryReceivedDebitsParams,
) -> Result<crate::List<stripe_types_15::treasury::TreasuryReceivedDebit>, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/received_debits", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "List all TransactionEntries"]
pub async fn list_treasury_transaction_entries(
    client: &crate::Client,
    params: &stripe_types_07::treasury::ListTreasuryTransactionEntriesParams,
) -> Result<crate::List<stripe_types_15::treasury::TreasuryTransactionEntry>, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/transaction_entries", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "List all Transactions"]
pub async fn list_treasury_transactions(
    client: &crate::Client,
    params: &stripe_types_10::treasury::ListTreasuryTransactionsParams,
) -> Result<crate::List<stripe_types_15::treasury::TreasuryTransaction>, crate::Error> {
    let path = crate::params::format_path("/v1/treasury/transactions", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "treasury")]
#[doc = "Update a FinancialAccount"]
pub async fn update_treasury_financial_account(
    client: &crate::Client,
    financial_account: &str,
    params: &stripe_types_12::treasury::UpdateTreasuryFinancialAccountParams,
) -> Result<stripe_types_13::treasury::TreasuryFinancialAccount, crate::Error> {
    let path = crate::params::format_path(
        "/v1/treasury/financial_accounts/{financial_account}",
        &[("financial_account", financial_account)],
    );
    client.post_form(&path, params).await
}
