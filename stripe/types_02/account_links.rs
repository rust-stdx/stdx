#[cfg(feature = "account_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum AccountLinkObject {
    AccountLink,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "account_links")]
impl AccountLinkObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            AccountLinkObject::AccountLink => "account_link",
            AccountLinkObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "account_links")]
impl Default for AccountLinkObject {
    fn default() -> Self {
        AccountLinkObject::Unknown(String::new())
    }
}

#[cfg(feature = "account_links")]
impl<'json> json::Decode<'json> for AccountLinkObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(AccountLinkObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_link" => AccountLinkObject::AccountLink,
            other => AccountLinkObject::Unknown(other.to_string()),
        })
    }
}
