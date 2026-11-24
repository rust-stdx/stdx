#[cfg(feature = "payment_links")]
#[doc = "Create a payment link"]
pub async fn create_payment_link(
    client: &crate::Client,
    params: &stripe_types_13::payment_links::CreatePaymentLinkParams,
) -> Result<stripe_types_15::payment_links::PaymentLink, crate::Error> {
    let path = crate::params::format_path("/v1/payment_links", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_links")]
#[doc = "Retrieve payment link"]
pub async fn get_payment_link(
    client: &crate::Client,
    payment_link: &str,
    params: &stripe_types_03::payment_links::GetPaymentLinkParams,
) -> Result<stripe_types_15::payment_links::PaymentLink, crate::Error> {
    let path = crate::params::format_path("/v1/payment_links/{payment_link}", &[("payment_link", payment_link)]);
    client.get(&path, params).await
}

#[cfg(feature = "payment_links")]
#[doc = "Retrieve a payment link's line items"]
pub async fn list_payment_link_line_items(
    client: &crate::Client,
    payment_link: &str,
    params: &stripe_types_03::payment_links::ListPaymentLinkLineItemsParams,
) -> Result<crate::List<stripe_types_15::common::Item>, crate::Error> {
    let path =
        crate::params::format_path("/v1/payment_links/{payment_link}/line_items", &[("payment_link", payment_link)]);
    client.get(&path, params).await
}

#[cfg(feature = "payment_links")]
#[doc = "List all payment links"]
pub async fn list_payment_links(
    client: &crate::Client,
    params: &stripe_types_04::payment_links::ListPaymentLinksParams,
) -> Result<crate::List<stripe_types_15::payment_links::PaymentLink>, crate::Error> {
    let path = crate::params::format_path("/v1/payment_links", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "payment_links")]
#[doc = "Update a payment link"]
pub async fn update_payment_link(
    client: &crate::Client,
    payment_link: &str,
    params: &stripe_types_13::payment_links::UpdatePaymentLinkParams,
) -> Result<stripe_types_15::payment_links::PaymentLink, crate::Error> {
    let path = crate::params::format_path("/v1/payment_links/{payment_link}", &[("payment_link", payment_link)]);
    client.post_form(&path, params).await
}
