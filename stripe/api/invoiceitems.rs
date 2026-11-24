#[cfg(feature = "invoiceitems")]
#[doc = "Create an invoice item"]
pub async fn create_invoiceitem(
    client: &crate::Client,
    params: &stripe_types_07::invoiceitems::CreateInvoiceitemParams,
) -> Result<stripe_types_15::invoiceitems::Invoiceitem, crate::Error> {
    let path = crate::params::format_path("/v1/invoiceitems", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "invoiceitems")]
#[doc = "Delete an invoice item"]
pub async fn delete_invoiceitem(
    client: &crate::Client,
    invoiceitem: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_06::invoiceitems::DeletedInvoiceitem, crate::Error> {
    let path = crate::params::format_path("/v1/invoiceitems/{invoiceitem}", &[("invoiceitem", invoiceitem)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "invoiceitems")]
#[doc = "Retrieve an invoice item"]
pub async fn get_invoiceitem(
    client: &crate::Client,
    invoiceitem: &str,
    params: &stripe_types_02::invoiceitems::GetInvoiceitemParams,
) -> Result<stripe_types_15::invoiceitems::Invoiceitem, crate::Error> {
    let path = crate::params::format_path("/v1/invoiceitems/{invoiceitem}", &[("invoiceitem", invoiceitem)]);
    client.get(&path, params).await
}

#[cfg(feature = "invoiceitems")]
#[doc = "List all invoice items"]
pub async fn list_invoiceitems(
    client: &crate::Client,
    params: &stripe_types_06::invoiceitems::ListInvoiceitemsParams,
) -> Result<crate::List<stripe_types_15::invoiceitems::Invoiceitem>, crate::Error> {
    let path = crate::params::format_path("/v1/invoiceitems", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "invoiceitems")]
#[doc = "Update an invoice item"]
pub async fn update_invoiceitem(
    client: &crate::Client,
    invoiceitem: &str,
    params: &stripe_types_10::invoiceitems::UpdateInvoiceitemParams,
) -> Result<stripe_types_15::invoiceitems::Invoiceitem, crate::Error> {
    let path = crate::params::format_path("/v1/invoiceitems/{invoiceitem}", &[("invoiceitem", invoiceitem)]);
    client.post_form(&path, params).await
}
