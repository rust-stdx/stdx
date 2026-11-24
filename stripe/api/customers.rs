#[cfg(feature = "customers")]
#[doc = "Create a customer"]
pub async fn create_customer(
    client: &crate::Client,
    params: &stripe_types_10::customers::CreateCustomerParams,
) -> Result<stripe_types_14::customers::Customer, crate::Error> {
    let path = crate::params::format_path("/v1/customers", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Create a customer balance transaction"]
pub async fn create_customer_balance_transaction(
    client: &crate::Client,
    customer: &str,
    params: &stripe_types_03::customers::CreateCustomerBalanceTransactionParams,
) -> Result<stripe_types_15::customers::CustomerBalanceTransaction, crate::Error> {
    let path = crate::params::format_path("/v1/customers/{customer}/balance_transactions", &[("customer", customer)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Create or retrieve funding instructions for a customer cash balance"]
pub async fn create_customer_funding_instruction(
    client: &crate::Client,
    customer: &str,
    params: &stripe_types_09::customers::CreateCustomerFundingInstructionParams,
) -> Result<stripe_types_12::payment_intents::FundingInstructions, crate::Error> {
    let path = crate::params::format_path("/v1/customers/{customer}/funding_instructions", &[("customer", customer)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Create a card"]
pub async fn create_customer_source(
    client: &crate::Client,
    customer: &str,
    params: &stripe_types_04::customers::CreateCustomerSourceParams,
) -> Result<stripe_types_14::common::PaymentSource, crate::Error> {
    let path = crate::params::format_path("/v1/customers/{customer}/sources", &[("customer", customer)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Create a Customer tax ID"]
pub async fn create_customer_tax_id(
    client: &crate::Client,
    customer: &str,
    params: &stripe_types_07::customers::CreateCustomerTaxIdParams,
) -> Result<stripe_types_14::tax_ids::TaxId, crate::Error> {
    let path = crate::params::format_path("/v1/customers/{customer}/tax_ids", &[("customer", customer)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Delete a customer"]
pub async fn delete_customer(
    client: &crate::Client,
    customer: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_05::customers::DeletedCustomer, crate::Error> {
    let path = crate::params::format_path("/v1/customers/{customer}", &[("customer", customer)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Delete a customer discount"]
pub async fn delete_customer_discount(
    client: &crate::Client,
    customer: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_15::coupons::DeletedDiscount, crate::Error> {
    let path = crate::params::format_path("/v1/customers/{customer}/discount", &[("customer", customer)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Delete a customer source"]
pub async fn delete_customer_source(
    client: &crate::Client,
    customer: &str,
    id: &str,
    params: &stripe_types_05::customers::DeleteCustomerSourceParams,
) -> Result<stripe_types_15::customers::DeleteCustomerSourceResponseExpanded, crate::Error> {
    let path =
        crate::params::format_path("/v1/customers/{customer}/sources/{id}", &[("customer", customer), ("id", id)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Delete a Customer tax ID"]
pub async fn delete_customer_tax_id(
    client: &crate::Client,
    customer: &str,
    id: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_05::tax_ids::DeletedTaxId, crate::Error> {
    let path =
        crate::params::format_path("/v1/customers/{customer}/tax_ids/{id}", &[("customer", customer), ("id", id)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Retrieve a customer"]
pub async fn get_customer(
    client: &crate::Client,
    customer: &str,
    params: &stripe_types_04::customers::GetCustomerParams,
) -> Result<stripe_types_15::customers::GetCustomerResponseExpanded, crate::Error> {
    let path = crate::params::format_path("/v1/customers/{customer}", &[("customer", customer)]);
    client.get(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Retrieve a customer balance transaction"]
pub async fn get_customer_balance_transaction(
    client: &crate::Client,
    customer: &str,
    transaction: &str,
    params: &stripe_types_05::customers::GetCustomerBalanceTransactionParams,
) -> Result<stripe_types_15::customers::CustomerBalanceTransaction, crate::Error> {
    let path = crate::params::format_path(
        "/v1/customers/{customer}/balance_transactions/{transaction}",
        &[("customer", customer), ("transaction", transaction)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Retrieve a cash balance"]
pub async fn get_customer_cash_balance(
    client: &crate::Client,
    customer: &str,
    params: &stripe_types_05::customers::GetCustomerCashBalanceParams,
) -> Result<stripe_types_08::customers::CashBalance, crate::Error> {
    let path = crate::params::format_path("/v1/customers/{customer}/cash_balance", &[("customer", customer)]);
    client.get(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Retrieve a cash balance transaction"]
pub async fn get_customer_cash_balance_transaction(
    client: &crate::Client,
    customer: &str,
    transaction: &str,
    params: &stripe_types_05::customers::GetCustomerCashBalanceTransactionParams,
) -> Result<stripe_types_14::customers::CustomerCashBalanceTransaction, crate::Error> {
    let path = crate::params::format_path(
        "/v1/customers/{customer}/cash_balance_transactions/{transaction}",
        &[("customer", customer), ("transaction", transaction)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Retrieve a Customer's PaymentMethod"]
pub async fn get_customer_payment_method(
    client: &crate::Client,
    customer: &str,
    payment_method: &str,
    params: &stripe_types_06::customers::GetCustomerPaymentMethodParams,
) -> Result<stripe_types_14::payment_methods::PaymentMethod, crate::Error> {
    let path = crate::params::format_path(
        "/v1/customers/{customer}/payment_methods/{payment_method}",
        &[("customer", customer), ("payment_method", payment_method)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "<p>Retrieve a specified source for a given customer.</p>"]
pub async fn get_customer_source(
    client: &crate::Client,
    customer: &str,
    id: &str,
    params: &stripe_types_06::customers::GetCustomerSourceParams,
) -> Result<stripe_types_14::common::PaymentSource, crate::Error> {
    let path =
        crate::params::format_path("/v1/customers/{customer}/sources/{id}", &[("customer", customer), ("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Retrieve a Customer tax ID"]
pub async fn get_customer_tax_id(
    client: &crate::Client,
    customer: &str,
    id: &str,
    params: &stripe_types_05::customers::GetCustomerTaxIdParams,
) -> Result<stripe_types_14::tax_ids::TaxId, crate::Error> {
    let path =
        crate::params::format_path("/v1/customers/{customer}/tax_ids/{id}", &[("customer", customer), ("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "List customer balance transactions"]
pub async fn list_customer_balance_transactions(
    client: &crate::Client,
    customer: &str,
    params: &stripe_types_07::customers::ListCustomerBalanceTransactionsParams,
) -> Result<crate::List<stripe_types_15::customers::CustomerBalanceTransaction>, crate::Error> {
    let path = crate::params::format_path("/v1/customers/{customer}/balance_transactions", &[("customer", customer)]);
    client.get(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "List cash balance transactions"]
pub async fn list_customer_cash_balance_transactions(
    client: &crate::Client,
    customer: &str,
    params: &stripe_types_00::customers::ListCustomerCashBalanceTransactionsParams,
) -> Result<crate::List<stripe_types_14::customers::CustomerCashBalanceTransaction>, crate::Error> {
    let path =
        crate::params::format_path("/v1/customers/{customer}/cash_balance_transactions", &[("customer", customer)]);
    client.get(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "List a Customer's PaymentMethods"]
pub async fn list_customer_payment_methods(
    client: &crate::Client,
    customer: &str,
    params: &stripe_types_06::customers::ListCustomerPaymentMethodsParams,
) -> Result<crate::List<stripe_types_14::payment_methods::PaymentMethod>, crate::Error> {
    let path = crate::params::format_path("/v1/customers/{customer}/payment_methods", &[("customer", customer)]);
    client.get(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "<p>List sources for a specified customer.</p>"]
pub async fn list_customer_sources(
    client: &crate::Client,
    customer: &str,
    params: &stripe_types_00::customers::ListCustomerSourcesParams,
) -> Result<crate::List<stripe_types_14::common::PaymentSource>, crate::Error> {
    let path = crate::params::format_path("/v1/customers/{customer}/sources", &[("customer", customer)]);
    client.get(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "List all Customer tax IDs"]
pub async fn list_customer_tax_ids(
    client: &crate::Client,
    customer: &str,
    params: &stripe_types_05::customers::ListCustomerTaxIdsParams,
) -> Result<crate::List<stripe_types_14::tax_ids::TaxId>, crate::Error> {
    let path = crate::params::format_path("/v1/customers/{customer}/tax_ids", &[("customer", customer)]);
    client.get(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "List all customers"]
pub async fn list_customers(
    client: &crate::Client,
    params: &stripe_types_05::customers::ListCustomersParams,
) -> Result<crate::List<stripe_types_14::customers::Customer>, crate::Error> {
    let path = crate::params::format_path("/v1/customers", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Search customers"]
pub async fn search_customers(
    client: &crate::Client,
    params: &stripe_types_04::customers::SearchCustomersParams,
) -> Result<crate::List<stripe_types_14::customers::Customer>, crate::Error> {
    let path = crate::params::format_path("/v1/customers/search", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Update a customer"]
pub async fn update_customer(
    client: &crate::Client,
    customer: &str,
    params: &stripe_types_12::customers::UpdateCustomerParams,
) -> Result<stripe_types_14::customers::Customer, crate::Error> {
    let path = crate::params::format_path("/v1/customers/{customer}", &[("customer", customer)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Update a customer credit balance transaction"]
pub async fn update_customer_balance_transaction(
    client: &crate::Client,
    customer: &str,
    transaction: &str,
    params: &stripe_types_00::customers::UpdateCustomerBalanceTransactionParams,
) -> Result<stripe_types_15::customers::CustomerBalanceTransaction, crate::Error> {
    let path = crate::params::format_path(
        "/v1/customers/{customer}/balance_transactions/{transaction}",
        &[("customer", customer), ("transaction", transaction)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Update a cash balance's settings"]
pub async fn update_customer_cash_balance(
    client: &crate::Client,
    customer: &str,
    params: &stripe_types_10::customers::UpdateCustomerCashBalanceParams,
) -> Result<stripe_types_08::customers::CashBalance, crate::Error> {
    let path = crate::params::format_path("/v1/customers/{customer}/cash_balance", &[("customer", customer)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Update a card"]
pub async fn update_customer_source(
    client: &crate::Client,
    customer: &str,
    id: &str,
    params: &stripe_types_10::customers::UpdateCustomerSourceParams,
) -> Result<stripe_types_15::customers::UpdateCustomerSourceResponseExpanded, crate::Error> {
    let path =
        crate::params::format_path("/v1/customers/{customer}/sources/{id}", &[("customer", customer), ("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "customers")]
#[doc = "Verify a bank account"]
pub async fn verify_customer_source(
    client: &crate::Client,
    customer: &str,
    id: &str,
    params: &stripe_types_02::customers::VerifyCustomerSourceParams,
) -> Result<stripe_types_14::payment_methods::BankAccount, crate::Error> {
    let path = crate::params::format_path(
        "/v1/customers/{customer}/sources/{id}/verify",
        &[("customer", customer), ("id", id)],
    );
    client.post_form(&path, params).await
}
