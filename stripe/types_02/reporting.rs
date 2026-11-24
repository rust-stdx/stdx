#[cfg(any(feature = "events", feature = "reporting"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ReportingReportTypeObject {
    ReportingReportType,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "reporting"))]
impl ReportingReportTypeObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ReportingReportTypeObject::ReportingReportType => "reporting.report_type",
            ReportingReportTypeObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "reporting"))]
impl Default for ReportingReportTypeObject {
    fn default() -> Self {
        ReportingReportTypeObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "reporting"))]
impl<'json> json::Decode<'json> for ReportingReportTypeObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ReportingReportTypeObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "reporting.report_type" => ReportingReportTypeObject::ReportingReportType,
            other => ReportingReportTypeObject::Unknown(other.to_string()),
        })
    }
}
