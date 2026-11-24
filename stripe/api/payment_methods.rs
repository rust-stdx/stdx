#[cfg(feature = "payment_methods")]
#[doc = "Attach a PaymentMethod to a Customer"]
pub async fn attach_payment_method(
    client: &crate::Client,
    payment_method: &str,
    params: &stripe_types_04::payment_methods::AttachPaymentMethodParams,
) -> Result<stripe_types_14::payment_methods::PaymentMethod, crate::Error> {
    let path = crate::params::format_path(
        "/v1/payment_methods/{payment_method}/attach",
        &[("payment_method", payment_method)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_methods")]
#[doc = "Create a PaymentMethod"]
pub async fn create_payment_method(
    client: &crate::Client,
    params: &stripe_types_11::payment_methods::CreatePaymentMethodParams,
) -> Result<stripe_types_14::payment_methods::PaymentMethod, crate::Error> {
    let path = crate::params::format_path("/v1/payment_methods", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_methods")]
#[doc = "Detach a PaymentMethod from a Customer"]
pub async fn detach_payment_method(
    client: &crate::Client,
    payment_method: &str,
    params: &stripe_types_05::payment_methods::DetachPaymentMethodParams,
) -> Result<stripe_types_14::payment_methods::PaymentMethod, crate::Error> {
    let path = crate::params::format_path(
        "/v1/payment_methods/{payment_method}/detach",
        &[("payment_method", payment_method)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_methods")]
#[doc = "Retrieve a PaymentMethod"]
pub async fn get_payment_method(
    client: &crate::Client,
    payment_method: &str,
    params: &stripe_types_03::payment_methods::GetPaymentMethodParams,
) -> Result<stripe_types_14::payment_methods::PaymentMethod, crate::Error> {
    let path =
        crate::params::format_path("/v1/payment_methods/{payment_method}", &[("payment_method", payment_method)]);
    client.get(&path, params).await
}

#[cfg(feature = "payment_methods")]
#[doc = "List PaymentMethods"]
pub async fn list_payment_methods(
    client: &crate::Client,
    params: &stripe_types_06::payment_methods::ListPaymentMethodsParams,
) -> Result<crate::List<stripe_types_14::payment_methods::PaymentMethod>, crate::Error> {
    let path = crate::params::format_path("/v1/payment_methods", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "payment_methods")]
#[doc = "Update a PaymentMethod"]
pub async fn update_payment_method(
    client: &crate::Client,
    payment_method: &str,
    params: &stripe_types_11::payment_methods::UpdatePaymentMethodParams,
) -> Result<stripe_types_14::payment_methods::PaymentMethod, crate::Error> {
    let path =
        crate::params::format_path("/v1/payment_methods/{payment_method}", &[("payment_method", payment_method)]);
    client.post_form(&path, params).await
}
