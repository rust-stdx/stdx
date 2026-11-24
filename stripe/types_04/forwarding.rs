#[cfg(feature = "forwarding")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateForwardingRequestParamsReplacementsItem {
    CardCvc,
    CardExpiry,
    CardNumber,
    CardholderName,
    RequestSignature,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "forwarding")]
impl CreateForwardingRequestParamsReplacementsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateForwardingRequestParamsReplacementsItem::CardCvc => "card_cvc",
            CreateForwardingRequestParamsReplacementsItem::CardExpiry => "card_expiry",
            CreateForwardingRequestParamsReplacementsItem::CardNumber => "card_number",
            CreateForwardingRequestParamsReplacementsItem::CardholderName => "cardholder_name",
            CreateForwardingRequestParamsReplacementsItem::RequestSignature => "request_signature",
            CreateForwardingRequestParamsReplacementsItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "forwarding")]
impl<'json> json::Decode<'json> for CreateForwardingRequestParamsReplacementsItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateForwardingRequestParamsReplacementsItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "card_cvc" => CreateForwardingRequestParamsReplacementsItem::CardCvc,
            "card_expiry" => CreateForwardingRequestParamsReplacementsItem::CardExpiry,
            "card_number" => CreateForwardingRequestParamsReplacementsItem::CardNumber,
            "cardholder_name" => CreateForwardingRequestParamsReplacementsItem::CardholderName,
            "request_signature" => CreateForwardingRequestParamsReplacementsItem::RequestSignature,
            other => CreateForwardingRequestParamsReplacementsItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "forwarding")]
impl crate::params::FormEncode for CreateForwardingRequestParamsReplacementsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "forwarding")]
#[derive(Debug, Clone, PartialEq)]
pub enum ForwardingRequestReplacementsItem {
    CardCvc,
    CardExpiry,
    CardNumber,
    CardholderName,
    RequestSignature,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "forwarding")]
impl ForwardingRequestReplacementsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ForwardingRequestReplacementsItem::CardCvc => "card_cvc",
            ForwardingRequestReplacementsItem::CardExpiry => "card_expiry",
            ForwardingRequestReplacementsItem::CardNumber => "card_number",
            ForwardingRequestReplacementsItem::CardholderName => "cardholder_name",
            ForwardingRequestReplacementsItem::RequestSignature => "request_signature",
            ForwardingRequestReplacementsItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "forwarding")]
impl<'json> json::Decode<'json> for ForwardingRequestReplacementsItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ForwardingRequestReplacementsItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "card_cvc" => ForwardingRequestReplacementsItem::CardCvc,
            "card_expiry" => ForwardingRequestReplacementsItem::CardExpiry,
            "card_number" => ForwardingRequestReplacementsItem::CardNumber,
            "cardholder_name" => ForwardingRequestReplacementsItem::CardholderName,
            "request_signature" => ForwardingRequestReplacementsItem::RequestSignature,
            other => ForwardingRequestReplacementsItem::Unknown(other.to_string()),
        })
    }
}
