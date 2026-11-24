#[cfg(feature = "invoice_rendering_templates")]
#[derive(Debug, Clone, Default)]
pub struct ListInvoiceRenderingTemplatesParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_04::invoice_rendering_templates::ListInvoiceRenderingTemplatesParamsStatus>,
}

#[cfg(feature = "invoice_rendering_templates")]
impl crate::params::FormEncode for ListInvoiceRenderingTemplatesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}

#[cfg(feature = "invoice_rendering_templates")]
pub type UnarchiveInvoiceRenderingTemplateParams =
    stripe_types_01::invoice_rendering_templates::ArchiveInvoiceRenderingTemplateParams;
