#[cfg(feature = "payment_method_domains")]
#[doc = "Create a payment method domain"]
pub async fn create_payment_method_domain(
    client: &crate::Client,
    params: &stripe_types_00::payment_method_domains::CreatePaymentMethodDomainParams,
) -> Result<stripe_types_08::payment_method_domains::PaymentMethodDomain, crate::Error> {
    let path = crate::params::format_path("/v1/payment_method_domains", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_method_domains")]
#[doc = "Retrieve a payment method domain"]
pub async fn get_payment_method_domain(
    client: &crate::Client,
    payment_method_domain: &str,
    params: &stripe_types_04::payment_method_domains::GetPaymentMethodDomainParams,
) -> Result<stripe_types_08::payment_method_domains::PaymentMethodDomain, crate::Error> {
    let path = crate::params::format_path(
        "/v1/payment_method_domains/{payment_method_domain}",
        &[("payment_method_domain", payment_method_domain)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "payment_method_domains")]
#[doc = "List payment method domains"]
pub async fn list_payment_method_domains(
    client: &crate::Client,
    params: &stripe_types_03::payment_method_domains::ListPaymentMethodDomainsParams,
) -> Result<crate::List<stripe_types_08::payment_method_domains::PaymentMethodDomain>, crate::Error> {
    let path = crate::params::format_path("/v1/payment_method_domains", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "payment_method_domains")]
#[doc = "Update a payment method domain"]
pub async fn update_payment_method_domain(
    client: &crate::Client,
    payment_method_domain: &str,
    params: &stripe_types_01::payment_method_domains::UpdatePaymentMethodDomainParams,
) -> Result<stripe_types_08::payment_method_domains::PaymentMethodDomain, crate::Error> {
    let path = crate::params::format_path(
        "/v1/payment_method_domains/{payment_method_domain}",
        &[("payment_method_domain", payment_method_domain)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_method_domains")]
#[doc = "Validate an existing payment method domain"]
pub async fn validate_payment_method_domain(
    client: &crate::Client,
    payment_method_domain: &str,
    params: &stripe_types_05::payment_method_domains::ValidatePaymentMethodDomainParams,
) -> Result<stripe_types_08::payment_method_domains::PaymentMethodDomain, crate::Error> {
    let path = crate::params::format_path(
        "/v1/payment_method_domains/{payment_method_domain}/validate",
        &[("payment_method_domain", payment_method_domain)],
    );
    client.post_form(&path, params).await
}
