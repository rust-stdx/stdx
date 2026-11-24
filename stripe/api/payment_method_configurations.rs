#[cfg(feature = "payment_method_configurations")]
#[doc = "Create a payment method configuration"]
pub async fn create_payment_method_configuration(
    client: &crate::Client,
    params: &stripe_types_12::payment_method_configurations::CreatePaymentMethodConfigurationParams,
) -> Result<stripe_types_10::payment_method_configurations::PaymentMethodConfiguration, crate::Error> {
    let path = crate::params::format_path("/v1/payment_method_configurations", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_method_configurations")]
#[doc = "Retrieve payment method configuration"]
pub async fn get_payment_method_configuration(
    client: &crate::Client,
    configuration: &str,
    params: &stripe_types_02::payment_method_configurations::GetPaymentMethodConfigurationParams,
) -> Result<stripe_types_10::payment_method_configurations::PaymentMethodConfiguration, crate::Error> {
    let path = crate::params::format_path(
        "/v1/payment_method_configurations/{configuration}",
        &[("configuration", configuration)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "payment_method_configurations")]
#[doc = "List payment method configurations"]
pub async fn list_payment_method_configurations(
    client: &crate::Client,
    params: &stripe_types_00::payment_method_configurations::ListPaymentMethodConfigurationsParams,
) -> Result<crate::List<stripe_types_10::payment_method_configurations::PaymentMethodConfiguration>, crate::Error> {
    let path = crate::params::format_path("/v1/payment_method_configurations", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "payment_method_configurations")]
#[doc = "Update payment method configuration"]
pub async fn update_payment_method_configuration(
    client: &crate::Client,
    configuration: &str,
    params: &stripe_types_12::payment_method_configurations::UpdatePaymentMethodConfigurationParams,
) -> Result<stripe_types_10::payment_method_configurations::PaymentMethodConfiguration, crate::Error> {
    let path = crate::params::format_path(
        "/v1/payment_method_configurations/{configuration}",
        &[("configuration", configuration)],
    );
    client.post_form(&path, params).await
}
