#[cfg(feature = "invoice_rendering_templates")]
#[derive(Debug, Clone, Default)]
pub struct GetInvoiceRenderingTemplateParams {
    pub expand: Option<Vec<String>>,
    pub version: Option<i64>,
}

#[cfg(feature = "invoice_rendering_templates")]
impl crate::params::FormEncode for GetInvoiceRenderingTemplateParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("version", self.version.as_ref());
    }
}

#[cfg(feature = "invoice_rendering_templates")]
#[derive(Debug, Clone, PartialEq)]
pub enum InvoiceRenderingTemplateObject {
    InvoiceRenderingTemplate,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoice_rendering_templates")]
impl InvoiceRenderingTemplateObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoiceRenderingTemplateObject::InvoiceRenderingTemplate => "invoice_rendering_template",
            InvoiceRenderingTemplateObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoice_rendering_templates")]
impl Default for InvoiceRenderingTemplateObject {
    fn default() -> Self {
        InvoiceRenderingTemplateObject::Unknown(String::new())
    }
}

#[cfg(feature = "invoice_rendering_templates")]
impl<'json> json::Decode<'json> for InvoiceRenderingTemplateObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoiceRenderingTemplateObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "invoice_rendering_template" => InvoiceRenderingTemplateObject::InvoiceRenderingTemplate,
            other => InvoiceRenderingTemplateObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoice_rendering_templates")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListInvoiceRenderingTemplatesParamsStatus {
    Active,
    Archived,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoice_rendering_templates")]
impl ListInvoiceRenderingTemplatesParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListInvoiceRenderingTemplatesParamsStatus::Active => "active",
            ListInvoiceRenderingTemplatesParamsStatus::Archived => "archived",
            ListInvoiceRenderingTemplatesParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoice_rendering_templates")]
impl<'json> json::Decode<'json> for ListInvoiceRenderingTemplatesParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListInvoiceRenderingTemplatesParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => ListInvoiceRenderingTemplatesParamsStatus::Active,
            "archived" => ListInvoiceRenderingTemplatesParamsStatus::Archived,
            other => ListInvoiceRenderingTemplatesParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoice_rendering_templates")]
impl crate::params::FormEncode for ListInvoiceRenderingTemplatesParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
