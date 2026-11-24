#[cfg(feature = "invoice_payments")]
#[doc = "Retrieve an InvoicePayment"]
pub async fn get_invoice_payment(
    client: &crate::Client,
    invoice_payment: &str,
    params: &stripe_types_04::invoice_payments::GetInvoicePaymentParams,
) -> Result<stripe_types_14::invoice_payments::InvoicePayment, crate::Error> {
    let path = crate::params::format_path(
        "/v1/invoice_payments/{invoice_payment}",
        &[("invoice_payment", invoice_payment)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "invoice_payments")]
#[doc = "List all payments for an invoice"]
pub async fn list_invoice_payments(
    client: &crate::Client,
    params: &stripe_types_08::invoice_payments::ListInvoicePaymentsParams,
) -> Result<crate::List<stripe_types_14::invoice_payments::InvoicePayment>, crate::Error> {
    let path = crate::params::format_path("/v1/invoice_payments", &[]);
    client.get(&path, params).await
}
