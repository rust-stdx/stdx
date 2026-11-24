#[cfg(any(feature = "events", feature = "sigma"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ScheduledQueryRunObject {
    ScheduledQueryRun,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "sigma"))]
impl ScheduledQueryRunObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ScheduledQueryRunObject::ScheduledQueryRun => "scheduled_query_run",
            ScheduledQueryRunObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "sigma"))]
impl Default for ScheduledQueryRunObject {
    fn default() -> Self {
        ScheduledQueryRunObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "sigma"))]
impl<'json> json::Decode<'json> for ScheduledQueryRunObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ScheduledQueryRunObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "scheduled_query_run" => ScheduledQueryRunObject::ScheduledQueryRun,
            other => ScheduledQueryRunObject::Unknown(other.to_string()),
        })
    }
}
