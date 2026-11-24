#[cfg(feature = "coupons")]
#[derive(Debug, Clone, Default)]
pub struct CreateCouponParamsCurrencyOptionsValue {
    #[doc = "A positive integer representing the amount to subtract from an invoice total."]
    pub amount_off: i64,
}

#[cfg(feature = "coupons")]
impl crate::params::FormEncode for CreateCouponParamsCurrencyOptionsValue {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount_off", &self.amount_off);
    }
}

#[cfg(feature = "coupons")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCouponParamsDuration {
    Forever,
    Once,
    Repeating,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "coupons")]
impl CreateCouponParamsDuration {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCouponParamsDuration::Forever => "forever",
            CreateCouponParamsDuration::Once => "once",
            CreateCouponParamsDuration::Repeating => "repeating",
            CreateCouponParamsDuration::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "coupons")]
impl<'json> json::Decode<'json> for CreateCouponParamsDuration {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCouponParamsDuration::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "forever" => CreateCouponParamsDuration::Forever,
            "once" => CreateCouponParamsDuration::Once,
            "repeating" => CreateCouponParamsDuration::Repeating,
            other => CreateCouponParamsDuration::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "coupons")]
impl crate::params::FormEncode for CreateCouponParamsDuration {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
