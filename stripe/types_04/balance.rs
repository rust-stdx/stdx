#[cfg(any(feature = "balance", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BalanceAmountBySourceType {
    #[doc = "Amount coming from [legacy US ACH payments](https://docs.stripe.com/ach-deprecated)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<i64>,
    #[doc = "Amount coming from most payment methods, including cards as well as non-legacy bank debits..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<i64>,
    #[doc = "Amount coming from [FPX](https://docs.stripe.com/payments/fpx), a Malaysian payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fpx: Option<i64>,
}

#[cfg(any(feature = "balance", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BalanceObject {
    Balance,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "balance", feature = "events"))]
impl BalanceObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BalanceObject::Balance => "balance",
            BalanceObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "balance", feature = "events"))]
impl Default for BalanceObject {
    fn default() -> Self {
        BalanceObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "balance", feature = "events"))]
impl<'json> json::Decode<'json> for BalanceObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BalanceObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balance" => BalanceObject::Balance,
            other => BalanceObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "balance")]
#[derive(Debug, Clone, Default)]
pub struct GetBalanceParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "balance")]
impl crate::params::FormEncode for GetBalanceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
