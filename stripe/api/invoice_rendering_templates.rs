#[cfg(feature = "invoice_rendering_templates")]
#[doc = "Archive an invoice rendering template"]
pub async fn archive_invoice_rendering_template(
    client: &crate::Client,
    template: &str,
    params: &stripe_types_01::invoice_rendering_templates::ArchiveInvoiceRenderingTemplateParams,
) -> Result<stripe_types_06::invoice_rendering_templates::InvoiceRenderingTemplate, crate::Error> {
    let path =
        crate::params::format_path("/v1/invoice_rendering_templates/{template}/archive", &[("template", template)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "invoice_rendering_templates")]
#[doc = "Retrieve an invoice rendering template"]
pub async fn get_invoice_rendering_template(
    client: &crate::Client,
    template: &str,
    params: &stripe_types_04::invoice_rendering_templates::GetInvoiceRenderingTemplateParams,
) -> Result<stripe_types_06::invoice_rendering_templates::InvoiceRenderingTemplate, crate::Error> {
    let path = crate::params::format_path("/v1/invoice_rendering_templates/{template}", &[("template", template)]);
    client.get(&path, params).await
}

#[cfg(feature = "invoice_rendering_templates")]
#[doc = "List all invoice rendering templates"]
pub async fn list_invoice_rendering_templates(
    client: &crate::Client,
    params: &stripe_types_05::invoice_rendering_templates::ListInvoiceRenderingTemplatesParams,
) -> Result<crate::List<stripe_types_06::invoice_rendering_templates::InvoiceRenderingTemplate>, crate::Error> {
    let path = crate::params::format_path("/v1/invoice_rendering_templates", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "invoice_rendering_templates")]
#[doc = "Unarchive an invoice rendering template"]
pub async fn unarchive_invoice_rendering_template(
    client: &crate::Client,
    template: &str,
    params: &stripe_types_05::invoice_rendering_templates::UnarchiveInvoiceRenderingTemplateParams,
) -> Result<stripe_types_06::invoice_rendering_templates::InvoiceRenderingTemplate, crate::Error> {
    let path = crate::params::format_path(
        "/v1/invoice_rendering_templates/{template}/unarchive",
        &[("template", template)],
    );
    client.post_form(&path, params).await
}
