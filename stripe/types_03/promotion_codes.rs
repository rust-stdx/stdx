#[cfg(feature = "promotion_codes")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePromotionCodeParamsPromotionType {
    Coupon,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "promotion_codes")]
impl CreatePromotionCodeParamsPromotionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePromotionCodeParamsPromotionType::Coupon => "coupon",
            CreatePromotionCodeParamsPromotionType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "promotion_codes")]
impl<'json> json::Decode<'json> for CreatePromotionCodeParamsPromotionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePromotionCodeParamsPromotionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "coupon" => CreatePromotionCodeParamsPromotionType::Coupon,
            other => CreatePromotionCodeParamsPromotionType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "promotion_codes")]
impl crate::params::FormEncode for CreatePromotionCodeParamsPromotionType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
