#[cfg(feature = "apple_pay")]
#[doc = "<p>Create an apple pay domain.</p>"]
pub async fn create_apple_pay_domain(
    client: &crate::Client,
    params: &stripe_types_02::apple_pay::CreateApplePayDomainParams,
) -> Result<stripe_types_05::apple_pay::ApplePayDomain, crate::Error> {
    let path = crate::params::format_path("/v1/apple_pay/domains", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "apple_pay")]
#[doc = "<p>Delete an apple pay domain.</p>"]
pub async fn delete_apple_pay_domain(
    client: &crate::Client,
    domain: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_06::apple_pay::DeletedApplePayDomain, crate::Error> {
    let path = crate::params::format_path("/v1/apple_pay/domains/{domain}", &[("domain", domain)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "apple_pay")]
#[doc = "<p>Retrieve an apple pay domain.</p>"]
pub async fn get_apple_pay_domain(
    client: &crate::Client,
    domain: &str,
    params: &stripe_types_02::apple_pay::GetApplePayDomainParams,
) -> Result<stripe_types_05::apple_pay::ApplePayDomain, crate::Error> {
    let path = crate::params::format_path("/v1/apple_pay/domains/{domain}", &[("domain", domain)]);
    client.get(&path, params).await
}

#[cfg(feature = "apple_pay")]
#[doc = "<p>List apple pay domains.</p>"]
pub async fn list_apple_pay_domains(
    client: &crate::Client,
    params: &stripe_types_04::apple_pay::ListApplePayDomainsParams,
) -> Result<crate::List<stripe_types_05::apple_pay::ApplePayDomain>, crate::Error> {
    let path = crate::params::format_path("/v1/apple_pay/domains", &[]);
    client.get(&path, params).await
}
