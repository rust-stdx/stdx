#[cfg(feature = "accounts")]
#[doc = "Create an account"]
pub async fn create_account(
    client: &crate::Client,
    params: &stripe_types_12::accounts::CreateAccountParams,
) -> Result<stripe_types_14::account::Account, crate::Error> {
    let path = crate::params::format_path("/v1/accounts", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "Create an external account"]
pub async fn create_account_external_account(
    client: &crate::Client,
    account: &str,
    params: &stripe_types_00::accounts::CreateAccountExternalAccountParams,
) -> Result<stripe_types_14::accounts::ExternalAccount, crate::Error> {
    let path = crate::params::format_path("/v1/accounts/{account}/external_accounts", &[("account", account)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "Create a login link"]
pub async fn create_account_login_link(
    client: &crate::Client,
    account: &str,
    params: &stripe_types_05::accounts::CreateAccountLoginLinkParams,
) -> Result<stripe_types_05::common::LoginLink, crate::Error> {
    let path = crate::params::format_path("/v1/accounts/{account}/login_links", &[("account", account)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "Create a person"]
pub async fn create_account_person(
    client: &crate::Client,
    account: &str,
    params: &stripe_types_10::accounts::CreateAccountPersonParams,
) -> Result<stripe_types_12::accounts::Person, crate::Error> {
    let path = crate::params::format_path("/v1/accounts/{account}/persons", &[("account", account)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "Delete an account"]
pub async fn delete_account(
    client: &crate::Client,
    account: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_05::account::DeletedAccount, crate::Error> {
    let path = crate::params::format_path("/v1/accounts/{account}", &[("account", account)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "Delete an external account"]
pub async fn delete_account_external_account(
    client: &crate::Client,
    account: &str,
    id: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_09::accounts::DeletedExternalAccount, crate::Error> {
    let path = crate::params::format_path(
        "/v1/accounts/{account}/external_accounts/{id}",
        &[("account", account), ("id", id)],
    );
    client.delete_form(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "Delete a person"]
pub async fn delete_account_person(
    client: &crate::Client,
    account: &str,
    person: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_05::accounts::DeletedPerson, crate::Error> {
    let path = crate::params::format_path(
        "/v1/accounts/{account}/persons/{person}",
        &[("account", account), ("person", person)],
    );
    client.delete_form(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "Retrieve account"]
pub async fn get_account(
    client: &crate::Client,
    account: &str,
    params: &stripe_types_03::accounts::GetAccountParams,
) -> Result<stripe_types_14::account::Account, crate::Error> {
    let path = crate::params::format_path("/v1/accounts/{account}", &[("account", account)]);
    client.get(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "Retrieve an Account Capability"]
pub async fn get_account_capability(
    client: &crate::Client,
    account: &str,
    capability: &str,
    params: &stripe_types_06::accounts::GetAccountCapabilityParams,
) -> Result<stripe_types_15::accounts::Capability, crate::Error> {
    let path = crate::params::format_path(
        "/v1/accounts/{account}/capabilities/{capability}",
        &[("account", account), ("capability", capability)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "Retrieve an external account"]
pub async fn get_account_external_account(
    client: &crate::Client,
    account: &str,
    id: &str,
    params: &stripe_types_05::accounts::GetAccountExternalAccountParams,
) -> Result<stripe_types_14::accounts::ExternalAccount, crate::Error> {
    let path = crate::params::format_path(
        "/v1/accounts/{account}/external_accounts/{id}",
        &[("account", account), ("id", id)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "Retrieve a person"]
pub async fn get_account_person(
    client: &crate::Client,
    account: &str,
    person: &str,
    params: &stripe_types_06::accounts::GetAccountPersonParams,
) -> Result<stripe_types_12::accounts::Person, crate::Error> {
    let path = crate::params::format_path(
        "/v1/accounts/{account}/persons/{person}",
        &[("account", account), ("person", person)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "List all account capabilities"]
pub async fn list_account_capabilities(
    client: &crate::Client,
    account: &str,
    params: &stripe_types_05::accounts::ListAccountCapabilitiesParams,
) -> Result<crate::List<stripe_types_15::accounts::Capability>, crate::Error> {
    let path = crate::params::format_path("/v1/accounts/{account}/capabilities", &[("account", account)]);
    client.get(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "List all external accounts"]
pub async fn list_account_external_accounts(
    client: &crate::Client,
    account: &str,
    params: &stripe_types_05::accounts::ListAccountExternalAccountsParams,
) -> Result<crate::List<stripe_types_14::accounts::ExternalAccount>, crate::Error> {
    let path = crate::params::format_path("/v1/accounts/{account}/external_accounts", &[("account", account)]);
    client.get(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "List all persons"]
pub async fn list_account_persons(
    client: &crate::Client,
    account: &str,
    params: &stripe_types_05::accounts::ListAccountPersonsParams,
) -> Result<crate::List<stripe_types_12::accounts::Person>, crate::Error> {
    let path = crate::params::format_path("/v1/accounts/{account}/persons", &[("account", account)]);
    client.get(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "List all connected accounts"]
pub async fn list_accounts(
    client: &crate::Client,
    params: &stripe_types_05::accounts::ListAccountsParams,
) -> Result<crate::List<stripe_types_14::account::Account>, crate::Error> {
    let path = crate::params::format_path("/v1/accounts", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "Reject an account"]
pub async fn reject_account(
    client: &crate::Client,
    account: &str,
    params: &stripe_types_05::accounts::RejectAccountParams,
) -> Result<stripe_types_14::account::Account, crate::Error> {
    let path = crate::params::format_path("/v1/accounts/{account}/reject", &[("account", account)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "Unreject an account"]
pub async fn unreject_account(
    client: &crate::Client,
    account: &str,
    params: &stripe_types_05::accounts::UnrejectAccountParams,
) -> Result<stripe_types_14::account::Account, crate::Error> {
    let path = crate::params::format_path("/v1/accounts/{account}/unreject", &[("account", account)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "Update an account"]
pub async fn update_account(
    client: &crate::Client,
    account: &str,
    params: &stripe_types_13::accounts::UpdateAccountParams,
) -> Result<stripe_types_14::account::Account, crate::Error> {
    let path = crate::params::format_path("/v1/accounts/{account}", &[("account", account)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "Update an Account Capability"]
pub async fn update_account_capability(
    client: &crate::Client,
    account: &str,
    capability: &str,
    params: &stripe_types_01::accounts::UpdateAccountCapabilityParams,
) -> Result<stripe_types_15::accounts::Capability, crate::Error> {
    let path = crate::params::format_path(
        "/v1/accounts/{account}/capabilities/{capability}",
        &[("account", account), ("capability", capability)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "Update a bank account"]
pub async fn update_account_external_account(
    client: &crate::Client,
    account: &str,
    id: &str,
    params: &stripe_types_11::accounts::UpdateAccountExternalAccountParams,
) -> Result<stripe_types_14::accounts::ExternalAccount, crate::Error> {
    let path = crate::params::format_path(
        "/v1/accounts/{account}/external_accounts/{id}",
        &[("account", account), ("id", id)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "accounts")]
#[doc = "Update a person"]
pub async fn update_account_person(
    client: &crate::Client,
    account: &str,
    person: &str,
    params: &stripe_types_12::accounts::UpdateAccountPersonParams,
) -> Result<stripe_types_12::accounts::Person, crate::Error> {
    let path = crate::params::format_path(
        "/v1/accounts/{account}/persons/{person}",
        &[("account", account), ("person", person)],
    );
    client.post_form(&path, params).await
}
