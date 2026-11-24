#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline {
    #[doc = "The IP address from which the Mandate was accepted by the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The user agent of the browser from which the Mandate was accepted by the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    Always,
    Limited,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
impl ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::Always => "always",
            ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::Limited => "limited",
            ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::Unspecified => "unspecified",
            ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
impl<'json> json::Decode<'json> for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::Always,
            "limited" => ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::Limited,
            "unspecified" => ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::Unspecified,
            other => ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::Unknown(other.to_string()),
        })
    }
}
