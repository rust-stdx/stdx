#[cfg(feature = "invoices")]
#[doc = "Bulk add invoice line items"]
pub async fn add_lines(
    client: &crate::Client,
    invoice: &str,
    params: &stripe_types_12::invoices::AddLinesParams,
) -> Result<stripe_types_14::invoices::Invoice, crate::Error> {
    let path = crate::params::format_path("/v1/invoices/{invoice}/add_lines", &[("invoice", invoice)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "invoices")]
#[doc = "Attach a payment to an Invoice"]
pub async fn attach_payment(
    client: &crate::Client,
    invoice: &str,
    params: &stripe_types_01::invoices::AttachPaymentParams,
) -> Result<stripe_types_14::invoices::Invoice, crate::Error> {
    let path = crate::params::format_path("/v1/invoices/{invoice}/attach_payment", &[("invoice", invoice)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "invoices")]
#[doc = "Create an invoice"]
pub async fn create_invoice(
    client: &crate::Client,
    params: &stripe_types_13::invoices::CreateInvoiceParams,
) -> Result<stripe_types_14::invoices::Invoice, crate::Error> {
    let path = crate::params::format_path("/v1/invoices", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "invoices")]
#[doc = "Create a preview invoice"]
pub async fn create_preview(
    client: &crate::Client,
    params: &stripe_types_13::invoices::CreatePreviewParams,
) -> Result<stripe_types_14::invoices::Invoice, crate::Error> {
    let path = crate::params::format_path("/v1/invoices/create_preview", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "invoices")]
#[doc = "Delete a draft invoice"]
pub async fn delete_invoice(
    client: &crate::Client,
    invoice: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_05::invoices::DeletedInvoice, crate::Error> {
    let path = crate::params::format_path("/v1/invoices/{invoice}", &[("invoice", invoice)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "invoices")]
#[doc = "Finalize an invoice"]
pub async fn finalize_invoice(
    client: &crate::Client,
    invoice: &str,
    params: &stripe_types_01::invoices::FinalizeInvoiceParams,
) -> Result<stripe_types_14::invoices::Invoice, crate::Error> {
    let path = crate::params::format_path("/v1/invoices/{invoice}/finalize", &[("invoice", invoice)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "invoices")]
#[doc = "Retrieve an invoice"]
pub async fn get_invoice(
    client: &crate::Client,
    invoice: &str,
    params: &stripe_types_02::invoices::GetInvoiceParams,
) -> Result<stripe_types_14::invoices::Invoice, crate::Error> {
    let path = crate::params::format_path("/v1/invoices/{invoice}", &[("invoice", invoice)]);
    client.get(&path, params).await
}

#[cfg(feature = "invoices")]
#[doc = "Retrieve an invoice's line items"]
pub async fn list_invoice_lines(
    client: &crate::Client,
    invoice: &str,
    params: &stripe_types_04::invoices::ListInvoiceLinesParams,
) -> Result<crate::List<stripe_types_14::invoices::LineItem>, crate::Error> {
    let path = crate::params::format_path("/v1/invoices/{invoice}/lines", &[("invoice", invoice)]);
    client.get(&path, params).await
}

#[cfg(feature = "invoices")]
#[doc = "List all invoices"]
pub async fn list_invoices(
    client: &crate::Client,
    params: &stripe_types_09::invoices::ListInvoicesParams,
) -> Result<crate::List<stripe_types_14::invoices::Invoice>, crate::Error> {
    let path = crate::params::format_path("/v1/invoices", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "invoices")]
#[doc = "Mark an invoice as uncollectible"]
pub async fn mark_uncollectible(
    client: &crate::Client,
    invoice: &str,
    params: &stripe_types_06::invoices::MarkUncollectibleParams,
) -> Result<stripe_types_14::invoices::Invoice, crate::Error> {
    let path = crate::params::format_path("/v1/invoices/{invoice}/mark_uncollectible", &[("invoice", invoice)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "invoices")]
#[doc = "Pay an invoice"]
pub async fn pay_invoice(
    client: &crate::Client,
    invoice: &str,
    params: &stripe_types_03::invoices::PayInvoiceParams,
) -> Result<stripe_types_14::invoices::Invoice, crate::Error> {
    let path = crate::params::format_path("/v1/invoices/{invoice}/pay", &[("invoice", invoice)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "invoices")]
#[doc = "Bulk remove invoice line items"]
pub async fn remove_lines(
    client: &crate::Client,
    invoice: &str,
    params: &stripe_types_07::invoices::RemoveLinesParams,
) -> Result<stripe_types_14::invoices::Invoice, crate::Error> {
    let path = crate::params::format_path("/v1/invoices/{invoice}/remove_lines", &[("invoice", invoice)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "invoices")]
#[doc = "Search invoices"]
pub async fn search_invoices(
    client: &crate::Client,
    params: &stripe_types_04::invoices::SearchInvoicesParams,
) -> Result<crate::List<stripe_types_14::invoices::Invoice>, crate::Error> {
    let path = crate::params::format_path("/v1/invoices/search", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "invoices")]
#[doc = "Send an invoice for manual payment"]
pub async fn send_invoice(
    client: &crate::Client,
    invoice: &str,
    params: &stripe_types_06::invoices::SendInvoiceParams,
) -> Result<stripe_types_14::invoices::Invoice, crate::Error> {
    let path = crate::params::format_path("/v1/invoices/{invoice}/send", &[("invoice", invoice)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "invoices")]
#[doc = "Update an invoice"]
pub async fn update_invoice(
    client: &crate::Client,
    invoice: &str,
    params: &stripe_types_13::invoices::UpdateInvoiceParams,
) -> Result<stripe_types_14::invoices::Invoice, crate::Error> {
    let path = crate::params::format_path("/v1/invoices/{invoice}", &[("invoice", invoice)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "invoices")]
#[doc = "Update an invoice's line item"]
pub async fn update_invoice_line(
    client: &crate::Client,
    invoice: &str,
    line_item_id: &str,
    params: &stripe_types_12::invoices::UpdateInvoiceLineParams,
) -> Result<stripe_types_14::invoices::LineItem, crate::Error> {
    let path = crate::params::format_path(
        "/v1/invoices/{invoice}/lines/{line_item_id}",
        &[("invoice", invoice), ("line_item_id", line_item_id)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "invoices")]
#[doc = "Bulk update invoice line items"]
pub async fn update_lines(
    client: &crate::Client,
    invoice: &str,
    params: &stripe_types_13::invoices::UpdateLinesParams,
) -> Result<stripe_types_14::invoices::Invoice, crate::Error> {
    let path = crate::params::format_path("/v1/invoices/{invoice}/update_lines", &[("invoice", invoice)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "invoices")]
#[doc = "Void an invoice"]
pub async fn void_invoice(
    client: &crate::Client,
    invoice: &str,
    params: &stripe_types_05::invoices::VoidInvoiceParams,
) -> Result<stripe_types_14::invoices::Invoice, crate::Error> {
    let path = crate::params::format_path("/v1/invoices/{invoice}/void", &[("invoice", invoice)]);
    client.post_form(&path, params).await
}
