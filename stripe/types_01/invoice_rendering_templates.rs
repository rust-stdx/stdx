#[cfg(feature = "invoice_rendering_templates")]
#[derive(Debug, Clone, Default)]
pub struct ArchiveInvoiceRenderingTemplateParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "invoice_rendering_templates")]
impl crate::params::FormEncode for ArchiveInvoiceRenderingTemplateParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "invoice_rendering_templates")]
#[derive(Debug, Clone, PartialEq)]
pub enum InvoiceRenderingTemplateStatus {
    Active,
    Archived,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoice_rendering_templates")]
impl InvoiceRenderingTemplateStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoiceRenderingTemplateStatus::Active => "active",
            InvoiceRenderingTemplateStatus::Archived => "archived",
            InvoiceRenderingTemplateStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoice_rendering_templates")]
impl Default for InvoiceRenderingTemplateStatus {
    fn default() -> Self {
        InvoiceRenderingTemplateStatus::Unknown(String::new())
    }
}

#[cfg(feature = "invoice_rendering_templates")]
impl<'json> json::Decode<'json> for InvoiceRenderingTemplateStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoiceRenderingTemplateStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => InvoiceRenderingTemplateStatus::Active,
            "archived" => InvoiceRenderingTemplateStatus::Archived,
            other => InvoiceRenderingTemplateStatus::Unknown(other.to_string()),
        })
    }
}
